use aws_sdk_iam::Client;
use aws_sdk_sts::Client as StsClient;
use tracing::{debug, error};

use std::env;

use crate::aws::constants::*;

use crate::aws::gitlab::{decode_token, generate_profile};
use crate::aws::utils::*;

pub struct StsMfaArgs {
    pub code: String,
    pub profile: Option<String>,
    pub creds_path: Option<String>,
    pub mfa: Option<String>,
}

/// Get sts credentials via mfa token and save as new profile.
pub async fn sts_via_mfa(args: StsMfaArgs) -> Result<String, String> {
    let profile: String = args
        .profile
        .or(env::var(AWS_PROFILE).ok())
        .expect("Either set env AWS_PROFILE or profile arg");

    let creds_path: String = args
        .creds_path
        .or(env::var(AWS_SHARED_CREDENTIALS_FILE).ok())
        .expect("Either set env AWS_SHARED_CREDENTIALS_FILE or creds_path arg");

    debug!(name = "Profile:", value = profile);
    debug!(name = "Creds path:", value = creds_path);

    let shared_config = get_config_from_profile(profile.clone()).await;
    let client = Client::new(&shared_config);
    let devices = client
        .list_mfa_devices()
        .send()
        .await
        .expect("Error fetching devices");

    if devices.mfa_devices.is_empty() {
        return Err("No MFA devices for the profile mentioned.".to_string());
    }

    let mut mfa_arn: String = "".to_string();

    if devices.mfa_devices.len() >= 2 {
        mfa_arn = args
            .mfa
            .expect("here are multiple ARNs please specify via mfa argument.")
    }

    if devices.mfa_devices.len() == 1 {
        mfa_arn.clone_from(&devices.mfa_devices.first().unwrap().serial_number);
    }

    let sts_client = StsClient::new(&shared_config);

    let result = sts_client
        .get_session_token()
        .serial_number(mfa_arn)
        .token_code(args.code)
        .duration_seconds(129600)
        .send()
        .await
        .expect("Error fetching new sts credentials");

    let new_credentials = result
        .credentials
        .expect("Expected new credentials but not found.");

    let new_profile = format!("{}_Temp", profile);

    let added = save_creds(new_credentials, new_profile.clone(), creds_path);
    match added {
        Ok(_data) => debug!("Data written successfully."),
        Err(_e) => error!(name = "Couldn't write to file: {}"),
    }

    Ok(new_profile)
}

pub enum IdTokenProvider {
    Gitlab,
}

pub struct StsIdTokenArgs {
    pub token: String,
    pub role_arn: Option<String>,
    pub creds_path: Option<String>,
    // pub provider: IdTokenProvider,
}

/// Get sts credentials via id token , can be usable in Pipelines
pub async fn sts_via_id_token(args: StsIdTokenArgs) -> Result<String, String> {
    let role_arn: Option<String> = args.role_arn.or(env::var(AWS_ROLE_ARN).ok());

    let shared_config = get_simple_config().await;
    let sts_client = StsClient::new(&shared_config);

    let token_data = decode_token(args.token.clone()).await;
    let new_profile = generate_profile(token_data.claims);

    let result = sts_client
        .assume_role_with_web_identity()
        .set_role_arn(role_arn)
        .web_identity_token(args.token)
        .role_session_name(new_profile.clone())
        .duration_seconds(3600)
        .send()
        .await
        .expect("Error fetching new sts credentials");

    let new_credentials = result
        .credentials
        .expect("Expected new credentials but not found.");

    let creds_path: String = args
        .creds_path
        .or(env::var(AWS_SHARED_CREDENTIALS_FILE).ok())
        .expect("Either set env AWS_SHARED_CREDENTIALS_FILE or creds_path arg");

    let added = save_creds(new_credentials, new_profile.clone(), creds_path);
    match added {
        Ok(_data) => debug!("Data written successfully."),
        Err(_e) => error!(name = "Couldn't write to file: {}"),
    }

    Ok(new_profile)
}

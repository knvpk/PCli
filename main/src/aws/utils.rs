use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_config::meta::region::RegionProviderChain;
use aws_config::SdkConfig;
use aws_sdk_sts::types::Credentials;
use aws_types::region::Region;
use ini::Ini;
use std::io::Result as IoResult;

pub async fn get_config_from_profile(profile: String) -> SdkConfig {
    let region = DefaultRegionChain::builder()
        .profile_name(&profile)
        .build()
        .region()
        .await;
    let creds = DefaultCredentialsChain::builder()
        .profile_name(&profile)
        .build()
        .await;
    aws_config::from_env()
        .credentials_provider(creds)
        .region(region)
        .load()
        .await
}

pub async fn get_simple_config() -> SdkConfig {
    let region = "us-west-2";
    let region_provider = RegionProviderChain::first_try(Region::new(region))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    aws_config::from_env().region(region_provider).load().await
}

// Save credentials to aws ini path with the profile name.
pub fn save_creds(creds: Credentials, profile: String, path: String) -> IoResult<()> {
    let cred_path_full = shellexpand::tilde(&path).to_string();

    let mut conf = Ini::load_from_file(cred_path_full.clone()).unwrap();

    conf.with_section(Some(profile))
        .set("aws_access_key_id", creds.access_key_id)
        .set("aws_secret_access_key", creds.secret_access_key)
        .set("aws_session_token", creds.session_token);

    conf.write_to_file(cred_path_full)
}

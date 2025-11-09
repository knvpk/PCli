use clap::Args;
use serde::{Deserialize, Serialize};
use tera::Context;
use valuable::Valuable;

use main::aws::sts::{sts_via_id_token, StsIdTokenArgs};

use crate::{args::Format, template};

#[derive(Debug, Args, Valuable, Clone)]
pub struct StsIdToken {
    #[arg(help = "ID token that is generated in pipeline")]
    token: String,

    #[arg(from_global)]
    debug: Option<bool>,

    #[arg(from_global)]
    format: Format,

    #[arg(
        short = 'r',
        long,
        help = "ARN of Role, checks in env `AWS_ROLE_ARN` if not provided"
    )]
    role_arn: Option<String>,

    #[arg(
        short = 'k',
        long = "cp",
        help = "AWS credentials path",
        default_value = "~/.aws/credentials"
    )]
    creds_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Valuable, Clone)]
struct StsIdTokenResult {
    profile: String,
}

pub async fn handle(cmd: StsIdToken) {
    let args = StsIdTokenArgs {
        token: cmd.token,
        role_arn: cmd.role_arn,
        creds_path: cmd.creds_path,
    };
    let result = sts_via_id_token(args).await;
    let data = StsIdTokenResult {
        profile: result.unwrap(),
    };

    if cmd.format == Format::Line {
        println!(
            "{}",
            template::TEMPLATES
                .render(
                    "sts_id_token.html",
                    &Context::from_serialize(&data).unwrap()
                )
                .unwrap()
        );
    }

    if cmd.format == Format::Json {
        println!("{}", serde_json::to_string(&data).unwrap());
    }
}

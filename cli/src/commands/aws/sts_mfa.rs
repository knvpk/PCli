use clap::Args;
use serde::{Deserialize, Serialize};
use tera::Context;
use valuable::Valuable;

use main::aws::sts::{sts_via_mfa, StsMfaArgs};

use crate::{args::Format, template};

#[derive(Debug, Args, Valuable, Clone)]
pub struct StsMfa {
    #[arg(help = "Code in authenticator APP.")]
    code: String,

    #[arg(from_global)]
    debug: Option<bool>,

    #[arg(from_global)]
    format: Format,

    #[arg(
        short = 'p',
        long,
        help = "AWS profile to use",
        default_value = "Vikalp"
    )]
    profile: Option<String>,

    #[arg(
        short = 'k',
        long = "cp",
        help = "AWS credentials path",
        default_value = "~/.aws/credentials"
    )]
    creds_path: Option<String>,

    //TODO:: how to get default action
    #[arg(
        short = 'm',
        long,
        help = "ARN of MFA you need to use, if not provided will auto detect based on profile."
    )]
    mfa: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Valuable, Clone)]
struct StsMfaResult {
    profile: String,
}

pub async fn handle(cmd: StsMfa) {
    let args = StsMfaArgs {
        code: cmd.code,
        profile: cmd.profile,
        creds_path: cmd.creds_path,
        mfa: cmd.mfa,
    };
    let result = sts_via_mfa(args).await;
    let data = StsMfaResult {
        profile: result.unwrap(),
    };

    if cmd.format == Format::Line {
        println!(
            "{}",
            template::TEMPLATES
                .render("sts_mfa.html", &Context::from_serialize(&data).unwrap())
                .unwrap()
        );
    }

    if cmd.format == Format::Json {
        println!("{}", serde_json::to_string(&data).unwrap());
    }
}

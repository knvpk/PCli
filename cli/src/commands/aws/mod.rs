use valuable::Valuable;

pub mod sts_id_token;
pub mod sts_mfa;

use clap::{Args, Subcommand};

#[derive(Debug, Args, Valuable, Clone)]
pub struct AwsCommand {
    #[clap(subcommand)]
    pub command: AwsSubcommand,
}

#[derive(Debug, Subcommand, Valuable, Clone)]
pub enum AwsSubcommand {
    #[command(about = "Get STS credentials from MFA code.")]
    STSMfa(sts_mfa::StsMfa),

    #[command(about = "Get STS credentials from ID token.")]
    STSIdToken(sts_id_token::StsIdToken),
}

pub async fn handle(cmd: AwsCommand) {
    match cmd.command {
        AwsSubcommand::STSMfa(data) => sts_mfa::handle(data).await,
        AwsSubcommand::STSIdToken(data) => sts_id_token::handle(data).await,
    }
}

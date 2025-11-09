mod args;
mod commands;
mod template;

use clap::Parser;

use args::{CommandArgs, EntityType};
use commands::aws::handle as handle_aws_command;
use commands::email::handle as handle_email_command;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;
//use valuable::Valuable;

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::minutely("logs", "minutely");
    let (writer, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_env_filter(EnvFilter::from_env("cli"))
        .with_max_level(Level::TRACE)
        .with_writer(writer)
        .with_ansi(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Starting up ...");

    let args = CommandArgs::parse();
    // debug!(value = args.as_value(), name = "Command Args");

    match args.entity_type {
        EntityType::Aws(data) => handle_aws_command(data).await,
        EntityType::Email(data) => handle_email_command(data),
    }
}

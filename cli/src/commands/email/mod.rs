use clap::Args;
//use tracing::debug;
use valuable::Valuable;

#[derive(Debug, Args, Valuable, Clone)]
pub struct EmailCommand {
    pub something: Option<String>,
}

pub fn handle(_cmd: EmailCommand) {
    //debug!(name = "Email Command", value = cmd.as_value())
}

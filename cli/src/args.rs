use crate::commands::{aws::*, email::*};
use clap::{value_parser, Parser, Subcommand, ValueEnum};
use valuable::Valuable;

#[derive(Parser, Debug, Valuable, Clone)]
#[clap(
    name = env!("NAME", "Package name env not mentioned."),
    bin_name = env!("SHORT_NAME", "Package short name env not mentioned."),
    author,
    version,
    about,
    long_about = "A console application for developers to deal with daily tasks."
)]
pub struct CommandArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,

    // default_missing_value not working
    #[arg(short = 'd', long, help = "Print debug data", default_value = "false", value_parser = value_parser!(bool), global = true, default_missing_value = "true")]
    debug: Option<bool>,

    #[arg(short = 'f', long, help = "output format", default_value_t = Format::Line, value_enum, global = true)]
    format: Format,
}

#[derive(Debug, Subcommand, Valuable, Clone)]
pub enum EntityType {
    #[command(about = "AWS related commands")]
    Aws(AwsCommand),

    #[command(about = "Email related commands")]
    Email(EmailCommand),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Valuable)]
pub enum Format {
    Line,
    Json,
}

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Configure {
        #[arg(short, long, value_name = "VALUE")]
        option1: Option<String>,
    },
    Start,
    Stop,
    Troubleshoot,
}

#[derive(Parser)]
#[command(name = "Your CLI Name")]
#[command(about = "A brief description of your CLI application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub fn available_commands() -> Vec<&'static str> {
    vec!["configure", "start", "stop", "troubleshoot"]
}


mod completer;

use clap::{Parser, Subcommand};
use rustyline::{Editor, Result};
use rustyline::error::ReadlineError;
use crate::completer::MyHelper;

#[derive(Subcommand)]
enum Commands {
    Configure {
        #[arg(short, long, value_name = "VALUE")]
        option1: Option<String>,
    },
    Start,
    Stop,
    Troubleshoot,
}

#[derive(clap::Parser)]
#[command(name = "Your CLI Name")]
#[command(about = "A brief description of your CLI application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let commands = vec!["configure", "start", "stop", "troubleshoot"];
    let helper = MyHelper { commands };

    // Initialize the Editor
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));

    // Load history if available
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let input = line.trim();
                if input == "q" {
                    println!("Exiting...");
                    break;
                }

                let args = input.split_whitespace().collect::<Vec<&str>>();
                let cli = match Cli::try_parse_from(&args) {
                    Ok(cli) => cli,
                    Err(err) => {
                        println!("Error: {}", err);
                        continue;
                    }
                };

                match &cli.command {
                    Commands::Configure { option1 } => {
                        if let Some(val) = option1 {
                            println!("Option 1 value: {}", val);
                            // Implement configuration logic here
                        } else {
                            println!("No value provided for option1.");
                        }
                    }
                    Commands::Start => {
                        // Handle start command
                        println!("Starting...");
                    }
                    Commands::Stop => {
                        // Handle stop command
                        println!("Stopping...");
                    }
                    Commands::Troubleshoot => {
                        // Handle troubleshoot command
                        println!("Troubleshooting...");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save history
    rl.save_history("history.txt")?;

    Ok(())
}

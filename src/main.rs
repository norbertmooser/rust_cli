mod completer;
mod commands;
mod command_handler;


use rustyline::{Editor, Result};
use rustyline::error::ReadlineError;
use crate::completer::MyHelper;
use crate::commands::{Cli, available_commands};
use crate::command_handler::handle_command;
use clap::Parser;

fn main() -> Result<()> {
    let commands = available_commands();
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

                handle_command(&cli.command);
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

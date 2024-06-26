use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::{Validator, ValidationResult, ValidationContext};
use rustyline::{Context, Helper};

pub struct MyHelper {
    pub commands: Vec<&'static str>,
}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> std::result::Result<(usize, Vec<Pair>), ReadlineError> {
        let mut pairs = Vec::new();
        for command in &self.commands {
            if command.starts_with(&line[..pos]) {
                pairs.push(Pair {
                    display: command.to_string(),
                    replacement: command.to_string(),
                });
            }
        }
        Ok((0, pairs)) // Return the starting position of the completion and the pairs
    }
}

impl Hinter for MyHelper {
    type Hint = String;
}

impl Highlighter for MyHelper {}

impl Validator for MyHelper {
    fn validate(&self, _: &mut ValidationContext<'_>) -> std::result::Result<ValidationResult, ReadlineError> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Helper for MyHelper {}

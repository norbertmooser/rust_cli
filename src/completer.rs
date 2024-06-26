use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::{Validator, ValidationResult, ValidationContext};
use rustyline::{Context, Helper};

/// A struct to assist with command-line completions, hints, highlighting, and validation.
pub struct MyHelper {
    pub commands: Vec<&'static str>,
}

impl Completer for MyHelper {
    type Candidate = Pair;

    /// Provides auto-completion for the commands.
    /// 
    /// # Arguments
    /// 
    /// * `line` - The current input line.
    /// * `pos` - The cursor position in the input line.
    /// * `_ctx` - The context of the completion.
    /// 
    /// # Returns
    /// 
    /// A tuple containing the start position for the completion and a vector of possible completions.
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
    /// Validates the input line.
    /// 
    /// # Arguments
    /// 
    /// * `_ctx` - The validation context.
    /// 
    /// # Returns
    /// 
    /// A validation result indicating the line is valid.
    fn validate(&self, _: &mut ValidationContext<'_>) -> std::result::Result<ValidationResult, ReadlineError> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Helper for MyHelper {}

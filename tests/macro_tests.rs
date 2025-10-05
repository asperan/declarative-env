use std::str::Utf8Error;
use std::{error::Error, process::Command};
use std::io::Error as IoError;

use hierrorchy::{error_leaf, error_node};
use string_sequence_tester::{Line, Sequence, SequenceTree};

error_node! {
    type TestExpansionError<CommandExecutionError, IoError, Utf8Error> = "expansion test failed"
}

#[error_leaf(format!("command execution failed: {}", self.stderr))]
struct CommandExecutionError {
    stderr: String,
}

fn check_expansion(test: &str, sequence: SequenceTree) -> Result<(), TestExpansionError> {
    let output = Command::new("cargo").arg("expand").arg("--test").arg(test).output()?;
    if output.status.success() {
        let stdout = std::str::from_utf8(&output.stdout)?;
        let expanded_lines: Vec<String> = stdout.lines().map(|it| it.trim().to_owned()).collect();
        if sequence.accept(&expanded_lines) {
            return Ok(());
        } else {
            panic!("sequence not found");
        }
    } else {
        let stderr = std::str::from_utf8(&output.stderr)?;
        return Err(CommandExecutionError { stderr: stderr.to_owned() }.into())
    }
}

#[test]
fn test_config_load() -> Result<(), TestExpansionError> {
    check_expansion("01-basic-load", SequenceTree::Sequence(Sequence::new(vec![Line::trimmed("struct MyConfig {"), Line::trimmed("TEST_VAR: u16,"), Line::trimmed("}")])))
}

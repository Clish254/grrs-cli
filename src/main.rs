use clap::Parser;

use anyhow::{Context, Result};

use grrs::find_matches;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short = 'p', long = "pattern")]
    pattern: String,
    /// The path to the file to read
    #[arg(short = 'f', long = "file")]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout())
}

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*; // Add methods on commands
    use assert_fs::prelude::*;
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command; // Run programs

    #[test]
    fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str("A test\nActual content\nMore content\nAnother test")?;

        let mut cmd = Command::cargo_bin("grrs")?;
        cmd.arg("--pattern")
            .arg("test")
            .arg("--file")
            .arg(file.path());
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("A test\nAnother test"));

        Ok(())
    }

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("grrs")?;

        cmd.arg("--pattern")
            .arg("foobar")
            .arg("--file")
            .arg("test/file/doesnt/exist");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Could not read file"));

        Ok(())
    }
}

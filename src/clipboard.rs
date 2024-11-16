use std::{
    env,
    io::Write,
    process::{Command, ExitStatus, Stdio},
};

use anyhow::{anyhow, Result};

pub struct Clipboard;

const COPY_COMMAND: &str = "IGR_COPY_COMMAND";

impl Clipboard {
    pub fn copy(s: String) -> Result<ExitStatus> {
        let mut child = Command::new(env::var(COPY_COMMAND).unwrap_or("pbcopy".to_string()))
            .stdin(Stdio::piped())
            .spawn()?;

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(s.as_bytes())
                .map_err(|_| anyhow!("Failed to write to stdin"))?;
        }

        child.wait().map_err(|e| anyhow!(e.to_string()))
    }
}

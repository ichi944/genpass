use std::io;
use std::process::{Command, Stdio};

/// Copy text to clipboard using platform-specific methods
pub fn copy_to_clipboard(text: &str) -> io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        copy_to_clipboard_macos(text)
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Clipboard support is only available on macOS",
        ))
    }
}

#[cfg(target_os = "macos")]
fn copy_to_clipboard_macos(text: &str) -> io::Result<()> {
    let mut child = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(text.as_bytes())?;
    }

    let status = child.wait()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "pbcopy command failed",
        ))
    }
}

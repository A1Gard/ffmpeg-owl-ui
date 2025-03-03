use std::process::Command;

pub fn mute(input: &str, output: &str) -> Result<(), std::io::Error> {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(input) // No need to quote here
        .arg("-c")
        .arg("copy")
        .arg("-an")
        .arg(output) // No need to quote here
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr),
        ))
    }
}

use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn mute(input: &str, output: &str) -> Result<(), std::io::Error> {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(input) // No need to quote here
        .arg("-c")
        .arg("copy")
        .arg("-an")
        .arg(output)
        .arg("-y") // No need to quote here
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

pub fn rotate(input: &str, output_file: &str, mode: &str) -> Result<(), io::Error> {
    let output = Command::new("mediainfo")
        .arg("--Output=Video;%FrameCount%")
        .arg(input)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr),
        ));
    }

    let total_frames = String::from_utf8_lossy(&output.stdout).trim().parse::<u64>().unwrap();
    println!("Total frames: {}", total_frames);

    let mut child = Command::new("ffmpeg")
        // .arg("-v")
        // .arg("warning")
        .arg("-hide_banner")
        // .arg("-stats")
        .arg("-i")
        .arg(input)
        .arg("-vf")
        .arg(mode)
        // .arg("-progress")
        // .arg("-")
        .arg("-loglevel")
        .arg("debug")
        .arg(output_file)
        .arg("-y")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let child_stderr = child.stderr.as_mut().expect("Unable to pipe stderr");
    let mut reader = BufReader::new(child_stderr);
    let mut buff = String::new();
    while reader.read_line(&mut buff).expect("Unable to read chunk") > 0 {
        // buff will contain only one line
        if buff.contains("frame=") {
            let frame_index = buff.find("frame=").unwrap();
            let frame_number = buff[frame_index + 6..].split_whitespace().next().unwrap();
            let current_frame = frame_number.trim().parse::<u64>().unwrap();
            let percentage = (current_frame as f64 / total_frames as f64) * 100.0;
            println!("current: {}, total: {}, {:.2}%", current_frame, total_frames, percentage);
        }
        buff.clear();
    }

    let output = child.wait_with_output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr),
        ))
    }
}

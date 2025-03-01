use std::fs;
use std::path::Path;
use std::process::Command;

pub fn install_font(font_path: &str) -> Result<(), String> {
    let font_file = Path::new(font_path);
    if !font_file.exists() {
        return Err(format!("Font file does not exist: {}", font_path));
    }

    let font_dir = match std::env::consts::OS {
        "linux" => {
            let home = std::env::var("HOME").map_err(|e| e.to_string())?;
            Path::new(&home).join(".local/share/fonts").to_string_lossy().to_string()
        }
        "macos" => {
            let home = std::env::var("HOME").map_err(|e| e.to_string())?;
            Path::new(&home).join("Library/Fonts").to_string_lossy().to_string()
        }
        "windows" => {
            "C:\\Windows\\Fonts".to_string()
        }
        _ => return Err("Unsupported OS".to_string()),
    };

    let destination = Path::new(&font_dir).join(font_file.file_name().unwrap());

    // Check if the font is already installed
    if destination.exists() {
        return Ok(());
    }

    fs::create_dir_all(&font_dir).map_err(|e| e.to_string())?;
    fs::copy(font_file, &destination).map_err(|e| e.to_string())?;

    if std::env::consts::OS == "linux" {
        Command::new("fc-cache")
            .arg("-f")
            .arg("-v")
            .output()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

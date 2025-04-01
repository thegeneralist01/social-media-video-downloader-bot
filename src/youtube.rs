use std::process::Command;

use anyhow::Result;
use teloxide::types::UserId;

pub async fn download(url: &str, user_id: UserId) -> Result<bool> {
    // The command I use:
    // yt-dlp "https://www.youtube.com/watch?v=$1" -o "$2" --merge-output-format mp4
    let cmd = Command::new("yt-dlp")
        .arg(url)
        .arg("-o")
        // .arg("%(title).90s.%(ext)s")
        .arg(format!("./downloads/{}.%(ext)s", user_id))
        .arg("--merge-output-format")
        .arg("mp4")
        .output()
        .expect("Failed to execute command");
    log::debug!("yt-dlp output: {:?}", cmd);
    Ok(cmd.status.success())
}

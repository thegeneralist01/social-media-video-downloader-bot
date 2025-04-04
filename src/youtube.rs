use std::process::Command;

use anyhow::Result;
use teloxide::types::UserId;

pub async fn download(url: &str, user_id: UserId) -> Result<bool> {
    let cmd = Command::new("yt-dlp")
        .arg(url)
        .arg("-o")
        .arg(format!("./downloads/{}.%(ext)s", user_id))
        .arg("--merge-output-format")
        .arg("mp4")
        .arg("--concurrent-fragments")
        .arg("12")
        .arg("--postprocessor-args")
        .arg("ffmpeg:-c:v libx264 -profile:v high -preset veryfast -b:v 4133k -bufsize 8266k -maxrate 4500k -r 60 -c:a aac -b:a 128k -threads 12")
        .output()
        .expect("Failed to execute command");

    log::debug!("yt-dlp output: {:?}", cmd);
    Ok(cmd.status.success())
}

use std::path::Path;

use anyhow::Result;
use social_media_video_downloader_bot::youtube::download;
use teloxide::{prelude::*, types::InputFile};

const URL_WHITELIST: [&str; 10] = [
    // HTTPS
    "https://www.youtube.com/",
    "https://youtu.be/",
    "https://www.youtube.com/shorts/",
    "https://youtube.com/shorts/",
    "https://x.com/",
    // HTTP
    "http://www.youtube.com/",
    "http://youtu.be/",
    "http://www.youtube.com/shorts/",
    "http://youtube.com/shorts/",
    "http://x.com/",
];

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting the bot");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            if text.to_lowercase() == "/start" {
                bot.send_message(msg.chat.id, "Send a social media video link to download it!\nCurrently, the following social medias are supported: YouTube, X (twitter).").await.ok();
            } else if URL_WHITELIST.iter().any(|url| text.starts_with(url)) {
                log::info!("Video downloaded");
                let prepping_msg = bot.send_message(msg.chat.id, "Prepping the video...").await;
                let user_id = msg.from.as_ref().map(|user| user.id).unwrap();
                let sent = download(text, user_id).await.unwrap();

                match sent {
                    true => {
                        let filename = format!("./downloads/{}.mp4", user_id);
                        let path = Path::new(&filename);
                        bot.send_video(msg.chat.id, InputFile::file(path)).await.ok();
                        if path.exists() {
                            std::fs::remove_file(path).unwrap();
                        }
                    },
                    false => {
                        bot.send_message(msg.chat.id, "Failed to download the video.\nPlease check if the link is valid.").await.ok();
                    },
                };
                if let Ok(prepping_msg) = prepping_msg {
                    bot.delete_message(msg.chat.id, prepping_msg.id).await.ok();
                }
            }
        };
        Ok(())
    })
    .await;

    Ok(())
}

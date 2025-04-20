# Social Media Video Downloader Bot

This is a minimalistic Telegram bot used to download videos from social media platforms.

It supports downloading from YouTube and X (twitter).

By default, the bot deletes the video link message after downloading. To prevent deletion, prefix the link with an exclamation mark ("!").

## Dependencies

- **yt-dlp**: Used for downloading videos from supported platforms.
- **ffmpeg**: Used for video preprocessing to ensure optimal quality and format.

*For setup, a `TELOXIDE_TOKEN` env variable (a Telegram bot's token) is needed. Could be placed in an `.env` file.*

## License

This project is licensed under the MIT License.


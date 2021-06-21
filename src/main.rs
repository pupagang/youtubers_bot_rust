mod config;
pub mod gdrive;
mod youtube;

use serenity::client::{Client, Context, EventHandler};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    utils::MessageBuilder,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        let _channel = match msg.channel_id.to_channel(&context).await {
            Ok(channel) => channel,
            Err(why) => {
                println!("Error getting channel: {:?}", why);

                return;
            }
        };

        if msg.content.contains("!youtube") {
            let video = youtube::DownloadVideo::new(msg.content.into());
            let video_url = video.download_video().await;
            let video_info = youtube::VideoInfos::new(video_url);

            let (a, _b) = youtube::VideoInfos::get_video_info(&video_info).await;
            let video_title = youtube::VideoInfos::get_video_title(&a).await;
            let video_uploader = youtube::VideoInfos::get_video_uploader(&a).await;

            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(format!(
                    " downloaded\n{}\nfrom {}",
                    &video_title, &video_uploader
                ))
                .build();
            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
async fn start_bot() {
    let app_config = config::Config::init();
    let token = &app_config.token;

    let auth_drive = gdrive::Drive::init(&app_config.persist_path).await;

    let drive_list = auth_drive.get_shared_drives().await;
    println!("{:?}", drive_list);
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
#[tokio::main]
async fn main() {
    start_bot().await;
}

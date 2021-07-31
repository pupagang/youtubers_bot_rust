mod config;
pub mod gdrive;
mod youtube;

use dotenv;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::utils::MessageBuilder;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
};
use std::env;

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
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
            let video_id = youtube::VideoInfos::get_video_id(&a).await;

            let bot = Upload::new(
                video_title.clone(),
                video_id.clone(),
                video_uploader.clone(),
            );
            let upload_url = bot.gdrive_upload().await;

            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(format!(
                    " downloaded\n{}\nfrom {}",
                    video_title, video_uploader
                ))
                .build();
            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

// Same error
// #[command]
// #[only_in(guilds)]
// async fn youtube(ctx: &Context, msg: &Message) -> CommandResult {
//     if msg.content.contains("!youtube") {
//         let video = youtube::DownloadVideo::new(msg.content.as_str().to_string());
//         let video_url = video.download_video().await;

//         let video_info = youtube::VideoInfos::new(video_url);
//         let (a, _b) = youtube::VideoInfos::get_video_info(&video_info).await;
//         let video_title = youtube::VideoInfos::get_video_title(&a).await;
//         let video_id = youtube::VideoInfos::get_video_id(&a).await;
//         let video_uploader = youtube::VideoInfos::get_video_uploader(&a).await;
//         let bot = Upload::new(
//             video_title.clone(),
//             video_id.clone(),
//             video_uploader.clone(),
//         );
//         let upload_url = bot.gdrive_upload().await;

//         let message = msg
//             .channel_id
//             .say(
//                 &ctx,
//                 format!(" downloaded\n{}\nfrom {}", &video_title, &video_uploader),
//             )
//             .await;
//         if let Err(e) = message {
//             println!("Error sending message: {}", e);
//         }
//     }
//     Ok(())
// }

pub struct Upload {
    video_title: String,
    video_id: String,
    video_uploader: String,
}

impl Upload {
    pub fn new(video_title: String, video_id: String, video_uploader: String) -> Upload {
        Upload {
            video_title: video_title,
            video_id: video_id,
            video_uploader: video_uploader,
        }
    }

    pub async fn gdrive_upload(self) {
        let file_path = format!("{}{}", self.video_id, ".mp4");
        let drive_root = env::var("drive_root").expect("Expected a token in the enviroment");
        let persist_path = env::var("persist_path").expect("Expected a token in the enviroment");
        let auth_drive = gdrive::Drive::init(&persist_path).await;
        auth_drive
            .upload_file(&file_path, &self.video_title, &drive_root)
            .await;
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    dotenv::dotenv().ok();

    let token = env::var("token").expect("Expected a token in the enviroment");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

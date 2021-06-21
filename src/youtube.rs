use rustube::{Stream, VideoInfo};

pub struct DownloadVideo {
    send_message: String,
}
pub struct VideoInfos {
    raw_link: String,
}

impl DownloadVideo {
    pub fn new(send_message: String) -> Self {
        Self { send_message }
    }
    pub async fn download_video(&self) -> String {
        let split: Vec<_> = self.send_message.split(" ").collect();
        for s in &split {
            if s.contains("https") {
                println!(
                    "downloaded video to {:?}",
                    rustube::download_best_quality(&s).await
                );
            }
        }
        (split[1]).to_string()
    }
}

impl VideoInfos {
    pub fn new(raw_link: String) -> Self {
        Self { raw_link }
    }
    pub async fn get_video_info(&self) -> (VideoInfo, Vec<Stream>) {
        let id = rustube::Id::from_raw(&self.raw_link).unwrap();
        let descrambler = rustube::VideoFetcher::from_id(id.into_owned())
            .unwrap()
            .fetch()
            .await
            .unwrap();
        descrambler.descramble().unwrap().into_parts()
    }

    pub async fn get_video_title(video_info: &VideoInfo) -> String {
        (*video_info.player_response.video_details.title).to_string()
    }

    pub async fn get_video_uploader(video_info: &VideoInfo) -> String {
        (*video_info.player_response.video_details.author).to_string()
    }

    pub async fn get_video_id(video_info: &VideoInfo) -> String {
        (*video_info.player_response.video_details.video_id).to_string()
    }
}

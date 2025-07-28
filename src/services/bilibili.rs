use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};
use reqwest::Client;

use crate::{
    errors::bilibili::BilibiliError,
    models::{
        response::{BilibiliApiResponse, VideoInfoResponse},
        video::{VideoId, VideoInfo},
    },
    opengraph::OpenGraph,
};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait BilibiliServiceTrait {
    async fn video_info(&self, video_id: &VideoId) -> Result<VideoInfo, BilibiliError>;
    async fn video_info_og(&self, video_id: &VideoId) -> Result<OpenGraph, BilibiliError>;
}

pub struct BilibiliService {
    client: Client,
}

impl BilibiliService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl BilibiliServiceTrait for BilibiliService {
    async fn video_info(&self, video_id: &VideoId) -> Result<VideoInfo, BilibiliError> {
        // https://api.bilibili.com/x/web-interface/wbi/view?bvid=BVabcdef
        // https://api.bilibili.com/x/web-interface/wbi/view?aid=AV12345
        let query = [match video_id {
            VideoId::Av(av_id) => ("aid", av_id.to_string()),
            VideoId::Bv(bv_id) => ("bvid", bv_id.to_string()),
        }];

        let response: BilibiliApiResponse<VideoInfoResponse> = self
            .client
            .get("https://api.bilibili.com/x/web-interface/wbi/view")
            .query(&query)
            .send()
            .await?
            .json()
            .await?;

        let Some(data) = response.data else {
            return Err(BilibiliError::Api(response.message));
        };

        let video_info = VideoInfo::new(
            data.title,
            data.desc,
            data.pubdate,
            data.ctime,
            Some(data.pic),
            data.duration,
            data.owner.name,
        );

        Ok(video_info)
    }

    async fn video_info_og(&self, video_id: &VideoId) -> Result<OpenGraph, BilibiliError> {
        let video_info = self.video_info(&video_id).await?;

        let mut og = OpenGraph::new(video_id.url(), video_info.title);
        og.site_name("Bilibili");
        og.r#type("video.episode");

        if let Some(description) = video_info.description {
            og.description(description);
        }
        if let Some(thumb) = video_info.thumbnail_url {
            og.image(thumb);
        }

        og.video_release_date(video_info.publish_time as u64);
        og.video_duration(video_info.duration as u64);

        Ok(og)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        models::video::{VideoId, VideoInfo},
        services::bilibili::{BilibiliServiceTrait, MockBilibiliServiceTrait},
    };

    #[tokio::test]
    async fn get_video_info() {
        let mut video_map = HashMap::new();
        let example_video_info = VideoInfo::new(
            "Example video 1",
            Some("The example video"),
            0,
            0,
            Some("https://example.com/thumbnail.png"),
            0,
            "example user",
        );
        video_map.insert(VideoId::Av(1), example_video_info.to_owned());

        let mut bilibili_service = MockBilibiliServiceTrait::new();
        bilibili_service
            .expect_video_info()
            .once()
            .returning(move |video_id| Ok(video_map.get(&video_id).unwrap().to_owned()));

        let res = bilibili_service.video_info(&VideoId::Av(1)).await.unwrap();
        assert_eq!(res, example_video_info);
    }
}

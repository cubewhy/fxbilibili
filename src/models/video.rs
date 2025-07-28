#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum VideoId {
    Av(i32),
    Bv(String),
}

impl VideoId {
    pub fn url(&self) -> String {
        let (id_type, id_str) = match self {
            VideoId::Av(id) => ("av", id.to_string()),
            VideoId::Bv(id) => ("BV", id.to_string()),
        };
        format!("https://www.bilibili.com/video/{id_type}{id_str}")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct VideoInfo {
    pub title: String,
    pub description: Option<String>,
    pub publish_time: i64,
    pub modified_time: i64,
    pub thumbnail_url: Option<String>,
    pub duration: i64,
    pub author_name: String,
}

impl VideoInfo {
    pub fn new(
        title: impl Into<String>,
        description: Option<impl Into<String>>,
        publish_time: i64,
        modified_time: i64,
        thumbnail_url: Option<impl Into<String>>,
        duration: i64,
        author_name: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.map(|s| s.into()),
            publish_time,
            modified_time,
            thumbnail_url: thumbnail_url.map(|s| s.into()),
            duration,
            author_name: author_name.into(),
        }
    }
}

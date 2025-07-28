use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct BilibiliApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub ttl: i32,
    pub data: Option<T>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfoResponse {
    pub bvid: String,
    pub aid: i64,
    pub videos: i64,
    pub pic: String,
    pub title: String,
    pub pubdate: i64,
    pub ctime: i64,
    pub desc: Option<String>,
    #[serde(rename = "desc_v2")]
    pub desc_v2: Option<Vec<DescV2>>,
    pub state: i64,
    pub duration: i64,
    pub owner: Owner,
    pub cid: i64,
    pub dimension: Dimension,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    pub width: i64,
    pub height: i64,
    pub rotate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescV2 {
    #[serde(rename = "raw_text")]
    pub raw_text: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "biz_id")]
    pub biz_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub mid: i64,
    pub name: String,
    pub face: String,
}

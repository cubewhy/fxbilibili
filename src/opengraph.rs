use std::time::{Duration, SystemTime};

use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct MetaNode {
    pub metadata_name: String,
    pub value: String,
}

impl MetaNode {
    pub fn new(metadata_name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            metadata_name: metadata_name.into(),
            value: value.into(),
        }
    }

    pub fn new_og(metadata_name: impl Into<String>, value: impl Into<String>) -> Self {
        MetaNode::new(format!("og:{}", metadata_name.into()), value)
    }

    pub fn new_twitter(metadata_name: impl Into<String>, value: impl Into<String>) -> Self {
        MetaNode::new(format!("twitter:{}", metadata_name.into()), value)
    }

    pub fn to_html(&self) -> String {
        format!(
            "<meta property=\"{}\" content=\"{}\" />",
            self.metadata_name, self.value
        )
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct OpenGraph {
    title: String,
    pub url: String,
    nodes: Vec<MetaNode>,
}

impl OpenGraph {
    pub fn new(url: impl Into<String>, title: impl Into<String>) -> Self {
        let title: String = title.into();
        Self {
            title: title.to_owned(),
            url: url.into(),
            nodes: vec![MetaNode::new("title", title)],
        }
    }

    pub fn r#type(&mut self, r#type: impl Into<String>) -> &mut Self {
        self.nodes.push(MetaNode::new("type", r#type.into()));
        self
    }

    pub fn image(&mut self, image: impl Into<String>) -> &mut Self {
        self.nodes.push(MetaNode::new("image", image.into()));
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.nodes
            .push(MetaNode::new("description", description.into()));
        self
    }

    pub fn site_name(&mut self, site_name: impl Into<String>) -> &mut Self {
        self.nodes
            .push(MetaNode::new("site_name", site_name.into()));
        self
    }

    pub fn video_release_date(&mut self, release_timestamp: u64) -> &mut Self {
        let iso8601_time: DateTime<Utc> = SystemTime::UNIX_EPOCH
            .checked_add(Duration::from_secs(release_timestamp))
            .unwrap()
            .into();

        self.nodes.push(MetaNode::new(
            "video:release_date",
            iso8601_time.to_string(),
        ));
        self
    }

    pub fn video_duration(&mut self, duration: u64) -> &mut Self {
        self.nodes
            .push(MetaNode::new("video:duration", duration.to_string()));
        self
    }

    pub fn into_html(self) -> String {
        // push title node
        let mut nodes = self.nodes;
        nodes.push(MetaNode::new_og("title", self.title.to_owned()));
        nodes.push(MetaNode::new("title", self.title.to_owned()));

        // push url node
        nodes.push(MetaNode::new_og("url", self.url.to_owned()));

        let metadata_lines: Vec<String> = nodes.iter().map(|node| node.to_html()).collect();

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="refresh" content="0; url={url}" />
    {metadata}
    <title>{title}</title>
</head>
<body>
    <h1>{title}</h1>
</body>
</html>"#,
            url = self.url,
            metadata = metadata_lines.join("\n"),
            title = self.title
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::opengraph::{MetaNode, OpenGraph};

    #[test]
    fn build_og_meta_node() {
        let node = MetaNode::new_og("title", "Example Site");
        assert_eq!(node.metadata_name, "og:title");
        assert_eq!(node.value, "Example Site");
        assert_eq!(
            node.to_html(),
            "<meta property=\"og:title\" content=\"Example Site\" />"
        );
    }

    #[test]
    fn build_meta_node() {
        let node = MetaNode::new("title", "Example Site");
        assert_eq!(node.metadata_name, "title");
        assert_eq!(node.value, "Example Site");
        assert_eq!(
            node.to_html(),
            "<meta property=\"title\" content=\"Example Site\" />"
        );
    }

    #[test]
    fn build_twitter_meta_node() {
        let node = MetaNode::new_twitter("title", "Example Site");
        assert_eq!(node.metadata_name, "twitter:title");
        assert_eq!(node.value, "Example Site");
        assert_eq!(
            node.to_html(),
            "<meta property=\"twitter:title\" content=\"Example Site\" />"
        );
    }

    #[test]
    fn build_opengraph() {
        let mut og = OpenGraph::new("https://example.com", "Example Site");
        og.description("The example site").site_name("Example Site");
        let html = og.into_html();
        assert_eq!(
            html,
            "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <meta http-equiv=\"refresh\" content=\"0; url=https://example.com\" />\n    <meta property=\"title\" content=\"Example Site\" />\n<meta property=\"description\" content=\"The example site\" />\n<meta property=\"site_name\" content=\"Example Site\" />\n<meta property=\"og:title\" content=\"Example Site\" />\n<meta property=\"title\" content=\"Example Site\" />\n<meta property=\"og:url\" content=\"https://example.com\" />\n    <title>Example Site</title>\n</head>\n<body>\n    <h1>Example Site</h1>\n</body>\n</html>"
        )
    }
}

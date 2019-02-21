use url::form_urlencoded;
pub struct Konachan {}

impl crate::engines::GelbooruEngine for Konachan {
    fn url(&self) -> &'static str {
        "https://konachan.net/post.xml?limit=100"
    }

    fn compose_request(&self, page: u32, tags: &str) -> String {
        form_urlencoded::Serializer::new(self.url().to_string())
            .append_pair("page", (page + 1).to_string().as_str())
            .append_pair("tags", tags)
            .finish()
            .to_string()
    }
}

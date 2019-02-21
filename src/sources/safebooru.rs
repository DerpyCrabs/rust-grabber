pub struct Safebooru {}

impl crate::engines::GelbooruEngine for Safebooru {
    fn url(&self) -> &'static str {
        "http://safebooru.org/index.php?page=dapi&s=post&q=index&limit=100"
    }

    fn file_url(&self, url: String) -> String {
        format!("https:{}", url)
    }
}

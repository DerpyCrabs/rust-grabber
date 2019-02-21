pub struct Rule34 {}

impl crate::engines::GelbooruEngine for Rule34 {
    fn url(&self) -> &'static str {
        "https://rule34.xxx/index.php?page=dapi&s=post&q=index&limit=100"
    }
}

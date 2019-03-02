mod engines;
pub mod sources;

pub type Result<T> = std::result::Result<T, failure::Error>;

pub trait Source {
    fn get_links(
        &self,
        tags: &str,
        latest: Option<u32>,
    ) -> crate::Result<(Vec<String>, Option<u32>)>;
}

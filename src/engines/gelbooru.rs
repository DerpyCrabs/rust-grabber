use reqwest;
use serde_derive::Deserialize;
use serde_xml_rs::deserialize;
use url::form_urlencoded;

#[derive(Debug, Deserialize)]
struct Post {
    pub id: String,
    pub file_url: String,
}

#[derive(Debug, Deserialize)]
struct Posts {
    #[serde(rename = "post", default)]
    pub posts: Vec<Post>,
}

pub trait GelbooruEngine: crate::Source {
    fn url(&self) -> &'static str;

    fn compose_request(&self, page: u32, tags: &str) -> String {
        form_urlencoded::Serializer::new(self.url().to_string())
            .append_pair("pid", page.to_string().as_str())
            .append_pair("tags", tags)
            .finish()
            .to_string()
    }

    fn file_url(&self, url: String) -> String {
        url
    }
}

impl<T: GelbooruEngine> crate::Source for T {
    fn get_links(
        &self,
        tags: &str,
        latest: Option<u32>,
    ) -> crate::Result<(Vec<String>, Option<u32>)> {
        let mut urls: Vec<String> = Vec::new();
        let mut page = 0;
        let mut last_id: Option<u32> = None;

        loop {
            let body = reqwest::get((&self).compose_request(page, tags).as_str())?.text()?;

            let Posts { posts } = deserialize(body.as_bytes())?;
            if posts.len() == 0 {
                return Ok((urls, last_id));
            }

            for post in posts.into_iter() {
                if let Some(latest_id) = latest {
                    if post.id.parse::<u32>()? <= latest_id {
                        return Ok((urls, last_id));
                    }
                }
                match last_id {
                    None => last_id = Some(post.id.parse()?),
                    _ => (),
                }
                urls.push(self.file_url(post.file_url));
            }
            page += 1;
        }
    }
}

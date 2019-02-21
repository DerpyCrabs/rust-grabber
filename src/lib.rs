#[macro_use]
extern crate error_chain;

mod engines;
pub mod sources;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
        Parse(::std::num::ParseIntError);
        Reqwest(::reqwest::Error);
        Serde(::serde_xml_rs::Error);
    }
}

pub trait Source {
    fn get_links(
        &self,
        tags: &str,
        latest: Option<u32>,
    ) -> crate::Result<(Vec<String>, Option<u32>)>;
}

use booru_grabber::{sources, Result, Source};
use clap::{App, Arg};
use std::fs;
use std::path::Path;
use std::process;

fn read_latest_id(filename: &str) -> Result<u32> {
    fs::read_to_string(filename)?
        .trim()
        .parse::<u32>()
        .map_err(|e| e.into())
}

fn write_latest_id(filename: &str, latest: u32) -> Result<()> {
    fs::write(filename, latest.to_string()).map_err(|e| e.into())
}

fn get_latest_id(latest_file: Option<&str>) -> Option<u32> {
    match latest_file {
        Some(latest_file) => match Path::new(latest_file).exists() {
            true => Some(read_latest_id(latest_file).expect("Failed to read latest id")),
            false => None,
        },
        None => None,
    }
}

fn main() {
    let matches = App::new("Picture grabber")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("source")
                .long("source")
                .short("s")
                .help("pictures source")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("latest")
                .long("latest")
                .short("l")
                .help("latest downloaded id")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tags")
                .long("tags")
                .short("t")
                .help("tags to look for")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("latest-file")
                .long("latest-file")
                .short("f")
                .help("where to save latest downloaded id")
                .takes_value(true),
        )
        .get_matches();

    let source: Box<Source> = match matches.value_of("source") {
        Some("safebooru") => Box::new(sources::Safebooru {}),
        Some("konachan") => Box::new(sources::Konachan {}),
        Some("rule34") => Box::new(sources::Rule34 {}),
        Some("yandere") => Box::new(sources::Yandere {}),
        _ => {
            println!("Invalid source");
            process::exit(1);
        }
    };

    let latest_file = matches.value_of("latest-file");

    let tags = matches.value_of("tags").unwrap_or("");

    let latest = match matches.value_of("latest") {
        Some(id) => Some(id.parse().expect("Invalid latest id")),
        None => get_latest_id(latest_file),
    };

    source
        .get_links(tags, latest)
        .map(|(posts, last_id)| {
            if matches.is_present("latest-file") {
                if let Some(id) = last_id {
                    write_latest_id(latest_file.unwrap(), id).expect("Failed to write latest id");
                }
            }
            posts
        })
        .expect("Failed to get picture links")
        .iter()
        .for_each(|link| println!("{}", link))
}

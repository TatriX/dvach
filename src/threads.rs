use colored::*;
use scraper::Html;
use serde_derive::Deserialize;
use std::io::Read;
use textwrap::{fill, indent};

/// Print all available threads for the board.
pub fn list_threads(board: &str, comment_width: usize) {
    let url = format!("https://2ch.hk/{}/catalog.json", board);
    let response = reqwest::get(&url).expect(&format!("Cannot get threads for {}", board));
    let threads = parse_threads(response).expect("Cannot parse threads");

    for thread in threads {
        println!(
            "{} {}\n{}",
            format!("{}", thread.id).blue(),
            thread.subject,
            indent(
                &fill(&parse_thread_comment(&thread.comment), comment_width),
                "  "
            ),
        );
    }
}

/// Parse comment from html and return it's first line.
fn parse_thread_comment(comment: &str) -> String {
    let fragment = Html::parse_fragment(comment);
    fragment
        .root_element()
        .text()
        .next()
        .map(Into::into)
        .unwrap_or_else(String::new)
}

/// Parse threads from JSON API response.
fn parse_threads(reader: impl Read) -> serde_json::Result<Vec<Thread>> {
    /// Thread list response
    #[derive(Deserialize, Debug)]
    struct Threads {
        threads: Vec<Thread>,
    }

    let wrapper: Threads = serde_json::from_reader(reader)?;
    Ok(wrapper.threads)
}

/// Thread from the list of threads
#[derive(Deserialize, Debug)]
struct Thread {
    /// Thread id
    #[serde(rename = "num")]
    id: String,

    /// Thread subject
    subject: String,

    /// Beginning of the first threads post
    comment: String,
}

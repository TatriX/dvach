//! dvach is a simple client cli tool for the 2ch.hk imageboard.
//!
//! Usage: dvach [board] [thread]
//! ```
//! $ dvach # list boards
//! $ dvach pr # list threads for the "pr" board
//! $ dvach pr 1299618 # show selected thread
//! ```

use colored::*;
use env_logger;
use log::debug;
use reqwest;
use scraper::Html;
use serde_derive::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use structopt::StructOpt;
use textwrap::{fill, indent};

/// Represent available cli args
#[derive(StructOpt, Debug)]
struct Cli {
    /// Board to list
    board: Option<String>,

    /// Thread to show
    thread: Option<usize>,

    /// Width of the comment in posts before wrapping
    #[structopt(short = "w", long = "comment-width", default_value = "80")]
    comment_width: usize,
}

fn main() {
    env_logger::init();

    let args = Cli::from_args();
    debug!("Got args: {:#?}", &args);

    // run appropriate action based on cli arguments
    match (args.board, args.thread) {
        (None, None) => list_boards(),
        (Some(board), None) => list_threads(&board, args.comment_width),
        (Some(board), Some(thread)) => list_thread(&board, thread, args.comment_width),
        _ => Cli::clap().print_help().expect("Cannot print help"),
    }
}

/// Print all available boards.
///
/// This function uses "mobile" API, because there is no "list boards"
/// functionality in the JSON API. At least I haven't found one.
fn list_boards() {
    // this is hardcoded for now
    const URL: &str = "https://2ch.hk/makaba/mobile.fcgi?task=get_boards";
    let response = reqwest::get(URL).expect("Cannot get boards");
    let boards = parse_boards(response).expect("Cannot parse boards");

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for board in boards {
        // if output was interrupted, e.g. by piping to `head`, ignore the error
        let _ = writeln!(
            handle,
            "{:>10} {:20} {}",
            board.id, board.category, board.name
        );
    }
}

/// Parse boards from JSON API response.
fn parse_boards(reader: impl Read) -> serde_json::Result<Vec<Board>> {
    let wrapper: HashMap<String, Vec<Board>> = serde_json::from_reader(reader)?;
    Ok(wrapper
        .into_iter()
        .map(|(_, boards)| boards)
        .flatten()
        .collect())
}

#[derive(Deserialize, Debug)]
struct Board {
    /// Board id, like "pr" or "b".
    id: String,

    /// Board's category
    category: String,

    /// Name of the board
    name: String,
}

/// Print all available threads for the board.
fn list_threads(board: &str, comment_width: usize) {
    let url = format!("https://2ch.hk/{}/catalog.json", board);
    let response = reqwest::get(&url).expect(&format!("Cannot get threads for {}", board));
    let threads = parse_threads(response).expect("Cannot parse threads");

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for thread in threads {
        // if output was interrupted, e.g. by piping to `head`, ignore the error
        let _ = writeln!(
            handle,
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

/// Print all messages in particular thread.
fn list_thread(board: &str, thread: usize, comment_width: usize) {
    let url = format!("https://2ch.hk/{}/res/{}.json", board, thread);
    let response = reqwest::get(&url).expect(&format!("Cannot get thread {}/{}", board, thread));
    let posts = parse_posts(response).expect("Cannot parse posts");

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for post in posts {
        // if output was interrupted, e.g. by piping to `head`, ignore the error
        let _ = writeln!(
            handle,
            "{} {}\n{}",
            format!("{}", post.id).blue(),
            post.date.green(),
            indent(&fill(&parse_comment(&post.comment), comment_width), "  "),
        );
    }
}

/// Parse posts's comment from html and return lines joined with newline
fn parse_comment(comment: &str) -> String {
    let fragment = Html::parse_fragment(comment);
    fragment
        .root_element()
        .text()
        .collect::<Vec<_>>()
        .join("\n")
}

/// Parse posts from JSON API response
fn parse_posts(reader: impl Read) -> serde_json::Result<Vec<Post>> {
    /// Posts list response
    #[derive(Deserialize)]
    struct Posts {
        threads: Vec<Threads>,
    }

    /// Actual posts wrapper
    #[derive(Deserialize)]
    struct Threads {
        posts: Vec<Post>,
    }

    let wrapper: Posts = serde_json::from_reader(reader)?;
    // Here I'm expecting threads[0] to be always present. It will panic otherwise.
    Ok(wrapper
        .threads
        .into_iter()
        .next()
        .expect("threads must be present")
        .posts)
}

#[derive(Deserialize)]
struct Post {
    #[serde(rename = "num")]
    id: usize,

    /// Post content
    comment: String,

    /// Post date string
    date: String,
}

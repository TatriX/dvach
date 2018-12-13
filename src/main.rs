//! dvach is a simple client cli tool for the 2ch.hk imageboard.
//!
//! Usage: dvach [board] [thread]
//! ```
//! $ dvach # list boards
//! $ dvach pr # list threads for the "pr" board
//! $ dvach pr 1299618 # show selected thread
//! ```

use reqwest;
use serde_derive::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::io::Read;
use structopt::StructOpt;

/// Represent available cli args
#[derive(StructOpt, Debug)]
struct Cli {
    /// Board to list
    board: Option<String>,

    /// Thread to show
    thread: Option<usize>,
}

fn main() {
    let args = Cli::from_args();
    println!("Got args: {:#?}", &args);

    // run appropriate action based on cli arguments
    match (args.board, args.thread) {
        (None, None) => list_boards(),
        (Some(board), None) => list_threads(&board),
        (Some(board), Some(thread)) => list_thread(&board, thread),
        _ => Cli::clap().print_help().expect("Cannot print help"),
    }
}

/// Print all available boards.
///
/// This function uses "mobile" API, because there is no "list boards"
/// functionality in the "json" API. At least I didn't find one.
fn list_boards() {
    // this is hardcoded for now
    const URL: &str = "https://2ch.hk/makaba/mobile.fcgi?task=get_boards";
    let response = reqwest::get(URL).expect("Cannot get boards");
    let boards = parse_boards(response).expect("Cannot parse boards");

    for board in boards {
        println!("{:>10} {:20} {} ", board.id, board.category, board.name);
    }
}

fn parse_boards(reader: impl Read) -> serde_json::Result<Vec<Board>> {
    let wrapper: HashMap<String, Vec<Board>> = serde_json::from_reader(reader)?;
    Ok(wrapper.into_iter().map(|(_, boards)| boards).flatten().collect())
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
fn list_threads(board: &str) {}

/// Print all messages in particular thread.
fn list_thread(board: &str, thread: usize) {}

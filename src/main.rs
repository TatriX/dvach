//! dvach is a simple client cli tool for the 2ch.hk imageboard.
//!
//! Usage: dvach [board] [thread]
//! ```
//! $ dvach # list boards
//! $ dvach pr # list threads for the "pr" board
//! $ dvach pr 1299618 # show selected thread
//! ```

use structopt::StructOpt;

/// Represent available cli args
#[derive(StructOpt, Debug)]
struct Cli {
    /// Board to list
    board: Option<String>,

    /// Thread to show
    thread: Option<usize>
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

}

/// Print all available threads for the board.
fn list_threads(board: &str) {
}

/// Print all messages in particular thread.
fn list_thread(board: &str, thread: usize) {
}

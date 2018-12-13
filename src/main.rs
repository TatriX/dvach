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
#[derive(StructOpt)]
struct Cli {
    /// Board to list
    board: Option<String>,

    /// Thread to show
    thread: Option<usize>
}

fn main() {
    let args = Cli::from_args();
    println!("{:?} {:?}", args.board, args.thread);
}

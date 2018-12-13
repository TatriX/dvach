//! dvach is a simple client cli tool for the 2ch.hk imageboard.
//!
//! Usage: dvach [board] [thread]
//! ```
//! $ dvach # list boards
//! $ dvach pr # list threads for the "pr" board
//! $ dvach pr 1299618 # show selected thread
//! ```

use env_logger;
use log::debug;
use reqwest;
use serde_json;
use structopt::StructOpt;

/// Custom println! version which exits cleanly when output was
/// interrupted, e.g. by piping to `head`.
macro_rules! println {
    ($fmt:expr, $($args:tt)*) => {
        {
            use std::io::{self, Write};
            use std::process::exit;

            let stdout = io::stdout();
            let mut handle = stdout.lock();
            if writeln!(handle, $fmt, $($args)*).is_err() {
                exit(1);
            }
        }
    }
}

mod boards;
mod posts;
mod threads;
mod download;

use self::boards::list_boards;
use self::posts::list_posts;
use self::threads::list_threads;
use self::download::download;

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

    /// Default imageboard url
    #[structopt(long = "base-url", default_value = "https://2ch.hk")]
    base_url: String,

    /// Width of the comment in posts before wrapping
    #[structopt(long = "download")]
    download: Option<String>,
}

fn main() {
    env_logger::init();

    let args = Cli::from_args();
    debug!("Got args: {:#?}", &args);

    // get thumb or full image
    if let Some(path) = args.download {
        download(&args.base_url, &path);
        return
    }

    // run appropriate list action based on cli arguments
    match (args.board, args.thread) {
        (None, None) => list_boards(),
        (Some(board), None) => list_threads(&board, args.comment_width),
        (Some(board), Some(thread)) => list_posts(&board, thread, args.comment_width),
        _ => Cli::clap().print_help().expect("Cannot print help"),
    }
}

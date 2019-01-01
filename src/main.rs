//! dvach is a simple client cli tool for the 2ch.hk imageboard.
//!
//! Usage: dvach [board] [thread]
//! ```
//! $ dvach # run tui
//!
//! $ dvach . # list boards
//! $ dvach pr # list threads for the "pr" board
//! $ dvach pr 1299618 # show selected thread
//! ```

use cursive::event::Key;
use cursive::traits::*;
use cursive::view::View;
use cursive::views::{Dialog, LinearLayout, OnEventView, Panel, SelectView, TextView};
use cursive::Cursive;
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
            use std::io::{stdout, Write};
            use std::process::exit;

            if writeln!(&mut stdout(), $fmt, $($args)*).is_err() {
                exit(1);
            }
        }
    }
}

mod boards;
mod download;
mod posts;
mod threads;

use self::boards::{get_boards, list_boards};
use self::download::download;
use self::posts::*;
use self::threads::*;

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
        return;
    }

    // run appropriate list action based on cli arguments
    match (args.board, args.thread) {
        (None, None) => init_tui(),
        (Some(ref board), None) if board == "." => list_boards(),
        (Some(board), None) => list_threads(&board, args.comment_width),
        (Some(board), Some(thread)) => list_posts(&board, thread, args.comment_width),
        _ => Cli::clap().print_help().expect("Cannot print help"),
    }
}

fn init_tui() {
    let mut siv = Cursive::default();
    siv.add_layer(
        Dialog::around(boards_view())
            .title("Добро пожаловать. Снова.")
            .button("Выйти", |s| s.quit()),
    );
    siv.run();
}

fn boards_view() -> impl View {
    let mut select = SelectView::new()
        .autojump()
        .on_submit(|s: &mut Cursive, board: &String| s.add_layer(threads_view(board.to_owned())));

    let mut board_ids = get_boards()
        .into_iter()
        .map(|board| board.id)
        .collect::<Vec<_>>();
    board_ids.sort();
    select.add_all_str(board_ids);

    select.scrollable().fixed_size((10, 7))
}

fn threads_view(board: String) -> impl View {
    let mut threads = get_threads(&board);
    threads.sort_by(|a, b| a.id.cmp(&b.id));

    let title = format!("/{}/", &board);

    let select = SelectView::new()
        .autojump()
        .on_submit(move |s: &mut Cursive, thread: &String| {
            s.add_layer(posts_view(
                board.clone(),
                thread.parse().expect("Cannot parse post id"),
            ))
        })
        .with_all(threads.into_iter().map(|board| {
            (
                format!("{} {}", board.id, parse_thread_comment(&board.comment)),
                board.id,
            )
        }));

    Dialog::around(
        OnEventView::new(select.scrollable()).on_event(Key::Esc, |s| {
            s.pop_layer();
        }),
    )
    .title(title)
    .button("Выйти", |s| s.quit())
}

fn posts_view(board: String, thread: usize) -> impl View {
    let mut layout = LinearLayout::vertical();
    for post in get_posts(&board, thread) {
        layout.add_child(Panel::new(TextView::new(
            parse_comment(&post.comment).replace("\n", "  "),
        )));
    }

    Dialog::around(
        OnEventView::new(layout.scrollable()).on_event(Key::Esc, |s| {
            s.pop_layer();
        }),
    )
    .title(format!("/{}/ #{}", &board, thread))
    .button("Выйти", |s| s.quit())
}

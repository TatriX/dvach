use cursive::event::Key;
use cursive::traits::*;
use cursive::view::View;
use cursive::views::{
    Dialog, EditView, LinearLayout, OnEventView, Panel, SelectView, TextArea, TextView,
};
use cursive::Cursive;

use crate::boards::get_boards;
use crate::posts::{get_posts, parse_comment};
use crate::threads::{get_threads, parse_thread_comment};

pub fn init_tui() {
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

    board_ids
        .iter()
        .for_each(|x| select.add_item_str(x.as_str()));

    LinearLayout::vertical()
        .child(Panel::new(EditView::new().on_edit_mut(
            move |s: &mut Cursive, value: &str, _| {
                s.call_on_id("boards", |select: &mut SelectView| {
                    select.clear();
                    board_ids
                        .iter()
                        .filter(|x| x.starts_with(value))
                        .for_each(|x| select.add_item_str(x.as_str()));
                });
            },
        )))
        .child(select.with_id("boards").scrollable().fixed_size((10, 7)))
}

fn threads_view(board: String) -> impl View {
    let mut threads = get_threads(&board)
        .into_iter()
        .map(|thread| {
            (
                format!("{} {}", thread.id, parse_thread_comment(&thread.comment)),
                thread.id.clone(),
            )
        })
        .collect::<Vec<_>>();
    threads.sort_by(|a, b| a.1.cmp(&b.1));

    let title = format!("/{}/", &board);

    let mut select =
        SelectView::new()
            .autojump()
            .on_submit(move |s: &mut Cursive, thread: &String| {
                s.add_layer(posts_view(
                    board.clone(),
                    thread.parse().expect("Cannot parse post id"),
                ))
            });

    threads
        .iter()
        .cloned()
        .for_each(|(a, b)| select.add_item(a, b));

    Dialog::around(
        OnEventView::new(
            LinearLayout::vertical()
                .child(Panel::new(EditView::new().on_edit_mut(
                    move |s: &mut Cursive, value: &str, _| {
                        let needle = value.to_lowercase();
                        s.call_on_id("threads", |view: &mut SelectView| {
                            view.clear();
                            threads
                                .iter()
                                .filter(|(content, _)| content.to_lowercase().contains(&needle))
                                .cloned()
                                .for_each(|(a, b)| view.add_item(a, b));
                        });
                    },
                )))
                .child(select.with_id("threads").scrollable()),
        )
        .on_event(Key::Esc, |s| {
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

    layout.add_child(TextArea::new());

    Dialog::around(
        OnEventView::new(layout.scrollable()).on_event(Key::Esc, |s| {
            s.pop_layer();
        }),
    )
    .title(format!("/{}/ #{}", &board, thread))
    .button("Выйти", |s| s.quit())
}

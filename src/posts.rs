use colored::*;
use scraper::Html;
use serde_derive::Deserialize;
use std::io::Read;
use textwrap::{fill, indent};

/// Print all messages in particular thread.
pub fn list_posts(board: &str, thread: usize, comment_width: usize) {
    let url = format!("https://2ch.hk/{}/res/{}.json", board, thread);
    let response = reqwest::get(&url).expect(&format!("Cannot get thread {}/{}", board, thread));
    let posts = parse_posts(response).expect("Cannot parse posts");

    for post in posts {
        println!(
            "{} {}{}\n{}",
            format!("{}", post.id).blue(),
            post.date.green(),
            format_images(&post.images),
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

    /// Post images
    #[serde(rename = "files")]
    images: Vec<Image>
}


#[derive(Deserialize)]
struct Image {
    /// Imageboard generate image name
    name: String,

    /// Image original full namme
    fullname: String,

    /// Relative path to a full image
    path: String,
}

fn format_images(images: &Vec<Image>) -> String {
    if images.is_empty() {
        return "".into();
    }
    format!("\n  {}", images.iter().map(format_image).collect::<Vec<_>>().join("\n"))
}

fn format_image(image: &Image) -> String {
    format!("{}\n  dvach --download {} > {name} && xdg-open {name}\n  ",
            image.fullname.yellow(),
            image.path,
            name = image.name,
    )
}

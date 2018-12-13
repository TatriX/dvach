use serde_derive::Deserialize;
use std::collections::HashMap;
use std::io::Read;

/// Print all available boards.
///
/// This function uses "mobile" API, because there is no "list boards"
/// functionality in the JSON API. At least I haven't found one.
pub fn list_boards() {
    // this is hardcoded for now
    const URL: &str = "https://2ch.hk/makaba/mobile.fcgi?task=get_boards";
    let response = reqwest::get(URL).expect("Cannot get boards");
    let boards = parse_boards(response).expect("Cannot parse boards");

    for board in boards {
        println!("{:>10} {:20} {}", board.id, board.category, board.name);
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

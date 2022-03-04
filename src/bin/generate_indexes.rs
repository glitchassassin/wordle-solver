use wordle_solver::algorithms::generate_indexes;
use serde_json::{json, to_writer};
use std::fs::File;

static WORD_LIST: &str = include_str!("../words.txt");

pub fn main() {
    let master_word_list = WORD_LIST.lines().map(|w| w.to_string()).collect::<Vec<String>>();
    let (starting_guess, indexes) = generate_indexes(&master_word_list);

    // Create file
    let file = File::create("src/indexes.json").unwrap();
    
    // Write indexes to file
    let value = json!({
        "starting_guess": &starting_guess,
        "indexes": &indexes
    });
    to_writer(file, &value).unwrap();
}
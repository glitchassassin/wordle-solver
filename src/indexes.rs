use std::collections::HashMap;

use serde::Deserialize;
use serde_json::from_str;

static INDEX: &str = include_str!("indexes.json");

#[derive(Deserialize)]
pub struct Index {
    pub starting_guess: String,
    pub indexes: HashMap<String, Option<String>>
}

pub fn get_indexes() -> Index {
    from_str(INDEX).unwrap()
}
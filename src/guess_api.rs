use serde::{Serialize, Deserialize};
use serde_json::json;

const API_URL: &str = "http://api:8000/guess";

#[derive(Serialize)]
struct GuessRequest {
    guess: String,
    word: Option<usize>,
}

#[derive(Deserialize)]
struct GuessResponse {
    result: Vec<String>,
    word: usize,
    win: bool,
}

pub fn get_random_word() -> Result<usize, reqwest::Error> {
    // Get random word
    let req = json!({
        "guess": "train"
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post(API_URL)
        .json(&req)
        .send()?;
    
    let data = res.json::<GuessResponse>()?;

    Ok(data.word)
}

pub fn guess(word: usize, guess: &str) -> Result<(Vec<String>, usize, bool), reqwest::Error> {
    let req = json!({
        "guess": guess,
        "word": word
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post(API_URL)
        .json(&req)
        .send()?;
    
    let data = res.json::<GuessResponse>()?;

    Ok((data.result, data.word, data.win))
}

pub fn guess_today(guess: &str) -> Result<(Vec<String>, bool), reqwest::Error> {
    let req = json!({
        "guess": guess
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post(format!("{}/today", API_URL))
        .json(&req)
        .send()?;
    
    let data = res.json::<GuessResponse>()?;

    Ok((data.result, data.win))
}

pub fn generate_test_guess() -> Result<impl Fn(&str) -> Result<(Vec<String>, usize, bool), reqwest::Error>, reqwest::Error> {
    let word = get_random_word()?;

    Ok(move |guessed_word: &str| {
        guess(word, guessed_word)
    })
}
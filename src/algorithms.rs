// Solver algorithms

// Min: 3
// Average: 4.2
// Max: 5
#[allow(dead_code)]
pub fn brute_force_optimized(word_list: &[String], previous_guesses: &[String], master_word_list: &[String]) -> String {
    // Short circuits
    if word_list.len() == 1 {
        return word_list[0].to_string();
    }
    if previous_guesses.is_empty() {
        return "serai".to_string(); // Best first guess
    }

    // Split into threads

    let (tx, rx) = mpsc::channel();
    let mut threads = vec![];

    const THREAD_COUNT: usize = 8;
    let words_per_thread = master_word_list.len() / THREAD_COUNT;
    let word_list_threads = Arc::new(Mutex::new(Vec::from(word_list)));

    for i in 0..THREAD_COUNT {
        let tx_thread = tx.clone();
        let word_list_thread = Arc::clone(&word_list_threads);
        let start = i * words_per_thread;
        let end = (i + 1) * words_per_thread;
        let word_list_slice = master_word_list[start..end].to_vec();
        let word_list_len = word_list.len();

        threads.push(thread::spawn(move || {
            let words = word_list_thread.lock().unwrap().clone();
            for word in word_list_slice {
                let mut unique_results = HashMap::new();
                for test_answer in words.iter() {
                    let result = compare_words(test_answer, &word);
                    let count = unique_results.get(&result).unwrap_or(&0) + 1;
                    unique_results.insert(result, count);
                }
                let worst_case = *unique_results.iter().map(|(_, score)| score).max_by(|score1, score2| score1.cmp(score2)).unwrap_or(&word_list_len);
                tx_thread.send((worst_case, word)).unwrap();
                if worst_case == 1 {
                    break;
                }
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap()
    }

    drop(tx);

    let mut best_answer = (word_list.len(), word_list[0].to_string());
    for answer in rx {
        if answer.0 < best_answer.0 {
            best_answer = answer;
        }
    }

    best_answer.1
}

// Reducer algorithms

use std::{collections::{HashMap, HashSet}, sync::{mpsc, Arc, Mutex}, thread};

pub fn reduce_word_list(word_list: &mut Vec<String>, guess: &str, results: Vec<String>) {
    let mut i = 0;
    while i < word_list.len() {
        if compare_words(&word_list[i], guess) != results {
            word_list.remove(i);
        } else {
            i += 1;
        }
    }
}

fn compare_words<'a,'b>(actual: &'a str, guess: &'a str) -> Vec<&'b str> {
    let mut word_chars: Vec<char> = actual.to_lowercase().chars().collect();
    let guess_chars: Vec<char> = guess.to_lowercase().chars().collect();
    let mut result = vec![];

    'letters: for index in 0..word_chars.len() {
        let guessed = guess_chars[index];
        let actual = word_chars[index];

        if guessed == actual {
            result.push("CORRECT");
            word_chars[index] = '_';
        } else {
            // Check if guessed letter has a match in another position
            // in the target word, but ONLY if that letter isn't correctly
            // matched, and ONLY if the out-of-position match hasn't
            // already been matched to another guessed letter
            for actual_char_index in 0..word_chars.len() {
                if word_chars[actual_char_index] != guess_chars[actual_char_index] && word_chars[actual_char_index] == guessed {
                    result.push("ALMOST");
                    word_chars[actual_char_index] = '_';
                    continue 'letters;
                }
            }
            result.push("WRONG");
        }
    }

    result
}
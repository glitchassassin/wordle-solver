// Solver algorithms

// Min: 2
// Average: 5
// Max: 13
pub fn baseline<'a>(word_list: &[&'a str]) -> &'a str {
    word_list[0]
}

// Min: 2
// Average: 5.8
// Max: 14
pub fn most_common_letters<'a>(word_list: &[&'a str]) -> &'a str {
    let mut frequencies: HashMap<char, usize> = HashMap::new();

    for word in word_list {
        word.chars().for_each(|c| { 
            frequencies.insert(c, *frequencies.get(&c).unwrap_or(&0) + 1);
        })
    }

    let score = |word: &str| word.chars().map(|c| frequencies.get(&c).unwrap_or(&0)).sum::<usize>();

    word_list
        .iter()
        .max_by(|word1, word2| score(word1).cmp(&score(word2)))
        .unwrap_or(&word_list[0])
}

// Min: 2
// Average: 6.3
// Max: 13
pub fn least_common_letters<'a>(word_list: &[&'a str]) -> &'a str {
    let mut frequencies: HashMap<char, usize> = HashMap::new();

    for word in word_list {
        word.chars().for_each(|c| { 
            frequencies.insert(c, *frequencies.get(&c).unwrap_or(&0) + 1);
        })
    }

    let score = |word: &str| word.chars().map(|c| frequencies.get(&c).unwrap_or(&0)).sum::<usize>();

    word_list
        .iter()
        .min_by(|word1, word2| score(word1).cmp(&score(word2)))
        .unwrap_or(&word_list[0])
}

// Min: 2
// Average: 5
// Max: 16
pub fn most_common_letters_by_index<'a>(word_list: &[&'a str]) -> &'a str {
    let mut frequencies: HashMap<(usize, char), usize> = HashMap::new();

    for word in word_list {
        word.char_indices().for_each(|c| { 
            frequencies.insert(c, *frequencies.get(&c).unwrap_or(&0) + 1);
        })
    }

    // dbg!(&frequencies);

    let score = |word: &str| word.char_indices().map(|c| frequencies.get(&c).unwrap_or(&0)).sum::<usize>();

    word_list
        .iter()
        .max_by(|word1, word2| score(word1).cmp(&score(word2)))
        .unwrap_or(&word_list[0])
}

// Min: 3
// Average: 5.9
// Max: 13
pub fn most_common_letters_weighting_unguessed<'a>(word_list: &[&'a str], previous_guesses: &[&'a str]) -> &'a str {
    let mut frequencies: HashMap<char, f64> = HashMap::new();

    let guessed_letters = previous_guesses.iter().flat_map(|word| word.chars()).collect::<Vec<char>>();

    for word in word_list {
        word.chars().for_each(|c| { 
            frequencies.insert(c, *frequencies.get(&c).unwrap_or(&0.0) + if guessed_letters.contains(&c) {50.0 / word_list.len() as f64} else {1.0});
        })
    }

    let score = |word: &str| word.chars().map(|c| frequencies.get(&c).unwrap_or(&0.0)).sum::<f64>();

    word_list
        .iter()
        .max_by(|word1, word2| score(word1).partial_cmp(&score(word2)).unwrap())
        .unwrap_or(&word_list[0])
}

// Min: 2
// Average: 4.74
// Max: 12
pub fn most_common_letters_by_index_weighting_unguessed<'a>(word_list: &[&'a str], previous_guesses: &[&'a str]) -> &'a str {
    let mut frequencies: HashMap<(usize, char), f64> = HashMap::new();

    let guessed_letters = previous_guesses.iter().flat_map(|word| word.chars()).collect::<Vec<char>>();

    for word in word_list {
        word.char_indices().for_each(|c| { 
            frequencies.insert(c, *frequencies.get(&c).unwrap_or(&0.0) + if guessed_letters.contains(&c.1) {50.0 / word_list.len() as f64} else {1.0});
        })
    }

    let score = |word: &str| word.char_indices().map(|c| frequencies.get(&c).unwrap_or(&0.0)).sum::<f64>();

    word_list
        .iter()
        .max_by(|word1, word2| score(word1).partial_cmp(&score(word2)).unwrap())
        .unwrap_or(&word_list[0])
}

// Min: 2
// Average: 4.79
// Max: 10
pub fn most_common_letters_by_index_weighting_unguessed_from_master_list<'a>(word_list: &[&'a str], previous_guesses: &[&'a str], master_word_list: &[&'a str]) -> &'a str {
    let mut frequencies: HashMap<(usize, char), f64> = HashMap::new();

    let guessed_letters = previous_guesses.iter().flat_map(|word| word.chars()).collect::<Vec<char>>();

    for word in word_list {
        word.char_indices().for_each(|c| { 
            frequencies.insert(c, *frequencies.get(&c).unwrap_or(&0.0) + if guessed_letters.contains(&c.1) {50.0 / word_list.len() as f64} else {1.0});
        })
    }

    let score = |word: &str| word.char_indices().map(|c| frequencies.get(&c).unwrap_or(&0.0)).sum::<f64>();

    master_word_list
        .iter()
        .max_by(|word1, word2| score(word1).partial_cmp(&score(word2)).unwrap())
        .unwrap_or(&word_list[0])
}

// Min: 2
// Average: 4.72
// Max: 10
pub fn most_common_letters_by_index_weighting_unguessed_from_master_list_weighting_no_duplicates<'a>(word_list: &[&'a str], previous_guesses: &[&'a str], master_word_list: &[&'a str]) -> &'a str {
    let mut frequencies: HashMap<(usize, char), f64> = HashMap::new();

    let guessed_letters = previous_guesses.iter().flat_map(|word| word.chars()).collect::<Vec<char>>();

    for word in word_list {
        word.char_indices().for_each(|c| { 
            frequencies.insert(c, *frequencies.get(&c).unwrap_or(&0.0) + if guessed_letters.contains(&c.1) {30.0 / word_list.len() as f64} else {1.0});
        })
    }

    let score = |word: &str| {
        let base_score = word.char_indices().map(|c| frequencies.get(&c).unwrap_or(&0.0)).sum::<f64>();
        let unique_chars: HashSet<char> = HashSet::from_iter(word.chars());
        // Weight score lower for duplicate characters
        let unique_weight = 0.5;
        base_score * (unique_weight + (unique_weight * (unique_chars.len() as f64 / word.len() as f64)))
    };

    master_word_list
        .iter()
        .max_by(|word1, word2| score(word1).partial_cmp(&score(word2)).unwrap())
        .unwrap_or(&word_list[0])
}

// Min: 2
// Average: 5.03
// Max: 13
pub fn most_common_letters_by_index_weighting_unguessed_weighting_no_duplicates<'a>(word_list: &[&'a str], previous_guesses: &[&'a str], master_word_list: &[&'a str]) -> &'a str {
    let mut frequencies: HashMap<(usize, char), f64> = HashMap::new();

    let guessed_letters = previous_guesses.iter().flat_map(|word| word.chars()).collect::<Vec<char>>();

    for word in word_list {
        word.char_indices().for_each(|c| { 
            frequencies.insert(c, *frequencies.get(&c).unwrap_or(&0.0) + if guessed_letters.contains(&c.1) {50.0 / word_list.len() as f64} else {1.0});
        })
    }

    let score = |word: &str| {
        let base_score = word.char_indices().map(|c| frequencies.get(&c).unwrap_or(&0.0)).sum::<f64>();
        let unique_chars: HashSet<char> = HashSet::from_iter(word.chars());
        // Weight score lower for duplicate characters
        let unique_weight = 0.5;
        base_score * (unique_weight + (unique_weight * (unique_chars.len() as f64 / word.len() as f64)))
    };

    word_list
        .iter()
        .max_by(|word1, word2| score(word1).partial_cmp(&score(word2)).unwrap())
        .unwrap_or(&word_list[0])
}

// Min: 
// Average: 
// Max: 
pub fn brute_force<'a>(word_list: &[&'a str], previous_guesses: &[&'a str], master_word_list: &[&'a str]) -> &'a str {
    // For each word in master_word_list
        // Test the word against each word in word_list
        // Reduce the word list as if the answer were correct. Record the length.
        // The word's score is the total length of all reduced word lists.
        // The best word has the lowest score.

    if word_list.len() == 1 {
        return word_list[0];
    }

    if previous_guesses.len() == 0 {
        return "tares";
    }

    let mut best_answer = (word_list.len(), word_list[0]);

    for word in master_word_list {
        let mut unique_results = HashMap::new();
        for test_answer in word_list {
            let result = compare_words(test_answer, word);
            let count = unique_results.get(&result).unwrap_or(&0) + 1;
            unique_results.insert(result, count);
        }
        let worst_case = unique_results.iter().max_by(|(_, score1), (_, score2)| score1.cmp(score2));
        if let Some((_, worst_count)) = worst_case {
            if worst_count < &best_answer.0 {
                best_answer = (*worst_count, word);
            }
        }
        if best_answer.0 == 1 {
            break;
        }
    }

    best_answer.1
}

// Reducer algorithms

use std::collections::{HashMap, HashSet};

pub fn reduce_word_list(word_list: &mut Vec<&str>, guess: &str, results: Vec<String>) {
    let mut i = 0;
    while i < word_list.len() {
        if compare_words(word_list[i], guess) != results {
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
// Solver algorithms

pub fn baseline<'a>(word_list: &[&'a str]) -> &'a str {
    word_list[0]
}

// Reducer algorithms

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
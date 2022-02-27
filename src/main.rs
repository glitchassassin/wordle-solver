mod guess_api;
mod algorithms;

static WORD_LIST: &str = include_str!("words.txt");

fn solve() -> Option<(&'static str, usize)> {
    let test_guess = guess_api::generate_test_guess().ok()?;
    let mut rounds: usize = 0;
    let mut word_list: Vec<&str> = WORD_LIST.lines().collect::<Vec<&str>>().clone();
    loop {
        let guess = algorithms::baseline(&word_list);
        rounds += 1;

        let (results, _, win) = test_guess(guess).ok()?;

        if win {
            return Some((guess, rounds));
        }

        algorithms::reduce_word_list(&mut word_list, guess, results);
    }
}

fn main() {
    let solution_times: Vec<usize> = (0..100).filter_map(|_| {
        if let Some((_, rounds)) = solve() {
            Some(rounds)
        } else {
            None
        }
    }).collect();
    let mean: f32 = (solution_times.iter().sum::<usize>() as f32) / (solution_times.len() as f32);
    let max = solution_times.iter().max().unwrap_or(&0);
    let min = solution_times.iter().min().unwrap_or(&0);
    println!("Average rounds to solve: {} \nMax: {} \nMin: {}", mean, max, min);

    // if let Some((solution, rounds)) = solve() {
    //     println!("Solved {} in {} rounds", solution, rounds);
    // } else {
    //     println!("Solver failed")
    // }
    
}

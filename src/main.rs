use std::time::Instant;

mod guess_api;
mod algorithms;

static WORD_LIST: &str = include_str!("words.txt");

fn solve() -> Option<(String, usize)> {
    let test_guess = guess_api::generate_test_guess().ok()?;
    let mut rounds: usize = 0;
    let mut word_list: Vec<String> = WORD_LIST.lines().map(|w| w.to_string()).collect::<Vec<String>>();
    let mut master_word_list: Vec<String> = WORD_LIST.lines().map(|w| w.to_string()).collect::<Vec<String>>();
    let mut previous_guesses: Vec<String> = vec![];
    loop {
        let guess = algorithms::brute_force_optimized(&word_list, &previous_guesses, &master_word_list);
        rounds += 1;

        println!("Round {}: Guessing {}", rounds, guess);

        let (results, _, win) = test_guess(&guess).ok()?;

        if win {
            return Some((guess, rounds));
        }

        algorithms::reduce_word_list(&mut word_list, &guess, results);
        if let Some(index) = &master_word_list.iter().position(|word| word == &guess) {
            master_word_list.remove(*index);
        }
        previous_guesses.push(guess);

        println!("{} remaining words", word_list.len());
    }
}

fn main() {
    // const TEST_COUNT: u32 = 100;
    // let start = Instant::now();
    // let solution_times: Vec<usize> = (0..TEST_COUNT).filter_map(|i| {
    //     println!("Round {round:>width$}/{total}", round=i, width=4, total=TEST_COUNT);
    //     if let Some((_, rounds)) = solve() {
    //         Some(rounds)
    //     } else {
    //         None
    //     }
    // }).collect();
    // let duration = start.elapsed();
    // let average_time = duration / TEST_COUNT;
    // let mean: f32 = (solution_times.iter().sum::<usize>() as f32) / (solution_times.len() as f32);
    // let max = solution_times.iter().max().unwrap_or(&0);
    // let min = solution_times.iter().min().unwrap_or(&0);
    // println!("Average rounds to solve: {} \nMax: {} \nMin: {}\nTime: {:?}", mean, max, min, average_time);

    if let Some((solution, rounds)) = solve() {
        println!("Solved {} in {} rounds", solution, rounds);
    } else {
        println!("Solver failed")
    }
}

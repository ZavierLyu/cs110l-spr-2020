// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn readin() -> char {
    print!("Please guess a letter: ");
    // Make sure the prompt from the previous line gets displayed:
    io::stdout().flush().expect("Error flushing stdout.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line.");
    let guess = guess.trim();
    if guess.len() != 1 {
        panic!("Error input.");
    }
    guess.chars().next().unwrap()
}

fn check_contain_and_fill(secret: &Vec<char>, target: &mut Vec<char>, c: &char) -> bool {
    for (idx, e) in secret.iter().enumerate() {
        if *e == *c && target[idx] == '-' {
            target[idx] = *c;
            return true;
        }
    }
    return false;
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);
    // Your code here! :)
    print!("Welcome to CS110L Hangman!\n");
    let mut left_chars = secret_word_chars.len();
    let mut extra_guess_times = NUM_INCORRECT_GUESSES;
    let mut guess_word_chars: Vec<char> = vec!['-'; secret_word_chars.len()];
    let mut guess_history: String = String::new();
    while left_chars > 0 && extra_guess_times > 0 {
        let word_so_far = guess_word_chars.iter().collect::<String>();
        print!("The word so far is {}\nYou have guessed the following letters: {}\nYou have {} guesses left\n", word_so_far, guess_history, extra_guess_times);
        let guess = readin();
        guess_history.push(guess);
        let has = check_contain_and_fill(&secret_word_chars, &mut guess_word_chars, &guess);
        if !has {
            extra_guess_times -= 1;
            print!("Sorry, that letter is not in the word\n");
        } else {
            left_chars -= 1;
        }
        print!("\n");
        io::stdout().flush().expect("Error flushing stdout.");
    }
    if left_chars == 0 {
        print!(
            "Congratulations you guessed the secret word: {}!",
            secret_word
        );
    } else {
        print!("Sorry, you ran out of guesses!");
    }
}

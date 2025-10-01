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

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");

    // idx_right_yet，记录了当前secret_word中已经被猜出来的字符下标
    let mut  idx_right_yet: Vec<bool> = vec![false; secret_word_chars.len()];

    // letter_yet记录历史输入
    let mut letter_yet: String = String::from("");

    // 两个counter分别记录剩余的猜测机会和猜对的字符数
    let mut chance_guess = NUM_INCORRECT_GUESSES;
    let mut count_right = 0;
    while chance_guess > 0 && count_right < secret_word_chars.len() {
        println!("The word so far is {}", word_with_mask(&secret_word_chars, &idx_right_yet));
        println!("You have guessed the following letters: {}", letter_yet);
        println!("You have {} guesses left", chance_guess);
        print!("Please guess a letter: ");

        // 字符串输入
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        // 更新输入历史
        let ch = guess.chars().nth(0).unwrap();
        letter_yet.push(ch);

        // 查看该guess是否在secret_word中
        let mut i = 0;
        while i < secret_word_chars.len() {
            if !idx_right_yet[i] && secret_word_chars[i] == ch {
                break;
            }
            i += 1;
        }

        if i == secret_word_chars.len() {
            println!("Sorry, that letter is not in the word");
            chance_guess -= 1;
        } else {
            idx_right_yet[i] = true;
            count_right += 1;
        }
        println!();
    }

    if count_right == secret_word_chars.len() {
        println!("Congratulations you guessed the secret word: {}!", secret_word);
    } else {
        println!("Sorry, you ran out of guesses!")
    }
}

// 构造带掩码的字符串
fn word_with_mask(secret_word_chars: &Vec<char>, idx: &Vec<bool>) -> String {
    let mut ans: String = String::from("");
    for i in 0..secret_word_chars.len() {
        if idx[i] {
            ans.push(secret_word_chars[i]);
        } else {
            ans.push_str("-");
        }
    }
    ans
}
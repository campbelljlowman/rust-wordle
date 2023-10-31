#[allow(dead_code)]

use colored::Colorize;
use std::{collections::HashMap, io};
use std::io::BufRead;
use std::fs;
use rand::Rng;

#[derive(PartialEq)]
enum WordleCharacterStatus {
    Correct,
    Close,
    Wrong,
    NotGuessed
}

struct WordleBoard<'a> {
    wordle_word: &'a str,
    valid_wordle_words: &'a Vec<&'a str>,
    is_completed: bool,
    words_guessed: Vec<Vec<(char, WordleCharacterStatus)>>,
    characters_guessed: HashMap<char, WordleCharacterStatus>
}

impl <'a> WordleBoard<'a> {
    fn new(wordle_word: &'a str, valid_wordle_words: &'a Vec<&'a str>) -> Self {
        let mut initial_characters_guessed: HashMap<char, WordleCharacterStatus> = HashMap::new();
        for char in "abcdefghijklmnopqrstuvwxyz".chars() {
            initial_characters_guessed.insert(char, WordleCharacterStatus::NotGuessed);
        }

        WordleBoard {
            wordle_word,
            valid_wordle_words,
            is_completed: false,
            words_guessed: Vec::new(),
            characters_guessed: initial_characters_guessed
        }
    }

    fn print_board_status(&self) {
        println!("\n");
        for guess in &self.words_guessed {
            for (char, wordle_character_status) in guess.iter() {
                print_character_with_color(char, wordle_character_status);
                print!(" ");
            }
            print!("\n\n");
        }
        for _ in 0..6-self.words_guessed.len() {
            println!("_ _ _ _ _");
        }

        println!("");
        for char in "qwertyuiop".chars() {
            print_character_with_color(&char, self.characters_guessed.get(&char).unwrap_or(&WordleCharacterStatus::NotGuessed));
            print!(" ")
        }
        println!("");
        print!(" ");
        for char in "asdfghjkl".chars() {
            print_character_with_color(&char, self.characters_guessed.get(&char).unwrap_or(&WordleCharacterStatus::NotGuessed));
            print!(" ")
        }
        println!("");
        print!("  ");
        for char in "zxcvbnm".chars() {
            print_character_with_color(&char, self.characters_guessed.get(&char).unwrap_or(&WordleCharacterStatus::NotGuessed));
            print!(" ")
        }
        println!("");
    }

    fn add_guess(&mut self, new_guess: String) -> Result<(), Box<dyn std::error::Error>>{
        if !self.is_completed {
            if new_guess.len() != 5 {
                Err("a guess must be 5 characters long")?
            }

            if !self.valid_wordle_words.contains(&new_guess.as_str()) {
                Err("guess is not a valid word")?
            }

            let new_guess_char_array = new_guess.chars();
            let mut new_word_guesses: Vec<(char, WordleCharacterStatus)> = Vec::new();
            for (i, char) in new_guess_char_array.enumerate() {
                if self.wordle_word.chars().nth(i).unwrap() == char {
                    new_word_guesses.push((char, WordleCharacterStatus::Correct));
                    self.characters_guessed.insert(char, WordleCharacterStatus::Correct);
                } else if self.wordle_word.contains(char) {
                    new_word_guesses.push((char, WordleCharacterStatus::Close));
                    if self.characters_guessed.get(&char).unwrap_or(&WordleCharacterStatus::NotGuessed) != &WordleCharacterStatus::Correct {
                        self.characters_guessed.insert(char, WordleCharacterStatus::Close);
                    }
                } else {
                    new_word_guesses.push((char, WordleCharacterStatus::NotGuessed));
                    if self.characters_guessed.get(&char).unwrap_or(&WordleCharacterStatus::NotGuessed) != &WordleCharacterStatus::Correct || 
                    self.characters_guessed.get(&char).unwrap_or(&WordleCharacterStatus::NotGuessed) != &WordleCharacterStatus::Close {
                        self.characters_guessed.insert(char, WordleCharacterStatus::Wrong);
                    }
                }
            }

            self.words_guessed.push(new_word_guesses);

            if self.wordle_word == new_guess || self.words_guessed.len() == 6{
                self.is_completed = true;
            }

            Ok(())
        } else {
            Err("wordle board is completed, no more guessing allowed")?
        }
    }
}

fn print_character_with_color(char: &char, wordle_character_status: &WordleCharacterStatus) {
    match wordle_character_status {
        WordleCharacterStatus::Correct => print!("{}", char.to_string().bold().on_green()),
        WordleCharacterStatus::Close => print!("{}", char.to_string().bold().on_yellow()),
        WordleCharacterStatus::Wrong => print!("{}", char.to_string().bold().on_truecolor(177, 177, 177)),
        WordleCharacterStatus::NotGuessed => print!("{}", char.to_string().bold())
    }
}
fn main() {
    let binding_valid_wordle_words_string = fs::read_to_string("./valid_wordle_words.txt").unwrap();
    let valid_wordle_words: Vec<&str> = binding_valid_wordle_words_string.split(['\n']).collect();

    let index_of_wordle_word = rand::thread_rng().gen_range(0..valid_wordle_words.len());

    let mut wordle_board = WordleBoard::new(valid_wordle_words.get(index_of_wordle_word).unwrap(), &valid_wordle_words);
    
    let stdin = io::stdin();

    loop {
        wordle_board.print_board_status();
        println!("Enter Wordle guess");

        let input_line = stdin.lock().lines().next().unwrap().unwrap();

        println!("");
        if let Err(e) = wordle_board.add_guess(input_line) {
            print!("Error adding guess: {}", e)
        }

        if wordle_board.is_completed {
            wordle_board.print_board_status();
            println!("Worlde word is: {}", wordle_board.wordle_word);
            break
        }
    }
}

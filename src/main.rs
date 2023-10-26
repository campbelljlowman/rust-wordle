#[allow(dead_code)]

use colored::Colorize;
use std::collections::HashMap;

#[derive(PartialEq)]
enum WordleCharacterStatus {
    Correct,
    Close,
    Wrong,
    NotGuessed
}
struct WordleBoard<'a> {
    wordle_word: &'a str,
    is_completed: bool,
    words_guessed: Vec<Vec<(char, WordleCharacterStatus)>>,
    characters_guessed: HashMap<char, WordleCharacterStatus>
}

impl <'a> WordleBoard<'a> {
    fn new(wordle_word: &'a str) -> Self {
        let mut initial_characters_guessed: HashMap<char, WordleCharacterStatus> = HashMap::new();
        for char in "abcdefghijklmnopqrstuvwxyz".chars() {
            initial_characters_guessed.insert(char, WordleCharacterStatus::NotGuessed);
        }

        WordleBoard {
            wordle_word,
            is_completed: false,
            words_guessed: Vec::new(),
            characters_guessed: initial_characters_guessed
        }
    }

    fn print_board_status(self) {
        println!("Wordle\n");
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

    fn add_guess(&mut self, new_guess: &'a str) -> Result<(), Box<dyn std::error::Error>>{
        if !self.is_completed {
            if new_guess.len() != 5 {
                Err("a guess must be 5 characters long")?
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
    let mut wordle_board = WordleBoard::new("hello");
    wordle_board.add_guess("olive").unwrap();
    wordle_board.add_guess("haaaa").unwrap();
    wordle_board.add_guess("omaha").unwrap();
    // wordle_board.add_guess("hello").unwrap();
    // wordle_board.add_guess("olive").unwrap();
    // wordle_board.add_guess("olive").unwrap();
    // wordle_board.add_guess("olive");

    wordle_board.print_board_status();

}

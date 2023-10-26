#[allow(dead_code)]

use colored::Colorize;

struct WordleBoard<'a> {
    wordle_word: &'a str,
    is_completed: bool,
    guesses: Vec<&'a str>
}

impl <'a> WordleBoard<'a> {
    fn new(wordle_word: &'a str) -> Self {
        WordleBoard {
            wordle_word,
            is_completed: false,
            guesses: Vec::new()
        }
    }

    fn print_status(self) {
        println!("Wordle\n");
        for guess in &self.guesses {
            for (i, char) in guess.chars().enumerate() {
                if self.wordle_word.chars().nth(i).unwrap() == char {
                    print!("{}", char.to_string().on_green());  
                } else if self.wordle_word.contains(char) {
                    print!("{}", char.to_string().on_yellow());
                } else {
                    print!("{}", char);                
                }
            }
            print!("\n");
        }
        for _ in 0..6-self.guesses.len() {
            println!("_____");
        }
    }

    fn add_guess(&mut self, new_guess: &'a str) -> Result<(), Box<dyn std::error::Error>>{
        if !self.is_completed {
            if new_guess.len() != 5 {
                Err("a guess must be 5 characters long")?
            }
            self.guesses.push(new_guess);

            if self.wordle_word == new_guess || self.guesses.len() == 6{
                self.is_completed = true;
            }

            Ok(())
        } else {
            Err("wordle board is completed, no more guessing allowed")?
        }
    }
}

fn main() {
    let mut wordle_board = WordleBoard::new("hello");
    wordle_board.add_guess("olive").unwrap();
    wordle_board.add_guess("ocean").unwrap();
    wordle_board.add_guess("heruo").unwrap();
    wordle_board.add_guess("hello").unwrap();
    // wordle_board.add_guess("olive").unwrap();
    // wordle_board.add_guess("olive").unwrap();
    // wordle_board.add_guess("olive");

    wordle_board.print_status();

}

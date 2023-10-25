#[allow(dead_code)]

struct WordleBoard<'a> {
    wordle_word: &'a str,
    is_solved: bool,
    guesses: Vec<&'a str>
}

impl <'a> WordleBoard<'a> {

    fn new(wordle_word: &'a str) -> WordleBoard {
        WordleBoard {
            wordle_word,
            is_solved: false,
            guesses: Vec::new()
        }
    }

    fn print_status(self) {
        for guess in self.guesses {
            println!("{}", guess)
        }
    }

    fn add_guess(&mut self, new_guess: &'a str) {
        self.guesses.push(new_guess)
    }
}

fn main() {
    let mut wordle_board = WordleBoard::new("hello");
    wordle_board.add_guess("new_guess");
    wordle_board.print_status();

}

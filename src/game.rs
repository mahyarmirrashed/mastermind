use std::io::{stdin, stdout, Stdout, Write};

use itertools::Itertools;
use rand::{distributions, Rng};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use mastermind::{ColorPeg, Feedback};

const INSTRUCTIONS: &str = "╔══════════════════════════════════╗\r\n\
                            ╟----------- Mastermind -----------╢\r\n\
                            ╠══════════════════════════════════╣\r\n\
                            ║ ◀ ▶ | move cursor left and right ║\r\n\
                            ║ ▼ ▲ | shuffle through colors     ║\r\n\
                            ║ ↵   | submit guess               ║\r\n\
                            ║ q   | quit the game              ║\r\n\
                            ╚══════════════════════════════════╝\r\n\
                            ";

pub struct Game {
    stdout: RawTerminal<Stdout>,
    colorblind: bool,
    pegs: usize,
    guesses: usize,
    answer: Vec<ColorPeg>,
    history: Vec<ColorPeg>,
    guess_count: usize,
}

impl Game {
    pub fn new(pegs: usize, guesses: usize, colorblind: bool) -> Self {
        let answer = rand::thread_rng()
            .sample_iter(distributions::Standard)
            .take(pegs)
            .collect();

        Self {
            stdout: stdout().into_raw_mode().unwrap(),
            colorblind,
            pegs,
            guesses,
            answer,
            history: vec![ColorPeg::White; pegs * guesses],
            guess_count: 0,
        }
    }

    pub fn run(&mut self) {
        while self.guess_count < self.guesses {
            let mut guess = vec![ColorPeg::White; self.pegs];
            let mut cursor = 0;

            self.display(Some(&guess), Some(cursor));

            for chr in stdin().keys() {
                match chr.unwrap() {
                    Key::Up | Key::Char('k') => guess[cursor] = guess[cursor].up(),
                    Key::Down | Key::Char('j') => guess[cursor] = guess[cursor].down(),
                    Key::Left | Key::Char('h') => cursor = (cursor + self.pegs - 1) % self.pegs,
                    Key::Right | Key::Char('l') => cursor = (cursor + self.pegs + 1) % self.pegs,
                    Key::Char('\n' | ' ') => break,
                    Key::Char('q') | Key::Ctrl('c' | 'd') => return,
                    _ => {}
                }
                self.display(Some(&guess), Some(cursor));
            }

            let offset = self.guess_count * self.pegs;
            self.history[offset..offset + self.pegs].copy_from_slice(&guess);
            self.guess_count += 1;

            if Feedback::new(&guess, &self.answer).unwrap().right == self.pegs {
                self.display(None, None);
                write!(self.stdout, "Congratulations, you won!\r\n").expect("Not written.");
                return;
            }
        }

        self.display(None, None);
        write!(self.stdout, "Sorry, you lost!\r\n").expect("Not written.");
    }

    fn display(&mut self, guess: Option<&[ColorPeg]>, guess_cursor: Option<usize>) {
        write!(
            self.stdout,
            "{}{}{}\r\n",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            INSTRUCTIONS
        )
        .expect("Not written.");
        self.stdout
            .flush()
            .expect("Unable to flush standard output!");

        let history_rows = &self.history[..self.guess_count * self.pegs];
        let has_history = !history_rows.is_empty();

        for (i, row) in history_rows.chunks(self.pegs).enumerate() {
            write!(
                self.stdout,
                "Guess {:0>2}: [ {} ] {}\r\n",
                i + 1,
                row.iter().map(|p| p.display(self.colorblind)).join(" "),
                Feedback::new(row, &self.answer)
                    .unwrap_or_default()
                    .display(self.colorblind)
            )
            .expect("Not written.");
        }

        if has_history {
            write!(self.stdout, "\r\n").expect("Not written.");
        }

        if let Some(guess) = guess {
            write!(
                self.stdout,
                "[ {} ]\r\n",
                guess.iter().map(|p| p.display(self.colorblind)).join(" ")
            )
            .expect("Not written.");
            let mut cursor_row = vec![' '; guess.len()];
            cursor_row[guess_cursor.unwrap_or_default()] = '^';
            write!(self.stdout, "  {}  \r\n", cursor_row.iter().join(" ")).expect("Not written.");
        }
    }
}

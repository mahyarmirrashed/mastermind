#![forbid(unsafe_code)]

use std::io::{stdin, stdout, Write};

use clap::Parser;
use itertools::Itertools;
use rand::{distributions::Standard, thread_rng, Rng};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use mastermind::{ColorPeg, Feedback};

/// Feedback peg for indicating correct color code peg placed in right/wrong
/// position with black/white colors, respectively
const FEEDBACK_PEG: &str = "\u{25c9}";

/// Mastermind is a game where the codebreaker tries to guess the pattern in both order and color.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Number of color code pegs to guess each turn
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(3..=6), default_value_t = 4)]
    pegs: u8,

    /// Number of turns before game ends
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(8..=12), default_value_t = 10)]
    turns: u8,
}

/// Program enters here.
fn main() {
    // parse arguments passed to program
    let args = Args::parse();
    // TODO: update when clap allows `usize` as value parser argument
    let pegs = args.pegs as usize;
    let turns = args.turns as usize;

    // generate answer that needs to be guessed
    let answer: Vec<ColorPeg> = thread_rng().sample_iter(Standard).take(pegs).collect();

    // create vector of vectors holding
    let mut history = vec![ColorPeg::White; pegs * turns];
    // create vector holding feedback history
    let feedback = vec![Feedback { wrong: 0, right: 0 }; turns];

    // track number of guesses made
    let mut guesses = 0usize;

    // enter into raw mode terminal parsing
    let mut stdout = stdout().into_raw_mode().unwrap();

    // loop all through all guesses
    while guesses < turns {
        // clear entire terminal output
        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .expect("Not written.");
        // flush output
        stdout.flush().expect("Unable to flush standard output!");

        // print guess history along with feedback
        for i in 0..guesses {
            let range = (i * pegs)..((i + 1) * pegs);
            write!(stdout, "[ {} ]\r\n", &history[range].iter().join("  ")).expect("Not written.");
        }

        // create vector holding user guesses
        let mut guess = vec![ColorPeg::White; pegs];
        // track current peg position
        let mut cursor = 0;
        // grab inputs from stdin
        for chr in stdin().keys() {
            match chr.unwrap() {
                Key::Up => guess[cursor] = guess[cursor].up(),
                Key::Down => guess[cursor] = guess[cursor].down(),
                Key::Left => cursor = (cursor + pegs - 1) % pegs,
                Key::Right => cursor = (cursor + pegs + 1) % pegs,
                Key::Char('\n') => break,
                Key::Char('q') => return,
                _ => {}
            }
        }

        // save guess into past history
        let range = (guesses * pegs)..((guesses + 1) * pegs);
        history[range].copy_from_slice(&guess);

        guesses += 1;
    }

    // println!(
    //     "[{} {}]",
    //     std::iter::repeat(FEEDBACK_PEG)
    //         .take(feedback[0].right)
    //         .intersperse(" ")
    //         .collect::<String>(),
    //     std::iter::repeat(FEEDBACK_PEG)
    //         .take(feedback[0].wrong)
    //         .intersperse(" ")
    //         .collect::<String>(),
    // );

    // println!("[ {} ]", answer.iter().join("  "));
}

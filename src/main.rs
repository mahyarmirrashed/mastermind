#![forbid(unsafe_code)]

use std::io::{stdin, stdout, Stdout, Write};

use clap::Parser;
use itertools::Itertools;
use rand::{distributions, Rng};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use mastermind::{ColorPeg, Feedback};

/// Mastermind is a game where the codebreaker tries to guess the pattern in both order and color.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Number of color code pegs to guess each turn
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(3..=6), default_value_t = 4)]
    pegs: u8,

    /// Number of guesses before game ends
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(8..=12), default_value_t = 10)]
    guesses: u8,
}

/// Program enters here. Main logic is performed here until game completes.
fn main() {
    // parse arguments pass to program
    let args = Args::parse();
    // XXX: https://github.com/clap-rs/clap/pull/3895
    let pegs = args.pegs as usize;
    let guesses = args.guesses as usize;

    // generate code (answer) needing to be guessed by player
    let answer: Vec<ColorPeg> = rand::thread_rng()
        .sample_iter(distributions::Standard)
        .take(pegs)
        .collect();

    // track guess history and guess count
    let mut guess_history = vec![ColorPeg::White; pegs * guesses];
    let mut guess_count = 0;

    // create raw mode standard output
    let mut stdout = stdout().into_raw_mode().unwrap();

    // continue until player guesses correctly or runs out of guesses
    while guess_count < guesses {
        // display guess history to user
        display(
            guess_history[..guess_count * pegs].chunks(pegs),
            &answer,
            &mut stdout,
        );

        // track current player guess and cursor location
        let mut guess = vec![ColorPeg::White; pegs];
        let mut guess_cursor = 0;

        // process based on keystroke
        for chr in stdin().keys() {
            match chr.unwrap() {
                Key::Up => guess[guess_cursor] = guess[guess_cursor].up(),
                Key::Down => guess[guess_cursor] = guess[guess_cursor].down(),
                Key::Left => guess_cursor = (guess_cursor + pegs - 1) % pegs,
                Key::Right => guess_cursor = (guess_cursor + pegs + 1) % pegs,
                Key::Char('\n') => break,
                Key::Char('q') | Key::Ctrl('c' | 'd') => return,
                _ => {}
            }
        }

        // save guess into guess history
        let offset = guess_count * pegs;
        guess_history[offset..offset + guess.len()].copy_from_slice(&guess);

        // quick escape if guess was correct
        if Feedback::new(&guess, &answer).unwrap().right == pegs {
            break;
        }

        guess_count += 1;
    }
}

/// Display guess history to user.
fn display(
    history: std::slice::Chunks<ColorPeg>,
    answer: &[ColorPeg],
    stdout: &mut RawTerminal<Stdout>,
) {
    // clear terminal output and place cursor in (1,1)
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .expect("Not written.");
    // flush output, clearing terminal is often buffered
    stdout.flush().expect("Unable to flush standard output!");

    // print guess history along with feedback
    for guess in history {
        write!(
            stdout,
            "[ {} ] ( {} )\r\n",
            guess.iter().join("  "),
            Feedback::new(guess, answer).unwrap_or(Feedback { wrong: 0, right: 0 })
        )
        .expect("Not written.");
    }
}

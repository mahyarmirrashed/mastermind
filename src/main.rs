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

const INSTRUCTIONS: &str = "╔══════════════════════════════════╗\r\n\
                            ╟----------- Mastermind -----------╢\r\n\
                            ╠══════════════════════════════════╣\r\n\
                            ║ ◀ ▶ | move cursor left and right ║\r\n\
                            ║ ▼ ▲ | shuffle through colors     ║\r\n\
                            ║ ↵   | submit guess               ║\r\n\
                            ║ q   | quit the game              ║\r\n\
                            ╚══════════════════════════════════╝\r\n\
                            ";

#[derive(Parser, Debug)]
/// Mastermind is a game where the codebreaker tries to guess the pattern in
/// both order and color.
///
/// To play Mastermind, the codemaster (this program) will generate a sequence
/// of randomly colored pegs. This sequence will be "the code". There can be
/// multiple pegs of the same color in a given code. The goal of the codebreaker
/// (you) is to guess the correct color and position of every peg in the
/// codemaster's code. If you can achieve this task in the number of guesses
/// required, you win! Feedback is provided at the end of each guess. A white
/// feedback peg indicates a peg in your guess is of the right color but, wrong
/// position. Similarly, a black feedback peg indicates a peg in your guess is
/// of the right color and right position.
///
/// You can learn more about Mastermind
/// [here](https://www.wikiwand.com/en/Mastermind_(board_game))
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
        // track current player guess and cursor location
        let mut guess = vec![ColorPeg::White; pegs];
        let mut guess_cursor = 0;

        // display terminal user interface
        display(
            guess_history[..guess_count * pegs].chunks(pegs),
            &answer,
            Some(&guess),
            Some(guess_cursor),
            &mut stdout,
        );

        // process based on keystroke
        for chr in stdin().keys() {
            // handle next character from standard input
            match chr.unwrap() {
                Key::Up => guess[guess_cursor] = guess[guess_cursor].up(),
                Key::Down => guess[guess_cursor] = guess[guess_cursor].down(),
                Key::Left => guess_cursor = (guess_cursor + pegs - 1) % pegs,
                Key::Right => guess_cursor = (guess_cursor + pegs + 1) % pegs,
                Key::Char('\n' | ' ') => break,
                Key::Char('q') | Key::Ctrl('c' | 'd') => return,
                _ => {}
            }

            // re-render terminal user interface with new guess
            display(
                guess_history[..guess_count * pegs].chunks(pegs),
                &answer,
                Some(&guess),
                Some(guess_cursor),
                &mut stdout,
            );
        }

        // save guess into guess history
        let offset = guess_count * pegs;
        guess_history[offset..offset + guess.len()].copy_from_slice(&guess);

        // increase guess count
        guess_count += 1;

        // quick escape if guess was correct
        if Feedback::new(&guess, &answer).unwrap().right == pegs {
            break;
        }
    }

    // display final game output without guess prompt
    display(
        guess_history[..guess_count * pegs].chunks(pegs),
        &answer,
        None,
        None,
        &mut stdout,
    );

    // display output based on win or loss
    if guess_count == guesses {
        write!(stdout, "Sorry, you lost!\r\n").expect("Not written.");
    } else {
        write!(stdout, "Congratulations, you won!\r\n").expect("Not written.");
    }
}

/// Display terminal user interface to user.
fn display(
    history: std::slice::Chunks<ColorPeg>,
    answer: &[ColorPeg],
    guess: Option<&[ColorPeg]>,
    guess_cursor: Option<usize>,
    stdout: &mut RawTerminal<Stdout>,
) {
    // clear terminal output, place cursor in (1, 1), and print instructions
    write!(
        stdout,
        "{}{}{}\r\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        INSTRUCTIONS
    )
    .expect("Not written.");
    // flush output, clearing terminal is often buffered
    stdout.flush().expect("Unable to flush standard output!");

    // determine whether to include guess separator
    let separate_guesses = history.len() != 0;

    // print guess history along with feedback
    for (i, guess) in history.enumerate() {
        write!(
            stdout,
            "Guess {:0>2}: [ {} ] {}\r\n",
            i + 1,
            guess.iter().join(" "),
            Feedback::new(guess, answer).unwrap_or_default()
        )
        .expect("Not written.");
    }

    // print newline to separate from guesses
    if separate_guesses {
        write!(stdout, "\r\n").expect("Not written.");
    }

    // print current guess, if necessary
    if let Some(guess) = guess {
        // current guess selected by user
        write!(stdout, "[ {} ]\r\n", guess.iter().join(" ")).expect("Not written.");
        // current cursor location on guess
        let mut cursor = vec![' '; guess.len()];
        cursor[guess_cursor.unwrap_or_default()] = '^';
        write!(stdout, "  {}  \r\n", cursor.iter().join(" ")).expect("Not written.");
    }
}

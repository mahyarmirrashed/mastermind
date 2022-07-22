#![forbid(unsafe_code)]

use clap::Parser;
use colored::Colorize;
use itertools::Itertools;

use mastermind::{ColorPeg, Feedback};

/// Feedback peg for indicating correct color code peg placed in right/wrong
/// position with black/white colors, respectively
const FEEDBACK_PEG: &str = "\u{25c9}";

/// Mastermind is a game where the codebreaker tries to guess the pattern in both order and color.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Number of color code pegs to guess each turn
    #[clap(short, long, value_parser = clap::value_parser!(i8).range(3..=6), default_value_t = 4)]
    pegs: i8,

    /// Number of turns before game ends
    #[clap(short, long, value_parser = clap::value_parser!(i8).range(8..=12), default_value_t = 10)]
    turns: i8,
}

/// Program enters here.
fn main() {
    // parse arguments passed to program
    let args = Args::parse();

    println!("Number of turns: {}.", args.turns);
    println!("{}", "I am red!".red());

    let guess = [
        ColorPeg::Blue,
        ColorPeg::Yellow,
        ColorPeg::Red,
        ColorPeg::Magenta,
    ];
    let answer = [
        ColorPeg::Blue,
        ColorPeg::Green,
        ColorPeg::Magenta,
        ColorPeg::Red,
    ];

    let feedback = Feedback::new(&guess, &answer).unwrap();

    println!(
        "[{} {}]",
        std::iter::repeat(FEEDBACK_PEG)
            .take(*feedback.right())
            .intersperse(" ")
            .collect::<String>()
            .black(),
        std::iter::repeat(FEEDBACK_PEG)
            .take(*feedback.wrong())
            .intersperse(" ")
            .collect::<String>()
            .white(),
    );

    println!("[{}]", answer.iter().join(" "));
}

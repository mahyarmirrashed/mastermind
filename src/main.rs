#![forbid(unsafe_code)]

mod game;

use clap::Parser;

use game::Game;

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

    /// Replace colors with numbers and feedback symbols for colorblind accessibility.
    /// Also activated by setting the NO_COLOR environment variable.
    #[clap(short, long)]
    colorblind: bool,
}

fn main() {
    let args = Args::parse();
    let colorblind = args.colorblind || std::env::var_os("NO_COLOR").is_some();

    Game::new(args.pegs as usize, args.guesses as usize, colorblind).run();
}

use clap::Parser;

/// Mastermind is a game where the codebreaker tries to guess the pattern in both order and color.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Number of turns before game ends
    #[clap(short, long, value_parser = clap::value_parser!(i8).range(8..=12), default_value_t = 10)]
    turns: i8,
}

fn main() {
    let args = Args::parse();

    println!("Number of turns: {}.", args.turns);
}

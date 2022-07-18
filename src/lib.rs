/// Color code pegs for hole guessing (will be colored)
const COLOR_PEG: &str = "\u{2b24}";
/// White key peg for correct color code peg placed in wrong position
const RCWP_PEG: &str = "\u{25cf}";
/// Black key peg for correct color code peg placed in correct position
const RCRP_PEG: &str = "\u{25e6}";

/// Subset of the standard eight ANSI colors
#[derive(Eq, Hash, PartialEq)]
enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

/// Feedback provided to codebreaker by codemaker
struct Feedback {
    /// Number of correct color code pegs in wrong position
    wrong: u8,

    /// Number of correct color code pegs in correct position
    correct: u8,
}

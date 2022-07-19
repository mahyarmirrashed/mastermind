use std::collections::HashMap;

/// Color code pegs for hole guessing (will be colored)
const COLOR_PEG: &str = "\u{2b24}";
/// White key peg for correct color code peg placed in wrong position
const RCWP_PEG: &str = "\u{25cf}";
/// Black key peg for correct color code peg placed in correct position
const RCRP_PEG: &str = "\u{25e6}";

/// Subset of the standard eight ANSI colors
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

/// Feedback provided to codebreaker by codemaker
pub struct Feedback {
    /// Number of correct color code pegs in wrong position
    wrong: u8,

    /// Number of correct color code pegs in right position
    right: u8,
}

impl Feedback {
    /// Creates a new feedback structure based on the frequency of color code pegs in the guess and answer.
    pub fn new(guess: &[Color], answer: &[Color]) -> Result<Feedback, &'static str> {
        // preconditions
        debug_assert!(guess.len() > 0);
        debug_assert!(guess.len() == answer.len());

        if guess.len() == 0 {
            return Err("Guess length was zero.");
        } else if guess.len() != answer.len() {
            return Err("Guess and answer length were not equal.");
        }

        // frequency hashmap to store frequencies of answer values
        let mut frequencies: HashMap<Color, u8> = HashMap::new();
        // feedback parameters
        let mut wrong: u8 = 0;
        let mut right: u8 = 0;

        // convert answer list into frequency hashmap
        for color in answer {
            *frequencies.entry(*color).or_insert(0) += 1;
        }

        // count number of incorrect color code pegs
        for color in guess {
            if let Some(frequency) = frequencies.get_mut(color) {
                *frequency -= 1;
                wrong += 1;
            }
        }

        // rebalance and count number of correct color code pegs
        for peg in guess.iter().zip(answer) {
            if peg.0 == peg.1 {
                wrong -= 1;
                right += 1;
            }
        }

        Ok(Feedback { wrong, right })
    }

    /// Getter for wrong field (immutable access).
    pub fn wrong(&self) -> &u8 {
        &self.wrong
    }

    /// Getter for right field (immutable access).
    pub fn right(&self) -> &u8 {
        &self.right
    }
}

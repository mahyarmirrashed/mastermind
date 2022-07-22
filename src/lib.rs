use colored::Colorize;
use core::fmt;
use std::{collections::HashMap, fmt::Display};

/// Color code pegs for hole guessing (will be colored)
const COLOR_PEG: &str = "\u{25cf}";

/// Subset of the standard eight ANSI colors
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ColorPeg {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Display for ColorPeg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let glyph = match self {
            Self::Red => COLOR_PEG.red(),
            Self::Green => COLOR_PEG.green(),
            Self::Yellow => COLOR_PEG.yellow(),
            Self::Blue => COLOR_PEG.blue(),
            Self::Magenta => COLOR_PEG.magenta(),
            Self::Cyan => COLOR_PEG.cyan(),
            Self::White => COLOR_PEG.white(),
        };

        write!(f, "{}", glyph)
    }
}

/// Feedback provided to codebreaker by codemaker
#[derive(Clone)]
pub struct Feedback {
    /// Number of correct color code pegs in wrong position
    pub wrong: usize,

    /// Number of correct color code pegs in right position
    pub right: usize,
}

impl Feedback {
    /// Creates a new feedback structure based on the frequency of color code pegs in the guess and answer.
    pub fn new(guess: &[ColorPeg], answer: &[ColorPeg]) -> Result<Feedback, &'static str> {
        // preconditions
        debug_assert!(guess.len() > 0);
        debug_assert!(guess.len() == answer.len());

        if guess.len() == 0 {
            return Err("Guess length was zero.");
        } else if guess.len() != answer.len() {
            return Err("Guess and answer length were not equal.");
        }

        // frequency hashmap to store frequencies of answer values
        let mut frequencies: HashMap<ColorPeg, usize> = HashMap::new();
        // feedback parameters
        let mut wrong: usize = 0;
        let mut right: usize = 0;

        // convert answer list into frequency hashmap
        for color_peg in answer {
            *frequencies.entry(*color_peg).or_insert(0) += 1;
        }

        // count number of incorrect color code pegs
        for color_peg in guess {
            if let Some(frequency) = frequencies.get_mut(color_peg) {
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
}

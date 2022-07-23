use core::fmt;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::{collections::HashMap, fmt::Display};

use termion::{color, style};

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

impl ColorPeg {
    /// Shifts color peg down wheel.
    pub fn down(&self) -> ColorPeg {
        match self {
            Self::Red => ColorPeg::Green,
            Self::Green => ColorPeg::Yellow,
            Self::Yellow => ColorPeg::Blue,
            Self::Blue => ColorPeg::Magenta,
            Self::Magenta => ColorPeg::Cyan,
            Self::Cyan => ColorPeg::White,
            Self::White => ColorPeg::Red,
        }
    }

    /// Shifts color peg up wheel.
    pub fn up(&self) -> ColorPeg {
        match self {
            Self::Red => ColorPeg::White,
            Self::Green => ColorPeg::Red,
            Self::Yellow => ColorPeg::Green,
            Self::Blue => ColorPeg::Yellow,
            Self::Magenta => ColorPeg::Blue,
            Self::Cyan => ColorPeg::Magenta,
            Self::White => ColorPeg::Cyan,
        }
    }

    // Returns ANSI color for given peg.
    fn color(&self) -> &'static dyn color::Color {
        match self {
            Self::Red => &color::Red,
            Self::Green => &color::Green,
            Self::Yellow => &color::Yellow,
            Self::Blue => &color::Blue,
            Self::Magenta => &color::Magenta,
            Self::Cyan => &color::Cyan,
            Self::White => &color::White,
        }
    }
}

impl Display for ColorPeg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            color::Fg(self.color()),
            COLOR_PEG,
            style::Reset
        )
    }
}

impl Distribution<ColorPeg> for Standard {
    fn sample<R>(&self, rng: &mut R) -> ColorPeg
    where
        R: Rng + ?Sized,
    {
        match rng.gen_range(0..7) {
            0 => ColorPeg::Red,
            1 => ColorPeg::Green,
            2 => ColorPeg::Yellow,
            3 => ColorPeg::Blue,
            4 => ColorPeg::Magenta,
            5 => ColorPeg::Cyan,
            6 => ColorPeg::White,
            _ => unreachable!(),
        }
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

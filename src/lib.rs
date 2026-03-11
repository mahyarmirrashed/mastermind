use core::fmt;
use itertools::Itertools;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::{collections::HashMap, fmt::Display, iter};

use termion::{color, style};

/// Color code pegs for hole guessing (will be colored)
const COLOR_PEG: &str = "\u{25cf}";

/// Feedback peg for indicating correct color code peg placed in right/wrong
/// position with black/white colors, respectively
const FEEDBACK_PEG: &str = "\u{25c9}";

/// Feedback character for correct position in colorblind mode
const FEEDBACK_RIGHT: char = '#';

/// Feedback character for correct color, wrong position in colorblind mode
const FEEDBACK_WRONG: char = 'O';

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
    /// Shifts color peg down wheel (decreases number in colorblind mode).
    #[must_use]
    pub fn down(&self) -> ColorPeg {
        match self {
            Self::White => ColorPeg::Cyan,
            Self::Red => ColorPeg::White,
            Self::Green => ColorPeg::Red,
            Self::Yellow => ColorPeg::Green,
            Self::Blue => ColorPeg::Yellow,
            Self::Magenta => ColorPeg::Blue,
            Self::Cyan => ColorPeg::Magenta,
        }
    }

    /// Shifts color peg up wheel (increases number in colorblind mode).
    #[must_use]
    pub fn up(&self) -> ColorPeg {
        match self {
            Self::White => ColorPeg::Red,
            Self::Red => ColorPeg::Green,
            Self::Green => ColorPeg::Yellow,
            Self::Yellow => ColorPeg::Blue,
            Self::Blue => ColorPeg::Magenta,
            Self::Magenta => ColorPeg::Cyan,
            Self::Cyan => ColorPeg::White,
        }
    }

    // Returns ANSI color for given peg.
    fn color(self) -> &'static dyn color::Color {
        match self {
            Self::White => &color::White,
            Self::Red => &color::Red,
            Self::Green => &color::Green,
            Self::Yellow => &color::Yellow,
            Self::Blue => &color::Blue,
            Self::Magenta => &color::Magenta,
            Self::Cyan => &color::Cyan,
        }
    }

    /// Returns the numeric label (1–7) for colorblind mode.
    fn number(self) -> u8 {
        match self {
            Self::White => 1,
            Self::Red => 2,
            Self::Green => 3,
            Self::Yellow => 4,
            Self::Blue => 5,
            Self::Magenta => 6,
            Self::Cyan => 7,
        }
    }

    /// Returns a display wrapper that renders according to the given mode.
    pub fn display(&self, colorblind: bool) -> ColorPegDisplay<'_> {
        ColorPegDisplay { peg: self, colorblind }
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
            0 => ColorPeg::White,
            1 => ColorPeg::Red,
            2 => ColorPeg::Green,
            3 => ColorPeg::Yellow,
            4 => ColorPeg::Blue,
            5 => ColorPeg::Magenta,
            6 => ColorPeg::Cyan,
            _ => unreachable!(),
        }
    }
}

/// Display wrapper for `ColorPeg` that respects colorblind mode.
pub struct ColorPegDisplay<'a> {
    peg: &'a ColorPeg,
    colorblind: bool,
}

impl Display for ColorPegDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.colorblind {
            write!(f, "{}", self.peg.number())
        } else {
            write!(f, "{}", self.peg)
        }
    }
}

/// Feedback provided to codebreaker by codemaker
#[derive(Clone, Default)]
pub struct Feedback {
    /// Number of correct color code pegs in wrong position
    pub wrong: usize,

    /// Number of correct color code pegs in right position
    pub right: usize,
}

impl Feedback {
    /// Creates a new feedback structure based on the frequency of color code pegs in the guess and answer.
    ///
    /// # Errors
    ///
    /// - Guess length is zero.
    /// - Guess length is not equal to answer length.
    pub fn new(guess: &[ColorPeg], answer: &[ColorPeg]) -> Result<Feedback, &'static str> {
        // preconditions
        debug_assert!(!guess.is_empty());
        debug_assert!(guess.len() == answer.len());

        if guess.is_empty() {
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
                if *frequency > 0 {
                    *frequency -= 1;
                    wrong += 1;
                }
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

    /// Returns a display wrapper that renders according to the given mode.
    pub fn display(&self, colorblind: bool) -> FeedbackDisplay<'_> {
        FeedbackDisplay { feedback: self, colorblind }
    }
}

impl Display for Feedback {
    #[allow(unstable_name_collisions)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display(false))
    }
}

/// Display wrapper for `Feedback` that respects colorblind mode.
pub struct FeedbackDisplay<'a> {
    feedback: &'a Feedback,
    colorblind: bool,
}

impl Display for FeedbackDisplay<'_> {
    #[allow(unstable_name_collisions)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.colorblind {
            let display = iter::repeat(FEEDBACK_RIGHT)
                .take(self.feedback.right)
                .chain(iter::repeat(FEEDBACK_WRONG).take(self.feedback.wrong))
                .map(|c| c.to_string())
                .intersperse(String::from(" "))
                .collect::<String>();
            write!(f, "{}", display)
        } else {
            let right = format!(
                "{}{}{}",
                color::Fg(color::Black),
                FEEDBACK_PEG,
                style::Reset
            );
            let wrong = format!(
                "{}{}{}",
                color::Fg(color::White),
                FEEDBACK_PEG,
                style::Reset
            );
            let display = iter::repeat(right)
                .take(self.feedback.right)
                .chain(iter::repeat(wrong).take(self.feedback.wrong))
                .intersperse(String::from(" "))
                .collect::<String>();
            write!(f, "{}", display)
        }
    }
}

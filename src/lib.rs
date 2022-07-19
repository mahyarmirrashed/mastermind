use std::collections::HashMap;

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
    wrong: usize,

    /// Number of correct color code pegs in right position
    right: usize,
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
        let mut frequencies: HashMap<Color, usize> = HashMap::new();
        // feedback parameters
        let mut wrong: usize = 0;
        let mut right: usize = 0;

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
    pub fn wrong(&self) -> &usize {
        &self.wrong
    }

    /// Getter for right field (immutable access).
    pub fn right(&self) -> &usize {
        &self.right
    }
}

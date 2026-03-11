mastermind(6)

# NAME

mastermind - a game of cunning and logic

# SYNOPSIS

*mastermind* [options]

# DESCRIPTION

Mastermind is a codebreaking game. The program generates a secret sequence of
colored pegs. You have a limited number of guesses to determine the correct
colors and positions. After each guess, feedback is given: a black pin for each
peg that is the correct color in the correct position, and a white pin for each
peg that is the correct color in the wrong position. Deduce the secret code
before you run out of guesses.

# OPTIONS

*-p, --pegs* <3..6>
	Number of color code pegs per guess. Default: 4.

*-g, --guesses* <8..12>
	Number of guesses before the game ends. Default: 10.

*-c, --colorblind*
	Replace colors with numbers (1-7) and use *#* and *O* for feedback pins
	instead of colored symbols. Also activated by setting the _NO_COLOR_
	environment variable.

# CONTROLS

*Left*, *Right* / *h*, *l*
	Move the cursor left and right.

*Up*, *Down* / *k*, *j*
	Cycle through colors (or numbers in colorblind mode).

*Enter*, *Space*
	Submit the current guess.

*q*, *Ctrl-C*, *Ctrl-D*
	Quit the game.

# AUTHORS

Mahyar Mirrashed

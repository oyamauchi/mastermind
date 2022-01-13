# mastermind

A solver for the strategy game
[Mastermind](<https://en.wikipedia.org/wiki/Mastermind_(board_game)>).

This uses Knuth's min-max algorithm. It can solve any 4-pin 6-color case with at
most 5 guesses.

The same algorithm can solve the game with more pins and more colors. The
numbers of pins and colors are compile-time constants defined at the top of
`pins.rs`.

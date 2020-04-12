# Tic Tac Toe

Terminal interpretation of the Tic Tac Toe game written in Rust.


### Roadmap
Turn it into "X On A Row". At the moment, it only supports 3 on a row.
I want to make it so the users can make that decision.

Todo:
- It is not possible to check if a player has won for 3+ on a row.
  - (Even when a player plays 5 on a row, the game ends when a player has 3 on a row)
- Improve code when user doesn't enter correct tile coordinates when prompt for them
  - (Inside prompt_player() function)
- Improve draw() board function. Look into differences between String and &str regarding performance.
  - (Execution of function becomes slower quickly if board length increases, atm)
- Add draw state to game. At the moment a player can only win, if all tiles are filled in and no one can win anymore, the game continues

Done
- It is possible to draw a board of a random size
- It is possible to generate all winnable combinations of an abitrary board size
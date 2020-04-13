# Tic Tac Toe
Terminal interpretation of the Tic Tac Toe game written in Rust.


### Roadmap
Turn it into "X On A Row". At the moment, it only supports 3 on a row.
I want to make it so the users can the decision.

Todo:
- It is not possible to check if a player has won for 3+ on a row.
  - (Even when a player plays 5 on a row, the game ends when a player has 3 on a row)
- Improve code when user doesn't enter correct tile coordinates when prompt for them
  - (Inside prompt_player() function)
- Improve draw() board function. Look into differences between String and &str regarding performance.
  - (Execution of function becomes slower quickly if board length increases, atm)
- Improve draw() board function. At the moment it only prints a board in a nice format if the board length is < 10
- Improve error flow in prompt_player() function.
  - At the moment it doesn't behave correctly when a tile is chosen that had already been chosen.
- Make it so people can play against each other on separate machines using the terminal.
  - Maybe using SSH (need to look into the security aspect aswell though)
- Add an option to play against AI
  - Can use algorithm to always choose corner tiles, if player starts and choses 2 corner tiles in a row -> gg

Done
- It is possible to draw a board of a random size
- It is possible to generate all winnable combinations of an abitrary board size
- Add draw state to game. At the moment a player can only win, if all tiles are filled in and no one can win anymore, the game continues
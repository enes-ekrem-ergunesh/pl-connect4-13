use crate::matrix;

/**
 * This function is used to drop a token into the specified column of the game board.
 * The token is dropped to the lowest available empty cell in the column.
 *
 * @param col: The column number of the game board where the token will be dropped.
 * @param color: The color of the token that will be dropped.
 */
pub fn drop_token(col: usize, color: matrix::Element) {
    // Get the number of rows and columns in the matrix
    let rows = crate::matrix::dimensions().0;

    if matrix::read(0, col) != "Empty" {
        println!("No space, try again!");
        return;
    }

    // Check the first row: if first row is empty and second is not, drop to first row
    if matrix::read(0, col) == "Empty" && matrix::read(1, col) != "Empty" {
        matrix::write(0, col, color);
        // Write the drop to history
        matrix::history_write(color, col);
    }

    // Check the middle rows
    for i in 1..rows {
        if matrix::read(i, col) == "Empty" {
            continue;
        }
        matrix::write(i - 1, col, color);
        // Write the drop to history
        matrix::history_write(color, col);
        break;
    }

    // Check the last row
    if matrix::read(rows - 1, col) == "Empty" {
        matrix::write(rows - 1, col, color);
        // Write the drop to history
        matrix::history_write(color, col);
    }
}

/**
 * This function is used to check if the game is finished by checking if
 * there is 4 consecutive tokens in a row, either horizontally, vertically
 * or diagonally.
 *
 * @param col: usize - The column number of the last token dropped.
 *
 * @return bool - Returns 'true' if the game is finished, 'false' if not.
 */
pub fn is_game_over(col: usize) -> bool {
    // Get the number of rows and columns in the matrix
    let (rows, cols) = crate::matrix::dimensions();

    let history_size = matrix::history_len();
    if history_size >= rows * cols {
      println!("Draw!");
      return true;
    }

    // Get the row position of the last token dropped
    let mut row = 0;
    for i in 0..rows {
        if matrix::read(i, col) != "Empty" {
            row = i;
            break;
        } else {
            continue;
        }
    }
    if row == 0 && matrix::read(0, col) == "Empty" {
        println!("No token found!");
        return false;
    }

    // Check horizontally for a win
    // Check to the right of the dropped token
    let mut i = col + 1;
    let mut in_row = 1;
    // while next token is same with dropped token
    while i < cols && matrix::read(row, i) == matrix::read(row, col) {
        in_row += 1;
        i += 1;
    }
    // Check to the left of the dropped token
    if col > 0 {
        let mut i = col - 1;
        // while next token is same with dropped token
        while matrix::read(row, i) == matrix::read(row, col) && i > 0 {
            in_row += 1;
            i -= 1;
        }
    }
    // println!("DEBUG: in row (horizontal): {}", in_row);

    // If there are 4 or more in a row horizontally, print the winner and return true
    if in_row >= 4 {
        println!("{} wins!", matrix::read(row, col));
        return true;
    }

    // Check vertically for a win
    // Check below the dropped token
    let mut i = row + 1;
    let mut in_row = 1;
    // while next token is same with dropped token
    while i < rows && matrix::read(i, col) == matrix::read(row, col) {
        in_row += 1;
        i += 1;
    }
    // Check above the dropped token
    if row > 0 {
        let mut i = row - 1;
        // while next token is same with dropped token
        while i > 0 && matrix::read(i, col) == matrix::read(row, col) {
            in_row += 1;
            i -= 1;
        }
    }
    // println!("DEBUG: in row (vertical): {}", in_row);

    // If there are 4 or more in a row vertically, print the winner and return true
    if in_row >= 4 {
        println!("{} wins!", matrix::read(row, col));
        return true;
    }

    // Check diagonally for a win (decrease)
    // Check below the dropped token
    let mut i = row + 1;
    let mut j = col + 1;
    let mut in_row = 1;
    // while next token is same with dropped token
    while i < rows && j < cols && matrix::read(i, j) == matrix::read(row, col) {
        in_row += 1;
        i += 1;
        j += 1;
    }
    // Check above the dropped token
    if row > 0 && col > 0 {
        let mut i = row - 1;
        let mut j = col - 1;
        // while next token is same with dropped token
        while matrix::read(i, j) == matrix::read(row, col) && i > 0 && j > 0 {
            in_row += 1;
            i -= 1;
            j -= 1;
        }
    }
    // println!("DEBUG: in row (decrease): {}", in_row);

    // If there are 4 or more in a row diagonally, print the winner and return true
    if in_row >= 4 {
        println!("{} wins!", matrix::read(row, col));
        return true;
    }

    // Check diagonally for a win (increase)
    // Check below the dropped token
    let mut in_row = 1;
    if col > 0 {
        let mut i = row + 1;
        let mut j = col - 1;
        // while next token is same with dropped token
        while i < rows && j > 0 && matrix::read(i, j) == matrix::read(row, col) {
            in_row += 1;
            i += 1;
            j -= 1;
        }
    }
    // Check above the dropped token
    if row > 0 {
        let mut i = row - 1;
        let mut j = col + 1;
        // while next token is same with dropped token
        while i > 0 && j < cols && matrix::read(i, j) == matrix::read(row, col) {
            in_row += 1;
            i -= 1;
            j += 1;
        }
    }
    // println!("DEBUG: in row (increase): {}", in_row);

    // If there are 4 or more in a row diagonally, print the winner and return true
    if in_row >= 4 {
        println!("{} wins!", matrix::read(row, col));
        return true;
    }

    // If there are no more than 4 in a row, print the game not finished message
    // and return false
    println!("Game is not finished!");
    false
}

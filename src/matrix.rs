use lazy_static::lazy_static;
use std::fmt;
use std::sync::RwLock;

/**
 * #[derive(Clone, Copy)] attribute is used to allow the enum to be copied and cloned.
 * 
 * The Element enum represents the possible values for each element in the matrix.
 * It has three variants:
 *     - Empty: representing an empty cell in the matrix
 *     - Red: representing a cell with red token
 *     - Yellow: representing a cell with yellow token * 
 */
#[derive(Clone, Copy)]
pub enum Element {
    Empty,
    Red,
    Yellow,
}

/**
 * Implementing the Display trait for Element enum
 * This allows us to use the {} format specifier to print Element values
 */
impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Element::Empty => write!(f, "Empty"),
            Element::Red => write!(f, "Red"),
            Element::Yellow => write!(f, "Yellow"),
        }
    }
}

lazy_static! {
    /**
     * A global variable of a 2D matrix represented as a vector of vectors
     * The RwLock allows multiple threads to read the matrix, but only one thread to write
     */
    static ref MATRIX: RwLock<Vec<Vec<Element>>> = RwLock::new(Vec::new());
}

/**
 * This function creates the matrix with the specified number of rows and columns
 * and initializes all elements to Element::Empty
 */
pub fn create_matrix(rows: usize, cols: usize) {
    let mut matrix = MATRIX.write().unwrap();
    *matrix = vec![vec![Element::Empty; cols]; rows];
}

/**
 * This function returns the element at the specified position as a string
 */
pub fn read(row: usize, col: usize) -> String {
    if let Ok(matrix) = MATRIX.try_read() {
        matrix[row][col].to_string()
    } else {
        "cannot acquire the lock".to_string()
    }
}

/**
 * This function updates the element at the specified position and column
 * with the specified Element
 */
pub fn write(row: usize, col: usize, color: Element) {
    if let Ok(mut matrix) = MATRIX.try_write() {
        matrix[row][col] = color;
    }
    else {
      println!("Couldn't write: the lock cannot be acquired!");
    }
}

/**
 * This function returns the number of rows and columns in the matrix as a tuple
 */
pub fn dimensions() -> (usize, usize) {
    if let Ok(matrix) = MATRIX.try_read() {
        (matrix.len(), matrix[0].len())
    } else {
        (0, 0)
    }
}

/**
 * This function prints the matrix with Element values, it uses the Display trait
 * implemented before to print the values with their names (Empty,Red,Yellow)
 */
pub fn print() {
    if let Ok(matrix) = MATRIX.try_read() {
        let (rows, cols) = dimensions();
        println!("Matrix data: [");
        for row in 0..rows {
            print!("\t | ");
            for col in 0..cols {
                let output = matrix[row][col].to_string();
                if output == "Yellow" {
                    print!("# | ")
                } else if output == "Red" {
                    print!("+ | ")
                } else {
                    print!("_ | ")
                }
            }
            println!("");
        }
        println!("]\n\tRed -> +\tYellow -> #");
    } else {
        println!("cannot acquire the lock");
    }
}

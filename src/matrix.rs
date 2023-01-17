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

    /**
     * A global variable of a drop history represented as a vector
     * The RwLock allows multiple threads to read the matrix, but only one thread to write
     */
    static ref HISTORY: RwLock<Vec<(Element, usize)>> = RwLock::new(Vec::new());
    static ref HISTORY_SIZE: RwLock<usize> = RwLock::new(0);
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
 * This function creates the history with the multiplication of rows and columns
 * and initializes all elements to Element::Empty
 */
pub fn create_history(rows: usize, cols: usize) {
    let mut history = HISTORY.write().unwrap();
    *history = vec![(Element::Empty, cols*rows+1); cols*rows];
}

/**
 * This function returns the drop move at the specified index of history 
 */
pub fn history_read(index: usize) -> (String, usize) {
    if let Ok(history) = HISTORY.try_read() {
        let output = history[index];
        (output.0.to_string(), output.1)
    } else {
        panic!("cannot acquire the lock!");
    }
}

/**
 * This function will add the specified drop operation to the history
 */
pub fn history_write(color: Element, col: usize) {
    let history_len = history_len();
    if let Ok(mut history) = HISTORY.try_write() {
        history[history_len] = (color, col);
        history_size_increment();
    }
    else {
        panic!("Couldn't write: the lock cannot be acquired!");
    }
}

/**
 * This function will add the specified drop operation to the history
 */
pub fn history_size_increment() {
    if let Ok(mut history_size) = HISTORY_SIZE.try_write() {
        *history_size += 1;
    }
    else {
        panic!("Couldn't write: the lock cannot be acquired!");
    }
}

/**
 * This function returns the length of history 
 */
pub fn history_len() -> usize {
    if let Ok(history_size) = HISTORY_SIZE.try_read() {
        *history_size
    } else {
        panic!("cannot acquire the lock!");
    }
}

/**
 * This function will add the specified drop operation to the history
 */
pub fn history_reset() {
    if let Ok(mut history_size) = HISTORY_SIZE.try_write() {
        *history_size = 0;
        create_history(0, 0);
    }
    else {
        panic!("Couldn't write: the lock cannot be acquired!");
    }
}

/**
 * This function prints the matrix with Element values, it uses the Display trait
 * implemented before to print the values with their names (Empty,Red,Yellow)
 */
pub fn print() {
    if let Ok(matrix) = MATRIX.try_read() {
        let (rows, cols) = dimensions();
        println!("\n");
        // println!("Matrix data: [");
        
        // print the column headers
            print!("\t   ");
        for col in 0..cols{
            print!("{}   ", col+1)
        }
        print!("\n");


        for row in 0..rows {
            print!("\t | ");
            for col in 0..cols {
                let output = matrix[row][col].to_string();
                if output == "Yellow" {
                    print!("# | ");
                } else if output == "Red" {
                    print!("+ | ");
                } else {
                    print!("_ | ");
                }
            }
            println!("");
        }
        // println!("]");
        println!("");

        // Prepare the history strings
        let mut history_red = String::new();
        let mut history_yellow = String::new();
        for i in 0..history_len(){
            let history_i = history_read(i);
            if history_i.0 == "Red"{
                history_red.push_str(&(history_i.1+1).to_string());
                history_red.push_str(", ");
            }
            else if history_i.0 == "Yellow"{
                history_yellow.push_str(&(history_i.1+1).to_string());
                history_yellow.push_str(", ");
            }
        }
        // Remove the last comma
        if history_red.len()>1{history_red.remove(history_red.len()-2);}
        if history_yellow.len()>1{history_yellow.remove(history_yellow.len()-2);}

        // Print Red history
        print!("\tRed (+) ->\t");
        println!("{}", history_red);

        // Print Yellow history
        print!("\tYellow (#) ->\t");
        println!("{}", history_yellow);
        println!("\n");
    } else {
        println!("cannot acquire the lock");
    }
}

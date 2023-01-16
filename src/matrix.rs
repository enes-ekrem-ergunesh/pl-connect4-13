use lazy_static::lazy_static;
use std::fmt;
use std::sync::RwLock;

#[derive(Clone, Copy)]
pub enum Element {
    Empty,
    Red,
    Yellow,
}

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
    static ref MATRIX: RwLock<Vec<Vec<Element>>> = RwLock::new(Vec::new());
}

pub fn create_matrix(rows: usize, cols: usize) {
    let mut matrix = MATRIX.write().unwrap();
    *matrix = vec![vec![Element::Empty; cols]; rows];
}

pub fn read(row: usize, col: usize) -> String {
    if let Ok(matrix) = MATRIX.try_read() {
        matrix[row][col].to_string()
    } else {
        "cannot acquire the lock".to_string()
    }
}

pub fn write(row: usize, col: usize, color: Element) {
    if let Ok(mut matrix) = MATRIX.try_write() {
        matrix[row][col] = color;
    }
    // handle the case where the lock cannot be acquired
    else {
        // e.g. log an error message or try again later
    }
}

pub fn dimensions() -> (usize, usize) {
    if let Ok(matrix) = MATRIX.try_read() {
        (matrix.len(), matrix[0].len())
    } else {
        (0, 0)
    }
}

pub fn print() {
    if let Ok(matrix) = MATRIX.try_read() {
        let (rows, cols) = dimensions();
        println!("Matrix data: [");
        for row in 0..rows {
          print!("[");
          for col in 0..cols {
            let output = matrix[row][col].to_string();
            print!("{}, ", output);
          }
          println!("]");
        }
        println!("]");
    } else {
      println!("cannot acquire the lock");
    }
}

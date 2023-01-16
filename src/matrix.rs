
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
    static ref MATRIX: RwLock<[[Element; 7]; 6]> = RwLock::new([[Element::Empty; 7]; 6]);
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
        matrix[0][col] = color;
    }
    // handle the case where the lock cannot be acquired
    else {
        // e.g. log an error message or try again later
    }
}

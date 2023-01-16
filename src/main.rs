// pub mod algorithms;
// pub mod algorithm_tester;
// pub mod practice;
// pub mod practice_main;
pub mod matrix;


fn main() {
    // crate::practice_main::practice_main::main(); // this is for practice purposes
    
    // Create a matrix with the following dimensions
    matrix::create_matrix(10, 12);

    // Print the element in 9th row 5th column
    println!("Data before: {}", matrix::read(9, 5));

    // Update the element in 9th row 5th column with the new value
    matrix::write(9, 5, matrix::Element::Yellow);

    // Print the element in 9th row 5th column
    println!("Data after: {}", matrix::read(9, 5));
    
    
}

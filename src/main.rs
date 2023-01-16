// pub mod algorithms;
// pub mod algorithm_tester;
// pub mod practice;
// pub mod practice_main;
pub mod matrix;


fn main() {
    // crate::practice_main::practice_main::main(); // this is for practice purposes
    
    println!("Data before: {}", matrix::read(0, 2));

    matrix::write(0, 2, matrix::Element::Red);

    println!("Data after: {}", matrix::read(0, 2));
    
    
}

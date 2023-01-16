pub mod algorithms;
pub mod matrix;
pub mod practice;
pub mod practice_main;

fn main() {
    // crate::practice_main::practice_main::main(); // this is for practice purposes

    // Initialize the matrix with the following dimensions
    matrix::create_matrix(6, 7);

    // Print the matrix to the terminal
    matrix::print();
    
    // Drop some tokens
    algorithms::drop_token(3, matrix::Element::Red);
    algorithms::drop_token(5, matrix::Element::Yellow);
    
    // Print it again
    matrix::print();

    // Check if the game is over (enter the collumn number of last drop)
    let status = algorithms::is_game_over(5);
    println!("{}", status);

}

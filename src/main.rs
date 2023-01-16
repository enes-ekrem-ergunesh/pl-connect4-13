pub mod algorithms;
pub mod matrix;
pub mod practice;
pub mod practice_main;

fn main() {
    // crate::practice_main::practice_main::main(); // this is for practice purposes

    // Create a matrix with the following dimensions
    matrix::create_matrix(6, 7);

    matrix::print();
    
    algorithms::drop_token(2, matrix::Element::Red);
    algorithms::drop_token(3, matrix::Element::Red);
    algorithms::drop_token(3, matrix::Element::Red);
    algorithms::drop_token(4, matrix::Element::Red);
    algorithms::drop_token(4, matrix::Element::Red);
    algorithms::drop_token(4, matrix::Element::Red);
    algorithms::drop_token(5, matrix::Element::Red);
    algorithms::drop_token(5, matrix::Element::Red);
    algorithms::drop_token(5, matrix::Element::Red);
    algorithms::drop_token(5, matrix::Element::Red);

    algorithms::drop_token(5, matrix::Element::Yellow);
    
    matrix::print();

    let stat = algorithms::is_game_over(4);

    // println!("status: {}", stat);

}

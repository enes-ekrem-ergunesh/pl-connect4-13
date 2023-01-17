pub mod algorithms;
pub mod matrix;
pub mod practice;
pub mod practice_main;
pub mod ui;
pub mod storage;


fn main() {

    /* SMALL SIMULATION */
    
    // New game 6*7
    let(rows, cols) = (6, 7);
    matrix::create_matrix(rows, cols);
    matrix::create_history(rows, cols);

    storage::store("test1");
   
    // Drop some tokens
    let drop_to = 4;
  //  algorithms::drop_token(drop_to, matrix::Element::Red);
    matrix::display_board();
    //algorithms::is_game_over(drop_to);
 /*
    let drop_to = 2;
    algorithms::drop_token(drop_to, matrix::Element::Yellow);
    matrix::display_board();
    algorithms::is_game_over(drop_to);
    
    let drop_to = 6;
    algorithms::drop_token(drop_to, matrix::Element::Red);
    matrix::display_board();
    algorithms::is_game_over(drop_to);
    
    let drop_to = 1;
    algorithms::drop_token(drop_to, matrix::Element::Yellow);
    matrix::display_board();
    algorithms::is_game_over(drop_to);
    
    let drop_to = 3;
    algorithms::drop_token(drop_to, matrix::Element::Red);
    matrix::display_board();
    algorithms::is_game_over(drop_to);
    
    let drop_to = 1;
    algorithms::drop_token(drop_to, matrix::Element::Yellow);
    matrix::display_board();
    algorithms::is_game_over(drop_to);
    
    let drop_to = 5;
    algorithms::drop_token(drop_to, matrix::Element::Red);
    matrix::display_board();
    algorithms::is_game_over(drop_to);
 */
    let mut count = 1;
    let mut game_array:[[char;6];7] = [[' ';6];7];

    loop {
        count = ui::ask_user(count, & mut game_array);
    }

    // let s = "5".to_string();
    // let x: usize = s.parse().unwrap();

    // println!("x is {}", x);
    
    
    // Another new game 8*6
  /*  let(rows, cols) = (8, 6);
    matrix::create_matrix(rows, cols);
    matrix::create_history(rows, cols);
    matrix::history_reset();
    */ 
    // Print the matrix
    //matrix::print();

    
}
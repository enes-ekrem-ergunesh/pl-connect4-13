/**
 * This is UI module, this module has the user interface, and act as a breach between user interface and other modules (algorithms, storage)
 */

/**
 * This function prints the menu to the terminal.
 */
use std;
use crate::matrix;
use crate::algorithms;



pub fn print_menu() {
  println!("Menu:");

  println!("\t1 - New Game");
  println!("\t2 - Load");
  println!("\t3 - Quit");
}


fn drop_in_available_slot(mut game_array: & mut [[char;6];7], col: usize, symbol: char, count: i8){
  for i in (0..=6).rev(){

      //If full, try again
      if i == 0{
          //If every slot is full, quit and output "Draw!"
          for x in 0..=7{
              if game_array[x][0] == '#' || game_array[x][0] ==  '+'{
                  if x == 7{
                      println!("Draw!");
                      std::process::exit(0);
                      }
                  }
                  else{
                      break;
                  }
              }
          println!("Column is Full!");
          ask_user(count, &mut game_array);
      }

       else if game_array[col][i-1] == ' '{
            game_array[col][i-1] = symbol;
            matrix::display_board();
            println!("\n\n");
            break;
        }

  }

}

fn get_symbol(count: i8) -> char{
  let symbol;
  if count % 2 == 1{
      symbol = '#';
  }
  else{
      symbol = '+';
  }
  symbol
}

pub(crate) fn ask_user(count: i8, mut game_array: & mut [[char;6];7]) -> i8{
  

  let mut error_count = count;
  
  //This let's us know who's turn it is.
  let symbol = get_symbol(count);
  let mut isRedPlayer = true;
  //Get the input from the user (String).
  let mut input_text = String::new();
  println!("Enter Slot to Drop (1-7) :");
  std::io::stdin()
      .read_line(&mut input_text)
      .expect("failed to read from stdin");

      if isRedPlayer {
        let mut drop_to = input_text.trim().parse().unwrap();
        drop_to -= 1;

        algorithms::drop_token(drop_to, matrix::Element::Red);
        matrix::display_board();
        algorithms::is_game_over(drop_to);
      }

  //Match statements are like switch statements.
  match input_text.trim().parse::<usize>() {
      Err(..) => {
          println!("this was not an integer: {}", input_text);
       
          error_count = count - 1
      },

      _ => {
         

          let input_int: usize = input_text
              .trim()
              .parse()
              .expect("Nope");
          match input_int{
           
              1..=7 => drop_in_available_slot(&mut game_array, input_int-1, symbol, count),
              _ =>  {
                  println!("Not a Valid Slot: {}", input_text);
               
                  error_count = count - 1
              }
          }

      }
  };
 
  error_count + 1
}

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::fs;
use crate::matrix;
use crate::algorithms;

/* Functions to store the data */
// #[derive(Serialize, Deserialize, Debug, Copy)]
// pub enum Token {
//     Element {
//         row: usize,
//         col: usize,
//         color: String,
//     },
// }

pub fn prepare_matrix() -> Vec<(usize, usize)> {
    let mut elements: Vec<(usize, usize)> = Vec::new();

    elements.push(matrix::dimensions());
    elements
}

pub fn prepare_history() -> Vec<(String, usize)> {
    let mut elements: Vec<(String, usize)> = Vec::new();

    for i in 0..matrix::history_len() {
        elements.push(matrix::history_read(i));
    }

    // let json_data = to_string(&elements).unwrap();
    // json_data
    elements
}

pub fn prepare_json(name: &str) -> String {
    let mut elements: Vec<(&str, Vec<(usize, usize)>, Vec<(String, usize)>)> = Vec::new();
    elements.push((name, prepare_matrix(), prepare_history()));

    let json_data = to_string(&elements).unwrap();
    json_data
}

pub fn write(filename: String, text: &str) {
    let mut path: String = "./data/".to_string();
    path.push_str(&filename);
    let data = text;
    fs::write(path, data).expect("Unable to write file");
}

pub fn store(name: &str){
  let mut filename: String = name.to_string();
  filename.push_str(".json");
  write(filename, &prepare_json(name))
}
/* Functions to store the data */

/* Functions to load the data */
pub fn load(name: &str){
  let mut filename: String = name.to_string();
  filename.push_str(".json");
  let json_data = read(filename);
  let data : Vec<(&str, Vec<(usize, usize)>, Vec<(String, usize)>)> = from_str(json_data.as_str()).unwrap();
  println!("name: {:?}", data[0].0);
  println!("matrix data: {:?}", data[0].1);
  let matrix_data = &data[0].1[0];
  println!("history data: {:?}", data[0].2);
  let history_data = &data[0].2;
  
  // Load the game data
  let (rows, cols) = *matrix_data;
  matrix::create_matrix(rows, cols);
  matrix::create_history(rows, cols);

  for i in history_data {
    if i.0 == "Red" {algorithms::drop_token(i.1, matrix::Element::Red)}
    else {algorithms::drop_token(i.1, matrix::Element::Yellow)}
  }
}


pub fn read(filename: String) -> String {
    let mut path: String = "./data/".to_string();
    path.push_str(&filename);
    let data = fs::read_to_string(path).expect("Unable to read file");
    data
}

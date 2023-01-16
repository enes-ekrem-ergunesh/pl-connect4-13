pub mod practice_main {
    use crate::practice::practice::references::modify;
    use crate::matrix;

    pub fn main() {
        let mut num: u32 = 1;
        modify(&mut num);

        crate::practice::practice::sleep(2000);

        println!("num in main: {}", num);

        println!(
            "Global X before: {}",
            crate::practice::practice::references::read_x()
        );

        crate::practice::practice::references::update_x(7);

        println!(
            "Global X later: {}",
            crate::practice::practice::references::read_x()
        );

        // Create a matrix with the following dimensions
        matrix::create_matrix(10, 12);

        // Print the element in 9th row 5th column
        println!("Data before: {}", matrix::read(9, 5));

        // Update the element in 9th row 5th column with the new value
        matrix::write(9, 5, matrix::Element::Red);

        // Print the element in 9th row 5th column
        println!("Data after: {}", matrix::read(9, 5));

        if matrix::read(9, 5) == "Yellow" {
            println!("It is yellow");
        } else {
            println!("It is not yellow");
        }
    }
}

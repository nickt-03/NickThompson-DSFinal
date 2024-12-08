use std::io::{self};

//function to randomly sample because program can take time to run on the full data
fn main() {
    let mut sample_size: Option<usize> = None;

    println!("Would you like to randomly sample the nodes from the graph? (yes/no)");

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut input = input.trim().to_lowercase();

    if input == "yes" {
        println!("How many nodes would you like to randomly sample?");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");


        //default to full dataset if not valid
        if let Ok(number) = input.trim().parse::<usize>() {
            sample_size = Some(number);
        } else {
            println!("Invalid number. Proceeding with the full dataset.");
        }
    }
}
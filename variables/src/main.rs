use std::io;

fn main() {
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    // let mut a:[u32; 5] = [0, 1, 2, 3, 4];
    for index in (0..12).rev() {
        print!("{} ,", months[index]);
    }
    println!("");

    println!("Please enter an array index");
    
    let mut index:String = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read the index from user");

    let index:usize = index.trim().parse().expect("Please enter a valid index number");

    let element = months[index];

    println!("The value at index: {index} is {element}");
    
}
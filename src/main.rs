use std::io;

fn main() {
    let array = [1, 2, 3, 4, 5];

    println!("Please enter an array index between 0..4 :");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number!");

    println!("The value of the element at index {} is : {}", index, array[index]);
}

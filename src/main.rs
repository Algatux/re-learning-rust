use std::io;

fn read_index() -> usize {
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    index
        .trim()
        .parse()
        .expect("Not a number!")
}

fn main() {
    let array = [1, 2, 3, 4, 5];

    println!("Please enter an array index between 0..4 :");

    let index = read_index();

    println!("The value of the element at index {} is : {}", index, array[index]);
}

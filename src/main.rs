use std::io;


fn main() {
    let mut name: String = String::new();

    println!("Please insert your name: ");

    io::stdin()
        .read_line(&mut name)
        .expect("Impossible to read line");

    greeter(&mut name);
} 


fn greeter(name: &mut String) {
    println!("Hello, {}", name)
}
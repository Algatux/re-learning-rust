
fn main() {
    let array = [1, 2, 3, 4, 5];

    let mut index: usize = 0;

    println!("loop");
    loop {
        println!("array element: {}", array[index]);

        index += 1;

        if index == 5 {
            break;
        }
    }

    println!("while");
    index = 0;
    while index < 5 {
        println!("array element: {}", array[index]);
        index += 1;
    }

    println!("for range");
    for number in 0..4 {
        println!("array element: {}", array[number]);
    }

    println!("for values");
    for value in array {
        println!("array element: {}", value);
    }    
}

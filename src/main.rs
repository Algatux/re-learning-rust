

fn main() {
    let tup: (&str, u8) = ("red", 255);
    println!("{} - {}", tup.0, tup.1);

    let (color, value) = tup;
    println!("{} - {}", color, value)
}



fn main() {
    let s: String = String::from("hello world");

    let slice: &str = &s[..5];

    println!("Slice {}", slice);
} 



struct User {
    name: String,
    email: String,
    age: u8
}

fn main() {
    let user = build_user(
        String::from("Alessandro"),
        String::from("fake@email.com"),
        38
    );

    println!("Name: {}, Email: {}, Age {}", user.name, user.email, user.age);
} 

fn build_user(name: String, email: String, age: u8) -> User {
    User { name, email, age }
}
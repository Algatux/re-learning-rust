

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

    let user2: User = User {
        name: String::from("Valeria"),
        ..user
    };

    println!("Name: {}, Email: {}, Age {}", user2.name, user2.email, user2.age);
} 

fn build_user(name: String, email: String, age: u8) -> User {
    User { name, email, age }
}
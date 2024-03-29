

struct User {
    name: String,
    email: String,
    age: u8
}

fn main() {
    let user = User{
        name: String::from("Alessandrio"),
        email: String::from("fake@email.com"),
        age: 38
    };

    println!("Name: {}, Email: {}, Age {}", user.name, user.email, user.age);
} 

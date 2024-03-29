

#[derive(Debug)]
struct Person{
    name: String,
    surname: String
}

fn main() {
    let ale:Person = Person {
        name: String::from("Alessandro"),
        surname: String::from("Galli") 
    };

    dbg!(&ale);

    println!("name: {}, surname: {}", ale.name, ale.surname);
    println!("Person: {:?}", ale);
}


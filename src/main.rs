

struct Person{
    name: String,
    surname: String
}

// Constructor block
impl Person {
    fn new(name: String, surname: String) -> Self {
        Self {name, surname}
    }
}

// Methods block
impl Person {
    fn complete_name(&self) -> String {
        format!("{} {}", self.name, self.surname)
    }

    fn change_name(&mut self, new_name: String) {
        self.name = new_name
    }
}

fn main() {
    let mut ale:Person = Person::new(String::from("Alessandro"),String::from("Galli"));

    ale.change_name(String::from("Giovanni"));

    println!("{}", ale.complete_name());
}


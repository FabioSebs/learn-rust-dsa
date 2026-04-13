// creating shared traits among all structs
trait Greet {
    fn greet(&self) -> String;
}

trait Describe {
    // trait with default implementation
    fn describe(&self) -> String {
        String::from("DESCRIBING...")
    }
}

trait Export {
    fn to_json(&self) -> String;
}

// creating structs to implement traits
struct Person {
    name: String,
}

struct Robot {
    model: String,
}

struct Cat {
    name: String,
}

// implementing traits
impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, I'm {}", self.name)
    }
}

impl Greet for Robot {
    fn greet(&self) -> String {
        format!("BEEP BOOP, I am model {}", self.model)
    }
}

impl Greet for Cat {
    fn greet(&self) -> String {
        format!("{} says meow", self.name)
    }
}

impl Describe for Person {
    fn describe(&self) -> String {
        format!("{} is a person", self.name)
    }
}

impl Export for Person {
    fn to_json(&self) -> String {
        format!(r#"{{"model" : "{}"}}"#, self.name)
    }
}

impl Export for Robot {
    fn to_json(&self) -> String {
        format!(r#"{{"model" : "{}"}}"#, self.model)
    }
}

// function that accepts any object implementing trait!
fn print_greeting(thing: &impl Greet) {
    println!("{}", thing.greet());
}

fn export_and_greet(thing: &(impl Greet + Export)) {
    println!("{}", thing.greet());
    println!("{}", thing.to_json());
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    let robot: Robot = Robot {
        model: String::from("RX-78"),
    };
    let cat: Cat = Cat {
        name: String::from("Whiskers"),
    };

    let entities: Vec<&dyn Greet> = vec![&person, &robot, &cat];

    for entity in &entities {
        println!("{}", entity.greet());
    }

    print_greeting(&person);
    export_and_greet(&robot);
}

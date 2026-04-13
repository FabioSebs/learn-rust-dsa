// traits

pub trait Animal {
    fn create_noise(&self) -> String;
}

pub trait Greet {
    fn get_name(&self) -> String {
        String::from("My name is")
    }

    fn greeting(&self) -> String;
}

// cat struct
struct Cat {
    name: String,
}

impl Cat {
    fn new(name: String) -> Self {
        Cat { name }
    }
}

impl Greet for Cat {
    fn greeting(&self) -> String {
        format!("Hello, {} {}", self.get_name(), self.name)
    }
}

impl Animal for Cat {
    fn create_noise(&self) -> String {
        format!("MEOWW!")
    }
}

// Trait Bounded Functions
fn verify_greeter(greeter: &impl Greet) {
    println!("{} and I am a greeter!", greeter.greeting())
}

// * this is the alternative method to write the function WITHOUT impl keyword
fn verify_greeter_2<T: Greet>(greeter: &T) {
    println!("{} and I am a greeter!", greeter.greeting())
}

fn verify_greeter_and_animal(entity: &(impl Greet + Animal)) {
    println!(
        "{} and I am a greeter and animal! {}",
        entity.greeting(),
        entity.create_noise(),
    )
}

fn verify_greeter_and_animal_2<T: Greet + Animal>(entity: &T) {
    println!(
        "{} and I am a greeter and animal! {}",
        entity.greeting(),
        entity.create_noise(),
    )
}

fn main() {
    let cat = Cat::new(String::from("Mumu"));
    verify_greeter(&cat);
    verify_greeter_2(&cat);

    println!("----------------------------------");

    verify_greeter_and_animal(&cat);
    verify_greeter_and_animal_2(&cat);
}

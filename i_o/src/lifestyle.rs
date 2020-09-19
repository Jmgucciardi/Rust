#[derive(Debug)]
pub struct Person {
    name: String,
    diet: Diet
}

#[derive(Debug)]
pub enum Diet {
    Carnavour,
    Vegetarian,
    Vegan,
    HalfAndHalf
}

pub fn new_person(name: String, diet: Diet) -> Person {
    Person {
        name,
        diet
    }
}

pub fn my_name() {
    println!("My name is Jon");
}

pub fn clayton_granola() {
    println!("Go Vegan!");
}

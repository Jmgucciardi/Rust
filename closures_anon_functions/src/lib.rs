pub struct Account {
    pub age: i32,
    pub body_mass: i32,
}

impl Account {
    pub fn get_stuff() {
        println!("Account: {:?}", Account)
    }
}
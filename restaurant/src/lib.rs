mod front_of_the_house;
// using pub here would enable code that calls our code to refer to the same as it was defined in that codes scope.
pub use crate::front_of_the_house::hosting;



fn serve_order() {}

mod back_of_the_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
    pub struct Breakfast {
        pub toast: String,
        season_fruit: String
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("peaces"),
            }
        }
    }
}


pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal = back_of_the_house::Breakfast::summer("Rye");
//    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_the_house::Appetizer::Soup;
    let order2 = back_of_the_house::Appetizer::Salad;

    // using the use method to bring the front of the house mod into scope lts us reference it like so
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
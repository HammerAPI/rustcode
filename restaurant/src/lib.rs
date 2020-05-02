/*
//#[cfg(test)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table () {}    
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}


    // Breakfast is public, and so is toast, but the fruit is private
    // Customer does not know what fruit will be with meal
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }


    // If an enum is made public, all of its contents are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restuarant() {
    // Absolute Path
    //crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative Path
    front_of_house::hosting::add_to_waitlist();


    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please,", meal.toast);

    // This line won't work as we aren't allowed to see or modify the fruit
    // meal.seasonal_fruit = String::from("blueberries");
    

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
*/
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restuarant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

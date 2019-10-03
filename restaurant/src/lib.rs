mod front_of_house;

fn serve_order() {}

mod back_of_house {
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

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

//re-exporting
pub use crate::front_of_house::hosting as aHosting;
//use self::front_of_house::hosting as rHosting;


pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Use use
    aHosting::add_to_waitlist();
    //rHosting::add_to_waitlist();

    //Relative path
    //front_of_house::hosting::add_to_waitlist();
}
mod front_of_house;

pub use crate::front_of_house::hosting;

fn deliver_order() {}

mod back_of_house {

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            };
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    
    fn cook_order() {}
}

mod customer {
    // use crate::front_of_house::hosting;
    use crate::back_of_house::{Appetizer, Breakfast};
    use crate::front_of_house::hosting::add_to_waitlist as addToWaitlist;
    pub fn eat_at_restaurant() {

        addToWaitlist();

        let mut meal = Breakfast::summer("Rye");
        println!("I chagnge my mind from {}",meal.toast);
        meal.toast = String::from("Wheat");
        println!("To {}", meal.toast);

        println!("What would you like to have in appetizer? We have {:?}, {:?}", Appetizer::Salad, Appetizer::Soup);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
        super::customer::eat_at_restaurant();
        assert_eq!(true, true);
    }
}


pub enum Appetizer {
    // Enum variants are public by default, so we don't need to annotate them with pub
    Soup,
    Salad,
}

fn fix_incorrect_order() {
    cook_order();
    // able to call the private function deliver_order because super:: goes up the module chain to the parent.
    super::deliver_order();
}

fn cook_order() {}

/*
Structs can also be public, but their fields are private by default.
This means that the struct is public, but the seasonal_fruit field is private.
toast is public because it is explicitly marked as public.
 */
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    /*
    This function is necessary to create an instance of Breakfast.
    This is due to the field seasonal_fruit being private on the struct Breakfast
     */
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

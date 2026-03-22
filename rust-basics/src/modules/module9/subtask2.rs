// Module 9: Generics and Traits
// Task 2: Defining Shared Behavior
//
// A trait defines shared behavior.
// Any type implementing the trait must provide the required methods.
pub trait Describable {
    fn describe(&self) -> String;
}

// Item is a concrete data type that can implement Describable.
pub struct Item {
    pub name: String,
    pub price: i32,
}

impl Describable for Item {
    fn describe(&self) -> String {
        // Build the exact output format requested:
        // "<name>: <price> cents"
        format!("{}: {} cents", self.name, self.price)
    }
}

pub fn run() {
    let apple = Item {
        name: "Apple".to_string(),
        price: 150,
    };
    println!("{}", apple.describe());
}

#[cfg(test)]
mod tests {
    use super::{Describable, Item};

    #[test]
    fn describes_item_in_required_format() {
        let item = Item {
            name: "Apple".to_string(),
            price: 150,
        };
        assert_eq!(item.describe(), "Apple: 150 cents");
    }
}

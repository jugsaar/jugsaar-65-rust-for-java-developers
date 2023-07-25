trait Entry {
    type Value;

    fn new(value: Self::Value) -> Self;

    fn get_value(&self) -> &Self::Value;
}

struct SimpleEntry<T> {
    item: T,
}

impl<T> Entry for SimpleEntry<T> {
    type Value = T;

    // "static" factory method
    fn new(value: T) -> SimpleEntry<T> {
        SimpleEntry { item: value } // create an instance of MyStruct
    }

    // "instance method"
    fn get_value(&self) -> &T {
        &self.item
    }
}

fn main() {
    let entry = SimpleEntry::new(42);
    let value_ref = entry.get_value();
    println!("Item: {}", value_ref); // Output: Item: 42
}

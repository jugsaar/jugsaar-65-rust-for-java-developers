trait MyTrait {
    type Item;

    fn create(value: Self::Item) -> MyStruct<Self::Item>;
    fn get_item(&self) -> &Self::Item;
}

struct MyStruct<T> {
    item: T,
}

impl<T> MyTrait for MyStruct<T> {
    type Item = T;

    fn create(value: T) -> MyStruct<T> {
        MyStruct { item: value } // create an instance of MyStruct
    }

    // "instance method"
    fn get_item(&self) -> &T {
        &self.item
    }
}

fn main() {
    let my_instance = MyStruct::create(42);
    let item_ref = my_instance.get_item();
    println!("Item: {}", item_ref); // Output: Item: 42
}

// "Interface" with generic type parameter Item
trait MyTrait {
    type Item;

    fn create_item() -> Self::Item;
}

// Empty struct
struct MyStruct;

// We implement MyTrate for MyStruct
impl MyTrait for MyStruct {
    type Item = i32;

    // "static method"
    fn create_item() -> Self::Item {
        // Self => Typ of implementing Struct
        42
    }
}

fn main() {
    let item: i32 = MyStruct::create_item();
    println!("Item: {}", item); // Output: Item: 42
}

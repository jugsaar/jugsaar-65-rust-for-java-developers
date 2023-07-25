fn main() {
    println!("this is a string literal");

    let x: &str = "this will be stored as a immutable string slice";
    println!("x => {}", x);

    println!("Len(X) => {}", x.len());

    println!("First 4 characters {} {} {} {}", //
             x.chars().nth(0usize).unwrap(), //
             x.chars().nth(1usize).unwrap(), //
             x.chars().nth(2usize).unwrap(), //
             x.chars().nth(3usize).unwrap());

    let y: String = "this will be stored as a mutable String".to_string();
    println!("y => {}", y);

    let mut z: String = y;
    z.push_str(" and now the string has changed");
    println!("z => {}", z);
}

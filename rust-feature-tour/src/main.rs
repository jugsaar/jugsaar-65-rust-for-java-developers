use std::num::ParseIntError;

fn main() {}

fn demo1() {

    // let introduces a variable binding
    let x;
    x = 42; // assigment

    let x = 42;

    // Declaration: Types can be annotated
    let x: i32; //i32 type annotation
    x = 42;

    // Declaration + Initialization: can also be a single line
    let x: i32 = 42;

    // which integer type to use?
    // by default just use i32 -> int in java
    // signed numbers have fewer surprises than unsigned numbers in arithmetic operations
}

// detect uninitialized values
fn demo2() {

    // you can't access unitialized variables
    let x;
    // compiler error:  possibly unitialized value -> In C it would crash at runtime
    // myfunc(x);
    x = 42;

    let y;
    y = 42;
    myfunc(y);
}

// underscore placeholder / to discard values
fn demo3() {

    // this does nothing because 42 is a constant;
    let _ = 42;

    // this calls return_something but throws away it's result
    let _ = return_something();
}

// Tuples
fn demo4() {
    let pair = ("Foo", 42);

    let s = pair.0; // This is "Foo"
    let n = pair.1; // this is 42

    // or, with explicit type annotation
    let pair: (&str, i32) = ("Foo", 13);
    // ...
}

// tuple destructuring
fn demo5() {
    let (some_str, some_num) = ('A', 17);
    assert_eq!(some_str, 'A');
    assert_eq!(some_num, 17);

    let (l, r) = "abcdef".split_at(3);
    // l = abc
    // r = def

    let (_, right) = "abcdef".split_at(3);
    // right = def
}

// semicolon marks the end of a statement
fn demo6() {
    let x = 3;
    let y = 5;
    let z = y + x;
}

// iterator example with fluent interface
// in rust nearly everything is an expression
fn demo7() {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8] // returns a Vec (dynamic array)
        .iter() // turn this to an iterator
        .map(|x| x * 2) // apply closure function to every element  In Java: x -> x * 2
        .fold(0, |x, y| x + y); // fold (reduce) items with the seed and binary operation
}

// functions
fn demo8() {
    // f -> void
    fn greet() {
        println!("Guten Abend!")
    }

    // f -> i32 (arrow denotes return type)
    fn roll_dice() -> i32 {
        4
    }
}

// block scope
fn demo9() {

    // ## block scope and shadowing

    let x = "out";
    {
        let x = "in";
        // this is a different x, only lives as long as block does and does not modify outer ex (shadowing)
        println!("{}", x) // print's in
    }
    println!("{}", x); // then "out"

    // ## blocks are also expressions and can yield a value

    // this
    let x = 42;

    //... is equivalent to this:
    let x = { 42 };

    // ## block can contain multiple statements
    let x = {
        let y = 1; // first statement
        let z = 2; // second statement
        y + z // NO semicolon, this is the "tail" of the block (last expression yields the return value)
    };
}

// implicit return result of last expression
fn demo10() {

    // the following functions are equivalent

    fn fun1() -> i32 {
        return 4;
    }

    fn fun2() -> i32 {
        4
    }
}

// if conditionals are also expressions
fn demo11() {
    fn roll_dice(feeling_lucky: bool) -> i32 {
        if feeling_lucky { // this if expression yields i32
            6
        } else {
            4
        }
    }
}

// match is also an expression, not a statement
fn demo12() {
    fn roll_dice(feeling_lucky: bool) -> i32 {
        match feeling_lucky { // this match yields i32
            true => 6, // true "arm" of match
            false => 4, // true "arm" of match
        }
    }
}

// access members via dot path
fn demo13() {

    //  dots are typically used to access fields of a value
    let a = (10, 20);
    a.0; // == 10

    let amos = get_person(42);
    amos.nickname;

    // or call a method on a value
    let nick = "Ralf";
    nick.len(); // 4
}

// Namespaces and fully qualified names
fn demo14() {
    let least = std::cmp::min(3, 8); // 3

    // my_crate::my_module::my_symbol
    // my_crate: name of crate (module) -> cargo.toml (package name)
    // my_module: name of source file
    // my_symbol: name of function, variable, constant, struct etc.

    // use can bring items into scope
    use std::cmp::min;
    let least = min(7, 1); // 1

    // what's not in the source cannot be used
}

// functions on types and values
fn demo15() {
    let x = "amos".len(); // 4
    let x = str::len("amos"); // also 4

    // str is a primitive type, but many non-primitive types are in scope by default
}

// structs - the backbone of rust's type system
fn demo16() {
    // structs can be declared with the struct keyword
    struct Number {
        odd: bool,
        value: i32,
    }

    // They can be initialized using literals:
    let x = Number { odd: false, value: 2 };
    let y = Number { value: 3, odd: true };
    // the order does not matter!
}

// structs match arms
fn demo17() {
    struct Number {
        odd: bool,
        value: i32,
    }

    fn print_number(n: Number) {
        match n.value {
            1 => println!("One"), // concrete match arm
            2 => println!("Two"), // concrete match arm
            _ => println!("{}", n.value), // catch-all match arm
        }
    }
}

// declare methods on own types
fn demo18() {
    struct Number {
        odd: bool,
        value: i32,
    }

    impl Number {
        fn is_positive(self) -> bool {
            self.value > 0
        }
    }

    let minus_two = Number {
        odd: false,
        value: -2,
    };

    println!("{}", minus_two.is_positive())
}

// immutable structs
fn demo19() {
    struct Number {
        odd: bool,
        value: i32,
    }

    let n = Number {
        odd: true,
        value: 17,
    };

    // n.odd = false; // compiler error: cannot assign to `n.odd` ... n is not declared to be mutable
}

// mutable struct
fn demo20() {
    struct Number {
        odd: bool,
        value: i32,
    }

    // mut makes a variable binding mutable
    let mut n = Number {
        odd: true,
        value: 17,
    };

    n.value = 19; // this is fine
}

// generic functions
fn demo21() {
    fn foobar1<T>(arg: T) {
        // work with arg
    }

    fn foobar2<L, R>(left: L, right: R) {
        // work with left  and right
    }
}

// structs can also be generic
fn demo22() {
    struct Pair<T> {
        a: T,
        b: T,
    }

    let p1 = Pair { a: 3, b: 9 };
    // p1 Pair<i32>

    let p2 = Pair { a: true, b: false };
    // p2 Pair<bool>
}

// vec
fn demo23() {
    let mut v1 = Vec::new();
    v1.push(1);

    let mut v2 = Vec::new();
    v2.push(false);

    // v1 == Vec<i32>
    // v2 == Vec<bool>

    let v1 = vec![1, 2, 3]; // vec! macro -> gives us "vec literals"
    let v2 = vec![true, true, false];
}


// enum
fn demo24() {
    enum Activity {
        Sleeping,
        Eating,
        Coding,
    }

    let activity = Activity::Sleeping;
    let message = match activity {
        Activity::Sleeping => format!("Shhhhh"),
        Activity::Eating => format!("Enjoy!"),
        Activity::Coding => format!("Hack hack hack"),
    };
    println!("{}", message);
}

// enum with state
fn demo25() {
    enum Activity {
        Sleeping(u8),
        // how many hours?
        Eating(String),
        // what food?
        Coding,
    }

    let activity = Activity::Sleeping(10);
    let message = match activity {
        Activity::Sleeping(hours) if hours > 8 => format!("Wake up! After {} hours", hours),
        Activity::Sleeping(_) => format!("Shhhhh"),
        Activity::Eating(food) => format!("Awesome! {}", food),
        Activity::Coding => format!("Java or Rust is fine!"),
    };
    println!("{}", message);
}

// macros
fn demo26() {

    // Examples for macros
    // name!()
    // name![]
    // name!{}
}

// println! is a macro
fn demo27() {

    // use rust playground to show macro expansion
    println!("{}", "Hello JUGSaar!");
}

// panic! is a macro
fn demo28() {
    panic!("This crashes");
}

// option types (similar to Optional in Java)
fn demo29() {
    let o1: Option<i32> = Some(128);
    o1.unwrap(); // yields 128

    let o2: Option<i32> = None;
    o2.unwrap(); // this panics!
}

// option is not a struct but an enum with state
fn demo30() {

    // Option is an enum with two variants -> None (no value) / Some (value present)

    // look at Option
    // look at Option#unwrap
}

// Result enum
fn demo31() {

    // Operations that can fail typically return Result<T, E>
    // errors are "values" in Rust instead of Exceptions as in Java
    // Result<T, E> (T value, E error)
    // functions that can fail typically return a result

    let input = "123";
    let result: Result<i32, ParseIntError> = input.to_string().parse::<i32>();
    let num = match result {
        Ok(num) => num,
        Err(_) => panic!("Could not parse string to int"),
    };
    println!("num={}", num);
}

// Result with error propagation on unwrap or expect
fn demo32(input: String) {
    let result = input.parse::<i32>();
    let num = result.unwrap();

    // expect tells us what to expect when unwrapping the result
    // let num = result.expect("valid input");

    println!("num={}", num);
}

// Result with "if let" destructuring
fn demo33(input: String) {
    if let Ok(num) = input.parse::<i32>() {
        println!("num={}", num);
    }
}

// Return Result to the caller
fn demo34(input: String) -> Result<i32, core::num::ParseIntError> {
    match input.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(e) => Err(e),
    }
}

// Return Result with syntax sugar "?"
fn demo35(input: String) -> Result<i32, core::num::ParseIntError> {
    Ok(input.parse::<i32>()?)
}

// iterators
fn demo36() {
    let natural_numbers = 1..; // constructs a lazily evaluated "Range"

    natural_numbers.take(10).for_each(|num| println!("{}", num));
}

// ranges
fn demo37() {

    // 0 or greater
    (0..).contains(&100);

    // 20 or less than 20
    (..=20).contains(&20);

    // only 3,4,5
    (3..6).contains(&4);
}

// iterable with for loop
fn demo38() {
    for i in vec![52, 49, 21] {
        println!("I like number {}", i)
    }

    // &[...] -> slice
    for i in &[1, 2, 3] {
        println!("I like number {}", i)
    }

    for c in "rust".chars() {
        println!("Give me a {}", c);
    }
}

// iterable over "stream""
fn demo39() {

    for c in "rust for java developers".chars()
        .filter(|c| c.is_lowercase())
        .flat_map(|c| c.to_uppercase())
    {
        print!("{}", c);
    }
    println!();
}


//// Helpers

fn myfunc<T>(value: T) {}

fn return_something() -> i32 {
    0
}

fn get_person(id: i32) -> Person {
    Person { nickname: String::from("Dolly") }
}

struct Person {
    nickname: String,
}
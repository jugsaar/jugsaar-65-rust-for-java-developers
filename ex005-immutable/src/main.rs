fn main() {
    let x = 13;
    x = 32; // ERROR cannot assign twice to immutable variable `x`

    println!("x: {}", x);
}

#[no_mangle]
pub extern "C" fn compute_sum(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

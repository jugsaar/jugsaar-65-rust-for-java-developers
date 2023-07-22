fn main() {
    demo_ownership_beginns_with_assignment();

    demo_ownership_beginns_with_assignment_ends_with_scope();

    demo_reassignment_moves_ownership();
}

fn demo_ownership_beginns_with_assignment() {
    let x = 42; // The assignee (x) becomes the value's sole owner

    println!("x: {}", x);
}

fn demo_ownership_beginns_with_assignment_ends_with_scope() {
    {
        let x = 21; // The assignee (x) becomes the value's sole owner

        println!("x: {}", x);
    }

    // println!("x: {}", x); // ERROR: x not in scope
}

fn demo_reassignment_moves_ownership() {
    let a = vec![1, 2, 3];
    let b = a;

    println!("b: {:?}", b);
    // println!("a: {:?}", a); // ERROR: borrow of moved value: `a`
}

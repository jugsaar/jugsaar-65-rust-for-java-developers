trait Action {
    // bound to an instance
    fn execute(&self);

    // not bound to an instance
    fn execute_static();
}

// Empty struct
struct SimpleAction;

// We implement Action for SimpleAction
impl Action for SimpleAction {
    fn execute(&self) {
        println!("execute Simple")
    }

    fn execute_static() {
        println!("execute_static Simple")
    }
}

fn main() {
    let simple_action = SimpleAction {};
    simple_action.execute();

    SimpleAction::execute_static();
}

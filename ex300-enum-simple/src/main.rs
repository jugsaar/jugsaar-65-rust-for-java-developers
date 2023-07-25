#![allow(unused)]

fn main() {
    let activity = Activity::Sleeping(10);
    let message = match activity {
        Activity::Sleeping(hours) if hours > 8 => format!("Wake up! After {} hours", hours),
        Activity::Sleeping(_) => format!("Shhhhh"),
        Activity::Swimming(location) => format!("Awesome! {}", location),
        Activity::Coding => format!("Java or Rust is fine!"),
    };

    println!("{}", message);
}

enum Activity {
    Sleeping(u8), // how many hours sleept?
    Swimming(String), // location
    Coding,
}

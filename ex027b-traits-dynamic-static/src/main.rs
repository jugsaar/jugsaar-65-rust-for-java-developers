#![allow(unused)]

use std::string;

// A struct that represents a Contact
struct Contact {
    full_name: String,
    since: u16,
}

// We implement some methods for the Contact struct
impl Contact {
    // this is a "static" factory method to create new Contact
    fn from_info(full_name: String, since: u16) -> Contact {
        Contact { full_name, since }
    }

    // This is an "instance" method to render the Contact details
    fn info(&self) -> String {
        format!("{} since {}", self.full_name, self.since)
    }
}

// BusinessCard is an (interface) trait that can render "BusinessCard strings"
trait BusinessCard {
    fn card(&self) -> String;
}

// we implement the BusinessCard trait for the Contact struct
impl BusinessCard for Contact {
    fn card(&self) -> String {
        format!("BusinessCard {}", self.full_name)
    }
}

// we create another HipsterContact struct as a wrapper for ordinary Contact structs
struct HipsterContact {
    // use composition instead of inheritance
    contact: Contact,
}

// We implement the business card trait for the HipsterContact as well
impl BusinessCard for HipsterContact {
    fn card(&self) -> String {
        format!("Hipster BusinessCard {}", self.contact.full_name)
    }
}

fn main() {
    let c = Contact::from_info("John Doe".into(), 2005);

    println!("Hello {}", c.card());

    print_card(&c);
    print_card_2(&c);

    let hc = HipsterContact { contact: c };
    print_card(&hc);
    print_card_2(&hc);
}

// impl is a "trait bound" and accepts objects implementing the BusinessCard trait defined at compile time
// static dispatch! Static dispatch, also known as early binding or compile-time dispatch
fn print_card(b: &impl BusinessCard) {
    println!("BC (static) {}", b.card());
}

// dynamic dispatch! Dynamic dispatch, also known as late binding or runtime dispatch
// allows polymorphism
fn print_card_2(b: &dyn BusinessCard) {
    println!("BC (dynamic) {}", b.card());
}
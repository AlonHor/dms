use crate::document::DocumentTrait;

mod document;

fn main() {
    let mut doc = document::Document::new("My document", "Hey!");

    doc.set_name("Renamed document").unwrap();
    doc.set_content("Hello!").unwrap();

    println!("id: {}", doc.id());
    println!("creation date: {}", doc.creation_date());
    println!("name: {}", doc.name().unwrap());
    println!("content: {}", doc.content().unwrap());
    println!("last modified: {}", doc.last_modified());

    println!("---- VERSIONS ----");

    for version in doc.history().unwrap() {
        println!(" - {}", version);
    }
}

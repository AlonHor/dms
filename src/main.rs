use crate::document::DocumentTrait;

mod document;

fn main() {
    let mut doc = document::Document::new("My document", "Hey!");

    doc.set_name("Renamed document");
    doc.set_new_content("Hello!").unwrap();

    println!("{}", doc.id);
    println!("{}", doc.creation_date);
    println!("{}", doc.read_name());
    println!("{}", doc.read_content());
    for version in doc.read_history() {
        println!(" - {}", version);
    }
}

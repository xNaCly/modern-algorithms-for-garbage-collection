use std::rc::Rc;

#[derive(Debug)]
struct Person {
    name: String,
    age: f64,
}

fn new_person(name: String, age: f64) -> Person {
    Person { name, age }
}

fn main() {
    // Create an Rc that contains a person
    let person = Rc::new(new_person("Rainer Zufall".into(), 42.0));

    // Clone the Rc to create additional references
    // These can be moved to other owners and outlive the original Rc instance
    let clone1 = Rc::clone(&person);
    let clone2 = Rc::clone(&person);

    println!("Reference count of person: {}", Rc::strong_count(&person));

    // Access the data through the cloned references
    println!("clone1 data: {:?}", clone1);
    println!("clone2 data: {:?}", clone2);

    // When the references go out of scope, the reference count decreases
    drop(clone1);
    println!("Count after dropping clone1: {}", Rc::strong_count(&person));

    drop(clone2);
    println!("Count after dropping clone2: {}", Rc::strong_count(&person));

    // At this point, the reference count drops to zero, and the memory is
    // deallocated because the last reference is dropped.
}

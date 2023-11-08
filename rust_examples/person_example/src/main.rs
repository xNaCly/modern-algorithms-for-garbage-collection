struct Person {
    name: String,
    age: f64,
}

fn new_person(name: String, age: f64) -> Person {
    Person { name, age }
}

fn print_person(person: Person) {
    println!("{} is {} years old.", person.name, person.age);
}

fn print_person_borrowed(person: &Person) {
    println!("{} is {} years old.", person.name, person.age);
}

fn main() {
    let person = new_person("Rainer Zufall".into(), 42.0);
    let person1 = person; // person is moved to person1
                          // print_person(person); error: use of moved value: `person`
    print_person(person1);

    {
        let person2 = new_person("Anna Zufall".into(), 13.0);
        print_person_borrowed(&person2);
        print_person_borrowed(&person2); // person2 is still valid
    } // person2 is dropped here
} // person is dropped here

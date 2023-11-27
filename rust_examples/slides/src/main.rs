use std::io::{self, Error};

fn main() -> Result<(), Error> {
    println!("Eingabe: ");

    let line = io::stdin().lines().next();
    if let Some(line) = line {
        println!("Gelesene Eingabe: \"{}\"", line?);
    } else {
        println!("\nKeine Eingabe");
    }

    Ok(())
}

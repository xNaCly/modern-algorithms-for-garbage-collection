use std::io::stdin;

fn main() {
    println!("Eingabe: ");

    let line = stdin().lines().next();
    if let Some(line) = line {
        let line = line.expect("Fehler beim Lesen");
        println!("Gelesene Eingabe: \"{line}\"");
    } else {
        println!("Keine Eingabe");
    }
}

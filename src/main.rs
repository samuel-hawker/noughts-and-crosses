use regex::Regex;
use std::io;

fn main() {
    println!("Start Game!");

    let valid_location = Regex::new(r"^[abc][123]").unwrap();

    let game_state = &mut Vec::new();

    loop {
        println!("Please input x location");

        let mut location = String::new();

        io::stdin()
            .read_line(&mut location)
            .expect("Failed to read line");

        if !valid_location.is_match(&location) {
            println!("Invalid value pick any value between a1 and c3");
            continue;
        }

        if game_state.contains(&location) {
            println!("This space is already filled");
            continue;
        }

        game_state.push(location)
    }
}

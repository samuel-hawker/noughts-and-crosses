use std::io;
mod state;

const BLANK: &str = ".";
const NOUGHT: &str = "O";
const CROSS: &str = "X";

fn main() {
    println!("Start Game!");

    let game_state = &mut state::GameState::new();
    print!("{}", game_state);

    loop {
        println!("Please input x location");

        let mut location = String::new();

        io::stdin()
            .read_line(&mut location)
            .expect("Failed to read line");
        // Trim newline from console
        if location.ends_with('\n') {
            location = location[0..location.len() - 1].to_string();
        }

        if !state::valid_location(&location) {
            println!(
                "Invalid value {} , pick any value between a1 and c3",
                location
            );
        }

        let value = game_state.set_value(location, -1);
        match value {
            Ok(value) => value,
            Err(value) => {
                println!("{}", value);
                continue;
            }
        };
        println!("game state:\n{}", game_state);
    }
}

use regex::Regex;

pub struct GameState {
    squares: [isize; 9],
}

pub fn valid_location(location: &String) -> bool {
    let valid_location = Regex::new(r"^[abc][123]$").unwrap();
    valid_location.is_match(&location)
}

impl GameState {
    pub fn new() -> Self {
        Self { squares: [0; 9] }
    }

    pub fn get_index(&self, location: String) -> Result<usize, String> {
        println!("location : {}", location.as_str());
        match location.as_str() {
            "a1" => Ok(0),
            "a2" => Ok(1),
            "a3" => Ok(2),
            "b1" => Ok(3),
            "b2" => Ok(4),
            "b3" => Ok(5),
            "c1" => Ok(6),
            "c2" => Ok(7),
            "c3" => Ok(8),
            _ => Err("invalid value".to_string()),
        }
    }

    pub fn get_value_by_index(&self, index: usize) -> isize {
        self.squares[index]
    }

    pub fn set_value_by_index(&mut self, index: usize, value: isize) {
        self.squares[index] = value;
    }

    pub fn get_value(&self, location: String) -> Result<isize, String> {
        let index = self.get_index(location);
        let index = match index {
            Ok(index) => {
                println!("index : {}", index);
                index
            }
            Err(index) => {
                // print!("{}", index);
                return Err(index);
            }
        };
        Ok(self.get_value_by_index(index))
    }

    pub fn set_value(&mut self, location: String, value: isize) -> Result<(), String> {
        let index = self.get_index(location);
        let index = match index {
            Ok(index) => {
                println!("index : {}", index);
                index
            }
            Err(index) => {
                // print!("{}", index);
                return Err(index);
            }
        };
        let old_value = self.get_value_by_index(index);
        // let value = match value {
        //     Ok(value) => value,
        //     Err(value) => {
        //         // print!("{}", value);
        //         return Err(value);
        //     }
        // };
        if old_value != 0 {
            return Err(format!("Value already set, found {}", old_value).to_string());
        }
        self.set_value_by_index(index, value);
        Ok(())
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}\n{:?}\n{:?}\n",
            &self.squares[0..3],
            &self.squares[3..6],
            &self.squares[6..9]
        )
    }
}

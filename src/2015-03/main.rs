#![allow(dead_code)]

use std::collections::HashMap;
use std::io;

fn get_input_file() -> String {
    include_str!("data.txt").trim().to_owned()
}

fn get_input() -> String {
    let mut instructions = String::new();

    io::stdin()
            .read_line(&mut instructions)
            .expect("Failed to read directions");

    instructions.trim().to_owned()
}

fn validate(instructions: &String) -> Result<(), char> {
    match instructions
      .chars()
      .find(|c| match valid_directions().get(&c) {
          None => true,
          Some(..) => false
      }) 
    {
          None => Ok(()),
          Some(c) => Err(c),
    }
}

fn hash_position(position: &(i32, i32)) -> String {
    format!("{}|{}", position.0, position.1)
}

fn valid_directions() -> HashMap<char, (i32, i32)> {
    HashMap::from([
        ('>', (1, 0)),
        ('<', (-1, 0)),
        ('^', (0, 1)),
        ('v', (0, -1)),
    ])
}

fn visit(mut visits: HashMap<String, i32>, position: &(i32, i32)) -> HashMap<String, i32> {
    let hashed_position = hash_position(position);
    let count = visits.get(&hashed_position).unwrap_or(&0);

    visits.insert(hashed_position, count+1);

    visits
}

fn main() {
    loop {
        let instructions = get_input_file();

        match validate(&instructions){
            Ok(_) => (),
            Err(c) => {
                println!("You entered an invalid character: {}", c);
                continue;
            }
        };

        let directions = valid_directions();
            
        // Part 1: First year
        let mut current_position = (0, 0);
        let mut year_1_houses_visits = HashMap::new();

        // Visit initial position
        year_1_houses_visits = visit(year_1_houses_visits, &current_position);

        for position in instructions.chars().enumerate() {
            let movement = directions.get(&position.1).unwrap();
            current_position = (current_position.0 + movement.0, current_position.1 + movement.1);
            
            year_1_houses_visits = visit(year_1_houses_visits, &current_position);
        }

        println!("Year 1: Houses visited at least once: {}", year_1_houses_visits.len());

        // Part 2: Next year
        let mut santa_current_position = (0, 0);
        let mut robos_current_position = (0, 0);

        let mut year_2_houses_visits = HashMap::new();
        year_2_houses_visits = visit(year_2_houses_visits, &santa_current_position);
        year_2_houses_visits = visit(year_2_houses_visits, &robos_current_position);

        for (i, position) in instructions.chars().enumerate() {
            let movement = directions.get(&position).unwrap();
            let position_to_visit;

            if i % 2 == 0 {
                santa_current_position = (santa_current_position.0 + movement.0, &santa_current_position.1 + movement.1);
                position_to_visit = &santa_current_position;
            } else {
                robos_current_position = (robos_current_position.0 + movement.0, &robos_current_position.1 + movement.1);
                position_to_visit = &robos_current_position;
            }
            
            year_2_houses_visits = visit(year_2_houses_visits, &position_to_visit);
        }

        println!("Year 2: Houses visited at least once: {}", year_2_houses_visits.len());

        break;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(52, 52)
    }
}
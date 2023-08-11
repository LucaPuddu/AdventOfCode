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

fn valid_directions() -> HashMap<char, (i32, &'static str)> {
    HashMap::from([
        (')', (-1_i32, "Down")),
        ('(', (1_i32, "Up")),
    ])
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

fn final_floor(instructions: &String) -> Option<i32> {
    instructions
        .chars()
        .map(|c| valid_directions().get(&c).unwrap().0)
        .reduce(|acc, dir| acc + dir)
}

fn first_to_basement(instructions: &String) -> Option<i32> {
    let mut current = 0;

    for (i, c) in instructions.chars().enumerate() {
        current += valid_directions().get(&c).unwrap().0;

        if current == -1 {
            return Some((i+1).try_into().unwrap());
        }
    }

    None
}

fn main() {
    println!("~ Help Santa ~");

    println!("Please help Santa find the floor by giving him directions!");
    println!("The valid directions are:");
    for (key, val) in valid_directions().iter() {
        println!("{key} -> move {} floor {}", val.0.abs(), val.1);
    }
    println!("You can enter a sequence of directions (e.g. \"())((()))\")");
    println!("");

    loop {
        println!("Please enter your instructions for Santa");
        let instructions = get_input_file();

        match validate(&instructions){
            Ok(_) => (),
            Err(c) => {
                println!("You entered an invalid character: {}", c);
                continue;
            }
        };
            
        println!("Final floor: {}", final_floor(&instructions).unwrap());

        match first_to_basement(&instructions) {
            Some(floor) => println!("First to basement: {}", floor),
            None => println!("Santa will never get to the basement (floor -1)")
        }

        break;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_correctly() {
        assert_eq!(Ok(()), validate(&"(".to_owned()));
        assert_eq!(Ok(()), validate(&")".to_owned()));
        assert_eq!(Ok(()), validate(&"()))))))))))))))))))))))".to_owned()));
        assert_eq!(Ok(()), validate(&"(((((".to_owned()));
        assert_eq!(Err('h'), validate(&"(((((h)))))".to_owned()));
        assert_eq!(Err('h'), validate(&"h)))))".to_owned()));
        assert_eq!(Err('h'), validate(&"(((((h".to_owned()));
        assert_eq!(Err('h'), validate(&"h".to_owned()));
        assert_eq!(Err('h'), validate(&"hd".to_owned()));
        assert_eq!(Err('h'), validate(&"(((((hijk)))))".to_owned()));
        assert_eq!(Err('k'), validate(&"(((((khijk)))))".to_owned()));
    }

    #[test]
    fn it_calculates_the_final_floor() {
        assert_eq!(0_i32, final_floor(&"(())".to_string()    ).unwrap());
        assert_eq!(0_i32, final_floor(&"()()".to_string()    ).unwrap());
        assert_eq!(3_i32, final_floor(&"(((".to_string()     ).unwrap());
        assert_eq!(3_i32, final_floor(&"(()(()(".to_string() ).unwrap());
        assert_eq!(3_i32, final_floor(&"))(((((".to_string() ).unwrap());
        assert_eq!(-1_i32, final_floor(&"())".to_string()    ).unwrap());
        assert_eq!(-1_i32, final_floor(&"))(".to_string()    ).unwrap());
        assert_eq!(-3_i32, final_floor(&")))".to_string()    ).unwrap());
        assert_eq!(-3_i32, final_floor(&")())())".to_string()).unwrap());
    }

    #[test]
    fn it_calculates_the_first_floor_to_basement() {
        assert_eq!(None,    first_to_basement(&"(())"   .to_string()));
        assert_eq!(None,    first_to_basement(&"()()"   .to_string()));
        assert_eq!(None,    first_to_basement(&"((("    .to_string()));
        assert_eq!(None,    first_to_basement(&"(()(()(".to_string()));
        assert_eq!(Some(1), first_to_basement(&"))(((((".to_string()));
        assert_eq!(Some(3), first_to_basement(&"())"    .to_string()));
        assert_eq!(Some(5), first_to_basement(&"()())"  .to_string()));
    }
}
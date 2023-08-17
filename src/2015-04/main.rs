use md5;
use std::{thread, sync::{Mutex, Arc}, time::Instant, cmp::min};

fn main() {
    let input = "ckczppom";

    //let result = single_threaded(input);
    let result = multi_threaded(input);
    println!("Number found: {}", result);
}

fn single_threaded(input: &str) -> i32 {
    let mut current_number = 0; 

    loop {
        let hash = md5::compute(format!("{}{}", input, current_number));

        if &format!("{:x}", &hash)[..5] == "00000" {
            return current_number;
        }    

        current_number += 1;
    }
}

fn multi_threaded(input: &str) -> i32 {
    let start = Instant::now();

    let answer = Arc::new(Mutex::new(-1));
    let mut handles = vec![];
    let threads = 16;

    for i in 1..=threads {
        let input = input.to_owned();
        let answer = Arc::clone(&answer);

        let handle = thread::spawn(move || {
            let mut current_number = i;
            loop {
                // Every x iterations, make sure other threads haven't already found an answer
                if (current_number - i) % 100000 == 0 {
                    let num = answer.lock().unwrap();
                    if *num != -1 {
                        break;
                    }
                }     

                let hash = md5::compute(format!("{}{}", input, current_number));

                if &format!("{:x}", &hash)[..5] == "00000" {
                    let mut num = answer.lock().unwrap();

                    if *num != -1 {
                        *num = min(*num, current_number);
                    }

                    *num = current_number;
                }    

                current_number += threads;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    let x = *answer.lock().unwrap();
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_threaded_works() {
        assert_eq!(609043, single_threaded("abcdef"));
        assert_eq!(1048970, single_threaded("pqrstuv"));
    }

    #[test]
    fn multi_threaded_works() {
        assert_eq!(609043, multi_threaded("abcdef"));
        assert_eq!(1048970, multi_threaded("pqrstuv"));
    }
}
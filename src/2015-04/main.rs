use md5;
use std::{thread, sync::{Mutex, Arc}, cmp::min, time::Instant};

fn main() {

    let input = "ckczppom";
    //let input = "iwrupvqb";

    println!("================================================== Single threaded");

    let now = Instant::now();
    let must_start_with = "00000";
    let (hash, result) = single_threaded(input, must_start_with);
    println!("Found in: {:?}", now.elapsed());
    print_result(input, must_start_with, result, hash);
    println!();

    let now = Instant::now();
    let must_start_with = "000000";
    let (hash, result) = single_threaded(input, must_start_with);
    println!("Found in: {:?}", now.elapsed());
    print_result(input, must_start_with, result, hash);
    println!();

    println!("================================================== Multi threaded");

    let now = Instant::now();
    let must_start_with = "00000";
    let (hash, result) = multi_threaded(input, must_start_with);
    println!("Found in: {:?}", now.elapsed());
    print_result(input, must_start_with, result, hash);
    println!();

    let now = Instant::now();
    let must_start_with = "000000";
    let (hash, result) = multi_threaded(input, must_start_with);
    println!("Found in: {:?}", now.elapsed());
    print_result(input, must_start_with, result, hash);
    println!();

    let now = Instant::now();
    let must_start_with = "0000000";
    let (hash, result) = multi_threaded(input, must_start_with);
    println!("Found in: {:?}", now.elapsed());
    print_result(input, must_start_with, result, hash);
    println!();
}

fn print_result(input: &str, must_start_with: &str, result: i64, hash: String) {
    println!("==== Hash starting with \"{}\" found for {} =====", must_start_with, input);
    println!("Value hashed: {}{}", input, result);
    println!("Hash: {}", hash);
    println!("Result: {}", result);
}

fn hash(input: &str) -> String {
    format!("{:x}", md5::compute(input))
}

fn single_threaded(input: &str, must_start_with: &str) -> (String, i64) {
    let mut current_number = 0;

    loop {
        let unhashed = format!("{}{}", input, current_number);
        let hash_string = hash(&unhashed);

        if &hash_string[..must_start_with.len()] == must_start_with {
            return (hash_string.to_string(), current_number);
        }

        current_number += 1;
    }
}

fn multi_threaded(input: &str, must_start_with: &str) -> (String, i64) {
    let answer = Arc::new(Mutex::new((String::new(), -1)));
    let mut handles = vec![];
    let threads = 16;

    // Every thread checks numbers with increments of {threads}.
    // Eg. (with 16 threads)
    // Thread 1 checks 1, 17, 34, 49, etc.
    // Thread 2 checks 2, 18, 35, 50, etc.
    // This way, there are no overlaps and the numbers are not constrained (up to the max of i64, that is)
    for i in 1..=threads {
        let input = input.to_owned();
        let must_start_with = must_start_with.to_owned();
        let answer = Arc::clone(&answer);

        let handle = thread::spawn(move || {
            let mut current_number = i;
            loop {
                let unhashed = format!("{}{}", input, current_number);
                let hash_string = hash(&unhashed);

                let mut num = answer.lock().unwrap();

                // Salt found. Make sure it's the smallest to date
                if hash_string[..must_start_with.len()] == must_start_with {
                    let answer;
                    if (*num).1 != -1 {
                        answer = min((*num).1, current_number);
                    } else {
                        answer = current_number;
                    }

                    *num = (hash_string.to_string(), answer);
                    break;
                }
                // Every 1000 iteration, make sure that the answer hasn't been found in another thread
                else if (current_number - i) % 1000 == 0 {
                    let current_answer = (*num).1;

                    // Only stop execution if this thread is already past the current "best answer"
                    if current_answer != -1 && current_number > current_answer {
                        break;
                    }
                } 

                current_number += threads;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    let x = answer.lock().unwrap();
    x.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_threaded_works() {
        assert_eq!(("000001dbbfa3a5c83a2d506429c7b00e".to_string(), 609043), single_threaded("abcdef", "00000"));
        assert_eq!(("000006136ef2ff3b291c85725f17325c".to_string(), 1048970), single_threaded("pqrstuv", "00000"));
    }

    #[test]
    fn multi_threaded_works() {
        assert_eq!(("000001dbbfa3a5c83a2d506429c7b00e".to_string(), 609043), multi_threaded("abcdef", "00000"));
        assert_eq!(("000006136ef2ff3b291c85725f17325c".to_string(), 1048970), multi_threaded("pqrstuv", "00000"));
    }
}
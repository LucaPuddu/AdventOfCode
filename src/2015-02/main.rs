use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Present {
    l: f32,
    w: f32,
    h: f32
}

impl Present {
    fn areas (&self) -> (f32, f32, f32) {
        (
            Present::side_area(self.l, self.w),
            Present::side_area(self.w, self.h),
            Present::side_area(self.h, self.l)
        )
    }

    fn total_area (&self) -> f32 {
        let areas = Present::areas(&self);

        areas.0 +
        areas.1 +
        areas.2
    }

    fn side_area (a: f32, b: f32) -> f32 {
        2.0 * a * b
    }

    fn smallest_area(&self) -> f32 {
        let areas = Present::areas(&self);

        (match areas.0 > areas.1 {
            true => match areas.1 > areas.2 {
                true => areas.2,
                false => areas.1
            },
            false => match areas.0 > areas.2 {
                true => areas.2,
                false => areas.0
            }
        }) / 2.0
    }

    fn total_paper(&self) -> f32 {
        Present::total_area(&self) + Present::smallest_area(&self)
    }
}

fn main() {
    let file = "./src/2015-02/data.txt";

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(file) {
        let mut total_paper = 0.0;

      // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(size) = line {
                let sides: Vec<f32> = size.split('x').map(|num| num.trim().parse().unwrap()).collect();

                total_paper += (Present {
                    l: sides[0],
                    w: sides[1],
                    h: sides[2]
                }).total_paper();
            }
        }

        println!("The total paper needed is {}", total_paper);
    } else {
        println!("File {} not found", file);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_total_area() {
        assert_eq!(52.0, (Present {l: 2.0, w: 3.0, h: 4.0}).total_area())
    }

    #[test]
    fn it_returns_smallest_area() {
        assert_eq!(6.0, (Present {l: 2.0, w: 3.0, h: 4.0}).smallest_area())
    }

    #[test]
    fn it_returns_total_paper() {
        assert_eq!(58.0, (Present {l: 2.0, w: 3.0, h: 4.0}).total_paper())
    }
}
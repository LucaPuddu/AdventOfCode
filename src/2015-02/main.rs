use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn smallest<T: std::cmp::PartialOrd>(a: T, b: T, c: T) -> T {
    match a > b {
        true => match b > c {
            true => c,
            false => b
        },
        false => match a > c {
            true => c,
            false => a
        }
    }
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

    fn perimeters (&self) -> (f32, f32, f32) {
        (
            Present::side_perimeter(self.l, self.w),
            Present::side_perimeter(self.w, self.h),
            Present::side_perimeter(self.h, self.l)
        )
    }

    fn total_area (&self) -> f32 {
        let areas = Present::areas(&self);

        2.0 * (areas.0 + areas.1 + areas.2)
    }

    fn side_area (a: f32, b: f32) -> f32 {
        a * b
    }

    fn side_perimeter (a: f32, b: f32) -> f32 {
        2.0 * (a + b)
    }

    fn smallest_area(&self) -> f32 {
        let areas = Present::areas(&self);

        smallest(areas.0, areas.1, areas.2)
    }

    fn shortest_perimeter(&self) -> f32 {
        let perimeters = Present::perimeters(&self);

        smallest(perimeters.0, perimeters.1, perimeters.2)
    }

    fn volume(&self) -> f32 {
        self.l * self.w * self.h
    }

    fn total_paper(&self) -> f32 {
        Present::total_area(&self) + Present::smallest_area(&self)
    }

    fn total_ribbon(&self) -> f32 {
        Present::shortest_perimeter(&self) + Present::volume(&self)
    }
}

fn main() {
    let file = "./src/2015-02/data.txt";

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(file) {
        let mut total_paper = 0.0;
        let mut total_ribbon = 0.0;

      // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(size) = line {
                let sides: Vec<f32> = size.split('x').map(|num| num.trim().parse().unwrap()).collect();
                let present = Present {
                    l: sides[0],
                    w: sides[1],
                    h: sides[2]
                };

                total_paper += present.total_paper();
                total_ribbon += present.total_ribbon();
            }
        }

        println!("The total paper needed is {}", total_paper);
        println!("The total ribbon needed is {}", total_ribbon);
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

    #[test]
    fn it_returns_shortest_perimeter() {
        assert_eq!(10.0, (Present {l: 2.0, w: 3.0, h: 4.0}).shortest_perimeter())
    }

    #[test]
    fn it_returns_total_ribbon() {
        assert_eq!(34.0, (Present {l: 2.0, w: 3.0, h: 4.0}).total_ribbon())
    }
}
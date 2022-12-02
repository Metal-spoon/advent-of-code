use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    println!("Advent of code day one");
    let mut current_segment: i32 = 0;
    let mut max_segment: i32 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(text) = line {
                if text == "" {
                    if current_segment > max_segment {
                        max_segment = current_segment;
                    }
                    current_segment = 0;
                } else {
                    current_segment += text.trim().parse::<i32>().expect("please give a valid number");
            }
            }
        }
        println!("{}", max_segment);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
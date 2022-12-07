use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;


fn main() {
    println!("Advent of code day one");
    let mut current_segment: i32 = 0;
    let mut segments: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(text) = line {
                if text == "" {
                    segments.push(current_segment);
                    current_segment = 0;
                } else {
                    current_segment += text.trim().parse::<i32>().expect("please give a valid number");
            }
            }
        }
        segments.sort_by(|a, b| b.cmp(a));
        let top_3_sum: i32 = segments.iter().take(3).sum();
        println!("Top segment: {}", segments[0]);
        println!("Sum of top 3: {}", top_3_sum)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
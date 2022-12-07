use std::{io::{self, BufRead}, fs::File, path::Path, string};

fn main() {
    let mut total_score = 0;
    let mut total_score_inverted: i32 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(strat) = line {
            let opponent_move = strat.chars().nth(0).unwrap().to_string();
            let my_move = strat.chars().nth(2).unwrap().to_string();
            let score = get_score(&opponent_move, &my_move);
            let score_inverted: i32 = get_score_inverted(&opponent_move, &my_move);
            total_score += score;   
            total_score_inverted += score_inverted;
        }
        }

        println!("{}", total_score);
        println!("{}", total_score_inverted);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_score(opponent_move: &str, my_move: &str) -> i32 {
    match opponent_move {
        "A" => match my_move {
            "X" => return 4,
            "Y" => return 8,
            "Z" => return 3,
            &_ => {println!("FOUT"); return 0}
        },
        "B" => match my_move {
            "X" => return 1,
            "Y" => return 5,
            "Z" => return 9,
            &_ => {println!("FOUT"); return 0}
        },
        "C" => match my_move {
            "X" => return 7,
            "Y" => return 2,
            "Z" => return 6,
            &_ => {println!("FOUT"); return 0}
        },
        &_ => return 100000
    }
}

fn get_score_inverted(opponent_move: &str, my_move: &str) -> i32 {
    match my_move {
        "X" => match opponent_move { //Lose
            "A" => return 3,
            "B" => return 1,
            "C" => return 2,
            &_ => {println!("FOUT"); return 0}
        },
        "Y" => match opponent_move { //Draw
            "A" => return 4,
            "B" => return 5,
            "C" => return 6,
            &_ => {println!("FOUT"); return 0}
        },
        "Z" => match opponent_move { //Win
            "A" => return 8,
            "B" => return 9,
            "C" => return 7,
            &_ => {println!("FOUT"); return 0}
        },
        &_ => return -1

    }
}

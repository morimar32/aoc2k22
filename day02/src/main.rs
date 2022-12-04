use std::fs;
#[path="./match.rs"]
pub mod day2;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let matchData = data.split("\n");
    let mut matches : Vec<day2::Match> = Vec::new();

    for m in matchData {
        let v : Vec<String> = m.split(" ").map(str::to_string).collect();
        //let round = day2::Match::new(String::from(&v[0]), String::from(&v[1]));
        let round = day2::Match::selectFromOutcome(String::from(&v[0]), String::from(&v[1]));
        matches.push(round);
    }

    let mut total = 0;
    for m in matches {
        total += m.score();
    }

    println!("the total score is: {}", total);
}

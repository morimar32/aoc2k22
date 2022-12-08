use std::fs;

fn main() {
    //let unique_range = 4; //part 1
    let unique_range = 14; //part 2
    let data: Vec<char> = fs::read_to_string("input.txt").unwrap().chars().collect();
    for i in unique_range..data.len() {

        let mut tmp = Vec::<char>::new();
        for j in 0..unique_range {
            tmp.push(data[(i - j)]);
        }
        tmp.sort();
        tmp.dedup();
        if tmp.len() == unique_range {
            println!("{}", i+1);
            break;
        }
    }
}

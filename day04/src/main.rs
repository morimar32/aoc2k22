use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.split("\n").collect::<Vec<&str>>();
    
    for line in lines {
        println!("{}", line);
        let (group1, group2) = split_to_groups(&line);
        let (group1_low, group1_high) = split_to_range(&group1);
        let (group2_low, group2_high) = split_to_range(&group2);

        let mut is_overlap = false;
        if group1_low <= group2_low && group1_high >= group2_high {
            is_overlap = true;
        }

        println!("{} - {} : {}", group1, group2, is_overlap);
    }
}

fn split_to_groups(input: &str) -> (&str, &str) {
    let vals = input.split(",").collect::<Vec<&str>>();
    return (vals[0], vals[1]);
}

fn split_to_range(input: &str) -> (i64, i64) {
    let vals = input.split("-").collect::<Vec<&str>>();
    let left = vals[0].parse::<i64>().unwrap();
    let right = vals[1].parse::<i64>().unwrap();
    return (left, right);
}
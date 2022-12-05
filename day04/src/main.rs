use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.split("\n").collect::<Vec<&str>>();
    let mut total_overlap = 0;
    let mut partial_overlap = 0;

    for line in lines {
        let (group1, group2) = split_to_groups(&line);
        let (group1_low, group1_high) = split_to_range(&group1);
        let (group2_low, group2_high) = split_to_range(&group2);

        let mut is_overlap = false;
        let mut partial = false;
        if group1_low <= group2_low && group1_high >= group2_high {
            is_overlap = true;
        } else if group2_low <= group1_low && group2_high >= group1_high {
            is_overlap = true;
        }
        if is_overlap {
            total_overlap += 1;
        }

        if group1_high >= group2_low && group1_high <= group2_low {
            partial = true;
        }
        if group2_high >= group1_low && group2_high <= group1_low {
            partial = true;
        }
        if group1_low >= group2_low && group1_low <= group2_high {
            partial = true;
        }
        if group2_low >= group1_low && group2_low <= group1_high {
            partial = true;
        }
        if partial || is_overlap {
            partial_overlap += 1;
        }
    }

    println!("total overlapping groups is {} and partial overlap {}", total_overlap, partial_overlap);
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
use std::fs;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.split("\n").collect::<Vec<&str>>();
    let mut stacks = setup_stacks();

    for i in 0..9 {
        println!("{}: {:?}", i+1, &stacks[i])
    }

    for line in lines {
        let (count, orig, dest) = get_inputs(&line);
        let mut tmp = Vec::<char>::new();
        for _ in 0..count {
            let t = stacks[orig - 1].pop().unwrap();
            tmp.push(t);
        }

        //comment out for part 1
        tmp.reverse();

        for t in tmp {
            stacks[dest - 1].push(t);
        }
    }
    println!("--------------------------------------------------");

    for i in 0..9 {
        println!("{}: {:?}", i+1, &stacks[i])
    }
    println!("--------------------------------------------------");
    println!("the top of each crate is: ");
    for i in 0..9 {
        print!("{:?}", &stacks[i].pop().unwrap());
    }
    println!("\n--------------------------------------------------");

}

//         [H]         [S]         [D]
//     [S] [C]         [C]     [Q] [L]
//     [C] [R] [Z]     [R]     [H] [Z]
//     [G] [N] [H] [S] [B]     [R] [F]
// [D] [T] [Q] [F] [Q] [Z]     [Z] [N]
// [Z] [W] [F] [N] [F] [W] [J] [V] [G]
// [T] [R] [B] [C] [L] [P] [F] [L] [H]
// [H] [Q] [P] [L] [G] [V] [Z] [D] [B]
// 1   2   3   4   5   6   7   8   9 

fn setup_stacks() -> [Vec<char>;9] {
    let one = std::vec!['H', 'T', 'Z', 'D'];
    let two = std::vec!['Q', 'R', 'W', 'T', 'G', 'C', 'S'];
    let three = std::vec!['P', 'B', 'F', 'Q', 'N', 'R', 'C', 'H'];
    let four = std::vec!['L', 'C', 'N', 'F', 'H', 'Z'];
    let five = std::vec!['G', 'L', 'F', 'Q', 'S'];
    let six = std::vec!['V', 'P', 'W', 'Z', 'B', 'R', 'C', 'S'];
    let seven = std::vec!['Z', 'F', 'J'];
    let eight = std::vec!['D', 'L', 'V', 'Z', 'R', 'H', 'Q'];
    let nine = std::vec!['B', 'H', 'G', 'N', 'F', 'Z', 'L', 'D'];

    return [one, two, three, four, five, six, seven, eight, nine];
}

fn get_inputs(input: &str) -> (usize, usize, usize) {
    let re = Regex::new(r" [\d]{1,2}( |$)").unwrap();
    let mv = re.find_iter(input).map(|item| item.as_str().trim().parse::<usize>().unwrap() ).collect::<Vec<usize>>();
    return (mv[0], mv[1], mv[2]);
}

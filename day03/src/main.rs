use std::fs;

fn main() {
    problem2();
}

fn problem2() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.split("\n").collect::<Vec<&str>>();

    let mut total: i32 = 0;
    for i in 0..lines.len() / 3 {
        let offset = 3 * i;
        let elf1 = lines[offset + 0];
        let elf2 = lines[offset + 1];
        let elf3 = lines[offset + 2];

        for c in elf1.chars() {
            if ( elf2.contains(c) && elf3.contains(c)) {
                let mut v = c as i32 - 96;
                if v < 0 {
                    v += 58
                }
                total += v;
                break;
            }
        }

    }
    println!("the total for the groups is {}", total);
}


fn problem1() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.split("\n").collect::<Vec<&str>>();
    let mut total: i32 = 0;
    for line in lines {
        let len = line.len() / 2;
        let left: &str = &line[0..len];
        let right: &str = &line[len..];
        let mut matches = Vec::<i32>::new();

        println!("{}\n{}", left, right);

        for c in left.chars() {
            if right.contains(c) {
                let mut v = c as i32 - 96;
                if v < 0 {
                    v += 58
                }
                if !matches.contains(&v) {
                    matches.push(v);
                    print!("{}", c);
                }
            }
        }
        let mut lineTotal: i32 = 0;
        for v in matches {
            lineTotal += v;
        }
        total += lineTotal;
        println!("-------------------------");
    }

    println!("total off is {}", total);
}
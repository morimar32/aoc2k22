use std::fs;

fn main() {
    let data = fs::read_to_string ("input.txt").unwrap();
    let elves = data.split("\n\n");
    let mut max = 0;
    let mut elfTotal = Vec::new();

    for s in elves {
        let items = s.split("\n");
        let mut groupVal = 0;
        for item in items {
            groupVal += item.parse::<i32>().unwrap();
        }
        elfTotal.push(groupVal);
        if (groupVal > max) {
            max = groupVal;
        }
    }
    elfTotal.sort();
    elfTotal.reverse();
    println!("the largest group is {}", elfTotal.first().unwrap() );

    let top3 = elfTotal[0..3].to_vec();
    let  mut top3Val = 0;
    for t in top3 {
        top3Val += t;
    }
    println!("the total of the top 3 {}", top3Val );

}

use std::fs;

pub fn solution() {
    let content = fs::read_to_string("input.txt").expect("Should have read the file");
    let mut cals = content.split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();
    cals.sort_by(|a, b| b.cmp(a));
    let sum:u32 = cals[0..3].iter().sum();
    println!("{:?}", sum);

}
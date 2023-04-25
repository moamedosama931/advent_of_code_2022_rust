use std::fs;

pub fn solution() {
    let content = fs::read_to_string("input.txt").expect("Should have read the file");
    let cals = content.split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();

    println!("{}", cals);
}
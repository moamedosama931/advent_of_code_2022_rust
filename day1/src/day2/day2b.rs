use std::fs::File;
use std::io::{ self, BufRead, BufReader };

pub fn solution() {
    let lines = read_lines("day2.txt");
    let mut result:usize = 0;
    for line in lines {
        let s = line.unwrap();
        let bytes = s.as_bytes();
        let opponent_shape = (bytes[0] - b'A') as i8;
        let outcome = (bytes[2] - b'X') as i8;
        let my_shape = (opponent_shape - 1 + outcome).rem_euclid(3);

        // Shape Score is my shape + 1 (Since X,Y,Z are 0,1,2)
        let my_shape_score = my_shape + 1;

        // total score = 3 * score since score are 0, 3, 6
        let total_score = (3 * outcome) + my_shape_score;

        println!("Opponent = {}, My = {}, Shape Score = {}, Outcome = {}", opponent_shape, my_shape, my_shape_score, outcome);
        result += (total_score as usize);
    }

    println!("The solution of Day2 b = {}", result)
}

fn read_lines(filename: &str) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

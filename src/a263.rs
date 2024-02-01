// Beautiful Matrix
// https://codeforces.com/contest/263/problem/A

/*
* We will read the matrix and find the position of 1.
* The nth line where 1 is present will be row position.
* The nth character which is 1 will be column position.
* The middle position is (3, 3).
* The difference between row~3 and column~3 will be the minimum number of moves.
*/

pub fn main() {
    let mut input = String::new();
    let mut row: i32 = 0;
    let mut column: i32 = 0;
    for _ in 0..5 {
        input.clear(); // Otherwise new line will be added to the previous input instead of a new input line.
        std::io::stdin().read_line(&mut input).unwrap();
        let input: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        row += 1;
        if input.contains(&1) {
            column = input.iter().position(|&x| x == 1).unwrap() as i32 + 1;
            break;
        }
    }
    println!("{}", (row - 3).abs() + (column - 3).abs());
}

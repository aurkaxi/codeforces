// Domino Piling
// https://codeforces.com/problemset/problem/50/A

/*
* AxB is the area of the board. Each domino covers 2 squares.
* So we divide the area by 2.
* If even than we can place all the dominoes.
* If odd then we have to leave one square empty. Which will be trunacated by integer division.
*/

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let m = input[0];
    let n = input[1];
    println!("{}", m * n / 2);
}

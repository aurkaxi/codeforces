// Elephant
// https://codeforces.com/problemset/problem/617/A

/*
* We will first try the maximum number of positions we can move.
* if it's divisible by the max, good to go, we take floor division amount step.
* for the remainder we try the next lower number of positions to move.
*/

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut target: i64 = input.trim().parse().unwrap();

    let mut steps = 0;
    for i in (1..=5).rev() {
        steps += target / i;
        target %= i;
    }
    println!("{}", steps);
}

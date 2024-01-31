// Watermelon
// https://codeforces.com/contest/4/problem/A

// If each of them gets even weight then the sum of weights will be even, too.
// also it must be greater than two otherwise each of them will get 1 kg which is not even.

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();

    let answer = if num > 2 && num % 2 == 0 { "YES" } else { "NO" };
    println!("{}", answer);
}

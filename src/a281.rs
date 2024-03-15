// Word Capitalization
// https://codeforces.com/contest/281/problem/A

/*
* We will iterate through each character and capitalize the first index.
*/

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let output: String = input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect();
    println!("{}", output);
}

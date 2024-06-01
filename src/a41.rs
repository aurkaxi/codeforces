// Translation
// https://codeforces.com/problemset/problem/41/A

/*
* Reverse a string and verify if it equal to another one.
*/

pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t = t.trim();

    if s.chars().rev().collect::<String>() == t.chars().collect::<String>() {
        println!("YES");
    } else {
        println!("NO")
    }
}

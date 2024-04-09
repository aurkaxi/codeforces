// Stones on the Table
// https://codeforces.com/problemset/problem/266/A

/*
* We will iterate through the array
* and count the number of chars that matches their previous char.
* that should be the number of chars needs to be removed.
*/

pub fn main() {
    let mut length = String::new();
    std::io::stdin().read_line(&mut length).unwrap();
    let length: i64 = length.trim().parse().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let stones: Vec<String> = input.trim().chars().map(|x| x.to_string()).collect();

    let mut count = 0;
    for i in 1..length as usize {
        if stones[i] == stones[i - 1] {
            count += 1;
        }
    }
    println!("{}", count);
}

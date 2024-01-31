// Way too long words
// https://codeforces.com/contest/71/problem/A

// If a word is longer than 10 letters, we will print the first letter, lenght of the word - 2, and the last letter.

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();

    for _ in 0..num {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let word = input.trim();

        if word.len() > 10 {
            println!(
                "{}{}{}",
                word.chars().next().unwrap(),
                word.len() - 2,
                word.chars().last().unwrap()
            );
        } else {
            println!("{}", word);
        }
    }
}

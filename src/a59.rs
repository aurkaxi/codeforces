// Word
// https://codeforces.com/problemset/problem/59/A

/*
* First we will count the number of uppercase and lowercase letters
* to decide final form.
* then w econvert the string to desired lettercase.
*/

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    let mut lower = 0;
    let mut upper = 0;

    s.chars().for_each(|x| {
        if x.is_lowercase() {
            lower += 1
        } else {
            upper += 1
        }
    });

    if lower < upper {
        println!("{}", s.to_uppercase())
    } else {
        println!("{}", s.to_lowercase())
    }
}

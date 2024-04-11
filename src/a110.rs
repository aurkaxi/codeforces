// Nearly Lucky Number
// https://codeforces.com/problemset/problem/110/A

/*
*we will iterate and count the number of 4 and 7
*if the count is 4 or 7. then it's nearly lucky number.
*/

fn is_near_luck(num: &str) -> bool {
    let mut n = 0;
    num.chars().for_each(|c| {
        if c == '4' || c == '7' {
            n += 1
        }
    });

    n == 4 || n == 7
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if is_near_luck(input) {
        println!("YES");
    } else {
        println!("NO");
    }
}

// Nearly Lucky Number
// https://codeforces.com/problemset/problem/110/A

/*
* simply treat the input as a string and iterate over each char.
* if we find any char other that 4 and 7, we immidiately break the iteration
* if no other char found than we return YES
*/

/*
* we will check if the iteration contains both 4 and 7.
* otherwise it's not a nearly lucky number.
* and then we check if it has no other digits.
*/

fn is_near_luck(num: &str) -> bool {
    // it has both 4 & 7
    if !(num.contains('4') && num.contains('7')) {
        return false;
    }

    // it doesn't have anything else
    for x in num.chars() {
        if x != '4' && x != '7' {
            return false;
        }
    }

    true
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

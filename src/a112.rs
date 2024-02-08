// Petya and Strings
// https://codeforces.com/problemset/problem/112/A

/*
* We will iter through each nth charcter for both and compare them.
* If they are not equal then we don't need to compare the rest of the string. We will just print the result.
* If they are equal then we continue to the next character untill we find a difference or reach the end of the string.
* If we reach the end of the string then the strings are equal.
*/

pub fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();
    std::io::stdin().read_line(&mut str1).unwrap();
    std::io::stdin().read_line(&mut str2).unwrap();
    let str1 = str1.trim().to_lowercase();
    let str2 = str2.trim().to_lowercase();

    let len = str1.len();

    for i in 0..len {
        let current_char_from_str1 = str1.chars().nth(i).unwrap();
        let current_char_from_str2 = str2.chars().nth(i).unwrap();
        match current_char_from_str1.cmp(&current_char_from_str2) {
            std::cmp::Ordering::Less => {
                println!("-1");
                return;
            }
            std::cmp::Ordering::Greater => {
                println!("1");
                return;
            }
            std::cmp::Ordering::Equal => {
                continue;
            }
        }
    }
    println!("0");
}

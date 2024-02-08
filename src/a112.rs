// Petya and Strings
// https://codeforces.com/problemset/problem/112/A

/*
* Umm, well in rust we can just use the cmp method to compare strings.
* It compares them lexicographically by default.
*/

pub fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();
    std::io::stdin().read_line(&mut str1).unwrap();
    std::io::stdin().read_line(&mut str2).unwrap();
    let str1 = str1.trim().to_lowercase();
    let str2 = str2.trim().to_lowercase();

    // let len = str1.len();

    // for i in 0..len {
    //     let current_char_from_str1 = str1.chars().nth(i).unwrap();
    //     let current_char_from_str2 = str2.chars().nth(i).unwrap();
    //     match current_char_from_str1.cmp(&current_char_from_str2) {
    //         std::cmp::Ordering::Less => {
    //             println!("-1");
    //             return;
    //         }
    //         std::cmp::Ordering::Greater => {
    //             println!("1");
    //             return;
    //         }
    //         std::cmp::Ordering::Equal => {
    //             continue;
    //         }
    //     }
    // }
    // println!("0");

    match str1.cmp(&str2) {
        std::cmp::Ordering::Less => {
            println!("-1");
        }
        std::cmp::Ordering::Greater => {
            println!("1");
        }
        std::cmp::Ordering::Equal => {
            println!("0");
        }
    }
}

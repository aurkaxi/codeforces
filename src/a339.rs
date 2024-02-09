// Helpful Maths
// https://codeforces.com/problemset/problem/339/A

/*
* We will split the input and take all the digits.
* Then Sort them and join them again with '+'
*/

pub fn main() {
    let mut problem = String::new();
    std::io::stdin().read_line(&mut problem).unwrap();
    let mut digits: Vec<u32> = problem
        .trim()
        .split('+')
        .map(|x| x.parse().unwrap())
        .collect();
    digits.sort();

    // let mut result = String::new();
    // for digit in digits {
    //     result.push_str(&digit.to_string());
    //     result.push('+');
    // }
    // result.pop();
    // println!("{}", result);

    let digit_str: Vec<String> = digits.iter().map(|x| x.to_string()).collect(); // Because we can't join u32 type with "+" which is string type.
    let result = digit_str.join("+");
    println!("{}", result);
}

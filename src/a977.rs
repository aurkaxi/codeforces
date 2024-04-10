// Wrong Subtraction
// https://codeforces.com/problemset/problem/977/A

/*
we will just iter over every turns and do what tanya would.
*/

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let mut num: isize = input.next().unwrap().parse().unwrap();
    let times: isize = input.next().unwrap().parse().unwrap();

    for _ in 0..times {
        if num % 10 == 0 {
            num /= 10;
        } else {
            num -= 1;
        }
    }

    println!("{}", num)
}

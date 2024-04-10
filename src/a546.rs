// Soldier and Bananas
// https://codeforces.com/problemset/problem/546/A

/*
* iter over each apple and sum their costs.
* if he doesn't have enough balance, print the difference.
*/

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let price: isize = input.next().unwrap().parse().unwrap();
    let balance: isize = input.next().unwrap().parse().unwrap();
    let count: isize = input.next().unwrap().parse().unwrap();
    let mut cost = 0;

    for i in 1..=count {
        cost += price * i;
    }

    if cost > balance {
        println!("{}", cost - balance);
    } else {
        println!("0");
    }
}

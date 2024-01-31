// Next Round
// https://codeforces.com/contest/158/problem/A

/*
* We will split the input and get a iterator.
* We will filter that iterator with a closure that checks if element is greater than 0 and less than or equal to kth element.
* That will give us a iterator with only elements that are greater than 0 and less than or equal to kth element.
* We will count the number of elements in that iterator.
* That will be the number of participants that will advance to the next round.
*/

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let kth_participant: i32 = input.nth(1).unwrap().parse().unwrap();

    let mut scores = String::new();
    std::io::stdin().read_line(&mut scores).unwrap();

    let scores = scores.split_whitespace().map(|x| x.parse::<i32>().unwrap());

    let kth_score = scores.clone().nth(kth_participant as usize - 1).unwrap();

    let advances = scores.filter(|&x| x >= kth_score && x > 0).count();

    println!("{}", advances);
}

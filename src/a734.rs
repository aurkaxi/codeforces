// Anton and Danik
// https://codeforces.com/problemset/problem/734/A

/*
* We will count the number of "A" in the input
* then substract from total games to count "D"
* print the respected output comparing them.
*/

pub fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input1).unwrap();
    std::io::stdin().read_line(&mut input2).unwrap();

    let total_games: isize = input1.trim().parse().unwrap();
    let anton = input2
        .trim()
        .chars()
        .fold(0, |acc, x| if x == 'A' { acc + 1 } else { acc });

    match anton.cmp(&(total_games - anton)) {
        std::cmp::Ordering::Greater => println!("Anton"),
        std::cmp::Ordering::Less => println!("Danik"),
        std::cmp::Ordering::Equal => println!("Friendship"),
    };
}

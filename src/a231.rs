// Team
// https://codeforces.com/problemset/problem/231/A

/*
* we will split the input and get a iterator.
* we will filter that iterator with a closure that checks if element is equal to 1.
* that will give us a iterator with only 1s.
* we will count the number of elements in that iterator.
* if we have atleast 2 1s, we will print "YES", otherwise "NO".
*/

pub fn main() {
    let mut num_of_problems = String::new();
    std::io::stdin().read_line(&mut num_of_problems).unwrap();
    let num_of_problems: i32 = num_of_problems.trim().parse().unwrap();

    let mut num_of_solved_problems = 0;

    for _ in 0..num_of_problems {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let confident = input
            .split_whitespace()
            .filter(|x| x.parse::<i32>().unwrap() == 1)
            .count();

        if confident >= 2 {
            num_of_solved_problems += 1;
        }
    }

    println!("{}", num_of_solved_problems);
}

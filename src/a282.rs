// Bit++
// https://codeforces.com/problemset/problem/282/A

/*
* We will match the input with a match statement. and increment or decrement the x variable.
*/

pub fn main() {
    let mut x = 0;
    let mut num_of_statements = String::new();
    std::io::stdin().read_line(&mut num_of_statements).unwrap();
    let num_of_statements: i32 = num_of_statements.trim().parse().unwrap();

    for _ in 0..num_of_statements {
        let mut statement = String::new();
        std::io::stdin().read_line(&mut statement).unwrap();
        match statement.trim() {
            "++X" | "X++" => x += 1,
            "--X" | "X--" => x -= 1,
            _ => continue,
        }
    }
    println!("{}", x);
}

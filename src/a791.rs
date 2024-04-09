// Bear and Big Brother
// https://codeforces.com/problemset/problem/791/A

/*
* We will just increase their age and count the years
* untill we meet our condition, limak > bob (in wieghts).
* this is the simplest and i think most unefficient.
* there should be some math L.M.S thingy, but my brain isn't working
*/
pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace();
    let mut limak: i64 = input.next().unwrap().parse().unwrap();
    let mut bob: i64 = input.next().unwrap().parse().unwrap();

    let mut years = 0;
    while limak <= bob {
        limak *= 3;
        bob *= 2;
        years += 1;
    }
    println!("{}", years);
}

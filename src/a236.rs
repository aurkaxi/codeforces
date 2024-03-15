// Boy or Girl
// https://codeforces.com/contest/236/problem/A

/*
 * We have to create a new string without duplicates.
 * and then count the length of the new string.
 * If the length is even, then the answer is "CHAT WITH HER!"
 * else, the answer is "IGNORE HIM!"
 */

use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut new_string = String::new();

    for character in input.chars() {
        if !new_string.contains(character) {
            new_string.push(character);
        }
    }

    /*
     * len() method won't output the way we count it.
     * So we have to use .chars().count()
     */

    if new_string.chars().count() % 2 == 0 {
        println!("CHAT WITH HER!");
    } else {
        println!("IGNORE HIM!");
    }
}

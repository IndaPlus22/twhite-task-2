use std::io;
use std::io::prelude::*;

fn main() {
    // get standard input stream
    let input = io::stdin();

    // get input lines as strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>(); // Vector not necessary, see example solution.

    //Convert strings in lines to integers and add them to a vector.
    for line in lines {
        let mut a: Vec<i64> = line
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>()
        .unwrap())
        .collect();

        //Print the abslute value of the two first integers then remove them from the array
        //and repeat until all pairs have been through the calculation procedure.
        while !a.is_empty() {
            for _i in 0..1 {
                println!("{}", (a[0]-a[1]).abs());
                a.remove(0);
                a.remove(0);
            }
        }
    }
}
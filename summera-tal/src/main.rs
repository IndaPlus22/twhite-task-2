use std::io;
use std::io::prelude::*;


fn main(){
    // get standard input stream
    let input = io::stdin();

    // get input lines as strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let mut _lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>(); // Vector not necessary, see example solution.

    let mut a: Vec<usize> = vec![];
    
    //Convert strings to integers and add them to a vector.
    for line in _lines { 
        let mut b: Vec<usize> = line
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>()
        .unwrap())
        .collect();

        a.append(&mut b);
    }

    let mut n: usize = a[0];

      //If only one element then print it.
    if n == 1 {
        println!("{}", a[1]);
    }

    else {
        //Update n to n+1 if it is an odd number
        if n%2 == 1 && n != 1 {
            n += 1;
        }
        //Remove the first element as it only represents the number of integers to add up
        //and is therefore not supposed to be part of the sum.
        //Sort the vector from highest to lowest.
        a.remove(0);
        a.sort(); 
        a.reverse();
    
        let mut sum: i32 = 0;
    
        //Iterate over the vector and add the sum of the n/2 largest integers.
        for i in 0..n/2 {
            sum += a[i] as i32;
        }
    
        println!("{}", sum); 
    }
}

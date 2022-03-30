use std::io;
use std::process::exit;

fn main() {
    println!("Enter which number of fibonacci sequence to generate");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if n == 0 {
        println!("Invalid number");
        exit(-1);
    }

    let mut x: u32 = 1;

    if n > 2 {
        let mut y: u32 = 1;
        for _ in 2..n {
            let z = x + y;
            y = x;
            x = z;
        }
    }

    println!("{}", x);
}

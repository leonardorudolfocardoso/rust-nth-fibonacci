use std::io::stdin;

fn main() {
    println!("Fibonacci generator!");
    loop {
        let mut n = String::new();

        println!("Enter the nth Fibonacci number you want:");
        stdin().read_line(&mut n).expect("Not able to read input.");

        let n: u32 = match n.trim().parse() {
            Ok(num) if num > 0 => num,
            Ok(_) => {
                println!("Number of Fibonacci must be more than 0. Try again. ");
                continue;
            },
            Err(_) => {
                println!("Not able to parse \"{}\" to a number. Try again. ", n.trim());
                continue;
            }
        };

        println!("{n}th Fibonacci number is {}", nth_fibonacci(n));
    }
}

fn nth_fibonacci (n: u32) -> u32 {
    match n {
        _ if n == 1 => 1,
        _ if n == 2 => 1,
        _ => nth_fibonacci(n-2) + nth_fibonacci(n-1) 
    }
}
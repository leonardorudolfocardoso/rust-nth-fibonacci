use std::io::stdin;

fn main() {
    println!("Fibonacci generator!");
    println!("Enter the number of fibonacci numbers you want:");

    loop {
        let mut n = String::new();

        stdin().read_line(&mut n).expect("Not able to read input.");

        let _n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not able to parse \"{}\" to a number. Try again. ", n.trim());
                continue;
            }
        };

        break;
    }
}

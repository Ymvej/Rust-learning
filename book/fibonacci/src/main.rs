use std::io;


fn main() {
    
    loop {
        println!("Input n :");
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
        if input.trim().to_lowercase() == "exit" {
            println!("Exiting...");
            break;
        }

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number. Please input a number or exit to leave.");
                continue;
            }
            
        };
        println!("n = {input}");
        if input == 0 {
            println!("Zero. Please input another number, or input exit to leave.");
            continue;
        } else if input < 0 {
            println!("Can't be negative. Please input another number, or input exit to leave.");
            continue;
        } else if input > 44 {
            println!("O(n²), don't be a dick.");
            println!("Please input another number, or input exit to leave.");
            continue;
        } else {
            let result = compute_fibonacci(input);
            println!("Fibonacci number of order {input} in sequence is {result}.");
            println!("Please input another number, or input exit to leave.");
        };    
    }
}

fn compute_fibonacci(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    } else {
        return compute_fibonacci(n-1) + compute_fibonacci(n-2)
    }
}
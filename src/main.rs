use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Not the correct usage!");
        process::exit(0);
    }

    let a: f64 = args[1].parse().expect("Invalid number a");
    let x = &args[2];
    let b: f64 = args[3].parse().expect("Invalid number b");

    match x.as_str() {
        "+" => println!("The result of {} + {}: {}", a, b, a + b),
        "-" => println!("The result of {} - {}: {}", a, b, a - b),
        "*" => println!("The result of {} * {}: {}", a, b, a * b),
        "/" => println!("The result of {} / {}: {}", a, b, a / b),
        _ => {
            println!("Wrong operator entered!");
            process::exit(0);
        }
    }
}

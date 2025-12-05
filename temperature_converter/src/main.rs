use std::io;

fn main() {
    println!("Temperature Converter");
    println!("====================\n");

    loop {
        println!("Choose conversion:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Exit");
        println!("\nEnter your choice (1-3):");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number.\n");
                continue;
            }
        };

        match choice {
            1 => fahrenheit_to_celsius(),
            2 => celsius_to_fahrenheit(),
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please choose 1, 2, or 3.\n"),
        }
    }
}

fn fahrenheit_to_celsius() {
    println!("\nEnter temperature in Fahrenheit:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let fahrenheit: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature!\n");
            return;
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{:.2}°F = {:.2}°C\n", fahrenheit, celsius);
}

fn celsius_to_fahrenheit() {
    println!("\nEnter temperature in Celsius:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let celsius: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature!\n");
            return;
        }
    };

    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("{:.2}°C = {:.2}°F\n", celsius, fahrenheit);
}

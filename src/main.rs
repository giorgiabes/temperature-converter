use std::io;

fn main() {
    loop {
        display_menu();
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter number.");
                continue;
            }
        };
        if choice == 1 {
            println!("Enter temperature in Fahrenheit:");
            let mut fahrenheit = String::new();
            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Failed to read line.");
            let fahrenheit: i32 = match fahrenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter number.");
                    continue;
                }
            };
            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("Temperature in Celsius: {celsius}");
            break;
        } else if choice == 2 {
            println!("Enter temperature in Celsius:");
            let mut celsius = String::new();
            io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read line.");
            let celsius: i32 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter number.");
                    continue;
                }
            };
            let fahrenheit = celsius_to_fahrenheit(celsius);
            println!("Temperature in Fahrenheit: {fahrenheit}");
            break;
        } else if choice == 3 {
            println!("Exiting program. Goodbye!");
            break;
        } else {
            println!("Invalid choice, Please try again.");
            continue;
        }
    }
}

fn display_menu() {
    println!("Choose an option:");
    println!("1. Convert Fahrenheit to Celsius");
    println!("2. Convert Celsius to Fahrenheit");
    println!("3. Exit");
    println!("=================================")
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    (celsius * 9 / 5) + 32
}

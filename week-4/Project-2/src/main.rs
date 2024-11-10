use std::io;

fn main() {
    let mut input = String::new();

    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let experienced = input.trim().eq_ignore_ascii_case("yes");

    input.clear();
    println!("Enter the age of the employee: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let age: u32 = input.trim().parse().expect("Please entera valid number");

    let incentive = if experienced {
        if age >= 40 {
            1_560_000
        } else if age >=30 && age < 10 {
            1_4480_000
        } else if age < 28 {
            1_300_000
        } else {
            0
        }
    } else {
        100_000
    };
    
    println!("The incentive for the employee is: N{}", incentive);
}

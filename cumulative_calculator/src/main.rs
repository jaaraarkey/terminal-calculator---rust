use std::io;

fn main() {
    let greeting = "Welcome to the Budget Calculator!\nThis program calculates the estimated budget at the end of a given number of days based on the initial budget and the percentage increase in the budget per day.\n";

    loop {
        let mut quit = String::new();
        println!("{}", greeting);
        let budget: f64 = parse_budget();
        let days: i64 = parse_days();
        let percentage: f64 = parse_percentage();
        let result = calculate_camulative_(budget, days, percentage);
        println!(
            "At the end of the {} days, the estimated budget is: {}",
            days, result
        );
        println!("Do you want to continue? (y/n)");
        io::stdin()
            .read_line(&mut quit)
            .expect("Failed to read line");
        match quit.trim() {
            "y" => continue,
            "n" => break,
            _ => println!("Invalid input!"),
        }
    }
}

fn roundup(value: f64) -> f64 {
    value.round()
}

fn calculate_camulative_(budget: f64, days: i64, percentage: f64) -> f64 {
    let camulative_budget: f64 = {
        let mut camulative_budget = budget as f64;
        for _ in 0..days {
            camulative_budget += camulative_budget * (percentage / 100.0);
        }
        camulative_budget
    };
    roundup(camulative_budget)
}

fn parse_budget() -> f64 {
    let mut budget = String::new();
    println!("Enter the budget: ");
    io::stdin()
        .read_line(&mut budget)
        .expect("Failed to read line");
    let budget: f64 = budget.trim().parse().expect("Please type a number!");
    budget
}
fn parse_days() -> i64 {
    let mut days = String::new();
    println!("Enter the number of days: ");
    io::stdin()
        .read_line(&mut days)
        .expect("Failed to read line");
    let days: i64 = days.trim().parse().expect("Please type a number!");

    days
}

fn parse_percentage() -> f64 {
    let mut percentage = String::new();
    println!("Enter the percentage: ");
    io::stdin()
        .read_line(&mut percentage)
        .expect("Failed to read line");
    let percentage: f64 = percentage.trim().parse().expect("Please type a number");
    percentage
}

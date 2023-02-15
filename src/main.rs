use std::io;

fn wallet() {
    
    fn set_current_balance() -> f64 {
        println!("Please, enter your current balance: ");
 
        let mut current_balance = String::new();
        
        io::stdin().read_line(&mut current_balance)
        .expect("Failed to read line");

        let current_balance: f64 = current_balance.trim().parse().unwrap();

        println!("Your current balance is: {}", current_balance);
        println!("Balance: {:?}", current_balance);

        return current_balance;
     
    }

    fn set_monthly_income() -> f64 {
        println!("Please, enter your monthly income: ");

        let mut monthly_income = String::new();
        
        io::stdin().read_line(&mut monthly_income)
        .expect("Failed to read line");

        let monthly_income: f64 = monthly_income.trim().parse().unwrap();

        println!("Your monthly income is: {}", monthly_income);

        return monthly_income;
        

    }

    fn update_balance() {
        let current_balance = set_current_balance();
        let monthly_income = set_monthly_income();

        let updated_balance = current_balance - monthly_income;

        println!("Your updated balance is: {}", updated_balance);
    }

    update_balance();

}

fn main() {
    wallet();

}
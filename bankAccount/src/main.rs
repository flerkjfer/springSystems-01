mod bank_account; //imports the rest module

use bank_account::BankAccount; //importing the struct

fn main(){
    // Create a new account
    let mut account = BankAccount::new(100.0);

    println!("Initial balance: {}", account.balance());

    // Deposit money
    account.deposit(50.0);
    println!("After depositing $50: {}", account.balance());

    // Withdraw money
    account.withdraw(30.0);
    println!("After withdrawing $30: {}", account.balance());

    // Apply interest
    account.apply_interest(0.05);
    println!("After applying 5% interest: {}", account.balance());

    println!("Final account state: {:?}", account);

}
#[derive(Debug)]

pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) { //&mut self means mutable access, keyword mut 
        // Implement this method
        if amount > 0.0 { //only non negative 
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance 
    }

    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(200.0);
        assert_eq!(account.balance(), 200.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(200.0);
        account.deposit(70.0);
        assert_eq!(account.balance(), 270.0);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(700.0);
        account.withdraw(100.0);
        assert_eq!(account.balance(), 600.0);
    }

    // Add more tests here
    //edge case test
    #[test]
    fn test_withdraw_too_much() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(200.0); //do no subtract 
        assert_eq!(account.balance(), 100.0);
    }

    // depositing negative amount should do nothing
    #[test]
    fn test_deposit_negative() {
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test] // withdrawing negative amount should do nothing
    fn test_withdraw_negative() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-30.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit_float_precision() {
        let mut account = BankAccount::new(100.0);
        account.deposit(0.1);
        account.deposit(0.2);

        let expected = 100.3;
        let epsilon = 1e-10; //sometimes 100.3000000001
        assert!((account.balance() - expected).abs() < epsilon);
    }

    #[test]
    fn test_apply_interest() {
        let mut account = BankAccount::new(100.0);
        account.apply_interest(0.05);
        let expected = 105.0;
        let epsilon = 1e-10;
        assert!((account.balance() - expected).abs() < epsilon);
    }

    #[test]
    fn test_float_precision() {
        let mut account = BankAccount::new(100.0);
        account.deposit(0.1);
        account.deposit(0.2);

        let expected = 100.3;
        let epsilon = 1e-10;
        assert!((account.balance() - expected).abs() < epsilon);
    }
}
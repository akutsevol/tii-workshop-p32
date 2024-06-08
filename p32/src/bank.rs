#[derive(Debug)]
pub struct User {
    name: String,
    credit_line: u64,
    balance: i64, // Positive number means debit, negative credit
}

impl User {
    pub fn new(name: String, credit_line: u64, balance: i64) -> Self {
        Self {
            name,
            credit_line,
            balance,
        }
    }
}

#[derive(Debug)]
pub struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64, // in basis points (0.01%)
    debit_interest: u64, // in basis points (0.01%)
}

impl Bank {
    pub fn new(name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Self {
            users: Vec::new(),
            name,
            credit_interest,
            debit_interest,
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    // Other methods for managing the bank
}

// Derive traits for User and Bank
impl Clone for User {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            credit_line: self.credit_line,
            balance: self.balance,
        }
    }
}

impl Clone for Bank {
    fn clone(&self) -> Self {
        Self {
            users: self.users.clone(),
            name: self.name.clone(),
            credit_interest: self.credit_interest,
            debit_interest: self.debit_interest,
        }
    }
}

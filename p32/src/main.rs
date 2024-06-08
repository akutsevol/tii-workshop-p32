mod bank;

fn main() {
    use bank::{Bank, User};

    let user1 = User::new("Alice".to_string(), 1000, 0);
    let user2 = User::new("Bob".to_string(), 2000, 500);
    
    let mut my_bank = Bank::new("MyBank".to_string(), 10, 20);
    my_bank.add_user(user1.clone());
    my_bank.add_user(user2.clone());

    println!("Initial Bank State:");
    println!("{}", my_bank);

    my_bank.transfer_funds("Bob", "Alice", 300).unwrap();
    my_bank.accrue_interest();

    println!("Bank State After Transfer and Accruing Interest:");
    println!("{}", my_bank);

    let (liabilities, assets) = my_bank.calc_balance();
    println!("Total Bank Liabilities: {}", liabilities);
    println!("Total Bank Assets: {}", assets);
}

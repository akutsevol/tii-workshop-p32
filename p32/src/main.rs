mod bank;

fn main() {
    use bank::{Bank, User};

    let user1 = User::new("Alice".to_string(), 1000, 0);
    let user2 = User::new("Bob".to_string(), 2000, 500);

    let mut my_bank = Bank::new("MyBank".to_string(), 10, 20);
    my_bank.add_user(user1);
    my_bank.add_user(user2);

    println!("{:?}", my_bank);
}

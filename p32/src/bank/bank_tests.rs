use super::*;

#[test]
fn test_bank_calc_balance() {
    let mut bank = Bank::new(0, 0);
    let mut user1 = User {
        id: 1,
        name: String::from("Alice"),
        credit_line: 1000,
        balance: -500,
    };

    let mut user2 = User {
        id: 2,
        name: String::from("Bob"),
        credit_line: 2000,
        balance: 1500,
    };

    bank.add_user(user1);
    bank.add_user(user2);

    println!("Bank balance: {:?}", bank.calc_balance());
}

#[test]
fn test_bank_transfer() {
    let mut bank = Bank::new(0, 0);
    let mut user1 = User {
        id: 1,
        name: String::from("Alice"),
        credit_line: 1000,
        balance: -500,
    };

    let mut user2 = User {
        id: 2,
        name: String::from("Bob"),
        credit_line: 2000,
        balance: 1500,
    };

    bank.add_user(user1);
    bank.add_user(user2);

    assert_eq!(
        bank.transfer(&mut String::from("Alice"), &mut String::from("Bob"), 100),
        Ok(())
    );
}

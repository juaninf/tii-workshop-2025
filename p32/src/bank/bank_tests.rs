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

    let user_name1 = String::from("Alice");
    let user_name2 = String::from("Bob");

    let res = bank.transfer(user_name1, user_name2, 1000);
    assert_eq!(res, Err("Insufficient credit line".to_string()));
}

#[test]
fn test_accrue_interest() {
    let mut bank: Bank = Bank::new(0, 0);
    let user1 = User {
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
    bank.accrue_interest();

    assert_eq!(bank.users[0].balance, -500);
}

#[test]
fn test_merge_bank() {
    let mut bank: Bank = Bank::new(0, 0);
    let user1 = User {
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

    let mut bank1: Bank = Bank::new(0, 0);
    let user1 = User {
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
    bank1.add_user(user1);
    bank1.add_user(user2);

    bank.merge_bank(&mut bank1);
    //checking if the balance is correct for Bob
    assert_eq!(bank1.users[1].balance, 3000);
}

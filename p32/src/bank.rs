trait BankAccount {
    fn deposit(&mut self, amount: u64);
    fn withdraw(&mut self, amount: u64) -> Result<(), String>;
    fn get_balance(&self) -> i64;
}

trait CreditAccount {
    fn get_credit_line(&self) -> u64;
}

trait BankOperations {
    //fn add_user(&mut self, user: User);
    fn get_user(&self, id: u32) -> Option<&User>;
    fn get_credit_interest(&self) -> u64;
    fn get_debit_interest(&self) -> u64;
}

impl BankAccount for User {
    fn deposit(&mut self, amount: u64) {
        self.balance += amount as i64;
    }

    fn withdraw(&mut self, amount: u64) -> Result<(), String> {
        if amount as i64 > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount as i64;
        Ok(())
    }

    fn get_balance(&self) -> i64 {
        self.balance
    }
}

impl CreditAccount for User {
    fn get_credit_line(&self) -> u64 {
        self.credit_line
    }
}

impl BankOperations for Bank {
    //fn add_user(&mut self, &mut user: User) {
    //    self.users.push(user);
    //}

    fn get_user(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|user| user.id == id)
    }

    fn get_credit_interest(&self) -> u64 {
        self.credit_interest
    }

    fn get_debit_interest(&self) -> u64 {
        self.debit_interest
    }
}

impl Bank {
    fn find_user_by_name(&mut self, name: &str) -> Option<&mut User> {
        self.users.iter_mut().find(|user| user.name == name)
    }

    fn find_two_users_by_name(&mut self, name1: &str, name2: &str) -> Option<&mut User> {
        let Some(x) = self
            .users
            .iter_mut()
            .find(|user| user.name == name1 || user.name == name2)
        else {
            return None;
        };
        println!("{}", x.id);
        Some(x)
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn new(credit_interest: u64, debit_interest: u64) -> Self {
        Bank {
            users: Vec::new(),
            credit_interest,
            debit_interest,
        }
    }

    fn calc_balance(&self) -> (i64, i64) {
        let mut total_bank_liabilities = 0;
        let mut total_bank_assets = 0;
        self.users.iter().for_each(|user| {
            println!("User: {:?}\n", user.name);
            if user.get_balance() < 0 {
                total_bank_liabilities += user.get_balance() as i64;
            } else {
                total_bank_assets += user.get_balance() as i64;
            }
        });
        (total_bank_liabilities, total_bank_assets)
    }

    fn transfer(
        &mut self,
        from_user_str: &mut String,
        to_user_str: &mut String,
        amount: u64,
    ) -> Result<(), String> {
        let Some(from_user) = self.find_two_users_by_name(from_user_str, to_user_str) else {
            return Err(String::from("User not found"));
        };
        //let Some(from_user) = self.find_user_by_name(from_user_str) else {
        //    return Err(String::from("User not found"));
        //};
        //
        //let Some(to_user) = self.find_user_by_name(to_user_str) else {
        //    return Err(String::from("User not found"));
        //};
        //
        //if from_user.credit_line < amount {
        //    return Err("Insufficient credit line".to_string());
        //}
        //
        //from_user.withdraw(amount)?;
        //to_user.deposit(amount);
        Ok(())
    }
}

struct User {
    id: u32,
    name: String,
    credit_line: u64,
    balance: i64,
}

struct Bank {
    users: Vec<User>,
    credit_interest: u64,
    debit_interest: u64,
}

#[cfg(test)]
mod bank_tests;

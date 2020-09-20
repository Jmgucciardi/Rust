#[derive(Debug)]
pub struct Account {
    pub account_number: i32,
    pub routing_number: i32,
    pub pin: i32
}

impl Account {
    pub fn get_account_info(account_number: i32, routing_number: i32, pin: i32) -> Account {
        let account = Account { account_number, routing_number, pin };
        account
    }
}
//! This contract implements simple profile info backed by storage on blockchain.
//!
//! The contract provides methods to [set_age] / [set_first_name] / [set_last_name] profile and
//! [get it's current information][get_age][get_first_name][get_last_name] or [introduce].
//!
//! [get_age]: struct.Profile.html#method.get_age
//! [get_first_name]: struct.Profile.html#method.get_first_name
//! [get_last_name]: struct.Profile.html#method.get_last_name
//! [set_age]: struct.Profile.html#method.set_age
//! [set_first_name]: struct.Profile.html#method.set_first_name
//! [set_last_name]: struct.Profile.html#method.set_last_name
//!
//!
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Profile {
    first_name: String,
    last_name: String,
    age: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
}

#[near_bindgen]
impl Profile {
    #[init]
    pub fn
    new() -> Self {
        Self::default()
    }

    pub fn
    set_age(&mut self, age: i8) {
        self.age = age;
        env::log(format!("age now is {}.", self.age).as_bytes());
    }

    pub fn
    get_age(&self) -> i8 {
        env::log(format!("age now is {}.", self.age).as_bytes());
        self.age
    }

    pub fn
    set_first_name(&mut self, f_name: String) {
        self.first_name = f_name;
        env::log(format!("first name now is {}.", self.first_name).as_bytes());
    }

    pub fn
    get_first_name(&self) -> String {
        env::log(format!("first name now is {}.", self.first_name).as_bytes());
        self.first_name.to_string()
    }

    pub fn
    set_last_name(&mut self, l_name: String) {
        env::log(format!("last name now is {}.", self.last_name).as_bytes());
        self.last_name = l_name
    }

    pub fn
    get_last_name(&self) -> String {
        env::log(format!("last name now is {}.", self.last_name).as_bytes());
        self.last_name.to_string()
    }

    pub fn
    introduce(&self) -> (String, String, String) {
        let log_data = (
            format!("My name is {}", self.first_name),
            format!("Full name is {} {}", self.last_name, self.first_name),
            format!("I'm {} years old", self.age),
        );
        env::log(format!("{:?}", log_data).as_bytes());
        return log_data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_get_first_name() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Profile::new();
        assert_eq!("", contract.get_first_name());
        println!("test get_first_name succeeded");
    }

    #[test]
    fn test_get_last_name() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Profile::new();
        assert_eq!("", contract.get_last_name());
        println!("test get_last_name succeeded");
    }

    #[test]
    fn test_get_age() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Profile::new();
        assert_eq!(0, contract.get_age());
        println!("test get_age succeeded");
    }

    #[test]
    fn test_set_first_name() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Profile::new();
        contract.set_first_name("a".to_string());
        assert_eq!("a", contract.get_first_name());
        println!("test set_first_name succeeded");
    }

    #[test]
    fn test_set_last_name() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Profile::new();
        contract.set_last_name("b".to_string());
        assert_eq!("b", contract.get_last_name());
        println!("test set_last_name succeeded");
    }

    #[test]
    fn test_set_age() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Profile::new();
        contract.set_age(28);
        assert_eq!(28, contract.get_age());
        println!("test set_age succeeded");
    }

    #[test]
    fn test_introduce() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Profile::new();
        assert_eq!("(\"My name is \", \"Full name is  \", \"I'm 0 years old\")".to_string(), format!("{:?}", contract.introduce()));
        println!("test introduce succeeded");
    }
}


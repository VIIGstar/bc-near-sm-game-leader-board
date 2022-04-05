extern crate core;

use near_sdk::init;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::collections::UnorderedMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

near_sdk::setup_alloc!();

pub trait NEP4 {
    // Grant the access to the given `accountId` for the given `tokenId`.
    // Requirements:
    // * The caller of the function (`predecessor_id`) should have access to the token.
    fn grant_access(&mut self, escrow_account_id: AccountId);

    // Revoke the access to the given `accountId` for the given `tokenId`.
    // Requirements:
    // * The caller of the function (`predecessor_id`) should have access to the token.
    fn revoke_access(&mut self, escrow_account_id: AccountId);

    // Returns `true` or `false` based on caller of the function (`predecessor_id) having access to the token
    fn check_access(&self, account_id: AccountId) -> bool;
}


pub type AccountIdHash = Vec<u8>;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ProfileState {
    state: UnorderedMap<AccountIdHash, Profile>
}

impl Default for ProfileState {
    fn default() -> Self {
        panic!("State should be initialized before usage")
    }
}

#[near_bindgen]
impl ProfileState {
    #[init]
    pub fn
    new() -> Self {
        Self {
            state: UnorderedMap::new(b"grant".to_vec())
        }
    }

    pub fn
    set_info(&mut self, account_id: AccountId, age: i8, f_name: String, l_name: String) -> bool {
        let escrow_hash = env::sha256(account_id.as_bytes());
        let opt = self.state.get(&escrow_hash);
        if opt.is_none() {
            print!("not found info to set");
            env::log("not found info to set".as_bytes());
            return false;
        }

        let mut profile = opt.unwrap();
        if age > 0 {
            profile.set_age(age);
        }

        if f_name.len() > 0 {
            profile.set_first_name(f_name);
        }

        if l_name.len() > 0 {
            profile.set_last_name(l_name);
        }

        self.state.insert(&escrow_hash, &profile);
        print!("info was set");
        env::log("info was set".as_bytes());
        return true;
    }

    pub fn
    get_info(&self, account_id: AccountId) -> Profile {
        let escrow_hash = env::sha256(account_id.as_bytes());
        return match self.state.get(&escrow_hash) {
            Some(profile) => {
                profile
            }
            _ => Profile::new()
        };
    }

    pub fn
    introduce(&self, account_id: AccountId) {
        let profile = match self.state.get(&env::sha256(account_id.as_bytes())){
            Some(profile) => {
                profile
            }
            _ => Profile::new()
        };
        if profile.is_empty() {
            env::log("not found to introduce".as_bytes());
            return;
        }
        profile.introduce();
    }
}

#[near_bindgen]
impl NEP4 for ProfileState {
    fn grant_access(&mut self, escrow_account_id: AccountId) {
        let escrow_hash = env::sha256(escrow_account_id.as_bytes());
        if self.check_access(escrow_account_id) {
            env::log("existed!".as_bytes());
            println!("existed!");
        } else {
            self.state.insert(&escrow_hash, &Profile::new());
            env::log("granted!".as_bytes());
            println!("granted!");
        }
    }

    fn revoke_access(&mut self, escrow_account_id: AccountId) {
        let escrow_hash = env::sha256(escrow_account_id.as_bytes());
        if self.check_access(escrow_account_id) {
            self.state.remove(&escrow_hash);
            env::log("revoked!".as_bytes());
            println!("revoked!");
        } else {
            env::log("not found to revoke!".as_bytes());
            println!("not found to revoke!");
        }
    }

    fn check_access(&self, account_id: AccountId) -> bool {
        let escrow_hash = env::sha256(account_id.as_bytes());
        return match self.state.get(&escrow_hash) {
            Some(_) => {
                true
            }
            _ => {
                false
            }
        };
    }
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Profile {
    first_name: String,
    last_name: String,
    age: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
}

impl Profile {
    #[init]
    pub fn
    new() -> Self {
        Self {
            age: 0,
            first_name: String::new(),
            last_name: String::new(),
        }
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
        self.last_name = l_name;
        env::log(format!("last name now is {}.", self.last_name).as_bytes());
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

    pub fn
    is_empty(&self) -> bool {
        return self.age == 0 && self.last_name.is_empty() && self.first_name.is_empty()
    }
}

impl Serialize for Profile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Profile", 3)?;
        state.serialize_field("age", &self.age)?;
        state.serialize_field("first_name", &self.first_name)?;
        state.serialize_field("last_name", &self.last_name)?;
        state.end()
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
    fn test_all() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = ProfileState::new();
        let account1: &str = "viigstar-2.testnet";
        let account2: String = "viigstar-nft.testnet".to_string();
        contract.grant_access(account1.to_string());
        contract.grant_access(account1.to_string());
        contract.revoke_access(account2);
        contract.revoke_access(account1.to_string());
    }

    #[test]
    fn test_grant_access() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = ProfileState::new();
        let account1: &str = "viigstar-2.testnet";
        contract.grant_access(account1.to_string());
        assert_eq!(true, contract.check_access(account1.to_string()));
    }

    #[test]
    fn test_revoke_access() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = ProfileState::new();
        let account1: &str = "viigstar-2.testnet";
        contract.grant_access(account1.to_string());
        assert_eq!(true, contract.check_access(account1.to_string()));
        contract.revoke_access(account1.to_string());
        assert_eq!(false, contract.check_access(account1.to_string()));
    }

    #[test]
    fn test_set_info() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = ProfileState::new();
        let account1: &str = "viigstar-2.testnet";
        contract.grant_access(account1.to_string());
        assert_eq!(true, contract.check_access(account1.to_string()));
        contract.set_info(account1.to_string(), 28, "Trung".to_string(), String::from("Nguyen Bao"));
        // let profile = contract.get_info(account1.to_string());
        // assert_eq!(false, profile.is_empty());
        // assert_eq!(28, profile.get_age());
        // println!();
        contract.introduce(account1.to_string());
    }

    #[test]
    fn test_check_access() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = ProfileState::new();
        assert_eq!(false,contract.check_access("viigstar-2.testnet".to_string()));
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

    #[test]
    fn test_is_empty() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Profile::new();
        assert_eq!(true, contract.is_empty());
        contract.set_age(1);
        assert_eq!(false, contract.is_empty());
        println!("test is_empty succeeded");
    }
}


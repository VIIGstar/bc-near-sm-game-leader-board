extern crate core;

use chrono::Utc;
use near_sdk::init;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::collections::{UnorderedMap};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

near_sdk::setup_alloc!();

trait NEP4 {
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

trait LeaderBoard {
    fn get_top_players(&self) -> Vec<(String, Profile)>;
    fn get_reward(&mut self, account_id: AccountId) -> i32;
    fn is_recently_rewarded(&self, account_id: AccountId) -> bool;
    fn save_new_score(&mut self, account_id: AccountId, score: i32) -> bool;
}

const REWARD_FREQUENCY: i64 = 3600 * 1000; // 1 hour = 3600 * 1000 miliseconds
const REWARD_RANGE:[(i32, i32); 5] = [
    (10, 10),
    (20, 20),
    (50, 50),
    (200, 200),
    (1000, 1000),
];
type AccountIdHash = Vec<u8>;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ProfileState {
    state: UnorderedMap<AccountIdHash, UnorderedMap<String, Profile>>
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
    get_list_user(&self, account_id: AccountId) -> Vec<Profile> {
        let escrow_hash = env::sha256(account_id.as_bytes());
        let map = self.state.get(&escrow_hash).unwrap();
        let keys = map.keys();
        let mut vec:Vec<Profile> = vec![];
        for key in keys {
            vec.push(map.get(&key).unwrap())
        }
        return vec;
    }

    fn
    flat_users(&self) -> Vec<(String, Profile)> {
        let mut vec: Vec<(String, Profile)> = vec![];
        for value in self.state.values() {
            vec.append(&mut value.to_vec())
        };

        return vec;
    }
}

#[near_bindgen]
impl NEP4 for ProfileState {
    fn grant_access(&mut self, account_id: AccountId) {
        let account_ref: &str = account_id.as_ref();
        let escrow_hash = env::sha256(account_id.as_bytes());
        if self.check_access(account_ref.to_string()) {
            env::log("existed!".as_bytes());
            println!("existed!");
        } else {
            let new_profile = &Profile::new(String::from(account_ref));
            let mut new_map = UnorderedMap::new(account_ref.as_bytes());
            new_map.insert(&String::from(account_ref), new_profile);
            self.state.insert(&escrow_hash, &new_map);
            env::log("granted!".as_bytes());
            println!("granted!");
        }
    }

    fn revoke_access(&mut self, account_id: AccountId) {
        let escrow_hash = env::sha256(account_id.as_bytes());
        if self.check_access(account_id) {
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

#[near_bindgen]
impl LeaderBoard for ProfileState {
    fn get_top_players(&self) -> Vec<(String, Profile)> {
        let mut flat_users = self.flat_users();
        return mut_to_sorted_vec(&mut flat_users, false);
    }

    fn get_reward(&mut self, account_id: AccountId) -> i32 {
        let account_ref: &str = account_id.as_ref();
        let escrow_hash = env::sha256(account_id.as_bytes());
        if !self.check_access(account_ref.to_string()) || self.is_recently_rewarded(account_ref.to_string()){
            return 0;
        }

        let map_users = self.state.get(&escrow_hash);
        let top_users = get_sorted_profiles_by_score(&map_users.unwrap(), true);
        let highest_score = top_users[0].1.score;
        let mut reward = 0;
        for max in REWARD_RANGE {
          if (highest_score as i32) < max.0 {
              reward = max.1;
              break;
          }
        };

        let reward_time = Utc::now().timestamp_millis();
        let clone_map = self.state.get(&escrow_hash).unwrap();
        let mut update_map = UnorderedMap::new(account_id.as_bytes());
        if reward > 0 {
            for key in clone_map.keys() {
                let mut update_profile = clone_map.get(&key).unwrap().clone();
                update_profile.rewarded(reward_time);
                update_map.insert(&key, &update_profile);
            }

            self.state.insert(&escrow_hash, &update_map);
        }

        return reward;
    }

    fn is_recently_rewarded(&self, account_id: AccountId) -> bool {
        let account_ref: &str = account_id.as_ref();
        let escrow_hash = env::sha256(account_id.as_bytes());
        if !self.check_access(account_ref.to_string()) {
            return true;
        }

        let user = self.state.get(&escrow_hash).unwrap().values().next().unwrap();
        return Utc::now().timestamp_millis() - user.hourly_reward_at < REWARD_FREQUENCY;
    }

    fn save_new_score(&mut self, account_id: AccountId, score: i32) -> bool {
        let account_ref: &str = account_id.as_ref();
        let escrow_hash = env::sha256(account_id.as_bytes());
        if !self.check_access(account_ref.to_string()) {
            return false;
        };
        let mut map_profile = self.state.get(&escrow_hash).unwrap();
        let user = map_profile.get(&account_ref.to_string());
        if user.is_none() {
            return false;
        };
        let mut profile = user.unwrap();
        if profile.get_score() < score {
            profile.set_score(score);
            map_profile.insert(&account_ref.to_string(), &profile);
            self.state.insert(&escrow_hash, &map_profile);
            return true;
        };

        return false;
    }
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Profile {
    username: String,
    score: i32,
    hourly_reward_at: i64
}

impl Profile {
    #[init]
    pub fn
    new(account_id: String) -> Self {
        Self {
            score: 0,
            username: account_id,
            hourly_reward_at: 0,
        }
    }

    pub fn
    set_score(&mut self, score: i32) {
        self.score = score;
        env::log(format!("score now is {}.", self.score).as_bytes());
    }

    pub fn
    get_score(&self) -> i32 {
        env::log(format!("score now is {}.", self.score).as_bytes());
        self.score
    }

    pub fn
    get_username(&self) -> String {
        return self.username.to_string()
    }

    pub fn
    is_empty(&self) -> bool {
        return self.username.is_empty()
    }

    fn
    rewarded(&mut self, timestamp: i64) {
        self.hourly_reward_at = timestamp;
    }
}

impl Serialize for Profile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Profile", 2)?;
        state.serialize_field("score", &self.score)?;
        state.serialize_field("username", &self.username)?;
        state.end()
    }
}

impl Clone for Profile {
    fn clone(&self) -> Self {
        return Profile{
            username: self.username.to_string(),
            hourly_reward_at: self.hourly_reward_at,
            score: self.score,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.score = source.score;
        self.hourly_reward_at = source.hourly_reward_at;
        self.username = source.username.to_string();
    }
}

// --- START: Utility --- //
fn get_sorted_profiles_by_score(list: &UnorderedMap<String, Profile>, only_highest: bool) -> Vec<(String, Profile)> {
    let mut result = list.to_vec();
    return mut_to_sorted_vec(&mut result, only_highest);
}

fn mut_to_sorted_vec(list: &mut Vec<(String, Profile)>, only_highest: bool) -> Vec<(String, Profile)> {
    let mut vec: Vec<(String, Profile)> = vec![];
    while list.len() > 0 {
        let mut profile: Profile = Profile::new(String::new());
        let mut max_at_index = 0;
        for index in 0..list.len() {
            let value: &Profile = &list[index].1;
            if profile.get_score() == 0 || profile.get_score() < value.get_score() {
                profile = Profile{
                    username: value.get_username(),
                    score: value.score,
                    hourly_reward_at: value.hourly_reward_at,
                };
                max_at_index = index;
            }
        }

        list.remove(max_at_index);
        vec.push((profile.get_username(), profile));
        if only_highest {
            break;
        }
    }

    return vec;
}
// --- END: Utility --- //

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

    fn get_test_map() -> UnorderedMap<String, Profile> {
        let mut map:UnorderedMap<String, Profile> = UnorderedMap::new(b"test".to_vec());
        for i in [1,2,3,9] {
            map.insert(&i.to_string(), &Profile{
                hourly_reward_at: 0,
                score: i,
                username: i.to_string(),
            });
        };

        return map;
    }

    #[test]
    fn test_all() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = ProfileState::new();
        let account1: &str = "viigstar-2.testnet";
        contract.grant_access(account1.to_string());
        contract.grant_access(account1.to_string());
        contract.revoke_access("viigstar-nft.testnet".to_string());
        contract.revoke_access(account1.to_string());
    }

    #[test]
    fn test_grant_access() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = ProfileState::new();
        let account1: &str = "viigstar-2.testnet";
        let escrow_hash = env::sha256(account1.as_bytes());
        contract.grant_access(account1.to_string());
        assert_eq!(true, contract.check_access(account1.to_string()));
        assert_eq!(1, contract.state.get(&escrow_hash).unwrap().len());
        for key in contract.state.get(&escrow_hash).unwrap().keys() {
            assert_eq!(account1.to_string(), key);
        }
        for mut user in contract.get_list_user(account1.to_string()) {
            assert_eq!(account1.to_string(), user.username);
            let reward_at = Utc::now().timestamp_millis();
            user.rewarded(reward_at);
            assert_eq!(reward_at, user.hourly_reward_at);
            println!("rewarded at {}", reward_at);
        }
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
    fn test_flat_users() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let map = get_test_map();
        let mut contract = ProfileState::new();
        let escrow_hash = env::sha256("1".as_bytes());
        contract.state.insert(&escrow_hash, &map);
        assert_eq!(4, contract.flat_users().len());
        assert_eq!("1".to_string(), contract.flat_users()[0].0);
        assert_eq!("9".to_string(), contract.flat_users()[3].0);
    }

    #[test]
    fn test_get_username() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = ProfileState::new();
        let account1: &str = "viigstar-2.testnet";
        let escrow_hash = env::sha256(account1.as_bytes());
        contract.grant_access(account1.to_string());
        for profile in contract.state.get(&escrow_hash).unwrap().values() {
            assert_eq!(account1.to_string(), profile.get_username());
        }
    }

    #[test]
    fn test_get_score() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Profile::new(String::new());
        assert_eq!(0, contract.get_score());
        println!("test get_score succeeded");
    }

    #[test]
    fn test_set_score() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Profile::new(String::new());
        contract.set_score(28);
        assert_eq!(28, contract.get_score());
        println!("test set_score succeeded");
    }

    #[test]
    fn test_save_new_score() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        let account_ref = "1";
        let map = get_test_map();
        let mut contract = ProfileState::new();
        contract.state.insert(&env::sha256(account_ref.as_bytes()), &map);
        // assert_eq!(10, contract.get_reward(account_ref.to_string()));
        assert_eq!(true, contract.save_new_score(account_ref.to_string(), 22));
        assert_eq!(50, contract.get_reward(account_ref.to_string()));
    }

    #[test]
    fn test_is_empty() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Profile::new(String::new());
        assert_eq!(true, contract.is_empty());
        println!("test is_empty succeeded");
    }

    #[test]
    fn test_get_sorted_profiles_by_score() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let map = get_test_map();
        println!("length before mutate reference {}", map.len());
        let result = get_sorted_profiles_by_score(&map, false);
        println!("length after mutate reference {}", map.len());
        assert_eq!(9, result[0].1.score);
        assert_eq!(3, result[1].1.score);
        assert_eq!(2, result[2].1.score);
        assert_eq!(1, result[3].1.score);
    }

    #[test]
    fn test_get_top_players() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let map = get_test_map();
        let mut map2:UnorderedMap<String, Profile> = UnorderedMap::new(b"test2".to_vec());
        for i in [4,5,7,11] {
            map2.insert(&i.to_string(), &Profile{
                hourly_reward_at: 0,
                score: i,
                username: i.to_string(),
            });
        };

        let mut contract = ProfileState::new();
        contract.state.insert(&env::sha256("1".as_bytes()), &map);
        contract.state.insert(&env::sha256("4".as_bytes()), &map2);
        let mut flat_users = contract.flat_users();
        assert_eq!(8, flat_users.len());
        let sorted_flat = mut_to_sorted_vec(&mut flat_users, false);
        assert_eq!(8, sorted_flat.len());
        assert_eq!(11, sorted_flat[0].1.score);
        assert_eq!(9, sorted_flat[1].1.score);
        assert_eq!(1, sorted_flat[7].1.score);
    }

    #[test]
    fn test_get_reward() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let map = get_test_map();
        let mut map2:UnorderedMap<String, Profile> = UnorderedMap::new(b"test2".to_vec());
        for i in [4,5,7,45] {
            map2.insert(&i.to_string(), &Profile{
                hourly_reward_at: 0,
                score: i,
                username: i.to_string(),
            });
        };

        let mut contract = ProfileState::new();
        let first_player:&str = "1".as_ref();
        let second_player = String::from("4");
        contract.state.insert(&env::sha256(first_player.to_string().as_bytes()), &map);
        assert_eq!(10, contract.get_reward(first_player.to_string()));

        contract.state.insert(&env::sha256("4".as_bytes()), &map2);
        assert_eq!(50, contract.get_reward(second_player.to_string()));
        assert_eq!(0, contract.get_reward(second_player.to_string()));
    }

    #[test]
    fn test_check_time_reward() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let map = get_test_map();
        let mut map2:UnorderedMap<String, Profile> = UnorderedMap::new(b"test2".to_vec());
        for i in [4,5,7,45] {
            map2.insert(&i.to_string(), &Profile{
                hourly_reward_at: 0,
                score: i,
                username: i.to_string(),
            });
        };

        let mut contract = ProfileState::new();
        let first_player:&str = "1".as_ref();
        let second_player = String::from("4");
        contract.state.insert(&env::sha256(first_player.to_string().as_bytes()), &map);
        // not rewarded yet
        assert_eq!(false, contract.is_recently_rewarded(first_player.to_string()));
        contract.state.insert(&env::sha256("4".as_bytes()), &map2);
        assert_eq!(false, contract.is_recently_rewarded(first_player.to_string()));

        // rewarded correct value in range
        assert_eq!(10, contract.get_reward(first_player.to_string()));
        assert_eq!(50, contract.get_reward(second_player.to_string()));

        // rewarded check
        assert_eq!(true, contract.is_recently_rewarded(first_player.to_string()));
        assert_eq!(true, contract.is_recently_rewarded(second_player.to_string()));
    }
}


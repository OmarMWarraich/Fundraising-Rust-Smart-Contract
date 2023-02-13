use crate::Contract;
use crate::ContractExt;

use near_sdk::{AccountId, near_bindgen};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Pledge {
    pub amount: u128,
    pub donor: AccountId,
    pub message: String,
}

#[near_bindgen]
impl Contract {
    pub fn get_num_pledges(&self) -> u64 {
        self.pledges.len()
    }
}
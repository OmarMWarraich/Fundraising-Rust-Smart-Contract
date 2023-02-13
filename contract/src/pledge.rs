use crate::Contract;
use crate::ContractExt;

use near_sdk::{AccountId, near_bindgen, env, Balance, Promise};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

pub const STORAGE_COST: u128 = 100_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Pledge {
    pub amount: U128,
    pub donor: AccountId
}

#[near_bindgen]
impl Contract {
    pub fn get_num_pledges(&self) -> u64 {
        self.pledges.len()
    }

    pub fn get_pledge(&self, donor: AccountId) -> Pledge {
        
        Pledge {
            donor: donor.clone(),
            amount: U128(self.pledges.get(&donor).unwrap_or(0))
        }
    }

    #[payable]
    pub fn pledge(&mut self) -> U128 {
        let pledger = env::predecessor_account_id();
        let pledge_amount: Balance = env::attached_deposit();
        
        let mut pledged = self.pledges.get(&pledger).unwrap_or(0);

        let to_transfer: Balance = if pledged == 0 {
            assert!(pledge_amount > STORAGE_COST, "Must pay storage cost of {} yoctoNEAR", STORAGE_COST);

            pledge_amount - STORAGE_COST
        } else {
            pledge_amount
        };

        pledged += pledge_amount;

        self.pledges.insert(&pledger, &pledged);

        Promise::new(self.fundraiser.clone()).transfer(to_transfer);

        U128(pledged)
    }
}
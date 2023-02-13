use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, env};
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub fundraiser: AccountId,
    pub pledges: UnorderedMap<AccountId, u128>
}

impl Default for Contract{
    fn default() -> Self{
        Self {
            fundraiser: "earthling.testnet".parse().unwrap(),
            pledges: UnorderedMap::new(b"p")
        }
    }
}

#[near_bindgen]
impl Contract {
    
    #[init]
    #[private]
    pub fn init(fundraiser_init: AccountId) -> Self {

        assert!(!env::state_exists(), "Already initialized");

        Self {
            fundraiser: fundraiser_init,
            pledges: UnorderedMap::new(b"p")
        }

    }
}

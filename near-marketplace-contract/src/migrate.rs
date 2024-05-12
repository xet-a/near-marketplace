use crate::*;
use near_sdk::near;


#[near(serializers=[borsh])]
pub struct OldMarketPlace {
    products: UnorderedMap<String, String>
}

#[near]
impl MarketPlace {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        // retrieve the current state from the contract
        let old: OldMarketPlace = env::state_read().expect("failed");

        // iterate through the state migrating it to the new version
        let mut new_marketplace = Self::default();

        for (idx, value) in old.products.iter().enumerate() {
            let product = Product {
                id: value.0.to_string(),
                name: value.1.to_string(),
                description: String::new(),
                image: String::new(),
                location: String::new(),
                price: String::new(),
                owner: env::signer_account_id(),
                sold: 0,
            };

            new_marketplace.listed_products.insert(&value.0, &product);
        }

        // return the new state
        new_marketplace
    }
}
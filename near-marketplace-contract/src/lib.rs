// UnorderedMap: storage로 사용할 data structure
// Borsh~: 직렬화, 역직렬화 format
// near_bindgen: 함수, 구조체가 외부에서 호출 가능하도록 생성 즉, 유효한 near contract로 변환
// PanicOnDefault: contract가 초기화되기 전에 호출되면 panic이 발생하는 Default 속성 구현
// Promise: allows to assemble transfer calls to other accounts
use near_sdk::collections::UnorderedMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, NearToken, PanicOnDefault, Promise};
use near_sdk::serde::{Deserialize, Serialize};

// mod migrate;

// declare the structure and its variable
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MarketPlace {
    listed_products: UnorderedMap<String, Product>
}

// impl Default for MarketPlace {
//     fn default() -> Self {
//         Self {
//             listed_products: UnorderedMap::new(b"listed_products".to_vec()),
//         }
//     }
// }

// implement marketplace methods
#[near_bindgen]
impl MarketPlace {

    // initialize the contract
    // create a new instance of `UnorderedMap`
    #[init]
    pub fn init() -> Self {
        Self {
            listed_products: UnorderedMap::new(b"listed_products".to_vec()),
        }
    }

    // add a new product to the `listed_products` map by using `Payload` struct
    // * first check if the product id already exists in the map
    // If it does, this throws an error
    pub fn set_product(&mut self, payload: Payload) {
        let product = Product::from_payload(payload);
        self.listed_products.insert(&product.id, &product);
    }

    // retreive a product from the `products` map
    // return either a product struct or `null`
    // * all read methods should be immutable
    pub fn get_product(&self, id: &String) -> Option<Product> {
        self.listed_products.get(id)
    }

    // return all products in the map
    pub fn get_products(&self) -> Vec<Product> {
        self.listed_products.values_as_vector().to_vec()
    }

    #[payable]
    pub fn buy_product(&mut self, product_id: &String) {
        // retrieve the product with the specified id
        match self.listed_products.get(product_id) {
            // check if the product exists
            Some(ref mut product) => {
                let price_yoctonear: u128 = product.price.parse::<u128>().unwrap();
                let price_token = NearToken::from_yoctonear(price_yoctonear);
                let deposit = env::attached_deposit();

                // check if the attached deposit is equl to the product's price
                assert_eq! (deposit, price_token, "attatched deposit should be equal to the product's");
                
                let owner = &product.owner.as_str();
                // then create a new `Promise` object and call the `transfer` method
                Promise::new(owner.parse().unwrap()).transfer(price_token);
                
                // increment the `sold` field of the product and update the product in the `listed_products` map
                product.increment_sold_amount();
                self.listed_products.insert(&product.id, &product);
            },
            _ => {
                env::panic_str("product not found");
            }
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct Product {
    id: String,
    name: String,
    description: String,
    image: String,
    location: String,
    price: String, // u128 큰 숫자를 문자열로 변환하여 저장
    owner: AccountId,
    sold: u32
}

// used as an intermediate object, Product 구조체로 매핑
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    id: String,
    name: String,
    description: String,
    image: String,
    location: String,
    price: String
}

impl Product {
    
    // map Payload to Product
    // excepts for two properties: `sold`, `owner`
    // *near-sdk 5.0.0부터 non-init method에서 Self 참조 불가능
    pub fn from_payload(payload: Payload) -> Product {
        Self {
            id: payload.id,
            description: payload.description,
            name: payload.name,
            location: payload.location,
            price: payload.price,
            sold: 0,
            image: payload.image,
            owner: env::signer_account_id()
        }
    }

    // increment the `sold` value after a product has been sold
    pub fn increment_sold_amount(&mut self) {
        self.sold = self.sold + 1;
    }
}
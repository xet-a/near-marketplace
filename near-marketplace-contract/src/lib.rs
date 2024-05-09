// UnorderedMap: storage로 사용할 data structure
// Borsh~: 직렬화, 역직렬화 format
// near_bindgen: 함수, 구조체가 외부에서 호출 가능하도록 생성
// PanicOnDefault: contract가 초기화되기 전에 호출되면 panic이 발생하는 Default 속성 구현
use near_sdk::collections::UnorderedMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

// declare the structure and its variable
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MarketPlace {
    products: UnorderedMap<String, String>
}

// implement marketplace methods
#[near_bindgen]
impl MarketPlace {

    // initialize the contract
    // create a new instance of `UnorderedMap`
    #[init]
    pub fn init() -> Self {
        Self {
            products: UnorderedMap::new(b"product".to_vec()),
        }
    }

    // add a new product to the `products` map
    // id: key, product_name: value
    // * the key for the persistent collection should be as short as possible to reduce storage space
    // because this key will be repeated for every record in the collection *
    pub fn set_product(&mut self, id: String, product_name: String) {
        self.products.insert(&id, &product_name);
    }

    // retreive a product from the `products` map
    // return either a product name or `null`
    // * all read methods should be immutable
    pub fn get_product(&self, id: &String) -> Option<String> {
        self.products.get(id)
    }
}
### Cargo.toml

- **`Borsh` 패키지 의존성 수정**
    
    `Could not find borsh in dependencies or dev-dependencies in ~`

    → `lib.rs`에서 사용하는 `Borsh` 패키지 의존성 추가 (`borsh = "0.10.0"`)

- **`near_sdk` 업데이트**

    `no matching package named parity-secp256k1 error`
    
    → `near_sdk` 최신 버전 5.1.0 으로 업데이트

- **legacy enable**
  
    `unresolved import near_sdk::collections`
    
    → 5.0.0 이후로 `near_sdk::collections` 사용 시 features = ["legacy"] 추가해야 함
    

- 기타 오류는 [near_sdk CHANGELOG](https://github.com/near/near-sdk-rs/blob/master/CHANGELOG.md) 참고

---

### lib.rs

- `near_bindgen` 매크로 중복 제거
    
    `MarketPlace` struct 정의 및 impl에만 `near_bindgen` 매크로 작성할 것
    
    `MarketPlace`에 종속되는 `Product`와 `Payload`에 추가로 적용할 필요 없음

- `price` type 변경
    
    `Product` 및 `Payload` 구조체의 `price` 필드는 u128 라는 큰 숫자를 String으로 변환하여 저장함.

    `buy_product` 메서드 사용 시 `price` 값을 불러오면서 아래와 같이 `NearToken` 형태로 변환해야 함.

    ```
    let price_yoctonear: u128 = product.price.parse::<u128>().unwrap();
    let price_token = NearToken::from_yoctonear(price_yoctonear);
    ```

---

### Contract 관련

- **contract를 배포할 subaccount 생성 시 충분한 `initial_balance`를 설정할 것**

    subaccount에 contract 배포 후 함수를 call 할 때, token이 부족하다고 나온다면,
    [near faucet](https://near-faucet.io/)에서 master account에 충분한 토큰을 전송하고,
    [testnet mynearwallet](https://testnet.mynearwallet.com/send-money)에서 subaccount로 토큰 전송할 수 있음.

- **contract 재배포**

    contract를 한 번 배포한 계정에 수정한 contract를 재배포하는 것은 가능하지만,
    
    `Marketplace` 구조체 내부 필드를 수정한 경우에는 함수 call 시 `deserialize` 오류가 발생함.

    이 경우에는 contract를 배포한 account를 삭제하고, 다시 생성하는 방법이 깔끔함.

    아래 링크들을 참고해서 `Migration` 관련 함수를 작성해 재배포할 수도 있으나 복잡하므로, 가능한 account를 새로 생성하는 것을 추천함.

    - [Updating Contracts](https://docs.near.org/build/smart-contracts/release/upgrade#updating-through-tools)
    - [How to Migrate the Contract](https://docs.welldonestudio.io/tutorials/near-ecosystem/migrate-contract/)
    - [니어 프로토콜 컨트랙트 패턴 이해하기](https://medium.com/dsrv/near-102-understanding-near-protocol-smart-contract-pattern-bc2f746fd4ba)
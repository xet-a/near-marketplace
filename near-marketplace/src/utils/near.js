// Connect to the NEAR network and the smart contract

import { environment, CONTRACT_NAME } from "./config";
import { connect, Contract, keyStores, WalletConnection } from "near-api-js";
import { formatNearAmount } from "near-api-js/lib/utils/format";

const nearEnv = environment("testnet");

export async function initializeContract() {
    const near = await connect(
        Object.assign(
            { keyStore: new keyStores.BrowserLocalStorageKeyStore() },
            nearEnv
        )
    );
    // AppKeyPrefix 추가할 것
    window.walletConnection = new WalletConnection(near, "xeta.testnet");
    window.accountId = window.walletConnection.getAccountId();
    // contract 객체 생성
    window.contract = new Contract(
        window.walletConnection.account(),
        CONTRACT_NAME,
        {
            // Rust contract는 method명 동일하게 작성
            viewMethods: ["get_product", "get_products"], // not modify the state
            changeMethods: ["buy_product", "set_product"], // modify the state
        }
    );
}

/*
    interact with the wallet
*/

export async function accountBalance() {
    return formatNearAmount(
        (await window.walletConnection.account().getAccountBalance()).total,
        2
    );
}

export async function getAccountId() {
    return window.walletConnection.getAccountId();
}

export function login() {
    return window.walletConnection.requestSignIn({
        contractId: environment.contractName,
    });
}

export function logout() {
    window.walletConnection.signOut();
    window.location.reload();
}
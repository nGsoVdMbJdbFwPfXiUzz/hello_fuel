use std::str::FromStr;

use fuels::{crypto::SecretKey, prelude::*};

use tokio::runtime;

use dotenv::dotenv;
use std::env;


async fn wallet(private_key: String) {
    // let rpc_url = "beta-5.fuel.network/graphql";
    let rpc_url = "testnet.fuel.network/v1/graphql";

    // Create a provider pointing to the testnet.
    // This example will not work as the testnet does not support the new version of fuel-core
    // yet
    let provider = Provider::connect(rpc_url).await.unwrap();

    // Setup a private key
    let secret =
        SecretKey::from_str(&private_key)
            .unwrap();

    // Create the wallet
    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider));

    // Get the wallet address. Used later with the faucet
    dbg!(wallet.address().to_string());

    let balances = wallet.get_balances().await.unwrap();
    dbg!(balances);
}

fn main() {
    dotenv().ok();
    let private_key = env::var("private_key").expect("private_key 没有在 .env 文件里设置");
    let rt = runtime::Runtime::new().unwrap();
    rt.block_on(wallet(private_key));
}

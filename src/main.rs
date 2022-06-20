use ethereum_abi::Abi;
use ethereum_abi::Value;
use primitive_types::H256;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::str::FromStr;
use web3::contract::Contract;
use web3::ethabi::Uint;
use web3::futures::StreamExt;
use web3::transports::Http;
use web3::types::Log;
use web3::types::TransactionId;
use web3::types::H160;
use web3::types::U256;
use web3_rust_wrapper::Web3Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventToken {
    pub token_address: String,
    pub token_a: String,
    pub token_b: String,
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let mut web3m: Web3Manager = init_web3_connection().await;

    web3m
        .load_account(
            &env::var("ACCOUNT_ADDRESS").unwrap(),
            &env::var("PRIVATE_TEST_KEY").unwrap(),
        )
        .await;

    //let abi: Abi = load_abi_from_json("factoryabi.json");
    let router_abi = include_bytes!("../factoryabi.json");
    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";
    let router_instance: Contract<Http> = web3m
        .instance_contract(router_address, router_abi)
        .await
        .expect("error creating the contract instance");

    let factory_abi = include_bytes!("../routerabi.json");
    let factory_address = "0xB7926C0430Afb07AA7DEfDE6DA862aE0Bde767bc";
    let factory_instance: Contract<Http> = web3m
        .instance_contract(factory_address, factory_abi)
        .await
        .expect("error creating the contract instance");

    let lp_pair_abi = include_bytes!("../pairabi.json");
    let lp_pair_factory_address = "0x5E2E7b76e56abc3A922aC2Ca75B3e84bC29D766d";
    let lp_pair_factory_instance: Contract<Http> = web3m
        .instance_contract(lp_pair_factory_address, lp_pair_abi)
        .await
        .expect("error creating the contract instance");

    let lp_pair_reserves: (Uint, Uint, Uint) =
        get_token_reserves(web3m, lp_pair_factory_instance).await;

        
    Ok(())
}

async fn get_token_reserves(
    web3m: Web3Manager,
    lp_pair_factory_instance: Contract<Http>,
) -> (U256, U256, U256) {
    let lp_pair_reserves: (Uint, Uint, Uint) = web3m
        .query_contract(&lp_pair_factory_instance, "getReserves", ())
        .await
        .unwrap();
    println!("lp_pair_reserves: {:?}", lp_pair_reserves);
    lp_pair_reserves
}

async fn init_web3_connection() -> Web3Manager {
    let web3_http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
    let web3_websocket_url =
        "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";
    let web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url, 97).await;
    web3m
}

fn load_abi_from_json(filename: &str) -> Abi {
    let abi: Abi = {
        let file = File::open(filename).expect("failed to open ABI file");
        serde_json::from_reader(file).expect("failed to parse ABI")
    };
    abi
}

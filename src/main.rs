use ethereum_abi::Abi;
use ethereum_abi::Value;
use primitive_types::H256;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use web3::contract::Contract;
use web3::futures::StreamExt;
use web3::transports::Http;
use web3::types::Log;
use web3::types::TransactionId;
use web3::types::H160;
use web3_rust_wrapper::Web3Manager;
use clap::Parser;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventToken {
    pub token_address: String,
    pub token_a: String,
    pub token_b: String,
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

        let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

    let mut web3m: Web3Manager = init_web3_connection().await;

    web3m
        .load_account(
            &env::var("ACCOUNT_ADDRESS").unwrap(),
            &env::var("PRIVATE_TEST_KEY").unwrap(),
        )
        .await;

    //let json_file_path = Path::new("tokens.json");
    //let file = File::open(json_file_path).expect("file not found");
    //let tokens: Vec<&mut EventToken>;// =  serde_json::from_reader(file).expect("error while reading");

    let tokens: Vec<EventToken> = Vec::new();

    let abi: Abi = load_abi_from_json("factoryabi.json");
    let factory_abi = include_bytes!("../factoryabi.json");
    let factory_address = "0xB7926C0430Afb07AA7DEfDE6DA862aE0Bde767bc";
    let factory_instance: Contract<Http> = web3m
        .instance_contract(factory_address, factory_abi)
        .await
        .expect("error creating the contract instance");

    println!("Listening...");
    let contract_event_subscription = web3m.build_contract_events(factory_address).await;

    contract_event_subscription
        .for_each(|log| async {
            let l: Log = log.unwrap();
            println!("transaction_hash: {:?}", l.transaction_hash.unwrap());

            let topics: &[H256] = &[
                H256::from_str(&format!("{:#x}", l.topics[0])).unwrap(),
                H256::from_str(&format!("{:#x}", l.topics[1])).unwrap(),
                H256::from_str(&format!("{:#x}", l.topics[2])).unwrap(),
            ];

            // Decode
            let (evt, decoded_data) = abi
                .decode_log_from_slice(topics, &l.data.0)
                .expect("failed decoding log");

            if let (
                Value::Address(token_address),
                Value::Address(token_a),
                Value::Address(token_b),
            ) = (
                decoded_data[0].value.clone(),
                decoded_data[1].value.clone(),
                decoded_data[2].value.clone(),
            ) {
                let event_token = EventToken {
                    token_address: token_address.to_string(),
                    token_a: token_a.to_string(),
                    token_b: token_b.to_string(),
                };
                //tokens.push(event_token.clone());
                println!("{}", serde_json::to_string(&event_token).unwrap());
            }
        })
        .await;

    Ok(())
}

async fn get_token_reserves() {

}

async fn init_web3_connection() -> Web3Manager {
    let web3_http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
    let web3_websocket_url =
        "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";
    let mut web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url, 97).await;
    web3m
}

fn load_abi_from_json(filename: &str) -> Abi {
    let abi: Abi = {
        let file = File::open(filename).expect("failed to open ABI file");
        serde_json::from_reader(file).expect("failed to parse ABI")
    };
    abi
}

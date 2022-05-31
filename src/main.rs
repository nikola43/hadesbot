use ethereum_abi::Abi;
use ethereum_abi::Value;
use primitive_types::H256;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::str::FromStr;
use web3::contract::Contract;
use web3::futures::StreamExt;
use web3::transports::Http;
use web3::types::Log;
use web3::types::TransactionId;
use web3::types::H160;
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

    let web3_http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
    let web3_websocket_url =
        "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";

    let mut web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url, 97).await;

    // load acount from .env file
    web3m
        .load_account(
            &env::var("ACCOUNT_ADDRESS").unwrap(),
            &env::var("PRIVATE_TEST_KEY").unwrap(),
        )
        .await;

    // init contract

    // Parse ABI JSON file
    // Parse ABI JSON file
    let abi: Abi = {
        let file = File::open("factoryabi.json").expect("failed to open ABI file");
        serde_json::from_reader(file).expect("failed to parse ABI")
    };

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
            println!("Address: {:?}", l.transaction_hash.unwrap());
            println!("Data: {:?}", l.data);
            println!("Data0: {:?}", l.data.0);
            println!("topics: {:?}", l.topics);
            println!("topics len: {:?}", l.topics.len());
            println!("log_type: {:?}", l.log_type);

            let tx = web3m
                .web3http
                .eth()
                .transaction(TransactionId::Hash(l.transaction_hash.unwrap()))
                .await
                // .unwrap()
                .unwrap();

            // let from_addr = tx.from.unwrap_or(H160::zero());
            // let to_addr = tx.to.unwrap_or(H160::zero());
            // let value = tx.value;
            // let input = tx.input;

            if let Some(transaction) = tx {
                let value = transaction.value;
                let input = transaction.input;
            }
            //println!("from_addr: {:?}", from_addr);
            //println!("to_addr: {:?}", to_addr);
            //println!("value: {:?}", value);
            //println!("input: {:?}", input);

            let topics: &[H256] = &[
                H256::from_str(&format!("{:#x}", l.topics[0])).unwrap(),
                H256::from_str(&format!("{:#x}", l.topics[1])).unwrap(),
                H256::from_str(&format!("{:#x}", l.topics[2])).unwrap(),
            ];

            println!("topics: {:?}", topics);

            // Decode
            let (evt, decoded_data) = abi
                .decode_log_from_slice(topics, &l.data.0)
                .expect("failed decoding log");

            println!("event: {}", evt.name);
            println!(
                "{}{} {:?}",
                0,
                decoded_data[0].value.clone().type_of().to_string(),
                decoded_data[0].value
            );
            println!(
                "{}{} {:?}",
                1,
                decoded_data[0].value.clone().type_of().to_string(),
                decoded_data[1].value
            );
            println!(
                "{}{} {:?}",
                2,
                decoded_data[0].value.clone().type_of().to_string(),
                decoded_data[2].value
            );
            println!(
                "{}{} {:?}",
                3,
                decoded_data[0].value.clone().type_of().to_string(),
                decoded_data[3].value
            );

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
                println!("{}", serde_json::to_string(&event_token).unwrap());
            }

            /*
            if let Value::Address(addressValue) = value {
                let wallet = addressValue.to_string();
                println!("{:?}", wallet);
            }
            */

            //let wallet = decoded_data[3].value.to_string();

            /*

            EventToken {
                token_address: decoded_data[3].value,
                token_a: decoded_data[3].value,
                token_b: decoded_data[3].value,
            }
            */

            //let _address = decoded_data[0].value; // 0xb05d02570e1A30eCa1F5DF0EefF8BBb2899e1784
            /*
            for i in 0..decoded_data.len() {
                println!("{} {:?}", i, decoded_data[i].value);
                println!(
                    "{} {:?}",
                    i,
                    decoded_data[i].value.clone().type_of().to_string()
                );
            }
            */
        })
        .await;

    /*
    // call example
    let account: H160 = web3m.first_loaded_account();
    let token_balance: Uint = web3m.query_contract(&contract_instance, "balanceOf", account).await.unwrap();
    println!("token_balance: {}", token_balance);

    let value = "100000000000000";
    //println!("value: {:?}", wei_to_eth(value));

    let path_address: Vec<&str> = vec![
        "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd", // WAVAX
        "0x7ef95a0FEE0Dd31b22626fA2e10Ee6A223F8a684"  // TOKEN
        ];

    let now = Instant::now();
    let slippage = 40usize;

    for _ in 0..10 {
        let tx_id: H256 = web3m
            .swap_eth_for_exact_tokens(account, "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3", value,&path_address, slippage)
            .await
            .unwrap();
        //let sleep_time = time::Duration::from_millis(100);
        //thread::sleep(sleep_time);
    }

    let elapsed = now.elapsed();
    println!("elapsed: {:?}", elapsed);
    */

    Ok(())
}

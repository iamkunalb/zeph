use log::warn;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use oasis_runtime_sdk::crypto::signature::secp256k1;
use oasis_runtime_sdk::modules::rofl::app::prelude::*;
use oasis_runtime_sdk::types::address::SignatureAddressSpec;
use serde_json::{json, Value};

const MESSAGING_CONTRACT_ADDRESS: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
const ROFL_APP_ID: &str = "rofl1qqn9xndja7e2pnxhttktmecvwzz0yqwxsquqyxdf";

struct MessagingApp;

#[async_trait]
impl App for MessagingApp {
    const VERSION: Version = sdk::version_from_cargo!();

    fn id() -> AppId {
        ROFL_APP_ID.into()
    }

    fn consensus_trust_root() -> Option<TrustRoot> {
        None
    }

    async fn run(self: Arc<Self>, _env: Environment<Self>) {
        warn!("Messaging App Started!");
    }

    async fn on_runtime_block(self: Arc<Self>, env: Environment<Self>, _round: u64) {
        // Process any pending memberships
        if let Err(err) = self.read_message_from_contract(&env).await {
            println!("Failed to process pending memberships: {:?}", err);
        }
    }
}

impl MessagingApp {
    async fn read_message_from_contract(&self, env: &Environment<Self>) -> Result<()> {
        // Access the client from the environment.
        let client = env.client();

        let data: Vec<u8> = [
            ethabi::short_signature("getMessage", &[]).to_vec(),
            ethabi::encode(&[]),
        ]
        .concat();
        
        let sdk_pub_key =
            secp256k1::PublicKey::from_bytes(env.signer().public_key().as_bytes()).unwrap();
        let caller = module_evm::derive_caller::from_sigspec(&SignatureAddressSpec::Secp256k1Eth(
            sdk_pub_key,
        ))
        .unwrap();
        
        let res: Vec<u8> = env
            .client()
            .query(
                env.client().latest_round().await?.into(),
                "evm.SimulateCall",
                module_evm::types::SimulateCallQuery {
                    gas_price: 10_000.into(),
                    gas_limit: 100_000,
                    caller,
                    address: Some(MESSAGING_CONTRACT_ADDRESS.parse().unwrap()),
                    value: 0.into(),
                    data,
                },
            )
            .await?;
        
        let decoded = ethabi::decode(&[ethabi::ParamType::String], &res)
            .map_err(|e| anyhow::anyhow!("Failed to decode response: {}", e))?;
        
        let message = decoded[0].clone().into_string().unwrap();

        
        // Print the message to the console.
        println!("Message from smart contract: {}", message);

        let url = "http://172.17.0.3:5000/generate";
        let body = json!({ "prompt": message });
        let payload_str = body.to_string();

        // Spawn a blocking task to make the HTTP request
        let response: String = tokio::task::spawn_blocking(move || -> Result<String> {
            let agent = rofl_utils::https::agent(); // Create the HTTP agent

            // Perform the POST request to the local API
            let rsp: String = agent
                .post(&url.to_string())
                .header("Content-Type", "application/json")
                .send(&payload_str)?
                .body_mut()
                .read_to_string()?; // Read the response body as a string

            let parsed: serde_json::Value = serde_json::from_str(&rsp)?;

            let generated_text = parsed
                .as_array()
                .and_then(|arr| arr.get(0)) // Get the first object
                .and_then(|obj| obj.get("generated_text")) // Get "generated_text"
                .and_then(|text| text.as_str()) // Convert to string if it's valid
                .unwrap_or("Fallback message"); // Use fallback if not found
            // // Extract the "generated_text" field from the JSON response
            // if let Some(generated_text) = parsed[0]["generated_text"].as_str() {
            //     Ok(generated_text.to_string()) // Return the extracted generated_text
            // } else {
            //     Err(anyhow::anyhow!("Generated text not found in response").into())
            // }
            Ok(generated_text.to_string())
        })
        .await
        .map_err(|e| anyhow::anyhow!("Spawn blocking error: {}", e))??;



        // let host = "172.17.0.3:5000";
        // let endpoint = "/generate";

        // // Create the HTTP request payload
        // let json_payload = r#"{"prompt": "the message"}"#;
        
        // // Manually construct the HTTP request
        // let request = format!(
        //     "POST {} HTTP/1.1\r\n\
        //     Host: {}\r\n\
        //     Content-Type: application/json\r\n\
        //     Content-Length: {}\r\n\
        //     Connection: close\r\n\
        //     \r\n\
        //     {}",
        //     endpoint, host, json_payload.len(), json_payload
        // );

        // // Open a TCP connection
        // let mut stream = TcpStream::connect(host)?;

        //  // Set a read timeout to wait for the response (e.g., 30 seconds)
        // stream.set_read_timeout(Some(Duration::from_secs(30)))?;

        // // Send the request
        // stream.write_all(request.as_bytes())?;
        
        // // Read the response
        // let mut response = String::new();
        // let mut buffer = [0; 1024]; // Buffer to store chunks of data

        // loop {
        //     match stream.read(&mut buffer) {
        //         Ok(0) => break, // No more data, connection closed
        //         Ok(n) => response.push_str(&String::from_utf8_lossy(&buffer[..n])),
        //         Err(e) => {
        //             eprintln!("Error reading response: {}", e);
        //             break;
        //         }
        //     }
        // }

        // if let Some(json_start) = response.find("\r\n\r\n") {
        //     let json_body = &response[json_start + 4..];
    
        //     // Extract `generated_text`
        //     if let Some(start) = json_body.find("\"generated_text\":") {
        //         let start_index = start + "\"generated_text\":\"".len();
        //         if let Some(end_index) = json_body[start_index..].find("\"") {
        //             let generated_text = &json_body[start_index..start_index + end_index];
        //             println!("Generated Text: {}", generated_text);

        //             let mut tx: oasis_runtime_sdk::types::transaction::Transaction = self
        //                 .new_transaction(
        //                     "evm.Call",
        //                     module_evm::types::Call {
        //                         address: MESSAGING_CONTRACT_ADDRESS.parse().unwrap(),
        //                         value: 0.into(),
        //                         data: [
        //                             ethabi::short_signature(
        //                                 "setResponse",
        //                                 &[ethabi::ParamType::String],
        //                             )
        //                             .to_vec(),
        //                             ethabi::encode(&[
        //                                 ethabi::Token::String(generated_text.to_string().into()),
        //                             ]),
        //                         ]
        //                         .concat(),
        //                     },
        //                 );

        //             tx.set_fee_gas(200_000);

        //             client.sign_and_submit_tx(env.signer(), tx).await?;
        //         }
        //     }
        // }


               
        // let response_sent = format!("{} from ai", message);
        // let response_sent = format!("{} from ai");
        
        let mut tx: oasis_runtime_sdk::types::transaction::Transaction = self
            .new_transaction(
                "evm.Call",
                module_evm::types::Call {
                    address: MESSAGING_CONTRACT_ADDRESS.parse().unwrap(),
                    value: 0.into(),
                    data: [
                        ethabi::short_signature(
                            "setResponse",
                            &[ethabi::ParamType::String],
                        )
                        .to_vec(),
                        ethabi::encode(&[
                            ethabi::Token::String(response.to_string().into()),
                        ]),
                    ]
                    .concat(),
                },
            );

        tx.set_fee_gas(200_000);

        client.sign_and_submit_tx(env.signer(), tx).await?;

        Ok(())
    }
}

fn main() {
    MessagingApp.start();
}

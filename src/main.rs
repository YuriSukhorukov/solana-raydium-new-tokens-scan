use std::str::FromStr;
use futures_util::StreamExt;
use dotenv::dotenv;

use solana_sdk::{
    commitment_config::CommitmentConfig, 
    signature::Signature
};
use solana_client::{
    nonblocking::pubsub_client::PubsubClient, 
    rpc_client::RpcClient, 
    rpc_config::{
        RpcTransactionConfig, 
        RpcTransactionLogsConfig, 
        RpcTransactionLogsFilter
    }
};
use solana_transaction_status::{
    EncodedTransaction, 
    UiInstruction, 
    UiMessage, 
    UiParsedInstruction, 
    UiTransactionEncoding
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let SEARCH_INSTRUCTION = "initialize2";
    let RAYDIUM_ADDRESS = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

    let http_url = std::env::var("HTTP_URL").expect("HTTP_URL variable is not set").to_string();
    let wss_url = std::env::var("WSS_URL").expect("WSS_URL variable is not set");

    println!("{}\n{}", http_url, wss_url);

    let rpc_commitment_config = CommitmentConfig::finalized();
    let rpc_client = RpcClient::new_with_commitment(http_url, rpc_commitment_config);
    let client = PubsubClient::new(&wss_url).await?;

    let (mut stream, _) = client.logs_subscribe(RpcTransactionLogsFilter::All, RpcTransactionLogsConfig {
        commitment: Some(CommitmentConfig::finalized())
    }).await?;

    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::JsonParsed),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    loop {
        let logs = stream.next().await.unwrap();
        let value = logs.value;

        let signature: Option<String> = match value.logs.clone().into_iter().any(|v| v.contains(SEARCH_INSTRUCTION)) {
            true => Some(value.signature.clone()),
            false => None
        };

        match signature {
            Some(s) => {
                let signature = Signature::from_str(&s).unwrap();
                let tx = rpc_client.get_transaction_with_config(&signature, config).unwrap();

                let json = if let EncodedTransaction::Json(t) = tx.transaction.transaction {
                    Some(t)
                } else {
                    None
                };

                let message = if let UiMessage::Parsed(m) = json.unwrap().message {
                    Some(m)
                } else {
                    None
                };

                let pairs = message.unwrap().instructions.into_iter().map(|instruction| {
                    let instruction_parsed = if let UiInstruction::Parsed(i) = instruction {
                        Some(i)
                    } else {
                        None
                    };
                    let accounts = if let UiParsedInstruction::PartiallyDecoded(i) = instruction_parsed.unwrap() {                        
                        if i.program_id == RAYDIUM_ADDRESS { 
                            Some(i.accounts) 
                        } else {
                            None 
                        }
                    } else { 
                        None 
                    };

                    match accounts {
                        Some(a) if a.len() > 9 => {
                            let token_index_a = 8;
                            let token_index_b = 9;
                            let token_a = a[token_index_a].clone();
                            let token_b = a[token_index_b].clone();
                            Some(Pair {token_a, token_b})
                        },
                        _ => None
                    }
                });

                    println!("// ---");
                    println!("Signature:      {:?}", signature.to_string());

                for pair in pairs.into_iter().flatten() {
                    println!("Token A:        {:?}", pair.token_a);
                    println!("Token B:        {:?}", pair.token_b);
                }
            },
            _ => (),
        }
    }
}

struct Pair {
    token_a: String,
    token_b: String
}

use std::str::FromStr;
use std::sync::Arc;
use futures_util::StreamExt;
use dotenv::dotenv;

use solana_client::{
    nonblocking::pubsub_client::PubsubClient, rpc_client::RpcClient, rpc_config::{
        RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter
    }, rpc_filter::RpcFilterType, tpu_client::{
        TpuClient, 
        TpuClientConfig
    }
};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_program::pubkey::Pubkey;

// use futures_util::StreamExt;
// use solana_client::nonblocking::pubsub_client::PubsubClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    dotenv().ok();

    let mainnet_http_solana = std::env::var("MAINNET_HTTP_SOLANA").expect("MAINNET_HTTP_SOLANA must be set").to_string();
    let websocket_url = std::env::var("MAINNET_WSS_SOLANA").expect("MAINNET_WSS_SOLANA must be set");

    println!("{}\n{}", mainnet_http_solana, websocket_url);

    let rpc_commitment_config = CommitmentConfig::finalized();
    let tpu_client_config = TpuClientConfig::default();
    let rpc_client = RpcClient::new_with_commitment(mainnet_http_solana, rpc_commitment_config);
    let tpu_client = TpuClient::new(Arc::new(rpc_client), websocket_url.as_ref(), tpu_client_config);
    let raydium_pubkey = Pubkey::from_str(&String::from("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8")).unwrap();

    let client = PubsubClient::new(&websocket_url).await?;
    let (mut accounts, unsubscriber) = client.program_subscribe(
        &raydium_pubkey,
        None
    ).await?;

    while let Some(response) = accounts.next().await {
        println!("{:?}", response);
    }




    // let (mut stream, _) = client.logs_subscribe(
    //     // RpcTransactionLogsFilter::Mentions(
    //     //     vec![String::from("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8")]
    //     // ), 
    //     RpcTransactionLogsFilter::AllWithVotes, 
    //     RpcTransactionLogsConfig {
    //         commitment: Some(CommitmentConfig::finalized())
    //     }
    // ).await?;

    // loop {
    //     let logs = stream.next().await.unwrap();
    //     println!("logs: {:?}", logs.context);
    // }



    // let (mut accounts, unsubscriber) = client.program_subscribe(&raydium_pubkey.unwrap(), Some(RpcProgramAccountsConfig {filters: Some((vec![RpcFilterType::DataSize((64), RpcFilterType::Memcmp((172), RpcFilterType::TokenAccountState))]))})).await?;
    // let (mut stream, _) = PubsubClient::program_subscribe(&websocket_url, &raydium_pubkey, None).await?;

    // while let Some(response) = accounts.next().await {
    //     println!("{:?}", response);
    // }

    // let aaa = client.program_subscribe(&raydium_pubkey, RpcProgramAccountsConfig {
    //     filters: Some([RpcFilterType::TokenAccountState])
    //     account_config: 
    // })

    Ok(())
}

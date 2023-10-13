use anyhow::Result;
use reqwest;
use serde_json::Value;
use super::http;
// Include the fetched module
use crate::collect::fetched;

pub async fn fetch_finalized_block_hash(url: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let response: Value = client
        .post(url)
        .json(&serde_json::json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "chain_getFinalizedHead",
            "params": []
        }))
        .send()
        .await?
        .json()
        .await?;

    if let Some(finalized_block_hash) = response["result"].as_str() {
        fetched::set_finalized_block_hash(finalized_block_hash.to_string());
        Ok(finalized_block_hash.to_string())
    } else {
        anyhow::bail!("Could not fetch the hash of the last finalized block.")
    }
}

pub async fn fetch_and_print_block_header(url: &str, hash: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let response: Value = client
        .post(url)
        .json(&serde_json::json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "chain_getHeader",
            "params": [hash]
        }))
        .send()
        .await?
        .json()
        .await?;

    fetched::set_block_header(response["result"].to_string());
    Ok(())
}

pub async fn fetch_and_print_extrinsics(url: &str, hash: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let response: Value = client
        .post(url)
        .json(&serde_json::json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "chain_getBlock",
            "params": [hash]
        }))
        .send()
        .await?
        .json()
        .await?;

    fetched::set_extrinsics(response["result"]["block"]["extrinsics"].to_string());
    Ok(())
}

pub async fn fetch_and_set_chain(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = http::post_rpc_request(url, "system_chain", Vec::new()).await?;
    let chain = response["result"].as_str().unwrap_or_default().to_string();
    fetched::set_chain(chain);
    Ok(())
}

pub async fn submit_extrinsic(url: &str, extrinsic: &str) -> Result<String, reqwest::Error> {
    let params = vec![serde_json::Value::String(extrinsic.to_string())];
    let response = http::post_rpc_request(url, "author_submitExtrinsic", params).await?;
    
    // Assuming the response contains the hash as a string.
    let hash = response.as_str().unwrap_or_default().to_string();
    
    // Setting the extrinsic hash
    fetched::set_extrinsic_hash(hash.clone());
    
    Ok(hash)
}


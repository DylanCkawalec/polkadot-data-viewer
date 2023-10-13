// src/utils/http.rs

use reqwest;
use serde_json::Value;

pub async fn post_rpc_request(url: &str, method: &str, params: Vec<Value>) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();

    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": "1",
        "method": method,
        "params": params
    });

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let json_response: Value = response.json().await?;
    Ok(json_response)
}

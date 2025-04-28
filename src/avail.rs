use avail_rust::prelude::*;
use avail_rust::subxt::utils::AccountId32;
use std::process::Command;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitResponse {
    block_number: u32,
    block_hash: String,
    hash: String,
    index: u32,
}

/// Retrieve the address of the wallet on the light node.
pub async fn get_wallet_address_from_avail() -> Result<AccountId32, ClientError> {
    let output = Command::new("docker")
        .arg("exec")
        .arg("-i")
        .arg("avail-light-node")
        .arg("bash")
        .arg("-c")
        .arg("grep avail_secret_uri /root/.avail/turing/identity/identity.toml")
        .output()
        .map_err(|e| ClientError::Custom(format!("Failed to execute Docker command: {}", e)))?;

    if !output.status.success() {
        return Err(ClientError::Custom(format!("Docker command failed: {:?}", output.status)));
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| ClientError::Custom(format!("Invalid UTF-8 output: {}", e)))?;

    let secret_phrase = stdout
        .split('\'')
        .nth(1)
        .ok_or_else(|| ClientError::Custom("Failed to parse secret URI".to_string()))?
        .trim()
        .to_string();

    let account = account::from_secret_uri(&secret_phrase)?;
    let address = account.public_key().to_account_id();

    Ok(address)
}

/// Prepare a blob: JSON serialize -> Base64 encode
pub async fn prepare_avail_blob<T: Serialize>(data: &T) -> Result<String, ClientError> {
    let json = serde_json::to_string(data)
        .map_err(|e| ClientError::Custom(format!("Failed to serialize JSON: {}", e)))?;
    let encoded = STANDARD.encode(&json);

    Ok(encoded)
}

/// Submit a blob to the avail light node
pub async fn submit_blob_to_avail(
    client_url: &str,
    blob: String,
) -> Result<SubmitResponse, ClientError> {
    let client = Client::new();

    let response = client
        .post(format!("{}/v2/submit", client_url))
        .header("Content-Type", "application/json")
        .body(json!({ "data": blob }).to_string())
        .send()
        .await
        .map_err(|e| ClientError::Custom(format!("HTTP request failed: {}", e)))?;

    let status = response.status();

    if status == StatusCode::OK {
        let submit_response: SubmitResponse = response
            .json()
            .await
            .map_err(|e| ClientError::Custom(format!("Failed to parse SubmitResponse: {}", e)))?;
        Ok(submit_response)
    } else {
        Err(ClientError::Custom(format!(
            "Failed to submit blob: HTTP {}",
            status
        )))
    }
}

/// Display final result to the user via console.
pub fn avail_log_submission_result(response: &SubmitResponse) {
    println!("✅ Game submitted to Avail!");
    println!("🔗 Included in block: {}", response.block_number);
    println!(
        "🌐 View on Avail Explorer: https://avail-turing.subscan.io/extrinsic/{}",
        response.hash
    );
}

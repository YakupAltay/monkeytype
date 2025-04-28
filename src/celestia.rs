use anyhow::Result;
use celestia_rpc::{Client, TxConfig, StateClient};
use celestia_types::{
    AppVersion,
    blob::{Blob, RawBlob},
    state::{Address, RawTxResponse},
};
use serde::Serialize;
use base64::Engine;
use crate::constants::get_monkeytype_namespace;

/// Retrieve the address of the wallet on the light node.
pub async fn get_wallet_address_from_celestia(client: &Client) -> Result<Address> {
    let address = client.state_account_address().await?;
    Ok(address)
}

/// Encode a serializable object to JSON, base64 it, and prepare a Blob.
pub fn prepare_celestia_blob<T: Serialize>(data: &T) -> Result<RawBlob> {
    let json = serde_json::to_string(data)?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&json);
    let namespace = get_monkeytype_namespace()?;

    let blob = Blob::new(
        namespace,
        encoded.into_bytes(),
        AppVersion::V2,
    )?;

    Ok(blob.into())
}

/// Submit a blob to Celestia using the RPC client.
pub async fn submit_blob_to_celestia(
    client: &Client,
    blob: RawBlob,
    config: TxConfig,
) -> Result<RawTxResponse> {
    match client.state_submit_pay_for_blob(&[blob], config).await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.into()),
    }
}

/// Display final result to the user via console.
pub fn celestia_log_submission_result(response: &RawTxResponse) {
    println!("✅ Game submitted to Celestia!");
    println!("🔗 Included in block: {}", response.height);
    println!(
        "🌐 View on Celenium: https://mocha-4.celenium.io/tx/{}",
        response.txhash
    );
}

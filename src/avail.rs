use dotenvy::dotenv;
use std::env;
use avail_rust::prelude::*;

type DataSubmissionCall = avail::data_availability::calls::types::SubmitData;

pub async fn submit_data() -> Result<(), ClientError> {

    // Create a new SDK instance
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Loading seed phrase and creating an account derived from the seed
    dotenv().ok();
    let seed = env::var("SEED").expect("SEED environment variable is not set");
    let account = account::from_secret_uri(&seed)?;
    println!("Account Address: {}", account.public_key().to_account_id());

    // Please note that the tx will fail if this application key does not exist
    let my_application_key = 1;

    // Data Submission
    let data = String::from("My Data").into_bytes();

    let options = Options::new().app_id(my_application_key);
    let tx = sdk.tx.data_availability.submit_data(data);
    let res = tx.execute_and_watch_inclusion(&account, options).await?;
    assert_eq!(res.is_successful(), Some(true), "Transactions must be successful");

    println!(
        "Block Hash: {:?}, Block Number: {}, Tx Hash: {:?}, Tx Index: {}",
        res.block_hash, res.block_number, res.tx_hash, res.tx_index
    );

    // Decoding
    let decoded = res.decode_as::<DataSubmissionCall>().await?;
    let Some(decoded) = decoded else {
        return Err("Failed to get Data Submission Call data".into());
    };

    let data = to_ascii(decoded.data.0).unwrap();
    println!("Call data: {:?}", data);

    println!("Data Submission finished correctly");

    Ok(())
}

// Add a main function to call submit_data
#[tokio::main]
async fn main() {
    if let Err(e) = submit_data().await {
        eprintln!("Error: {:?}", e);
    }
}
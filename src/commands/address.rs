use crate::{celestia, avail, constants, cli::DALayer};

pub async fn run(da_layers: Vec<DALayer>) {
    for layer in da_layers {
        match layer {
            DALayer::Celestia => {
                use celestia::get_wallet_address_from_celestia;
                use celestia_rpc::Client;

                let client = Client::new(constants::CELESTIA_RPC_URL, None)
                    .await
                    .expect("Celestia RPC connection failed");

                match get_wallet_address_from_celestia(&client).await {
                    Ok(addr) => println!("🔗 Celestia address: {addr}"),
                    Err(e) => eprintln!("❌ Could not get Celestia address: {e}"),
                }
            }
            DALayer::Avail => {
                use avail::get_wallet_address_from_avail;

                match get_wallet_address_from_avail().await {
                    Ok(addr) => println!("🔗 Avail address: {addr}"),
                    Err(e) => eprintln!("❌ Could not get Avail address: {:?}", e),
                }
            }
        }
    }
}

use crate::{api, game, celestia, avail, constants, cli::DALayer};
use tokio::net::TcpStream;
use std::time::Duration;

async fn is_node_running(host: &str, port: u16) -> bool {
    tokio::time::timeout(Duration::from_secs(2), TcpStream::connect((host, port)))
        .await
        .map(|r| r.is_ok())
        .unwrap_or(false)
}

pub async fn run(da_layers: Vec<DALayer>) {
    for layer in &da_layers {
        match layer {
            DALayer::Celestia => {
                if !is_node_running("127.0.0.1", 10101).await {
                    eprintln!("❌ Celestia light node not reachable. Exiting.");
                    std::process::exit(1);
                }
            }
            DALayer::Avail => {
                if !is_node_running("127.0.0.1", 10102).await {
                    eprintln!("❌ Avail light node not reachable. Exiting.");
                    std::process::exit(1);
                }
            }
        }
    }

    let words = api::fetch_words(50).await.expect("Failed to fetch words");
    let stats = game::start_typing_session(words).await;

    tokio::time::sleep(Duration::from_millis(1500)).await;
    println!("\n\n--- Session Ended ---");

    for layer in da_layers {
        match layer {
            DALayer::Celestia => {
                use celestia::{prepare_celestia_blob, submit_blob_to_celestia, celestia_log_submission_result};
                use celestia_rpc::{Client, TxConfig};

                let client = Client::new(constants::CELESTIA_RPC_URL, None)
                    .await
                    .expect("Celestia RPC connection failed");

                let blob = prepare_celestia_blob(&stats).expect("Blob creation failed");

                match submit_blob_to_celestia(&client, blob, TxConfig::default()).await {
                    Ok(res) => celestia_log_submission_result(&res),
                    Err(e) => eprintln!("❌ Celestia submission failed: {}", e),
                }
            }
            DALayer::Avail => {
                use avail::{prepare_avail_blob, submit_blob_to_avail, avail_log_submission_result};

                let blob = prepare_avail_blob(&stats).await.expect("Blob creation failed");

                match submit_blob_to_avail(constants::AVAIL_RPC_URL, blob).await {
                    Ok(res) => avail_log_submission_result(&res),
                    Err(e) => eprintln!("❌ Avail submission failed: {:?}", e),
                }
            }
        }
    }
}

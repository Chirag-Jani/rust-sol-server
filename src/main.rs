use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use dotenv::dotenv;
use serde::Deserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
// use solana_sdk::signature::read_keypair_file;
// use solana_sdk::signer::Signer;
// use solana_sdk::system_instruction;
// use solana_sdk::transaction::Transaction;
use std::env;

const DEFAULT_RPC_URL: &str = "https://api.mainnet-beta.solana.com";

#[derive(Deserialize)]
struct BalanceQuery {
    address: String,
}

// #[derive(Deserialize)]
// struct TransferBody {
//     to_address: String,
//     amount: f64,
// }

#[get("/balance")]
async fn get_balance(query: web::Query<BalanceQuery>) -> impl Responder {
    let rpc_url = env::var("MAINNET_RPC_URL").unwrap_or_else(|_| DEFAULT_RPC_URL.to_string());
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let pubkey_result = query.address.parse::<Pubkey>();

    if let Err(_) = pubkey_result {
        return HttpResponse::BadRequest().body("Invalid Solana address");
    }

    let pubkey = pubkey_result.unwrap();

    match client.get_balance(&pubkey).await {
        Ok(lamports) => {
            let sol = lamports as f64 / 1_000_000_000.0;
            HttpResponse::Ok().json(serde_json::json!({
                "address": query.address,
                "balance_sol": sol,
                "balance_lamports": lamports
            }))
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to get balance: {}", err))
        }
    }
}

// #[post("/transfer")]
// async fn transfer(body: web::Json<TransferBody>) -> impl Responder {
//     // Get RPC URL from environment variable or use default
//     let rpc_url = env::var("MAINNET_RPC_URL").unwrap_or_else(|_| DEFAULT_RPC_URL.to_string());

//     // Create RPC client
//     let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

//     // Read payer keypair from file
//     let payer = read_keypair_file("keypair.json").unwrap();

//     // Parse to_address
//     let to_pubkey = match body.to_address.parse::<Pubkey>() {
//         Ok(pubkey) => pubkey,
//         Err(_) => return HttpResponse::BadRequest().body("Invalid Solana address"),
//     };

//     // Convert amount to lamports
//     let lamports = (body.amount * 1_000_000_000.0) as u64;

//     // Create transfer instruction
//     let transfer_instruction =
//         system_instruction::transfer(&payer.try_pubkey().unwrap(), &to_pubkey, lamports);

//     // Get recent blockhash
//     let recent_blockhash = client.get_latest_blockhash().await.unwrap();

//     // Create transaction
//     let transaction = Transaction::new_signed_with_payer(
//         &[transfer_instruction],
//         Some(&payer.pubkey()),
//         &[&payer],
//         recent_blockhash,
//     );

//     // Send transaction
//     let signature = client.send_transaction(&transaction).await;

//     // Return signature or error
//     match signature {
//         Ok(signature) => HttpResponse::Ok().body(signature.to_string()),
//         Err(err) => {
//             HttpResponse::InternalServerError().body(format!("Failed to send transaction: {}", err))
//         }
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file
    println!("Server running on http://localhost:8080");
    HttpServer::new(|| App::new().service(get_balance))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

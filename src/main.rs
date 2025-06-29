use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenv::dotenv;
use serde::Deserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::env;

const DEFAULT_RPC_URL: &str = "https://api.mainnet-beta.solana.com";

#[derive(Deserialize)]
struct BalanceQuery {
    address: String,
}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file
    println!("Server running on http://localhost:8080");
    HttpServer::new(|| App::new().service(get_balance))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::io;
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

fn main() {
    let rpc: RpcClient = RpcClient::new(RPC_URL);
    let mut input: String = String::new();

    println!("----- Enter Wallet address -----");

    io::stdin()
        .read_line(&mut input)
        .expect("You need to enter a wallet");

    let mut input = input.trim();
    let trimmed_input = Pubkey::from_str(&input).expect("Invalid input");

    println!("Select action: \n1: Check Solana balance\n2: Request Airdrop (Devnet)");

    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("No action received");

    match &option.trim().parse::<u64>() {
        Ok(num) => match num {
            1 => {
                let req = match Pubkey::from_str(&input) {
                    Ok(s) => fetch_balance(&s, &rpc),
                    Err(e) => panic!("You need to enter a wallet: {}", e),
                };

                match req {
                    Ok(balance) => println!("Your wallet balance is: {:?} Sol", balance),
                    Err(e) => println!("Failed to fetch wallet balance: {}", e),
                }
            }
            2 => {
                println!("Input an amount");
                let mut inp = String::new();
                io::stdin().read_line(&mut inp).expect("Invalid amount!");
                let amt = inp.trim().parse::<u64>().unwrap();

                request_airdrop(&trimmed_input, &rpc, amt);
            }
            _ => println!("Option does not exist yet"),
        },
        Err(_) => println!("Choose btw 1 - Unknown"),
    }
}

// Gets wallet balance on specified RPC: Devnet or Mainnet
fn fetch_balance(key: &Pubkey, client: &RpcClient) -> Result<u64, Box<dyn std::error::Error>> {
    println!("Checking wallet balance...");
    let bal = client.get_balance(key).expect("Failed to fetch balance");

    let balance = &bal / 1_000_000_000u64;

    Ok(balance)
}

// Request airdrop from Devnet
fn request_airdrop(key: &Pubkey, client: &RpcClient, amount: u64) {
    if amount <= 0u64 {
        panic!("You cannot request a negative or zero amount");
    }
    println!(".......Airdrop Request in progress.......");

    let amt = amount * 1_000_000_000u64;
    match client.request_airdrop(key, amt) {
        Ok(sig) => {
            println!("Request Completed, check your transaction: ");
            println!(
                "https://explorer.solana.com/tx/{}?cluster=devnet",
                sig.to_string()
            );
        }
        Err(e) => println!("Request Failed: {:?}", e),
    }
}

// More Functions coming soon
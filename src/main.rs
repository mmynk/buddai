use clap::Parser;
use std::fs;

pub mod error;
pub mod gemini;

/// Ask BuddAI a question
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Query to ask BuddAI
    #[arg(short, long)]
    query: String,
}

fn load_env() {
    let env_vars = fs::read_to_string(".env").expect("Failed to read .env file");
    for line in env_vars.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            continue;
        }
        let key = parts[0];
        let value = parts[1];
        std::env::set_var(key, value);
    }
}

#[tokio::main]
async fn main() {
    load_env();

    let args = Args::parse();
    let query = args.query;
    let response = gemini::ask_gemini(&query).await;
    match response {
        Ok(answer) => println!("Gemini says: {}", answer),
        Err(e) => println!("Gemini failed: {}", e.message),
    }
}

use clap::Parser;
use std::{fs, future};

pub mod curl;
pub mod deepseek;
pub mod error;
pub mod gemini;

/// Ask BuddAI a question
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Query to ask BuddAI
    #[arg(index = 1)]
    query: String,
    /// AI to ask
    #[arg(short, long, default_value = "gemini")]
    ai: String,
}

fn load_env() {
    let env_vars = fs::read_to_string(".env");
    if env_vars.is_err() {
        println!("No .env file found");
        return;
    }
    let envs = env_vars.unwrap();
    for line in envs.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            continue;
        }
        let key = parts[0];
        let value = parts[1];
        std::env::set_var(key, value);
    }
}

pub trait Ask {
    fn name() -> &'static str;
    fn ask(query: &str) -> impl future::Future<Output = Result<String, error::Error>>;
}

#[tokio::main]
async fn main() {
    load_env();

    let args = Args::parse();
    let query = args.query;
    let ai = args.ai.to_lowercase();

    match ai.as_str() {
        "deepseek" => answer::<deepseek::Deepseek>(&query).await,
        "gemini" => answer::<gemini::Gemini>(&query).await,
        _ => println!("Unknown AI: {}", ai),
    }
}

async fn answer<T: Ask>(query: &str) {
    let response = T::ask(query).await;
    match response {
        Ok(answer) => println!("{} says: {}", T::name(), answer),
        Err(e) => println!("{} failed: {}", T::name(), e.message),
    }
}

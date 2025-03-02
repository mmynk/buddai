use clap::Parser;

pub mod curl;
pub mod deepseek;
pub mod env;
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

pub trait Ask {
    fn name() -> &'static str;
    fn ask(query: &str) -> impl future::Future<Output = Result<String, error::Error>>;
    fn error_message(key: &str) -> String {
        return format!("{} not found in environment variables. Either export it or add it to ${{HOME}}/.config/buddai.env.", key);
    }
}

#[tokio::main]
async fn main() {
    env::load_env();

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

use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use log::{error, info};
use std::{future, str::FromStr};

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
        _ => error!("Unknown AI: {}", ai),
    }
}

async fn answer<T: Ask>(query: &str) {
    let response = T::ask(query).await;
    match response {
        Ok(answer) => info!("{} says: {}", T::name(), answer),
        Err(e) => error!("{} failed: {}", T::name(), e.message),
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .debug(Color::White)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    let log_level = std::env::var("LOG_LEVEL").unwrap_or("info".to_string());
    fern::Dispatch::new()
        .format(move|out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::from_str(&log_level).unwrap())
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

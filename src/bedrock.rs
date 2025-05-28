use crate::error::{BedrockError, Error};
use crate::Ask;
use aws_sdk_bedrockruntime::config::{BehaviorVersion, Region};
use aws_sdk_bedrockruntime::operation::converse::{ConverseError, ConverseOutput};
use aws_sdk_bedrockruntime::types::{ContentBlock, ConversationRole, Message};
use aws_sdk_bedrockruntime::Client;

const BEDROCK_MODEL: &str = "BEDROCK_MODEL";
const DEFAULT_MODEL: &str = "us.anthropic.claude-3-7-sonnet-20250219-v1:0";
const DEFAULT_REGION: &str = "us-west-2";

pub struct Bedrock;

impl Ask for Bedrock {
    fn name() -> &'static str {
        "Bedrock"
    }

    async fn ask(query: &str) -> Result<String, Error> {
        let region = std::env::var("AWS_REGION").unwrap_or_else(|_| DEFAULT_REGION.to_string());
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(region.to_owned()))
            .load()
            .await;
        let client = Client::new(&config);

        let model = std::env::var(BEDROCK_MODEL).unwrap_or_else(|_| DEFAULT_MODEL.to_string());

        let request = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(format!("{query}\nBe concise and to the point. If the question can be answered in a single sentence, do so. Only give more information if the question asks for it.")))
            .build()
            .map_err(|e| Error::new(format!("Failed to build msg: {}", e).as_str()))?;

        let response = client
            .converse()
            .model_id(model)
            .messages(request)
            .send()
            .await
            .map_err(|e| {
                if let Some(service_err) = e.as_service_error() {
                    match service_err {
                        ConverseError::ModelTimeoutException(_) => BedrockError::ModelTimeout,
                        ConverseError::ModelNotReadyException(_) => BedrockError::ModelNotReady,
                        _ => BedrockError::ServiceError(service_err.to_string()),
                    }
                } else {
                    BedrockError::ServiceError(e.to_string())
                }
            })?;

        get_converse_output_text(response).map_err(|e| e.into())
    }
}

fn get_converse_output_text(output: ConverseOutput) -> Result<String, BedrockError> {
    output
        .output()
        .ok_or_else(|| BedrockError::ParseError("no output".into()))?
        .as_message()
        .map_err(|_| BedrockError::ParseError("output not a message".into()))?
        .content()
        .first()
        .ok_or_else(|| BedrockError::ParseError("no content in message".into()))?
        .as_text()
        .map_err(|_| BedrockError::ParseError("content is not text".into()))
        .map(|s| s.to_string())
}

use crate::models::general::llm::{ChatCompletion, APIResponse, Message};
use reqwest::Client;
use std::env;
use dotenv::dotenv;
use std::error::Error;
use std::fmt;

// Custom error type
#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}

// Call Large Language Model (i.e. GPT-4)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn Error + Send>> {
    dotenv().ok();

    // Extract API Key information
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY must be set");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG must be set");

    // Confirm API endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert("authorization", reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key))
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?);
    headers.insert("OpenAI-Organization", reqwest::header::HeaderValue::from_str(api_org.as_str())
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?);

    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;

    // Structure input chat
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages: messages,
    };

    // Send API Request
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?
        .json()
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;

    // Handle the response
    if !res.choices.is_empty() {
        let response_text: String = res.choices[0].message.content.clone();
        return Ok(response_text);
    }

    Err(Box::new(CustomError("No choices found in the API response".into())))
}

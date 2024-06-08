use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use std::error::Error as StdError;
use crate::models::general::llm::ChatCompletion;

#[derive(serde::Deserialize)]
struct APIResponse {
    choices: Vec<Choice>,
}

#[derive(serde::Deserialize)]
struct Choice {
    text: String,
}

async fn call_openai_api(api_key: &str, api_org: &str, url: &str, chat_completion: &ChatCompletion) -> Result<String, Box<dyn StdError + Send>> {
    let mut headers = HeaderMap::new();
    
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).map_err(|e| Box::new(e) as Box<dyn StdError + Send>)?,
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org).map_err(|e| Box::new(e) as Box<dyn StdError + Send>)?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| Box::new(e) as Box<dyn StdError + Send>)?;
    
    let res_raw = client.post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError + Send>)?;
    
    println!("Raw response: {:?}", res_raw.text().await.map_err(|e| Box::new(e) as Box<dyn StdError + Send>)?);
    
    let res: APIResponse = res_raw.json().await.map_err(|e| Box::new(e) as Box<dyn StdError + Send>)?;
    
    Ok(res.choices.get(0).ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No choices found")) as Box<dyn StdError + Send>)?.text.clone())
}

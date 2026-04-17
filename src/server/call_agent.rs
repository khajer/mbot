
use reqwest;
use serde;

#[derive(serde::Serialize, serde::Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(serde::Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
}

#[derive(serde::Deserialize)]
struct Choice {
    message: Message,
}

#[derive(serde::Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const HEADER_AUTHORIZATION: &str = "Authorization";
const HEADER_CONTENT_TYPE: &str = "Content-Type";
const HEADER_CONTENT_TYPE_VALUE: &str = "application/json";

pub async fn call_openai(prompt: &str, token: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let request_body = RequestBody {
        model: model.to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
    };

    let response = client
        .post(OPENAI_API_URL)
        .header(HEADER_AUTHORIZATION, format!("Bearer {}", token))
        .header(HEADER_CONTENT_TYPE, HEADER_CONTENT_TYPE_VALUE)
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("OpenAI API error: {}", error_text).into());
    }

    let api_response: Response = response.json().await?;

    let content = api_response.choices.first()
        .map(|choice| choice.message.content.clone())
        .ok_or("No response from OpenAI")?;

    Ok(content)
}

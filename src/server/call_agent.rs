
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
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
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

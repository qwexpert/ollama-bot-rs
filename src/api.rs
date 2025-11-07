use reqwest::Client as HttpClient;
use serde_json::{json, Value};

pub async fn gen_res(prompt: &str) -> Result<String, reqwest::Error> {
    let client = HttpClient::new();
    let prompt_formatted = format!(
        "Пиши кратко без символов и уложись в 900 токенов. {}",
        prompt
            .replace("\\", "\\\\")
            .replace("{}", "{{}}")
            .replace("\"", "\\\"")
            .replace("\n", " ")
    );

    let data = json!({
        "model": "gpt-oss:120b-cloud",
        "messages": [
            {
                "role": "user",
                "content": prompt_formatted
            }
        ],
        "stream": false,
        "max_tokens": 900
    });

    let res = client
        .post("http://localhost:11434/v1/chat/completions")
        .json(&data)
        .send()
        .await?;

    let res = res.text().await?;

    if let Ok(parsed_json) = serde_json::from_str::<Value>(&res) {
        if let Some(content) = parsed_json["choices"][0]["message"]["content"].as_str() {
            return Ok(content.to_string());
        }
    }

    Ok("Сорри, я сломался :(((( Попробуйте повторить запрос.".to_string())
}

use reqwest::Client as HttpClient;
use serde_json::Value;

pub async fn gen_res(prompt: &str) -> Result<String, reqwest::Error> {
    let prompt_formatted = prompt
        .replace("\\", "\\\\")
        .replace("{}", "{{}}")
        .replace("\"", "\\\"")
        .replace("\n", " ");
    let data = format!(
        r#"
        {{
            "model": "{}",
            "prompt": "{}",
            "stream": false
        }}
    "#,
        "gpt-oss:120b-cloud", prompt_formatted
    );
    let client = HttpClient::new();
    let res = client
        .post("http://localhost:11434/api/generate")
        .body(data)
        .send()
        .await?;
    let text = res.text().await?;

    for line in text.lines() {
        if let Ok(parsed_json) = serde_json::from_str::<Value>(line) {
            if let Some(response) = parsed_json["model"].as_str() {
                println!("{:?}", response);
            }
            if let Some(response) = parsed_json["created_at"].as_str() {
                println!("{:?}", response);
            }
            if let Some(response) = parsed_json["response"].as_str() {
                println!(
                    "{:?}\n################################################\n\n",
                    response
                );
                return Ok(response.to_string());
            }
        } else {
            eprintln!("АШИПКА json");
        }
    }

    Ok("Сорри, я сломался :(((( Попробуйте повторить запрос.".to_string())
}

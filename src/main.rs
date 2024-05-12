use crate::sys::unix::waker::eventfd::WakerInternal;
use reqwest::Client as HttpClient;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!!");

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", "Bearer YOUR_API_KEY")
        .header("Content-Type", "application/json")
        .body(
            json!({
                "model": "text-davinci-003",
                "prompt": "Hello, world!",
                "max_tokens": 50
            })
            .to_string(),
        )
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        eprintln!("Failed to post message");
    }
}

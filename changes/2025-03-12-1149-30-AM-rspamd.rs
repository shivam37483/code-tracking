use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RspamdResponse {
    pub action: String,
    pub score: f32,
}

pub async fn check_spam(message: &str) -> Result<bool, reqwest::Error> {
    let client = Client::new();
    let res = client
        .post("http://localhost:11334/checkv2")
        .header("Password", "2099") // Replace with your password
        .body(message.to_string())
        .send()
        .await?
        .json::<RspamdResponse>()
        .await?;
    Ok(res.action == "reject" || res.score > 5.0) // Spam if rejected or score > 5
}
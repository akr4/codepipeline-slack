use serde::Serialize;

type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Serialize, Default)]
pub struct Payload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<Attachment>,
}

#[derive(Serialize)]
pub struct Attachment {
    pub color: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub text: Option<String>,
}

pub fn send_message(webhook_url: &str, payload: &Payload) -> Result<()> {
    let client = reqwest::Client::new();
    let body = serde_json::to_string(payload)?;
    let _response = client.post(webhook_url).body(body).send()?;
    Ok(())
}

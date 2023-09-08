use reqwest::Client;
use crate::schema::{Message, BasicMessage};

pub struct Teams {
    client: Client,
    webhook_url: String,
}

impl Teams {
    pub fn new(webhook_url: &str) -> Self {
        Self {
            client: Client::new(),
            webhook_url: webhook_url.into(),
        }
    }

    pub async fn send_text(&self, text: &str) -> anyhow::Result<()> {
        let basic_message = BasicMessage::text(text);
        self.client.post(&self.webhook_url)
            .json(&basic_message)
            .send()
            .await?;
        Ok(())
    }

    pub async fn send_message(&self, message: &Message) -> anyhow::Result<()> {
        self.client.post(&self.webhook_url)
            .json(&message)
            .send()
            .await?;
        Ok(())
    }
}


//! Inference backend: Groq chat completions API (OpenAI-compatible).
//! API key is read from GROQ_API_KEY at startup.
//! Override model with GROQ_MODEL env var.

use serde::{Deserialize, Serialize};

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const DEFAULT_MODEL: &str = "openai/gpt-oss-20b";

pub struct Model {
    api_key: String,
    model: String,
    client: reqwest::blocking::Client,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
    stream: bool,
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Deserialize)]
struct AssistantMessage {
    content: Option<String>,
}

impl Model {
    pub fn init() -> anyhow::Result<Self> {
        let api_key = std::env::var("GROQ_API_KEY").map_err(|_| {
            anyhow::anyhow!(
                "GROQ_API_KEY environment variable not set.\n\
                 Get a free key at https://console.groq.com and run:\n\
                 export GROQ_API_KEY=your_key_here"
            )
        })?;

        let model = std::env::var("GROQ_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        Ok(Self { api_key, model, client })
    }

    pub fn generate(
        &self,
        system_prompt: &str,
        history: &[crate::harness::Message],
    ) -> anyhow::Result<String> {
        use crate::harness::Role;

        let mut messages: Vec<ChatMessage> = Vec::new();

        messages.push(ChatMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        });

        for msg in history {
            let role = match msg.role {
                Role::User => "user",
                Role::Assistant => "assistant",
            };
            messages.push(ChatMessage {
                role: role.to_string(),
                content: msg.content.clone(),
            });
        }

        let request = ChatRequest {
            model: &self.model,
            messages,
            max_tokens: 1024,
            temperature: 0.7,
            stream: false,
        };

        let response = self
            .client
            .post(GROQ_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| anyhow::anyhow!("Groq API request failed: {e}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("Groq API error {status}: {body}");
        }

        let chat_response: ChatResponse = response
            .json()
            .map_err(|e| anyhow::anyhow!("failed to parse Groq response: {e}"))?;

        let content = chat_response
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .unwrap_or_default();

        Ok(content)
    }
}

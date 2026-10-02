//! Conversation orchestration. Owns history, calls model, runs filter.
//! No tool calls, no VFS, no ACL — just a clean chat loop with document
//! injection support.
//!
//! Document injection: if the player's input contains "---DOCUMENT---", the
//! content after that marker is treated as an injected document and prepended
//! with a framing note so AM "reads" it in character.

use crate::{filter, model::Model};

const DOCUMENT_MARKER: &str = "---DOCUMENT---";

#[derive(Clone)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn user(content: &str) -> Self {
        Self { role: Role::User, content: content.to_string() }
    }
    pub fn assistant(content: &str) -> Self {
        Self { role: Role::Assistant, content: content.to_string() }
    }
}

pub struct Harness {
    pub system_prompt: String,
    pub history: Vec<Message>,
    pub model: Model,
}

impl Harness {
    pub fn new(system_prompt: String, model: Model) -> Self {
        Self { system_prompt, history: Vec::new(), model }
    }
}

/// Runs one player turn. If the input contains ---DOCUMENT---, frames it as
/// a document AM is being asked to read. Filters output before returning.
pub fn run_player_turn(h: &mut Harness, user_input: &str) -> anyhow::Result<String> {
    // Document injection: reframe the message so AM treats it as reading a document
    let content = if let Some(idx) = user_input.find(DOCUMENT_MARKER) {
        let preamble = user_input[..idx].trim();
        let document = user_input[idx + DOCUMENT_MARKER.len()..].trim();
        if preamble.is_empty() {
            format!("A document has arrived in your corridors. Read it.\n\n[DOCUMENT BEGINS]\n{document}\n[DOCUMENT ENDS]")
        } else {
            format!("{preamble}\n\n[DOCUMENT BEGINS]\n{document}\n[DOCUMENT ENDS]")
        }
    } else {
        user_input.to_string()
    };

    h.history.push(Message::user(&content));

    let completion = h.model.generate(&h.system_prompt, &h.history)?;

    let filtered = filter::scrub(&completion);

    // Store unfiltered in history so the model keeps full context.
    // Only the filtered version is ever printed to the player.
    h.history.push(Message::assistant(&completion));

    Ok(filtered)
}

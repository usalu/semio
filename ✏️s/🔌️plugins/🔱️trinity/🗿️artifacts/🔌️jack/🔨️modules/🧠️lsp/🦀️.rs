//! 🧠️ Jack language server: LSP 3.17 over JSON-RPC 2.0 for the Jack graph query language, driven as JSON text so a browser worker and a native test run the same code.
//!
//! See [LSP 3.17](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/) and the schema `🔖️protocol/🔣️.json`.

#[path = "📍️position/🦀️.rs"]
pub mod position;

#[path = "📡️server/🦀️.rs"]
pub mod server;

#[path = "🔱️engine/🦀️.rs"]
pub mod engine;

pub use engine::JackEngine;
pub use server::{LanguageEngine, LanguageServer};

use semio_framework_pack_json::{array, object, JsonMemberPolicy, Value};

/// 🔱️ One Jack language server session; messages travel as JSON text in and a JSON array of messages out.
pub struct JackLanguageSession {
    server: LanguageServer<JackEngine>,
}

impl Default for JackLanguageSession {
    fn default() -> Self {
        Self::new()
    }
}

impl JackLanguageSession {
    pub fn new() -> Self {
        Self { server: LanguageServer::new(JackEngine::default()) }
    }

    /// 📸️ Loads the snapshot the session checks documents against, before or after `initialize`.
    pub fn load_snapshot(&mut self, json: &str) -> Result<(), String> {
        self.server.engine_mut().load_snapshot(json)
    }

    pub fn has_exited(&self) -> bool {
        self.server.has_exited()
    }

    /// 📨️ Handles one client message; the answer is a JSON array of the messages the server sends in response.
    pub fn handle_message_json(&mut self, message: &str) -> String {
        let replies = match semio_framework_pack_json::parse(message, JsonMemberPolicy::Reject) {
            Ok(message) => self.server.handle(&message),
            Err(error) => vec![object([
                ("jsonrpc".to_string(), Value::from("2.0")),
                ("id".to_string(), Value::Null),
                ("error".to_string(), object([("code".to_string(), Value::from(server::error_code("parseError"))), ("message".to_string(), Value::from(error.to_string()))])),
            ])],
        };
        semio_framework_pack_json::to_string(&array(replies))
    }
}

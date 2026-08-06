//! @emoji 📡️ `dsl_lsp` — LSP 3.17 JSON-RPC subset and in-process [`LanguageSession`] over
//! [`crate::os_dsl::LanguageSpec`] hooks (semantic tokens, completion, canonicalize).

use crate::os_dsl::{CompletionItem, LanguageSpec, TextError, TokenClass};
use serde_json::{json, Value};

//#region 🔖️Session
/// @emoji 🗣️ In-process language host for editor surfaces (writer, playground).
pub struct LanguageSession {
    spec: LanguageSpec,
    text: String,
}

impl LanguageSession {
    pub fn open(spec: LanguageSpec, text: impl Into<String>) -> Self {
        Self { spec, text: text.into() }
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn language_id(&self) -> &'static str {
        self.spec.id
    }

    pub fn semantic_tokens_lsp(&self) -> Value {
        let classified = (self.spec.hooks.classify)(&self.text);
        let mut data: Vec<u32> = Vec::new();
        let mut prev_line = 0u32;
        let mut prev_start = 0u32;
        for (class, span) in classified {
            let line = span.line.saturating_sub(1);
            let start = span.column.saturating_sub(1);
            let len = span.length.max(1);
            let type_index = match class {
                TokenClass::Keyword => 0,
                TokenClass::String => 1,
                TokenClass::Number => 2,
                TokenClass::Operator => 3,
                TokenClass::Ident => 4,
                _ => 5,
            };
            data.push(line.saturating_sub(prev_line));
            data.push(if line == prev_line { start.saturating_sub(prev_start) } else { start });
            data.push(len);
            data.push(type_index);
            data.push(0);
            prev_line = line;
            prev_start = start;
        }
        json!({ "data": data })
    }

    pub fn completions_at(&self, offset: usize) -> Vec<CompletionItem> {
        (self.spec.hooks.complete)(&self.text, offset)
    }

    pub fn canonicalize(&self) -> Result<String, TextError> {
        (self.spec.hooks.canonicalize)(&self.text)
    }
}
//#endregion 🔖️Session

//#region 🔖️JsonRpc
/// @emoji 📨 Handles one LSP JSON-RPC request string; returns optional response JSON text.
pub fn handle_json_rpc(line: &str, session: &LanguageSession) -> Option<String> {
    let msg: Value = serde_json::from_str(line).ok()?;
    let id = msg.get("id").cloned();
    let method = msg.get("method")?.as_str()?;
    let result = match method {
        "initialize" => json!({ "capabilities": { "semanticTokensProvider": { "full": true } } }),
        "semanticTokens/full" => session.semantic_tokens_lsp(),
        "shutdown" => json!(null),
        _ => json!({}),
    };
    id.map(|id| serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": result }).to_string())
}
//#endregion 🔖️JsonRpc

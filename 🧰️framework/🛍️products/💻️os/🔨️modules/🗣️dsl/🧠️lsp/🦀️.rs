//! 📡️ `dsl_lsp` — LSP 3.17 JSON-RPC subset and in-process [`LanguageSession`] over
//! [`crate::os_dsl::LanguageSpec`] hooks (semantic tokens, completion, canonicalize).

use semio_framework_dsl::CompletionItem;
use semio_framework_dsl::GrammarFile;
use semio_framework_dsl::LanguageSpec;
use semio_framework_dsl::ProtocolFile;
use semio_framework_diagnostic::TextError;
use semio_framework_dsl::TokenClass;
use semio_framework_pack_json::{object, Object, Value};

//#region 🔖️Session
/// 🗣️ In-process language host for editor surfaces (writer, playground).
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
        object([("data".to_string(), Value::Array(data.into_iter().map(Value::from).collect()))])
    }

    pub fn completions_at(&self, offset: usize) -> Vec<CompletionItem> {
        (self.spec.hooks.complete)(&self.text, offset)
    }

    pub fn canonicalize(&self) -> Result<String, TextError> {
        (self.spec.hooks.canonicalize)(&self.text)
    }

    /// 🩺 Text diagnostics from hooks + grammar dialect checks when `grammar` is present.
    pub fn diagnostics(&self) -> Vec<TextError> {
        let mut out = Vec::new();
        if self.spec.is_text_role() {
            if let Err(error) = self.canonicalize() {
                out.push(error);
            }
            if let Err(error) = self.spec.parsed_grammar() {
                out.push(error);
            }
        }
        out
    }

    /// 📡️ Byte-level protocol verification when `protocol` text is present on the spec.
    pub fn verify_protocol_bytes(&self, bytes: &[u8]) -> Result<(), String> {
        self.spec.verify_protocol(bytes)
    }

    /// 📖️ Parsed grammar file for text roles (`None` when unset).
    pub fn grammar_file(&self) -> Result<Option<GrammarFile>, TextError> {
        self.spec.parsed_grammar()
    }

    /// 📡️ Parsed protocol file for binary verification (`None` when unset).
    pub fn protocol_file(&self) -> Result<Option<ProtocolFile>, TextError> {
        self.spec.parsed_protocol()
    }
}
//#endregion 🔖️Session

//#region 🔖️JsonRpc
/// 📨 Handles one LSP JSON-RPC request string; returns optional response JSON text.
pub fn handle_json_rpc(line: &str, session: &LanguageSession) -> Option<String> {
    let msg: Value = semio_framework_pack_json::parse(line, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
    let id = msg.get("id").cloned();
    let method = msg.get("method")?.as_str()?;
    let result = match method {
        "initialize" => object([("capabilities".to_string(), object([("semanticTokensProvider".to_string(), object([("full".to_string(), Value::from(true))]))]))]),
        "semanticTokens/full" => session.semantic_tokens_lsp(),
        "shutdown" => Value::Null,
        _ => Value::Object(Object::new()),
    };
    id.map(|id| {
        let mut response = Object::new();
        response.insert("jsonrpc", Value::from("2.0"));
        response.insert("id", id);
        response.insert("result", result);
        semio_framework_pack_json::to_string(&Value::Object(response))
    })
}
//#endregion 🔖️JsonRpc

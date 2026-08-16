//! ⚠️ Text errors, structured diagnostics, fault reporting, and parse limits.

use serde::{Deserialize, Serialize};
use thiserror::Error;
pub use crate::os_dsl::span::TextSpan;

//#region 🔖️Errors
/// @emoji 🚧️ Span-carrying parse/print failure — the one error type every DSL surface returns.
#[derive(Clone, Debug, PartialEq, Error, Serialize, Deserialize)]
#[error("{message} at {}:{}", span.line, span.column)]
pub struct TextError {
    pub message: String,
    pub span: TextSpan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,
}

impl TextError {
    pub fn new(message: impl Into<String>, span: TextSpan) -> Self {
        Self { message: message.into(), span, expected: None }
    }

    pub fn expected(message: impl Into<String>, span: TextSpan, expected: impl Into<String>) -> Self {
        Self { message: message.into(), span, expected: Some(expected.into()) }
    }

    pub fn from_diagnostic(diagnostic: Diagnostic) -> Self {
        diagnostic.into_text_error()
    }
}

/// @emoji 🏷️ Stable dotted fault/diagnostic code (e.g. `module.pack.checksum-mismatch`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FaultCode(pub String);

impl FaultCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
}

impl From<&'static str> for FaultCode {
    fn from(value: &'static str) -> Self {
        Self(value.to_string())
    }
}

/// @emoji 🏷️ Stable, greppable diagnostic identifier, e.g. `"DSL0001"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticCode(pub &'static str);

impl From<String> for FaultCode {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<DiagnosticCode> for FaultCode {
    fn from(value: DiagnosticCode) -> Self {
        FaultCode::new(value.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Severity {
    Fatal,
    Error,
    Warning,
    Hint,
}

/// @emoji 🧭️ What the parser would have accepted at the failure point — the raw material for
/// completions and for `TextError.expected`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ExpectedSet {
    pub tokens: Vec<String>,
    pub keywords: Vec<String>,
    pub keys: Vec<String>,
}

impl ExpectedSet {
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();
        if !self.keywords.is_empty() {
            parts.push(self.keywords.join("|"));
        }
        if !self.keys.is_empty() {
            parts.push(self.keys.iter().map(|k| format!("{k}=")).collect::<Vec<_>>().join("|"));
        }
        if !self.tokens.is_empty() {
            parts.push(self.tokens.join("|"));
        }
        parts.join(" or ")
    }
}

/// @emoji 🩺️ A structured diagnostic anchored to a span, with an optional `ExpectedSet` for
/// completions/fixes. Lowers into `TextError` at API boundaries that predate diagnostics.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub code: FaultCode,
    pub severity: Severity,
    pub span: TextSpan,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<ExpectedSet>,
    #[serde(default)]
    pub scope: FaultScope,
}

impl Diagnostic {
    pub fn error(code: &'static str, span: TextSpan, message: impl Into<String>) -> Self {
        Self { code: FaultCode::new(code), severity: Severity::Error, span, message: message.into(), expected: None, scope: FaultScope::default() }
    }

    pub fn with_expected(mut self, expected: ExpectedSet) -> Self {
        self.expected = Some(expected);
        self
    }

    pub fn into_text_error(self) -> TextError {
        let expected = self.expected.as_ref().map(ExpectedSet::describe);
        match expected {
            Some(expected) => TextError::expected(self.message, self.span, expected),
            None => TextError::new(self.message, self.span),
        }
    }
}

//#region 🔖️Fault
/// @emoji 🧭️ Which layer of the os stack produced a {@link Fault}.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FaultOrigin {
    Edge,
    Renderer,
    Os,
    Module,
    Plugin,
    App,
    Extension,
    /// 🚪️👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.3: origin for the
    /// five frozen `surface.*`/`viewer.*` fault codes (`AppRouter`/`OpeningResolver`/`VcsArtifactApp`
    /// role guard) — additive variant, no existing variant touched, no match site in this crate is
    /// exhaustive over it (verified with a repo-wide grep before adding).
    Framework,
}

/// @emoji 🎯️ Optional ids locating a fault/diagnostic to a plugin app surface.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaultScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_key: Option<String>,
}

/// @emoji 🔗️ One hop in a {@link Fault} cause chain.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaultCause {
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<FaultCode>,
}

/// @emoji 🧯️ Structured abort report crossing every os boundary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fault {
    pub origin: FaultOrigin,
    pub code: FaultCode,
    pub severity: Severity,
    pub message: String,
    #[serde(default)]
    pub scope: FaultScope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<TextSpan>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub causes: Vec<FaultCause>,
    #[serde(default)]
    pub retryable: bool,
}

impl From<&str> for Fault {
    fn from(value: &str) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl From<String> for Fault {
    fn from(value: String) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl Fault {
    pub fn new(origin: FaultOrigin, code: impl Into<FaultCode>, message: impl Into<String>) -> Self {
        Self {
            origin,
            code: code.into(),
            severity: Severity::Error,
            message: message.into(),
            scope: FaultScope::default(),
            span: None,
            causes: Vec::new(),
            retryable: false,
        }
    }

    pub fn with_scope(mut self, scope: FaultScope) -> Self {
        self.scope = scope;
        self
    }

    pub fn with_retryable(mut self, retryable: bool) -> Self {
        self.retryable = retryable;
        self
    }
}

/// @emoji 🔁️ Maps a domain error enum into a {@link Fault} at a boundary.
pub trait FaultFrom {
    fn fault_origin(&self) -> FaultOrigin;
    fn fault_code(&self) -> FaultCode;
    fn fault_severity(&self) -> Severity;
    fn fault_message(&self) -> String;
    fn fault_scope(&self) -> FaultScope {
        FaultScope::default()
    }
    fn fault_span(&self) -> Option<TextSpan> {
        None
    }
    fn fault_causes(&self) -> Vec<FaultCause> {
        Vec::new()
    }
    fn fault_retryable(&self) -> bool {
        false
    }

    fn into_fault(self) -> Fault
    where
        Self: Sized,
    {
        Fault {
            origin: self.fault_origin(),
            code: self.fault_code(),
            severity: self.fault_severity(),
            message: self.fault_message(),
            scope: self.fault_scope(),
            span: self.fault_span(),
            causes: self.fault_causes(),
            retryable: self.fault_retryable(),
        }
    }
}

impl FaultFrom for TextError {
    fn fault_origin(&self) -> FaultOrigin {
        FaultOrigin::Module
    }

    fn fault_code(&self) -> FaultCode {
        FaultCode::new("module.dsl.text")
    }

    fn fault_severity(&self) -> Severity {
        Severity::Error
    }

    fn fault_message(&self) -> String {
        self.message.clone()
    }

    fn fault_span(&self) -> Option<TextSpan> {
        Some(self.span)
    }
}

/// @emoji 📦️ JSON wire encoding for {@link Fault} crossing host/WIT boundaries.
pub fn encode_fault_bytes(fault: &Fault) -> Vec<u8> {
    serde_json::to_vec(fault).unwrap_or_else(|_| fault.message.as_bytes().to_vec())
}

/// @emoji 🌐️ Decodes a {@link Fault} from JSON wire bytes; falls back to an os-level message fault.
pub fn decode_fault_bytes(bytes: &[u8]) -> Fault {
    serde_json::from_slice(bytes).unwrap_or_else(|_| Fault::new(FaultOrigin::Os, "os.fault.decode", String::from_utf8_lossy(bytes)))
}

/// @emoji 🔁️ Maps a `thiserror` enum into {@link Fault} with a stable dotted code namespace.
#[macro_export]
macro_rules! fault_from_thiserror {
    ($ty:ty, $origin:expr, $prefix:literal) => {
        impl $crate::FaultFrom for $ty {
            fn fault_origin(&self) -> $crate::FaultOrigin {
                $origin
            }

            fn fault_code(&self) -> $crate::FaultCode {
                $crate::FaultCode::new($prefix)
            }

            fn fault_severity(&self) -> $crate::Severity {
                $crate::Severity::Error
            }

            fn fault_message(&self) -> String {
                self.to_string()
            }
        }
    };
}

#[cfg(all(feature = "js", target_arch = "wasm32"))]
/// @emoji 🌐️ Surfaces a structured {@link Fault} to JavaScript callers.
pub fn fault_to_js(fault: Fault) -> wasm_bindgen::JsValue {
    wasm_bindgen::JsValue::from_str(&serde_json::to_string(&fault).unwrap_or(fault.message))
}

#[cfg(all(feature = "js", target_arch = "wasm32"))]
/// @emoji 🌐️ Maps `Result<T, Fault>` into `Result<T, JsValue>` for wasm exports.
pub fn result_fault_to_js<T>(result: Result<T, Fault>) -> Result<T, wasm_bindgen::JsValue> {
    result.map_err(fault_to_js)
}
//#endregion 🔖️Fault
//#endregion 🔖️Errors

//#region 🔖️Limits
/// @emoji 🛡️ Resource budgets threaded through every parse — exceeding one yields a budget
/// diagnostic (`DSL0100`), never a panic or unbounded recursion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_bytes: usize,
    pub max_tokens: usize,
    pub max_depth: usize,
    pub max_nodes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self { max_bytes: 16 * 1024 * 1024, max_tokens: 1_000_000, max_depth: 64, max_nodes: 1_000_000 }
    }
}

pub const BUDGET_EXCEEDED_CODE: &str = "DSL0100";

impl Limits {
    pub fn check_bytes(&self, len: usize) -> Result<(), TextError> {
        if len > self.max_bytes {
            return Err(TextError::new(format!("input exceeds max_bytes limit ({} > {})", len, self.max_bytes), TextSpan::at(1, 1)));
        }
        Ok(())
    }

    pub fn check_depth(&self, depth: usize, span: TextSpan) -> Result<(), TextError> {
        if depth > self.max_depth {
            return Err(TextError::new(format!("nesting exceeds max_depth limit ({} > {})", depth, self.max_depth), span));
        }
        Ok(())
    }

    pub fn check_tokens(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_tokens {
            return Err(TextError::new(format!("token count exceeds max_tokens limit ({} > {})", count, self.max_tokens), span));
        }
        Ok(())
    }

    pub fn check_nodes(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_nodes {
            return Err(TextError::new(format!("node count exceeds max_nodes limit ({} > {})", count, self.max_nodes), span));
        }
        Ok(())
    }
}
//#endregion 🔖️Limits

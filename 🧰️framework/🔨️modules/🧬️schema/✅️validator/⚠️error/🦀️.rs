//! ⚠️ Structural validation errors and diagnostics independent of schema catalogs.

#[derive(Debug, PartialEq, Eq)]
pub enum SchemaError {
    Validation(String),
    Cancelled,
    LimitExceeded(usize),
}

impl std::fmt::Display for SchemaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(message) => write!(formatter, "validation failed: {message}"),
            Self::Cancelled => formatter.write_str("validation cancelled"),
            Self::LimitExceeded(limit) => write!(formatter, "validation node limit exceeded: {limit}"),
        }
    }
}

impl std::error::Error for SchemaError {}

/// 🩺 One structural validation failure as the owned draft-07 validator renders it: the instance path
/// the failure is attributed to (`$`, `$.name`, `$.items[0]`) and the reason, joined by `": "` inside
/// [`SchemaError::Validation`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationDiagnostic {
    pub instance_path: String,
    pub reason: String,
}

impl ValidationDiagnostic {
    /// 🧾 Renders the diagnostic exactly as [`SchemaError::Validation`] carries it.
    pub fn message(&self) -> String {
        format!("{}: {}", self.instance_path, self.reason)
    }

    /// 🧭 The instance path as an RFC 6901 JSON pointer, the spelling third-party validators report.
    pub fn json_pointer(&self) -> String {
        self.instance_path.trim_start_matches('$').replace(['.', '['], "/").replace(']', "")
    }

    /// 🔎 Reads the diagnostic back out of a validation failure; any other error kind carries no path.
    pub fn from_error(error: &SchemaError) -> Option<Self> {
        let SchemaError::Validation(message) = error else { return None };
        let (instance_path, reason) = message.split_once(": ")?;
        if !instance_path.starts_with('$') {
            return None;
        }
        Some(Self { instance_path: instance_path.to_string(), reason: reason.to_string() })
    }
}

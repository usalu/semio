//! 🧯️ Owns Editor failure categories and keeps backend diagnostics private.
use std::error::Error;
use std::fmt;

/// 🧭️ The Editor boundary that produced a failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorErrorKind {
    Json,
    Pack,
    Scene,
}

/// 🧯️ An owned Editor failure with an inspectable standard diagnostic chain.
#[derive(Debug)]
pub struct EditorError {
    kind: EditorErrorKind,
    cause: Box<dyn Error + Send + Sync>,
}

impl EditorError {
    /// 🪪️ Returns the owned failure category.
    pub const fn kind(&self) -> EditorErrorKind {
        self.kind
    }

    pub(super) fn from_cause(kind: EditorErrorKind, cause: impl Error + Send + Sync + 'static) -> Self {
        Self { kind, cause: Box::new(cause) }
    }
}

impl fmt::Display for EditorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.kind { EditorErrorKind::Json => "json", EditorErrorKind::Pack => "pack", EditorErrorKind::Scene => "scene" };
        write!(formatter, "{prefix}: {}", self.cause)
    }
}

impl Error for EditorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.cause.as_ref())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

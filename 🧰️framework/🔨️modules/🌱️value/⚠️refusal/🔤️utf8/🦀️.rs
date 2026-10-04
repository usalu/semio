use super::{ValueError, ValueRefusalKind};

/// 🔡️ Converts an explicit standard decoder rejection into its owned value refusal.
impl From<std::str::Utf8Error> for ValueError {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::new(ValueRefusalKind::InvalidValue, error.to_string())
    }
}

/// 🧵️ Consumes rejected owned bytes while preserving the standard decoder's refusal display.
impl From<std::string::FromUtf8Error> for ValueError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self::from(error.utf8_error())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

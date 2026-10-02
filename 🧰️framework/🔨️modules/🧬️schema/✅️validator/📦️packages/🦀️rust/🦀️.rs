//! ✅️ Neutral structural schema validation over owned JSON and dynamic values.

#[path = "../../⚠️error/🦀️.rs"]
mod error;
pub use error::{SchemaError, ValidationDiagnostic};
#[path = "../../🦀️.rs"]
mod validator;
pub use validator::*;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

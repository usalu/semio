//! 🛡️ The directory's declared access policy and its one decision authority, one crate because the policy type owns both the schema and the evaluating methods.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

#[path = "🚪️io/📝️text/🦀️.rs"]
mod text;
pub use text::directory_access_permits;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//! 🧬️ Direct CC5 mutation IO over its declared externally tagged payloads.

#[path = "📝️text/🦀️.rs"]
pub mod text;
#[path = "💾️binary/🦀️.rs"]
pub mod binary;

pub use crate::standards::v_ap214::subsets::cc5::schema::mutations::StepCc5Mutation;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

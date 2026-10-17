//! 🔤️ Token-sampling vocabulary: ids, errors, limits, JSON and UTF-8 text, numerics, token bitsets, seeded random streams and the token vocabulary.

#![allow(async_fn_in_trait)]

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};

#[path = "🆔️ids/🦀️.rs"]
mod ids;
pub use ids::*;

#[path = "⚠️errors-fallbacks-and-limits/🦀️.rs"]
mod errors_fallbacks_and_limits;
pub use errors_fallbacks_and_limits::*;

#[path = "🧾️json-value-and-parser/🦀️.rs"]
mod json_value_and_parser;
pub use json_value_and_parser::*;

#[path = "📤️json-writer-and-utf8/🦀️.rs"]
mod json_writer_and_utf8;
pub use json_writer_and_utf8::*;

#[path = "🧮️numerics/🦀️.rs"]
mod numerics;
pub use numerics::*;

#[path = "🎯️selection-and-cumulative-distribution/🦀️.rs"]
mod selection_and_cumulative_distribution;
pub use selection_and_cumulative_distribution::*;

#[path = "🧱️token-bitset/🦀️.rs"]
mod token_bitset;
pub use token_bitset::*;

#[path = "🎲️random-streams/🦀️.rs"]
mod random_streams;
pub use random_streams::*;

#[path = "📖️vocabulary-and-text-adapters/🦀️.rs"]
mod vocabulary_and_text_adapters;
pub use vocabulary_and_text_adapters::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

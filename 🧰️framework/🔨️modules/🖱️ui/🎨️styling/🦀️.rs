//! @emoji 🎨️ Framework-neutral styling tokens and authored color behavior.

#[allow(clippy::excessive_precision, reason = "🎨️ float literals mirror ui/styling/🔣️.json verbatim; truncating them by hand would drift from the source data on the next regeneration")]
#[path = "🔤️tokens/🦀️.rs"]
mod generated;
#[path = "🌗️mixing/🦀️.rs"]
pub mod color;
#[path = "🌓️theme/🦀️.rs"]
pub mod appearance;

pub use generated::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

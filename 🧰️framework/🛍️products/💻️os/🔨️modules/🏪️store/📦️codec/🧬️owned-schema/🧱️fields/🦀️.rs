//! 🧱️ Envelope field decoders: field catalog, SPR edit and array authorities, history entry decoders.
//!
//! 🧭️ Skeleton of `semio-framework-os-kernel` (WP 3): every target; its sources move in with the work package.

#[path = "📥️lifted/🧬️owned-schema/🦀️.rs"]
pub mod lifted_owned_schema_decode;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;

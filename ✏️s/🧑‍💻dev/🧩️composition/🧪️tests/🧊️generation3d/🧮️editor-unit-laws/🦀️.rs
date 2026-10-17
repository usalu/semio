//! 🧮️ Editor unit laws: the mesh-component, extension round-trip, engine-compute and hot-path law groups.
#![allow(dead_code, unused_imports)]

include!("../🦀️.rs");

#[path = ".."]
mod editor_domain {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🧮️editor-unit-laws/📚️editor-laws/🦀️.rs"]
pub(crate) mod editor_laws;
}


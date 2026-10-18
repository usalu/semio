//! 🔬️ Unit tests of the Store that exercise the demo document through the public API of the Store crates.
//!
//! 🧭️ Moved from the former store unit test module (chunk 4 of the cut by subject).

use super::*;

include!("🛤️history-lane-interaction-store-and-immutable-root-laws/🦀️.rs");
include!("🏁️checkpoint-alternative-history-column-and-backbone-laws/🦀️.rs");
include!("📮️document-codec-replay-and-projection-gate-laws/🦀️.rs");
include!("⬅️backbone-attach-undo-and-text-fidelity-laws/🦀️.rs");
include!("🌌️stateful-current-and-space-history-laws/🦀️.rs");
include!("🧾️document-text-metadata-conflict-and-preview-wire-laws/🦀️.rs");
include!("🔁️spr-round-trip-message-vocabulary-and-text-format-helper-laws/🦀️.rs");

//! 🔬️ Unit tests of the Store that exercise the demo document through the public API of the Store crates.
//!
//! 🧭️ Moved from the former store unit test module (chunk 3 of the cut by subject).

use super::*;

include!("🧳️retained-clone-preparation-fixtures/🦀️.rs");
include!("♻️retained-clone-preparation-lifecycle-laws/🦀️.rs");
include!("📚️batch-publication-and-stamped-clock-laws/🦀️.rs");
include!("💨️ephemeral-one-item-publication-laws/🦀️.rs");
include!("📸️group-envelope-read-canonical-revision-and-remote-ingest-laws/🦀️.rs");
include!("⚔️modify-vs-delete-quarantine-and-testkit-wiring-laws/🦀️.rs");
include!("🛡️conflict-robustness-and-composition-pin-laws/🦀️.rs");

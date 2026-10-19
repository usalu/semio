//! 🔬️ Unit tests of the Store that exercise the demo document through the public API of the Store crates.
//!
//! 🧭️ Moved from the former store unit test module (chunk 1 of the cut by subject).

use super::*;
use semio_framework_test_fixtures::fixture_path;

include!("🏷️catalog-staging-command-refusal-and-direct-fixture-laws/🦀️.rs");
include!("🥇️one-item-publication-fixture-and-snapshot-read-lease-laws/🦀️.rs");
include!("📜️edit-retirement-message-ledger-and-displaced-owner-laws/🦀️.rs");
include!("🎫️completed-record-registry-and-envelope-decode-laws/🦀️.rs");
include!("🔭️envelope-close-field-registry-and-snapshot-read-laws/🦀️.rs");

/// ➕️ L3 for one fixture leaf: the concrete inverse's diffs sum to the negative of the forward diff on every listed demo document (`None` is the deleted value).
pub(crate) async fn assert_fixture_sum_law<Op: semio_framework_os_spr::Mutation<DemoSnapshot> + semio_framework_os_spr::ColdMutation<DemoSnapshot,ReferenceDiff=DemoDiff>>(mutation: &Op, bases: &[Option<i32>]) {
    for n in bases {
        semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(mutation, &DemoSnapshot { n: *n }).await;
    }
}


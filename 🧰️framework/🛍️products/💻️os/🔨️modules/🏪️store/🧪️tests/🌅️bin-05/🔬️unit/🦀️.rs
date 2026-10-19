//! 🔬️ Unit tests of the Store that exercise the demo document through the public API of the Store crates.
//!
//! 🧭️ Moved from the former store unit test module (chunk 5 of the cut by subject).

use protocol::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
use super::*;

#[path = "../../../🏗️initialization/🏠️owners/🧪️tests/🦀️.rs"]
mod shared_semantic_source_tests;

/// 🪪️ Runs one authoring call under a fixture caller-owned identity authority and returns the call's own result after the authority's cumulative receipt is settled.
macro_rules! with_fixture_identity {
    (|$identity:ident| $call:expr) => {{
        const FIXTURE_IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
        let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            assert!(progress.owned_bytes <= FIXTURE_IDENTITY_CEILING);
            true
        };
        let mut $identity = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(FIXTURE_IDENTITY_CEILING, &mut observer).expect("declared fixture identity authority");
        let result = $call;
        drop($identity.pause().expect("original cumulative fixture identity receipt"));
        result
    }};
}

include!("📄️document-text-wire-and-command-error-laws/🦀️.rs");
include!("🔀️reconcile-alternative-event-log-and-backbone-port-laws/🦀️.rs");
include!("🗜️pack-value-fixture-and-typed-child-member-laws/🦀️.rs");
include!("🕸️composition-graph-child-id-and-group-dispatch-laws/🦀️.rs");

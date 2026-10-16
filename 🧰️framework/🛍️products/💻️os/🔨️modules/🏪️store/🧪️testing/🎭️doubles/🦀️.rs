//! 🎭️ Test doubles of the Store that need the Store engine: the test store wrapper, the retirement and preparation doubles, the original-caller doubles and the envelope builders.
//!
//! 🧭️ This crate sits above the fixtures crate and below the companion test crates; it depends only on the Store crates and the fixtures crate.
#![allow(unused_imports, dead_code, ambiguous_glob_reexports, async_fn_in_trait, clippy::all)]

#[path = "♻️retirement/🦀️.rs"]
mod retirement;
pub use retirement::*;
#[path = "🗄️store/🦀️.rs"]
pub mod store;
pub use store::{DemoMemberStoreProducer, DemoMemberWirePreparation, DemoMemberWirePreparationFactory, FixtureAuthoringFactory, close_demo_artifact_store, close_durable_publication, close_erased_member_publication, close_test_store, complete_test_catalog, demo_base_store_owners, demo_closable_store_owners, drain_channel_for_test, drive_test_report_replay, fixture_authoring_birth_demand, fixture_authoring_catalog, fixture_authoring_member_owners, fixture_wire_source, retained_demo_member_owners};
#[path = "🏭️preparation/🦀️.rs"]
mod preparation;
pub use preparation::*;
#[path = "🧾️callers/🦀️.rs"]
mod callers;
pub use callers::*;
#[path = "📨️envelopes/🦀️.rs"]
mod envelopes;
pub use envelopes::*;

//! 🍎️ Companion test crate `os-store-tests-09`: the root-mounted store unit tests packed by subject.
//!
//! 🧭️ The tests use the public API of the Store crates only; the shared demo document, the mutation fixtures and the store prelude come from the fixtures crate.
#![cfg(test)]
#![allow(unused_imports, dead_code, ambiguous_glob_reexports, async_fn_in_trait, clippy::all)]

#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

use semio_framework_os_store_testing_prelude::*;
use semio_framework_os_store_testing_fixtures::*;
use semio_framework_os_store_testing_doubles::*;
use semio_framework_os_store_testing_doubles::store::ArtifactStore;
use semio_framework_os_store_testing_fixtures::demo::{AddN, AssignN, DeleteN, DemoMutation, SetN};
use semio_framework_os_store_testing_fixtures::lossy::{LossyMutation, SetN as LossySetN};
use semio_framework_os_store_testing_fixtures::severity::{SetErrorN, SetFatalN, SetN as SeveritySetN, SetWarningN, SeverityMutation};
use semio_framework_os_store_testing_fixtures::timestamped::{SetN as TimestampedSetN, TimestampedMutation};
use semio_framework_os_store_testing_fixtures::validated::{AssignN as ValidatedAssignN, SetN as ValidatedSetN, ValidatedMutation};
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_os_store_snapshot_clone::RetainedCloneEdit;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditCursor;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditStep;
use semio_framework_os_store_snapshot_clone::RetainedClonePreparationFactory;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef};
use semio_framework_pack_json_retained_schema::testing::{OWNED_SCHEMA_TEST_FIELDS, owned_schema_test_cursor, drive_owned_schema, schema_test_close_grant};
use semio_framework_value::testing::physical_test_close_grant;

#[path = "../../🧵️canonical-edit/📖️reader/🧪️tests/📖️reader/🦀️.rs"]
mod reader_tests;
#[path = "../../🧵️canonical-edit/🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs"]
mod canonical_edit_borrowed_tests;
#[path = "../../🧵️canonical-edit/🧪️tests/🔬️unit/🦀️.rs"]
mod canonical_edit_tests;
#[path = "../♻️bounded-value-retirement/🦀️.rs"]
mod bounded_value_retirement_tests;
#[path = "../🧩️composed-pack-schema/🦀️.rs"]
mod composed_pack_schema_tests;

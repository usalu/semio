#![cfg(test)]
#![allow(unused_imports, unused_qualifications)]

use db_sync::*;
use db_sync::testkit::*;
use db_durability::Frontier;
use db_durability::{DurabilityClass, ResumeToken};
use db_ids::{ActorId, ArtifactId, DbError};
use db_io_core::SnapshotStorage;

use semio_framework_test_fixtures::fixture_text;

pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

include!("../🔬️unit/🔁️wal-command-decoder-and-replay-laws/🦀️.rs");
include!("../🔬️unit/🧭️codec-state-frontier-and-bootstrap-laws/🦀️.rs");
include!("../🔬️unit/🪢️retained-sync-hello-laws/🦀️.rs");

//! 🥪️ Companion tests of os-dsl: grammar, family kits and record laws that exercise the grammars through the kernel facade.
//!
//! 🧭️ The test files keep their `use super::*` shape: this root carries the imports the dsl component root used to carry, and re-exports the crate under test.
#![cfg(test)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;

pub(crate) use semio_framework_trace_memory_testing as test_allocation;

#[global_allocator]
static TEST_ALLOCATION_OBSERVER: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

use semio_framework_diagnostic::TextSpan;
use semio_framework_dsl::*;
use semio_framework_dsl_record::*;
use semio_framework_dsl_record_derive::{DslEnum, DslRecord, DslScalar};
use semio_framework_ui_viewport::{Viewport2d, Viewport3dOrbit};
use semio_framework_value::{DslValue, FromValue, Number, NativeDecodeControl, NativeEncodeControl, ToValue, ValueError, ValueRefusalKind};

pub use semio_framework_os_dsl::*;

//#region 🧪️Tests
#[path = "../../🔢️ieee754/🧪️tests/🦀️.rs"]
mod ieee_payload_tests;

#[path = "../📦️boxed-fields/🦀️.rs"]
mod boxed_field_tests;

#[path = "../🔢️checked-integers/🦀️.rs"]
mod checked_integer_tests;

#[path = "../🏷️protocol-record/🦀️.rs"]
mod protocol_record_tests;

#[path = "../🧪️hygienic-bindings/🦀️.rs"]
mod hygienic_binding_tests;

#[path = "../🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "../../🧬️schema/🧪️tests/🧬️intrinsic-bytes/🦀️.rs"]
mod canonical_record_intrinsic_consumers;

#[path = "../../🧬️schema/🧪️tests/🧾️record-list/🦀️.rs"]
mod canonical_record_list_consumers;

#[path = "../../🧬️schema/🏭️producer/🧪️tests/🦀️.rs"]
mod canonical_record_producer_consumers;

#[path = "../../🪟️viewport/🧪️tests/🪟️poses/🦀️.rs"]
mod canonical_viewport_pack_consumers;

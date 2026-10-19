//! 🌅️ Companion tests of io-sqlite-snapshot: snapshot unit, limits, refusal, retirement, receiving and artifact reader tests.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

pub use semio_framework_io_sqlite_snapshot::*;
use semio_framework_io_control as original_control;

#[path = "."]
pub mod component {
    pub use semio_framework_io_sqlite_snapshot::testkit::component::*;

    #[path = "../🔬️unit/🦀️.rs"]
    mod tests;

    #[path = "../../⚠️refusal/🧪️tests/🦀️.rs"]
    mod typed_refusal_tests;

    #[path = "../../♻️retirement/🧪️tests/🦀️.rs"]
    mod original_field_retirement_tests;

    #[path = "../../⚠️refusal/💰️limits/🧪️tests/🦀️.rs"]
    mod original_fixed_limit_tests;

    #[path = "../../🔁️transfer/🫴️receiving/🧪️tests/🦀️.rs"]
    mod original_physical_receiving_tests;

    #[path = "."]
    pub mod artifact {
        pub use semio_framework_io_sqlite_snapshot::testkit::component::artifact::*;
        use semio_framework_io_sqlite_snapshot::testkit::component::{SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase, ValueError, ValueRefusalKind};
        pub use semio_framework_io_sqlite_snapshot::testkit::component::artifact::testkit::row_writer;

        #[path = "../../🧩️artifact/🧪️tests/🫳️reconstruction/🦀️.rs"]
        mod reconstruction_copy_tests;

        #[path = "../../🧩️artifact/🧪️tests/🧮️allocation/🦀️.rs"]
        mod shared_provider_allocation_tests;

        #[path = "../../🧩️artifact/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;

        #[path = "../../🔢️ieee754/🧪️tests/🔬️unit/🦀️.rs"]
        mod ieee_tests;

        #[path = "../../🧩️artifact/🫴️receiving/🧪️tests"]
        pub mod receiving {
            pub use semio_framework_io_sqlite_snapshot::testkit::component::artifact::receiving::*;
            use semio_framework_io_sqlite_snapshot::testkit::component::{SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotControl};
            use semio_framework_value::{ValueError, ValueRefusalKind, NativeEncodeControl, RetainedCloneGrant, RetainedCloneProgress};
            use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

            #[path = "🦀️.rs"]
            mod tests;
        }
    }

    #[path = "../../🧵️composition/🧪️tests"]
    pub mod composition {
        pub use semio_framework_io_sqlite_snapshot::testkit::component::composition::*;

        #[path = "🦀️.rs"]
        mod tests;
    }
}

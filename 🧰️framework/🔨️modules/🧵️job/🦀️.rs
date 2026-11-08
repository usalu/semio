//! 🧵️ Job machines compose the genuine context protocol and retained domain owners.
pub use semio_framework_job_context::*;
pub use semio_framework_job_park::*;
pub use semio_framework_job_capture::*;
pub use semio_framework_job_native_turn::*;
pub use semio_framework_job_return::*;
pub use semio_framework_job_native_encoding::*;
#[cfg(any(test,feature="testkit"))]
pub use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,MaybeUninit};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool,AtomicPtr,AtomicU8,AtomicU32,AtomicU64,AtomicUsize,Ordering};
use semio_framework_trace::Watchdog;
#[path="📥️snapshot/🧬️schema/🦀️.rs"]
mod original_snapshot_intake;
pub use original_snapshot_intake::{OriginalSnapshotIntake,OriginalSnapshotIntakeProducer};
#[path="📬️outcome/📮️return/🦀️.rs"]
mod original_payload_return;
pub use original_payload_return::{OriginalJobPageReturnRegistry,OriginalJobPageReservation,OriginalSuppliedJobPage,OriginalContiguousJobBytes,OriginalJobBytesReturnRegistry,OriginalJobBytesReservation,OriginalSuppliedJobBytes};
#[path="📬️outcome/📦️builder/📮️return/🦀️.rs"]
mod builder_supplied_page_return;
pub use builder_supplied_page_return::RetainedPayloadBuilderSupply;
#[path="📬️outcome/👷️worker/🦀️.rs"]
mod original_worker_payload;
pub use original_worker_payload::{OriginalWorkerPayload,OriginalWorkerPageReservation};
#[path="📥️field/🔢️float/🦀️.rs"]
mod original_float_field;
pub use original_float_field::OriginalFloatFieldWork;
#[path="🏭️construction/🦀️.rs"]
mod original_construction;
pub use original_construction::{OriginalConstructionDemand,OriginalConstructionAdmission,OriginalConstructionStatus,OriginalConstructionFailure,original_construction_reserve,admit_original_construction};
#[path="🚦️clock-stride-and-job-protocol/▶️drive/🦀️.rs"]
mod original_driver;
pub use original_driver::drive_step;
#[path="🔎️reconcile/🧬️schema/🦀️.rs"]
pub mod reconcile;

#[cfg(test)]
use semio_framework_value_testing_allocator as _;

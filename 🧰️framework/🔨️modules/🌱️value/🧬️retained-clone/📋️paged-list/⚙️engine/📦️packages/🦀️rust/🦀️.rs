//! ⚙️ Fixed-page original cloning owns traversal, source custody and bounded retirement.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_paged_list::{PagedList,PagedListReservationOutcome,PagedListReservationFailure};
pub use semio_framework_value_retirement::{OriginalRetirementOutcome,OriginalRetirementPoll,RetireOwned,retirement};
pub use semio_framework_value_clone_source::{RetainedCloneBinding,RetainedCloneRef,RetainedCloneClose,close_retained_binding};
#[path="../../protocol.rs"] mod protocol;
pub use protocol::PagedCloneElement;
#[path="../../child.rs"] mod child;
use child::*;
#[path="../../🦀️.rs"] mod engine;
pub use engine::*;

#[cfg(test)]
pub mod retained_clone{pub use semio_framework_value_retained_clone::retained_clone::*;pub use semio_framework_value_clone_source_testing::{FixtureSource,FixtureSourceBirth};}
#[cfg(test)] pub use semio_framework_value_derive::{RetainedClone,RetireOwned};
#[cfg(test)] use semio_framework_value_retained_clone::retained_clone::{RetainedClone,RetainedCloneCursor};
#[cfg(test)] pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;

#[cfg(test)]
#[test]
fn paged_clone_laws_observe_real_allocation_and_release(){let(_,heap)=observe_retirement_allocations(||{let mut value=Vec::<u8>::with_capacity(73);value.resize(73,9);std::hint::black_box(&value);drop(value)});assert_eq!(heap,(73,73));}

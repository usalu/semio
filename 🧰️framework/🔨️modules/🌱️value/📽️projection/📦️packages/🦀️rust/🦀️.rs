//! 📽️ Original snapshot projection contracts and owned-field cursors share one neutral owner.
pub use semio_framework_value_control_contract::{RetainedWorkOutcome, ValueError, ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedAllocationEffect};
pub use semio_framework_value_retirement::{retirement, RetirementDemand,OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,forward_original_value_reply};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_shared_issuance as shared_issuance;
#[path = "../../🦀️.rs"]
pub mod snapshot_projection;
#[doc(hidden)]
pub mod __private {
    pub use super::snapshot_projection;
}
pub use snapshot_projection::{ImmutableRootProjection, MaybeUninit, NativeArrayFieldPlacement, ProjectionOutput, SnapshotProjection, SnapshotProjectionCursor};
#[cfg(test)]
pub use semio_framework_value_derive::{RetireOwned, SnapshotProjection};
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;

#[cfg(test)]pub(crate) use semio_framework_value_testing_admission as source_fixture;
#[cfg(test)]fn projection_test_wallet()->source_fixture::Wallet{let law:serde_json::Value=serde_json::from_str(include_str!("../../🎟️work/🧪️tests/bounded/🔣️.json")).unwrap();source_fixture::Wallet::from_profile(&law["sourceStock"])}
#[cfg(test)]pub(crate) fn projection_test_source<T:retirement::RetireOwned+Sync>(value:T)->retained_clone::RetainedCloneSource<T>{let mut wallet=projection_test_wallet();source_fixture::capture(value,&mut wallet,|w|w.loan(1,65536),source_fixture::Wallet::work_remaining,|w,p,work,h,g|w.post(p,work,g,h))}
#[cfg(test)]pub(crate) fn projection_test_shared<T:retirement::RetireOwned+Send+Sync>(value:T)->shared_issuance::SealedShared<T>{use retained_clone::{RetainedCloneGrant,RetainedCloneProgress};let mut wallet=projection_test_wallet();let mut actual=None;let g=wallet.loan(1,65536);assert!(shared_issuance::SealedShared::<T>::birth_work_demand()<=wallet.work_remaining());let(turn,heap)=allocation_testing::observe_retirement_allocations(||shared_issuance::SealedShared::admit(value,shared_issuance::SharedIssuer::owned(),g,&mut actual));let(result,p)=match turn.result{Ok(v)=>v,Err((error,original))=>{let _original=std::mem::ManuallyDrop::new(original);panic!("original fixture shared birth refused: {error:?}")}};assert_eq!(actual,Some(heap.0));wallet.post(p,turn.work,g,heap);let clear=RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of_val(&actual),..Default::default()};let g:RetainedCloneGrant=wallet.loan(1,65536);assert!(clear.fits(g)&&wallet.work_remaining()>=1);actual.take();wallet.post(clear,1,g,(0,0));result}

pub use snapshot_projection::immutable_clone::ImmutableRootCloneCursor;

pub use snapshot_projection::{NativeProjectionBacking,NativeSeedTemplate};

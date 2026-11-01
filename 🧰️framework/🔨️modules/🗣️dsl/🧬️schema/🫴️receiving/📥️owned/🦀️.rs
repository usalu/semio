//! 📥️ Owned typed binding retains real child backing and returns its defining work with every physical outcome.
use crate::{FieldValue,RecordValue};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::ManuallyDrop;
#[path="🧬️contract/🦀️.rs"]mod contract;
pub use contract::*;
#[path="📦️storage/🦀️.rs"]mod storage;
pub use storage::OwnedReceivingSlot;
#[path="🔢️scalar/🦀️.rs"]mod scalar;
pub use scalar::{OwnedScalar,ScalarReceiving};
#[doc(hidden)]
pub use scalar::scalar_sealed;
#[path="📋️vector/🦀️.rs"]mod vector;
pub use vector::VectorReceiving;
#[path="🔢️array/🦀️.rs"]mod array;
pub use array::ArrayReceiving;
#[path="🎁️wrapping/🦀️.rs"]mod wrapping;
pub use wrapping::{WrappedReceiving,OwnedWrapping};
#[path="🔄️mapping/🦀️.rs"]mod mapping;
pub use mapping::{MappedReceiving,OwnedReceivingMap};
#[path="🌳️intrinsic/🦀️.rs"]mod intrinsic;
pub use intrinsic::{NativeIntrinsicRecord,NativeIntrinsicReceiving};
pub use semio_framework_value_native_owned::owned::NativeRecordStorage;
/// 🪡️ The concrete frame allocation reports actual work and native post refusal without losing its typed backing.
pub fn reserve_record_frame<T>(frame:&mut NativeRecordStorage<T>,native:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let d=NativeRecordStorage::<T>::reserve_demand();if let Err(e)=storage::capacity(native,d.capacity_bytes){return RetainedWorkOutcome{result:Err(e.into()),work:0}}physical(native.receive_retained_work(g,frame.reserve(g)))}
/// 🛂️ Borrowed declaration validation retains the existing IO owner's real source and accounting port.
pub fn declared_record(record:&RecordValue,declared:&[(u16,bool)],owner:&mut super::NativeSnapshotDecodeOwner<'_,'_>)->Result<(),ValueError>{let mut shape=DeclaredRecordShape::default();while !shape.is_ready(){let grant=owner.remaining_grant();if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"declared record shape requires original work authority"))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"declared record shape requires original depth authority"))}match shape.step(record,declared,owner.native(),grant){Ok(progress)=>owner.record_progress(progress)?,Err(error)=>{owner.record_progress(error.retained_progress())?;return Err(error)}}}Ok(())}
#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod work_tests;

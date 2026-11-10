//! 📥️ Owned typed binding retains real child backing and returns its defining work with every physical outcome.
use crate::{FieldValue,RecordValue};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkPoll,RetirementDemand,ValueRefusalKind,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::ManuallyDrop;
#[path="🧬️contract/🦀️.rs"]mod contract;
pub use contract::*;
#[path="📦️storage/🦀️.rs"]mod storage;
pub use storage::OwnedReceivingSlot;
#[path="🔢️scalar/🦀️.rs"]mod scalar;
pub use scalar::{OwnedScalar,ScalarReceiving};
pub use semio_framework_value::{RetainedWorkOutcome,ValueError};
#[doc(hidden)]
pub use scalar::scalar_sealed;
#[path="📋️vector/🦀️.rs"]mod vector;
pub use vector::VectorReceiving;
#[path="📋️persistent/🦀️.rs"]mod persistent;
pub use persistent::PersistentReceiving;
#[path="📝️text/🦀️.rs"]mod text;
pub use text::TextReceiving;
#[path="🔢️array/🦀️.rs"]mod array;
pub use array::ArrayReceiving;
#[path="🎁️wrapping/🦀️.rs"]mod wrapping;
pub use wrapping::{WrappedReceiving,OwnedWrapping,StatementReceiving};
#[path="🔄️mapping/🦀️.rs"]mod mapping;
pub use mapping::{MappedReceiving,OwnedReceivingMap};
#[path="🌳️intrinsic/🦀️.rs"]mod intrinsic;
pub use intrinsic::{IntrinsicOrigin,NativeIntrinsicReceiving,NativeIntrinsicRecord,NativeSourceOrigin,NativeSourceReceiving,SourceOwner,SourceReceived,WholeOrigin};
#[path="🧊️field/🦀️.rs"]mod field;
pub use field::{MapReceiving,NativeFieldReceiving,SetReceiving};
pub use semio_framework_value_native_owned::owned::NativeRecordStorage;
pub use semio_framework_value_native_owned::native_decoding::owned::OriginalSourceTurnWitness;
/// 🪡️ The concrete frame allocation reports actual work and keeps its typed backing under the caller's original effect.
pub fn reserve_record_frame<T>(frame:&mut NativeRecordStorage<T>,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{let poll=normal(physical(frame.reserve_empty(effect)));settle(effect,poll)}
/// 🛂️ Borrowed declaration validation retains the existing IO owner's real source and accounting port.
pub fn declared_record(record:&RecordValue,declared:&[(u16,bool)],owner:&mut super::NativeSnapshotDecodeOwner<'_,'_>,meter:&mut super::ReceivingWork)->Result<(),ValueError>{let mut shape=DeclaredRecordShape::default();while !shape.is_ready(){let grant=owner.remaining_grant();if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"declared record shape requires original work authority"))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"declared record shape requires original depth authority"))}owner.native().checkpoint()?;let progress=receipt(metadata::<()>());let result=shape.check(record,declared);meter.take(semio_framework_value::RetainedWorkOutcome::<_,ValueError>{result:Ok(()),work:1}).map_err(|_|ValueError::invariant("shape work report has no refusal"))?;match result{Ok(())=>owner.record_progress(progress)?,Err(error)=>{owner.record_progress(progress)?;return Err(error.with_retained_progress(progress))}}}Ok(())}

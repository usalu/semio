//! 🧳️ Native receiving constructs retained owners from genuine original input under explicit physical grants.
use crate::{NativeDecodeControl,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep}};
#[path="🪡️field/🦀️.rs"]mod field;
pub use field::NativeIntrinsicSharedField;
#[path="🧩️record/🦀️.rs"]pub mod record;
pub use record::{NativeRecordSource,NativeRecordKey,NativeRecordField,NativeFlatRecord,NativeFlatRecordReceiver};
#[path="🧳️original/🦀️.rs"]mod original;
pub use original::NativeReceivingOriginal;
#[path="📋️list/🦀️.rs"]mod list;
pub use list::{NativeListSource,NativePersistentListReceiver};

#[path="🔗️sealed-shared-receiver/🦀️.rs"]mod shared;
pub use shared::NativeSharedReceiver;
#[path="📥️default/🦀️.rs"]
mod defaults;
pub use defaults::NativeDefault;
#[cfg(test)]
pub(crate) struct OriginalConstructionFixture;
#[cfg(test)]
impl super::NativeDecodeConstructionRecipient for OriginalConstructionFixture{
 fn grant(&mut self,_:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64})}
 fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{assert!(p.fits(g));Ok(())}
}

#[path="🌳️value/🦀️.rs"]pub mod tree;

fn receiving_child_body_budget(body:usize,metadata:usize)->usize{body.saturating_sub(metadata)}

/// 🏭️ Native construction declares a required receiving owner independently of synchronous wire decoding.
pub trait NativeReceive<I:?Sized>:RetireOwned+Sized{
 type Receiver:NativeReceiver<I,Self>;
 fn receive_birth_demand()->RetirementDemand;
 /// 🌱️ Birth performs no input work; refusal retains no partial owner and an admitted receiver remains caller-owned before receipt validation.
 fn begin_receive(grant:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>;
 /// 🪡️ Declared flattened records receive their own fields from the same original enclosing record.
 fn begin_record_subset(_grant:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native owner has no original record subset receiving port"))}
}

/// 🧬️ Schema membership quotes every fixed key comparison before a flattened original record read.
pub trait NativeRecordFields{
 fn native_field_memberships(key:&str)->usize;
 fn native_field_key_bytes()->usize;
}

/// 🫴️ The original input remains held until staged output and every alias complete funded transfer or closure.
pub trait NativeReceiver<I:?Sized,T>:Send+'static{
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn output_ready(&self)->bool;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(T,RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
#[cfg(test)]#[path="📋️list/🧪️tests/🦀️.rs"]mod list_tests;
#[cfg(test)]#[path="../../✨️derive/🛬️receive/🔀️enum/🧪️tests/🦀️.rs"]mod payload_enum_tests;

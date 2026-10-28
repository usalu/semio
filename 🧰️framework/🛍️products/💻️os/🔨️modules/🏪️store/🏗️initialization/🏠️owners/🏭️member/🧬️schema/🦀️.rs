//! 🏭️ The owned selected child producer retains the actual member snapshot and catalog capabilities.
use super::{DocumentStoreOwners,DocumentStoreOwnersAdmissionError};
use crate::lifted_operation::MemberSnapshotOpenOperation;
use semio_framework_os_spr::Mutation;
use semio_framework_value::{ToValue,FromValue,ValueError};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand,RetainedCloneGrant,RetainedCloneProgress};

/// 📦️ Original stateless factory selection lives in the actual retained member body and owns no substitute source.
pub trait MemberStoreProducer<P,M>:Copy+Send+Sync+'static
where P:ToValue,M:ToValue+FromValue+Mutation<P>,
{
 type SnapshotOpen:MemberSnapshotOpenOperation<Snapshot=P>;
 fn member_store_owners_copy_byte_demand(&self)->Result<usize,ValueError>;
 fn member_store_owners_birth_demand(&self)->Result<RetainedCloneBirthDemand,ValueError>;
 fn member_store_owners(&self,grant:RetainedCloneGrant)->Result<(DocumentStoreOwners<P,M>,RetainedCloneProgress),DocumentStoreOwnersAdmissionError<P,M>>;
}

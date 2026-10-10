//! 🔐️ Operation and trace children retain the same original Tick until funded handback.
use super::{ToolRunTick,ToolRunTracePage};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::shared::sealed::{SealedShared,SharedIssuer}};

#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunTickSource{original:SealedShared<ToolRunTick>}
#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunOperationSource{original:SealedShared<ToolRunTick>,ordinal:usize}
#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunTraceSource{original:SealedShared<ToolRunTick>,ordinal:usize}
impl ToolRunOperationSource{pub fn bytes(&self)->&[u8]{&self.original.get().append_ops[self.ordinal]}}
impl ToolRunTraceSource{pub fn page(&self)->&ToolRunTracePage{&self.original.get().trace[self.ordinal]}}
impl ToolRunTickSource{
 pub fn birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<ToolRunTick>(),capacity_bytes:SealedShared::<ToolRunTick>::birth_bytes(),depth:1,..Default::default()}}
 pub fn admit(original:ToolRunTick,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,ToolRunTick)>{if grant.maximum_copy_bytes<Self::birth_demands().copy_bytes{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Tick source move is not funded"),original));}SealedShared::admit(original,SharedIssuer::owned(),grant).map(|(original,mut receipt)|{receipt.copied_bytes=Self::birth_demands().copy_bytes;(Self{original},receipt)})}
 pub fn operation_capture_copy_bytes()->usize{size_of::<ToolRunOperationSource>()}
 pub fn trace_capture_copy_bytes()->usize{size_of::<ToolRunTraceSource>()}
 pub fn operation(&self,ordinal:usize,grant:RetainedCloneGrant)->Result<(ToolRunOperationSource,RetainedCloneProgress),ValueError>{if ordinal>=self.borrow().append_ops.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original Tick operation ordinal is out of range"));}if grant.maximum_copy_bytes<Self::operation_capture_copy_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original operation capture is not funded"));}self.original.try_duplicate(grant).map(|(original,mut receipt)|{receipt.copied_bytes=Self::operation_capture_copy_bytes();(ToolRunOperationSource{original,ordinal},receipt)})}
 pub fn trace(&self,ordinal:usize,grant:RetainedCloneGrant)->Result<(ToolRunTraceSource,RetainedCloneProgress),ValueError>{if ordinal>=self.borrow().trace.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original Tick trace ordinal is out of range"));}if grant.maximum_copy_bytes<Self::trace_capture_copy_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original trace capture is not funded"));}self.original.try_duplicate(grant).map(|(original,mut receipt)|{receipt.copied_bytes=Self::trace_capture_copy_bytes();(ToolRunTraceSource{original,ordinal},receipt)})}
 pub fn borrow(&self)->&ToolRunTick{self.original.get()}
 pub fn take_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<ToolRunTick>(),release_bytes:SealedShared::<ToolRunTick>::birth_bytes(),depth:1,..Default::default()}}
 pub fn try_into_tick(self,grant:RetainedCloneGrant)->Result<(ToolRunTick,RetainedCloneProgress),(ValueError,Self)>{self.original.try_unwrap(grant).map_err(|(error,original)|(error,Self{original}))}
}

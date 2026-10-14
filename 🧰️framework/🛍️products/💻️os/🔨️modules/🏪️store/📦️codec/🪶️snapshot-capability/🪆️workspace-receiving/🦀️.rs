//! 🪆️ One shared receiving implementation for every handwritten relational snapshot model.
//!
//! The authored `to_sqlite_database` / `from_sqlite_database` / `validate_sqlite_snapshot_subset` already define each artifact's
//! tables, rows and subset policy; this module runs them under the original receiving owner instead of re-authoring them per
//! editor: every allocation they admit through the control is charged to the original native control, and whatever they retain retires into the
//! operation workspace the codec installed on the control (`export` / `import`), which is closed under the caller's grant.
//!
//! @see 🪶️ `ArtifactSqliteSnapshotHost` the trait whose receiving defaults delegate here.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::{ArtifactSqliteSnapshotHost,NativeSnapshotDecodeOwner,NativeSnapshotEncodeOwner};
use crate::io_schema::{IoError,IoResult};
use crate::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{ValueError,ValueRefusalKind,native_decoding::NativeDecodeRetirementRecipient};

fn overflow()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"SQLite receiving extent overflow")}
fn unbacked()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"SQLite receiving has no original backing allowance")}

macro_rules! receiving_allocation{
 ($owner:ident,$control:ident,|$child:ident| $body:expr)=>{{
  let remaining=$control.allocation_remaining_bytes();if remaining==0{return Err(unbacked())}
  let maximum=$owner.native().owned_bytes().checked_add(remaining).ok_or_else(overflow)?;
  let result=$owner.scoped_native(maximum,&mut |_|true,|$owner|{
   $owner.native().checkpoint()?;
   $owner.native().with_encoding_receiver(|room,_observer,allocation|$control.with_original_allocation(room,allocation,|$child|$body))
  });
  result.map_err(|error|error.with_retained_progress($owner.progress()))
 }};
}

/// 🗄️ Projects the snapshot through its authored relational model under the original decode owner.
pub fn project<P:ArtifactSqliteSnapshotHost>(snapshot:&P,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<SqliteDatabase,ValueError>{
 receiving_allocation!(owner,control,|child| snapshot.to_sqlite_database(child))
}

/// 🪆️ Reconstructs the snapshot from its authored relational model under the original encode owner.
pub fn reconstruct<P:ArtifactSqliteSnapshotHost>(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->Result<P,ValueError>{
 receiving_allocation!(owner,control,|child| P::from_sqlite_database(database,child))
}

/// 🛡️ Applies the authored subset policy after the original decode owner observed cancellation.
pub fn validate_decoding<P:ArtifactSqliteSnapshotHost>(snapshot:&P,dialect:&ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->IoResult<()>{
 owner.native().checkpoint().map_err(IoError::from_value_error)?;
 snapshot.validate_sqlite_snapshot_subset(dialect,database,control)
}

/// 🛃️ Applies the authored subset policy after the original encode owner observed cancellation.
pub fn validate_encoding<P:ArtifactSqliteSnapshotHost>(snapshot:&P,dialect:&ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->IoResult<()>{
 owner.native().checkpoint().map_err(IoError::from_value_error)?;
 snapshot.validate_sqlite_snapshot_subset(dialect,database,control)
}

/// ♻️ An original receiving owner that can retire a stage workspace under its own grant.
pub trait StageOwner{fn retire_workspace(&mut self,workspace:NativeDecodeRetirementRecipient)->Result<(),ValueError>;}
impl StageOwner for NativeSnapshotDecodeOwner<'_,'_>{fn retire_workspace(&mut self,workspace:NativeDecodeRetirementRecipient)->Result<(),ValueError>{self.receive::<NativeDecodeRetirementRecipient,()>(|slot,_,_|{*slot=Some(workspace);Ok(())})}}
impl StageOwner for NativeSnapshotEncodeOwner<'_,'_>{fn retire_workspace(&mut self,workspace:NativeDecodeRetirementRecipient)->Result<(),ValueError>{self.receive::<NativeDecodeRetirementRecipient,()>(|slot,_,_|{*slot=Some(workspace);Ok(())})}}

/// 🗄️ Runs one authored stage with its own empty workspace recipient on the control and retires whatever the stage retained under the owner's grant, so a stage that retains nothing costs nothing.
pub fn staged<T,E:From<ValueError>,O:StageOwner>(control:&mut SqliteSnapshotControl<'_>,owner:&mut O,stage:impl FnOnce(&mut SqliteSnapshotControl<'_>,&mut O)->Result<T,E>)->Result<T,E>{
 let mut workspace=NativeDecodeRetirementRecipient::new();
 let result=control.with_workspace_recipient(&mut workspace,|control|stage(control,owner));
 if !semio_framework_value::ErasedSnapshotRetirement::terminal_is_empty(&workspace){owner.retire_workspace(workspace).map_err(E::from)?;}
 result
}

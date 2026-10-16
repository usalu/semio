//! 📥️ Items of `sqlite_snapshot_workspace` that belong to `os-store-codec-registry` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs`.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_io_control::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
use semio_framework_io_sqlite_snapshot::SqliteSnapshotControl;
use semio_framework_value::ValueError;
use semio_framework_value::native_decoding::NativeDecodeRetirementRecipient;
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

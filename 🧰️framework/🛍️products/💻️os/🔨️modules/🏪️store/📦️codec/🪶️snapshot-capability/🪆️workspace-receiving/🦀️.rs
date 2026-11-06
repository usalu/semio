//! 🪆️ One shared receiving implementation for every handwritten relational snapshot model.
//!
//! The authored `to_sqlite_database` / `from_sqlite_database` / `validate_sqlite_snapshot_subset` already define each artifact's
//! tables, rows and subset policy; this module runs them under the original receiving owner instead of re-authoring them per
//! editor: every allocation they admit through the control is charged to the original native control, and whatever they retain retires into the
//! operation workspace the codec installed on the control (`export` / `import`), which is closed under the caller's grant.
//!
//! @see 🪶️ `ArtifactSqliteSnapshotHost` the trait whose receiving defaults delegate here.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::lifted_serialization::ArtifactSqliteSnapshotHost;
use semio_framework_io_control::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner, NativeSnapshotFailure};
use semio_framework_io_schema::IoResult;
use semio_framework_io_sqlite_snapshot::{SqliteDatabase, SqliteSnapshotControl};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{ValueError, ValueRefusalKind, RetainedWorkOutcome};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;

fn overflow()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"SQLite receiving extent overflow")}

macro_rules! receiving_allocation{
 ($owner:ident,$control:ident,$output:ident,|$child:ident| $body:expr)=>{{
  use semio_framework_io_sqlite_snapshot::SqliteStageFailure as Stage;
  if $output.is_some(){RetainedWorkOutcome{result:Err(Stage::NotStarted(ValueError::literal(ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied"))),work:0}}
  else{
   let turn=$owner.receive::<(),_,NativeSnapshotFailure>($output,&mut |_frame,output,_marker,_unwind,native,_body|{
    let before=$control.performed_work();
    let result:Result<(),ValueError>=(||{native.checkpoint()?;let maximum=native.owned_bytes().checked_add($control.allocation_remaining_bytes()).ok_or_else(overflow)?;native.scoped_maximum(maximum,|native|native.with_encoding_receiver(|room,_observer,allocation|$control.with_original_allocation(room,allocation,|$child|{*output=Some($body.map_err(|error:ValueError|Stage::Operation(error.into())));Ok(())})))})();
    RetainedWorkOutcome{result,work:$control.performed_work().checked_sub(before).expect("original SQL producer work is monotone")}
   });
   RetainedWorkOutcome{result:turn.result.map_err(Stage::Operation),work:turn.work}
  }
 }};
}
/// 🗄️ Projects the snapshot through its authored relational model under the original decode owner.
pub fn project<P:ArtifactSqliteSnapshotHost>(snapshot:&P,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<SqliteDatabase>>)->crate::lifted_serialization::SnapshotReceivingOutcome{
 receiving_allocation!(owner,control,output,|child| snapshot.to_sqlite_database(child))
}
/// 🪆️ Reconstructs the snapshot from its authored relational model under the original encode owner.
pub fn reconstruct<P:ArtifactSqliteSnapshotHost>(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<P>>)->crate::lifted_serialization::SnapshotReceivingOutcome{
 receiving_allocation!(owner,control,output,|child| P::from_sqlite_database(database,child))
}
macro_rules! validate{
 ($name:ident,$owner:ident)=>{
 pub fn $name<P:ArtifactSqliteSnapshotHost>(snapshot:&P,dialect:&ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut $owner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<IoResult<()>>>)->crate::lifted_serialization::SnapshotReceivingOutcome{
  receiving_allocation!(owner,control,output,|child| Ok::<IoResult<()>,ValueError>(snapshot.validate_sqlite_snapshot_subset(dialect,database,child)))
 }
 };
}
validate!(validate_decoding,NativeSnapshotDecodeOwner);
validate!(validate_encoding,NativeSnapshotEncodeOwner);

#[cfg(test)]
mod zero_allocation_tests{
 use super::*;
 use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
 use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeConstructionRecipient,NativeDecodeRetirementRecipient};
 #[derive(semio_framework_value::RetireOwned)]
 struct Zero(bool);
 impl pack::snapshot::ArtifactSqliteSnapshot for Zero{
  const SQLITE_SCHEMA:&'static str="";
  fn to_sqlite_database(&self,_:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{Ok(SqliteDatabase{tables:Vec::new()})}
  fn from_sqlite_database(_:&SqliteDatabase,_:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{Ok(Self(false))}
 }
 impl ArtifactSqliteSnapshotHost for Zero{
  fn validate_sqlite_snapshot_subset(&self,_:&ArtifactDialect,_:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->IoResult<()>{control.record_performed_work(1).map_err(semio_framework_io_schema::IoError::from_value_error)?;if self.0{Err(semio_framework_io_schema::IoError::from_value_error(ValueError::literal(ValueRefusalKind::InvalidValue,"zero-allocation authored refusal")))}else{Ok(semio_framework_io_schema::IoOutcome::clean(()))}}
 }
 struct Work(u64);
 impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn admits(&mut self,observed:u64,additional:u64)->std::result::Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
 struct Recipient{grant:RetainedCloneGrant,received:RetainedCloneProgress,deny:bool,refused:bool}
 impl NativeDecodeConstructionRecipient for Recipient{
  fn submitted(&self)->RetainedCloneProgress{self.received}
  fn grant(&mut self,_:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{Ok(self.grant)}
  fn accept(&mut self,_:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{self.received=self.received.checked_add(progress)?;if self.deny&&!self.refused&&progress.copied_items==1&&progress.copied_bytes==0&&progress.retained_capacity_bytes==0&&progress.released_bytes==0{self.refused=true;Err(ValueError::literal(ValueRefusalKind::Canceled,"zero-allocation publication refused"))}else{Ok(())}}
 }
 #[test]
 fn zero_sql_backing_preserves_valid_and_refused_publications(){
  let grant=RetainedCloneGrant{maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64};let dialect=ArtifactDialect{artifact_kind:"zero".into(),standard:"1".into(),subset:"*".into()};let database=SqliteDatabase{tables:Vec::new()};
  for deny in [false,true]{let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,semio_framework_io_sqlite_snapshot::SqliteDatabaseLimits{max_allocation_bytes:0,..Default::default()});let mut observer=|_|true;let mut retirement=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(grant.maximum_capacity_bytes,&mut observer);native.install_retirement_recipient(&mut retirement).unwrap();let mut recipient=Recipient{grant,received:Default::default(),deny,refused:false};let mut output=None;
   let mut work=Work(1000000);native.with_work_recipient(&mut work,|native|{let turn=native.with_construction_recipient(&mut recipient,|native|{let mut owner=NativeSnapshotDecodeOwner::new(native,grant);Ok::<_,ValueError>(Zero(deny).validate_sqlite_snapshot_subset_decoding(&dialect,&database,&mut control,&mut owner,&mut output))}).unwrap();assert_eq!(control.performed_work(),1);assert_eq!(control.allocation_remaining_bytes(),0);assert!(turn.work>0);assert_eq!(turn.result.is_err(),deny);assert_eq!(recipient.refused,deny);match output.take(){Some(Ok(Ok(_))) if !deny=>{},Some(Ok(Err(error))) if deny=>assert_eq!(error.cause.message,"zero-allocation authored refusal"),_=>panic!("original zero-allocation validation output was lost")};while native.has_retirement_owner(){native.close_retirement_recipient(grant).result.unwrap();}Ok::<(),ValueError>(())}).unwrap();
  }
 }
}

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
    let result:Result<(),ValueError>=(||{native.checkpoint()?;let maximum=native.owned_bytes().checked_add($control.allocation_remaining_bytes()).ok_or_else(overflow)?;native.scoped_maximum(maximum,|native|{
     let mut frame=None;let mut call=semio_framework_value_native_owned::native_decoding::NativeCallFailure::<_,ValueError>{output:None,operation:None,context:None,receipt_refusal:None};
     let mut late=None;
     let ran=native.with_encoding_receiver(&mut frame,&mut call,|room,_observer,allocation,_work,_construction,out,_operation|{if let Err(error)=$control.with_original_allocation(room,allocation,|$child|{*out=Some($body.map_err(|error:ValueError|Stage::Operation(error.into())));Ok(())}){late=Some(error)}});
     let settled=(||->Result<(),ValueError>{if let Some(frame)=frame.as_mut(){frame.settle_pair()?;if !frame.take_allocation_failure()?.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite receiving retained an allocation refusal"))}native.release_encoding_receiver(frame)?}Ok(())})();
     ran?;settled?;if let Some(error)=late{return Err(error)}if let Some(error)=call.context.take().or_else(||call.receipt_refusal.take()){return Err(error)}*output=call.output.take();Ok(())
    })})();
    RetainedWorkOutcome{result:result.map_err(NativeSnapshotFailure::from),work:u128::from($control.performed_work().checked_sub(before).expect("original SQL producer work is monotone"))}
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
 use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeRetirementRecipient};
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
 impl semio_framework_value_native_owned::native_decoding::OriginalWorkRecipient for Work{
  type Context<'turn,'source>=() where 'source:'turn;
  fn validate_context<'turn,'source>(&self,_:&())->std::result::Result<(),ValueError> where 'source:'turn{Ok(())}
  fn source_turn<'turn,'source>(&self,_:&())->Option<semio_framework_value_native_owned::native_decoding::OriginalSourceTurnIdentity> where 'source:'turn{None}
  fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:semio_framework_value_native_owned::native_decoding::OriginalWorkQuote<'reply>,observed:u64,additional:u64)->semio_framework_value_native_owned::native_decoding::OriginalWorkQuoteComplete<'reply> where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0)))}
  fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheck<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(Ok(()))}
  fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheck<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.pending()}
 }
 struct Recipient{grant:RetainedCloneGrant,received:RetainedCloneProgress,deny:bool,refused:bool}
 impl Recipient{
  fn submitted_progress(&self)->RetainedCloneProgress{self.received}
  fn grant_for(&mut self,_demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{Ok(self.grant)}
  fn accept_for(&mut self,_:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{self.received=self.received.checked_add(progress)?;if self.deny&&!self.refused&&progress.copied_items==1&&progress.copied_bytes==0&&progress.retained_capacity_bytes==0&&progress.released_bytes==0{self.refused=true;Err(ValueError::literal(ValueRefusalKind::Canceled,"zero-allocation publication refused"))}else{Ok(())}}
 }
 impl semio_framework_value_native_owned::native_decoding::OriginalConstructionRecipient for Recipient{
  type Context<'turn,'source>=() where 'source:'turn;
  fn validate_context<'turn,'source>(&self,_:&())->std::result::Result<(),ValueError> where 'source:'turn{Ok(())}
  fn source_turn<'turn,'source>(&self,_:&())->Option<semio_framework_value_native_owned::native_decoding::OriginalSourceTurnIdentity> where 'source:'turn{None}
  fn grant<'reply,'turn,'source>(&mut self,_:&mut (),quote:semio_framework_value_native_owned::native_decoding::OriginalConstructionGrant<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalConstructionGrantComplete<'reply> where 'source:'turn{let result=self.grant_for(quote.demand());quote.complete(result)}
  fn accept<'reply,'turn,'source>(&mut self,_:&mut (),receipt:semio_framework_value_native_owned::native_decoding::OriginalConstructionAccept<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalConstructionAcceptComplete<'reply> where 'source:'turn{let result=self.accept_for(receipt.grant(),receipt.progress());receipt.complete(result)}
  fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted_progress()}
 }
 #[test]
 fn zero_sql_backing_preserves_valid_and_refused_publications(){
  let grant=RetainedCloneGrant{maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64};let dialect=ArtifactDialect{artifact_kind:"zero".into(),standard:"1".into(),subset:"*".into()};let database=SqliteDatabase{tables:Vec::new()};
  for deny in [false,true]{let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,semio_framework_io_sqlite_snapshot::SqliteDatabaseLimits{max_allocation_bytes:0,..Default::default()});let mut observer=|_|true;let mut retirement=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(grant.maximum_capacity_bytes,&mut observer);native.install_retirement_recipient(&mut retirement).unwrap();let mut recipient=semio_framework_value_native_owned::native_decoding::OriginalConstructionAuthority::new(Recipient{grant,received:Default::default(),deny,refused:false});let mut output=None;
   let mut work=semio_framework_value_native_owned::native_decoding::OriginalWorkAuthority::new(Work(1000000));work.with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|{let turn=recipient.with_loan(&mut (),|construction|native.with_construction_recipient(construction,|native|{let mut owner=NativeSnapshotDecodeOwner::new(native,grant);Ok::<_,ValueError>(Zero(deny).validate_sqlite_snapshot_subset_decoding(&dialect,&database,&mut control,&mut owner,&mut output))})).unwrap().unwrap();assert_eq!(control.performed_work(),1);assert_eq!(control.allocation_remaining_bytes(),0);assert!(turn.work>0);assert_eq!(turn.result.is_err(),deny);assert_eq!(recipient.inspect(|r|r.refused),deny);match output.take(){Some(Ok(Ok(_))) if !deny=>{},Some(Ok(Err(error))) if deny=>assert_eq!(error.cause.message,"zero-allocation authored refusal"),_=>panic!("original zero-allocation validation output was lost")};while native.has_retirement_owner(){native.close_retirement_recipient(grant).result.unwrap();}Ok::<(),ValueError>(())})).unwrap().unwrap();
  }
 }
}

//! 📥️ The persisted `ArtifactBody` statement receives through two mirror records, because its blob variant wraps the foreign `BlobRef`.
use super::ArtifactBody;
use semio_framework_dsl_record::{FieldValue,receiving::owned::{DslOwnedReceivingField,DslOwnedReceivingVariants,OwnedAdvanceSelection,OwnedClose,OwnedFieldReceiving,OwnedPoll,OwnedTake,OriginalSourceTurnWitness,StatementReceiving}};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,native_decoding::NativeDecodeControl};

/// 📄️ The document variant's two text fields, in the hand-written record spec's order.
#[derive(semio_framework_value::RetireOwned,semio_framework_dsl_record_derive::DslRecord,semio_framework_dsl_record_derive::DslReceiving)]
pub struct ArtifactBodyDocumentFields{pub schema:String,pub document_id:String}
/// 🧱️ The blob variant's three scalar fields, in the hand-written record spec's order.
#[derive(semio_framework_value::RetireOwned,semio_framework_dsl_record_derive::DslRecord,semio_framework_dsl_record_derive::DslReceiving)]
pub struct ArtifactBodyBlobFields{pub hash:String,pub size:u64,pub media_type:String}

type DocumentCursor=<ArtifactBodyDocumentFields as DslOwnedReceivingField>::Cursor;
type BlobCursor=<ArtifactBodyBlobFields as DslOwnedReceivingField>::Cursor;

/// 🎯️ Selects the mirror record cursor of one declared variant and moves its finished fields into the variant.
pub enum ArtifactBodyReceiving{Document(DocumentCursor),Blob(BlobCursor)}
fn unsupported(keyword:&str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,match keyword{_=>"ArtifactBody statement names an undeclared variant"})}
impl DslOwnedReceivingVariants for ArtifactBody{
 type Cursor=ArtifactBodyReceiving;
 fn receiving_cursor(keyword:&str)->Result<Self::Cursor,ValueError>{match keyword{"document"=>Ok(ArtifactBodyReceiving::Document(<ArtifactBodyDocumentFields as DslOwnedReceivingField>::receiving_cursor())),"blob"=>Ok(ArtifactBodyReceiving::Blob(<ArtifactBodyBlobFields as DslOwnedReceivingField>::receiving_cursor())),other=>Err(unsupported(other))}}
}
impl DslOwnedReceivingField for ArtifactBody{type Cursor=StatementReceiving<Self>;fn receiving_cursor()->Self::Cursor{Default::default()}}
macro_rules! delegate{($self:ident,$child:ident=>$call:expr)=>{match $self{Self::Document($child)=>$call,Self::Blob($child)=>$call}}}
impl OwnedFieldReceiving<ArtifactBody> for ArtifactBodyReceiving{
 fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{delegate!(self,child=>child.demands(source,body))}
 fn select_advance(&self,source:&Option<FieldValue>,body:usize,maximum_work_units:u64)->Result<OwnedAdvanceSelection,ValueError>{delegate!(self,child=>child.select_advance(source,body,maximum_work_units))}
 fn advance_nested(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->OwnedPoll{delegate!(self,child=>child.advance_nested(source,native,witness,body,maximum_work_units,grant))}
 fn advance_local(&mut self,source:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{delegate!(self,child=>child.advance_local(source,body,effect))}
 fn is_ready(&self)->bool{delegate!(self,child=>child.is_ready())}
 fn take_work_demand(&self)->Result<u64,ValueError>{delegate!(self,child=>child.take_work_demand())}
 fn take_into(&mut self,output:&mut Option<ArtifactBody>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{
  if output.is_some(){return semio_framework_dsl_record::receiving::owned::done(effect,Err(semio_framework_dsl_record::receiving::owned::invariant()))}
  match self{
   Self::Document(child)=>{let mut fields=None;let turn=child.take_into(&mut fields,effect);if let Some(ArtifactBodyDocumentFields{schema,document_id})=fields{*output=Some(ArtifactBody::Document{schema,document_id});}turn}
   Self::Blob(child)=>{let mut fields=None;let turn=child.take_into(&mut fields,effect);if let Some(ArtifactBodyBlobFields{hash,size,media_type})=fields{*output=Some(ArtifactBody::Blob{blob:store::BlobRef{hash,size,media_type}});}turn}
  }
 }
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{delegate!(self,child=>child.close_demands(body))}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{delegate!(self,child=>child.close_work_demand(body))}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{delegate!(self,child=>child.close_step(effect))}
 fn terminal_is_empty(&self)->bool{delegate!(self,child=>child.terminal_is_empty())}
}
impl semio_framework_dsl_record::DslReceivingField for ArtifactBody{
 fn receive_value(value:&FieldValue,output:&mut Option<Self>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),store::NativeSnapshotFailure>{
  use semio_framework_dsl_record::DslReceivingRecord;
  let refused=|reason:&'static str|RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,reason).into()),work:0};
  if output.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"ArtifactBody receiving destination is occupied").into()),work:0}}
  let FieldValue::Statements(list)=value else{return refused("ArtifactBody field requires one named statement")};
  let [(keyword,record)]=list.as_slice() else{return refused("ArtifactBody field requires exactly one statement")};
  match keyword.as_str(){
   "document"=>{let mut fields=None;let turn=ArtifactBodyDocumentFields::receive_record(record,&mut fields,owner);if let Some(fields)=fields{*output=Some(ArtifactBody::Document{schema:fields.schema,document_id:fields.document_id});}turn},
   "blob"=>{let mut fields=None;let turn=ArtifactBodyBlobFields::receive_record(record,&mut fields,owner);if let Some(fields)=fields{*output=Some(ArtifactBody::Blob{blob:store::BlobRef{hash:fields.hash,size:fields.size,media_type:fields.media_type}});}turn},
   _=>refused("ArtifactBody statement names an undeclared variant"),
  }
 }
}

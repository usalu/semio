//! 📥️ The persisted `ArtifactBody` statement receives through two mirror records, because its blob variant wraps the foreign `BlobRef`.
use super::ArtifactBody;
use semio_framework_dsl_record::{FieldValue,receiving::owned::{DslOwnedReceivingField,DslOwnedReceivingVariants,OwnedFieldReceiving,StatementReceiving}};
use semio_framework_value::{OriginalReceivingPrimary,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,native_decoding::NativeDecodeControl};

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
impl OwnedFieldReceiving<ArtifactBody> for ArtifactBodyReceiving{
 fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{match self{Self::Document(child)=>child.demands(source,body),Self::Blob(child)=>child.demands(source,body)}}
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{match self{Self::Document(child)=>child.step(source,native,body,grant),Self::Blob(child)=>child.step(source,native,body,grant)}}
 fn is_ready(&self)->bool{match self{Self::Document(child)=>child.is_ready(),Self::Blob(child)=>child.is_ready()}}
 fn take_into(&mut self,output:&mut Option<ArtifactBody>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{
  if output.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"ArtifactBody receiving destination is occupied")),work:0}}
  match self{
   Self::Document(child)=>{let mut fields=None;let turn=child.take_into(&mut fields,grant);if let Some(ArtifactBodyDocumentFields{schema,document_id})=fields{*output=Some(ArtifactBody::Document{schema,document_id});}turn}
   Self::Blob(child)=>{let mut fields=None;let turn=child.take_into(&mut fields,grant);if let Some(ArtifactBodyBlobFields{hash,size,media_type})=fields{*output=Some(ArtifactBody::Blob{blob:store::BlobRef{hash,size,media_type}});}turn}
  }
 }
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self{Self::Document(child)=>child.close_demands(body),Self::Blob(child)=>child.close_demands(body)}}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{match self{Self::Document(child)=>child.close_step(grant),Self::Blob(child)=>child.close_step(grant)}}
 fn terminal_is_empty(&self)->bool{match self{Self::Document(child)=>child.terminal_is_empty(),Self::Blob(child)=>child.terminal_is_empty()}}
}
impl semio_framework_dsl_record::DslReceivingField for ArtifactBody{
 fn receive_value(value:&FieldValue,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,store::NativeSnapshotFailure>{
  use semio_framework_dsl_record::DslReceivingRecord;
  let refused=|reason:&'static str|RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,reason).into()),work:0};
  let FieldValue::Statements(list)=value else{return refused("ArtifactBody field requires one named statement")};
  let [(keyword,record)]=list.as_slice() else{return refused("ArtifactBody field requires exactly one statement")};
  match keyword.as_str(){
   "document"=>{let turn=ArtifactBodyDocumentFields::receive_record(record,owner);RetainedWorkOutcome{result:turn.result.map(|fields|ArtifactBody::Document{schema:fields.schema,document_id:fields.document_id}),work:turn.work}},
   "blob"=>{let turn=ArtifactBodyBlobFields::receive_record(record,owner);RetainedWorkOutcome{result:turn.result.map(|fields|ArtifactBody::Blob{blob:store::BlobRef{hash:fields.hash,size:fields.size,media_type:fields.media_type}}),work:turn.work}},
   _=>refused("ArtifactBody statement names an undeclared variant"),
  }
 }
}

//! 🌳️ Original heterogeneous value sources and bounded schema keys.
use crate::{DslValue,Number,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeTreeKind{Null,Bool,Number,Text,Array,Object}
impl NativeTreeKind{pub const fn as_str(self)->&'static str{match self{Self::Null=>"null",Self::Bool=>"bool",Self::Number=>"number",Self::Text=>"text",Self::Array=>"array",Self::Object=>"object"}}}
/// 🔎️ One bounded scalar query against the exact original Source.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(u8)]
pub enum NativeScalarQuery{Kind,Null,Boolean,Number,EnumOrdinal,RecordLength,SequenceLength}
/// 🪪️ Preserves the typed original scalar answer without cloning Source ownership.
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum NativeSourceScalar{Kind(Option<NativeTreeKind>),Null(bool),Boolean(bool),Number(Number),EnumOrdinal(u32),Length(usize)}
pub trait NativeValueSource:Sync+'static{
 /// 📬️ Records the actual scalar answer or original owned error in the borrowed reply.
 fn query_scalar<'reply>(&self,query:NativeScalarQuery,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'reply,NativeSourceScalar,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'reply,NativeSourceScalar,ValueError>,ValueError>;
 /// 🔁️ Recovers only the same original Source query without replay.
 fn recover_scalar<'reply>(&self,query:NativeScalarQuery,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'reply,NativeSourceScalar,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'reply,NativeSourceScalar,ValueError>,ValueError>;
 fn as_source(&self)->&dyn NativeValueSource;
 fn tree_kind(&self)->Option<NativeTreeKind>;
 fn enum_ordinal(&self)->Result<u32,ValueError>{Err(invalid())}
 fn variant_tag(&self)->Option<&str>{None}
 fn variant_payload(&self)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
 fn is_null(&self)->bool{false}
 fn boolean(&self)->Result<bool,ValueError>{Err(invalid())}
 fn number(&self)->Result<Number,ValueError>{Err(invalid())}
 fn text(&self)->Result<&String,ValueError>{Err(invalid())}
 fn record_len(&self)->Result<usize,ValueError>{Err(invalid())}
 fn record_key(&self,_index:usize)->Result<crate::record::NativeRecordKey<'_>,ValueError>{Err(invalid())}
 fn record_name(&self,index:usize)->Result<&str,ValueError>{match self.record_key(index)?{crate::record::NativeRecordKey::Name(name)=>Ok(name),crate::record::NativeRecordKey::Ordinal(_)=>Err(invalid())}}
 fn record_entry(&self,_index:usize)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
 fn record_child(&self,_index:usize)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
 fn sequence_len(&self)->Result<usize,ValueError>{Err(invalid())}
 fn sequence_child(&self,_index:usize)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
}
semio_framework_value_retirement::artifact_retire_leaf!(NativeScalarQuery,NativeSourceScalar);
/// 🎛️ Owns the original scalar query, Source identity and recorded reply.
pub struct NativeScalarQueryProducer<I:NativeValueSource+?Sized>{identity:(u64,usize,usize,usize),query:NativeScalarQuery,answer:std::mem::ManuallyDrop<Option<Result<NativeSourceScalar,ValueError>>>,input:std::marker::PhantomData<fn(&I)>}
/// 🪢️ Lends the exact source or its caller-resident paid answer slot.
pub enum NativeScalarQueryOperation<'turn,'source,I:NativeValueSource+?Sized>{Query(crate::retained_clone::RetainedCloneRef<'source,I>),Transfer(crate::retained_clone::RetainedCloneRef<'source,I>,&'turn mut Option<Result<NativeSourceScalar,ValueError>>)}
impl<I:NativeValueSource+?Sized> NativeScalarQueryProducer<I>{
 /// 🌱️ Produces a pure inline seed; the caller funds its resident publication before any callback.
 pub fn seed(input:crate::retained_clone::RetainedCloneRef<'_,I>,query:NativeScalarQuery)->Self{Self{identity:input.reference_identity(),query,answer:std::mem::ManuallyDrop::new(None),input:std::marker::PhantomData}}
 /// 📐️ Quotes the original inner and outer recorded reply slots.
 pub fn demand()->RetirementDemand{fixed(std::mem::size_of::<Option<Result<NativeSourceScalar,ValueError>>>()+std::mem::size_of::<Option<Result<crate::RetainedCloneStep,ValueError>>>())}
 /// ⏱️ Quotes Source inspection and the separately recorded outer reply.
 pub fn work_demand()->u64{2}
 /// 📦️ Reports whether the original owned answer remains resident.
 pub fn answer_ready(&self)->bool{self.answer.is_some()}
 /// 📐️ Quotes movement from the retained answer into its caller-resident slot.
 pub fn transfer_demand()->RetirementDemand{fixed(2*std::mem::size_of::<Option<Result<NativeSourceScalar,ValueError>>>())}
 /// ⏱️ Quotes the selected paid answer transfer.
 pub fn transfer_work_demand(&self)->u64{u64::from(self.answer.is_some())}
 fn record_outer<'reply>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'reply,crate::RetainedCloneStep,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,ValueError>,ValueError>{if self.answer.is_none(){return Ok(reply.pending())}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar reply recording exceeded its work authority"))))}Ok(reply.complete(Ok(crate::RetainedCloneStep::Complete(receipt(Self::demand())))))}
 fn transfer<'reply>(&mut self,output:&mut Option<Result<NativeSourceScalar,ValueError>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'reply,crate::RetainedCloneStep,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,ValueError>,ValueError>{if self.answer.is_none()||output.is_some(){return Ok(reply.complete(Err(ValueError::invariant("scalar answer transfer requires its original occupied and empty slots"))))}let d=Self::transfer_demand();if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar answer transfer exceeded its work authority"))))}*output=self.answer.take();Ok(reply.complete(Ok(crate::RetainedCloneStep::Complete(receipt(d)))))}
}
impl<I:NativeValueSource+?Sized> semio_framework_value_retirement::retirement::RetireOwned for NativeScalarQueryProducer<I>{type RetirementCursor=<((u64,usize,usize,usize),NativeScalarQuery,Option<Result<NativeSourceScalar,ValueError>>) as semio_framework_value_retirement::retirement::RetireOwned>::RetirementCursor;fn retirement_seed(mut self)->Self::RetirementCursor{let answer=self.answer.take();semio_framework_value_retirement::retirement::RetireOwned::retirement_seed((self.identity,self.query,answer))}fn controlled_retirement_supported()->bool{true}}
impl<I:NativeValueSource+?Sized> Drop for NativeScalarQueryProducer<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.answer.is_none(),"scalar Source query requires funded answer transfer or retirement")}}
impl<I:NativeValueSource+?Sized> semio_framework_value_native_decoding::native_decoding::OriginalEffectProducer for NativeScalarQueryProducer<I>{
 type Output=crate::RetainedCloneStep;type Failure=ValueError;type Context<'turn,'source>=NativeScalarQueryOperation<'turn,'source,I> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{let input=match context{NativeScalarQueryOperation::Query(input)|NativeScalarQueryOperation::Transfer(input,_)=>*input};if input.reference_identity()!=self.identity{Err(ValueError::invariant("original scalar Source changed"))}else{Ok(())}}
 fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'reply,crate::RetainedCloneStep,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,ValueError>,ValueError>where 'source:'turn{match context{NativeScalarQueryOperation::Transfer(_,output)=>self.transfer(output,effect,reply),NativeScalarQueryOperation::Query(input)=>{if self.answer.is_some(){return Err(ValueError::invariant("original scalar query answer already retained"))}let query=self.query;{let inner=semio_framework_value_native_decoding::native_decoding::OriginalEffectReply::borrow(&mut self.answer)?;let _=input.get().query_scalar(query,effect,inner)?;}self.record_outer(effect,reply)}}}
 fn recover<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'reply,crate::RetainedCloneStep,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,ValueError>,ValueError>where 'source:'turn{match context{NativeScalarQueryOperation::Transfer(_,_)=>Ok(reply.pending()),NativeScalarQueryOperation::Query(input)=>{if self.answer.is_none(){let query=self.query;let inner=semio_framework_value_native_decoding::native_decoding::OriginalEffectReply::borrow(&mut self.answer)?;let _=input.get().recover_scalar(query,effect,inner)?;}self.record_outer(effect,reply)}}}
}
pub fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native receiving original type, field or phase is invalid")}
pub fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
pub fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
pub fn nested(mut d:RetirementDemand,copy:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(copy).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
pub fn child_grant(g:RetainedCloneGrant,copy:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-copy,maximum_depth:g.maximum_depth-1,..g}}
pub fn add_receipt(mut p:RetainedCloneProgress,copy:usize)->Result<RetainedCloneProgress,ValueError>{p.copied_bytes=p.copied_bytes.checked_add(copy).ok_or_else(invalid)?;Ok(p)}
macro_rules! scalar_source_methods{($read:expr)=>{
 fn query_scalar<'reply>(&self,query:NativeScalarQuery,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'reply,NativeSourceScalar,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'reply,NativeSourceScalar,ValueError>,ValueError>{if effect.maximum_work_units()==0{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar query lacks its original work grant"))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar query exceeded its original work authority"))))}let result=($read)(self,query);Ok(reply.complete(result))}
 fn recover_scalar<'reply>(&self,_query:NativeScalarQuery,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'reply,NativeSourceScalar,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'reply,NativeSourceScalar,ValueError>,ValueError>{Ok(reply.pending())}
};}
impl NativeValueSource for DslValue{scalar_source_methods!(|source:&Self,query:NativeScalarQuery|->Result<NativeSourceScalar,ValueError>{match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(match source{Self::Null=>Some(NativeTreeKind::Null),Self::Bool(_)=>Some(NativeTreeKind::Bool),Self::Number(_)=>Some(NativeTreeKind::Number),Self::String(_)=>Some(NativeTreeKind::Text),Self::Array(_)=>Some(NativeTreeKind::Array),Self::Object(_)=>Some(NativeTreeKind::Object),_=>None})),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(matches!(source,Self::Null))),NativeScalarQuery::Boolean=>match source{Self::Bool(value)=>Ok(NativeSourceScalar::Boolean(*value)),_=>Err(invalid())},NativeScalarQuery::Number=>match source{Self::Number(value)=>Ok(NativeSourceScalar::Number(*value)),_=>Err(invalid())},NativeScalarQuery::RecordLength=>match source{Self::Object(value)=>Ok(NativeSourceScalar::Length(value.len())),_=>Err(invalid())},NativeScalarQuery::SequenceLength=>match source{Self::Array(value)=>Ok(NativeSourceScalar::Length(value.len())),_=>Err(invalid())},_=>Err(invalid())}});
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{match self{Self::Null=>Some(NativeTreeKind::Null),Self::Bool(_)=>Some(NativeTreeKind::Bool),Self::Number(_)=>Some(NativeTreeKind::Number),Self::String(_)=>Some(NativeTreeKind::Text),Self::Array(_)=>Some(NativeTreeKind::Array),Self::Object(_)=>Some(NativeTreeKind::Object),_=>None}}
 fn is_null(&self)->bool{matches!(self,Self::Null)}
 fn boolean(&self)->Result<bool,ValueError>{match self{Self::Bool(value)=>Ok(*value),_=>Err(invalid())}}
 fn number(&self)->Result<Number,ValueError>{match self{Self::Number(value)=>Ok(*value),_=>Err(invalid())}}
 fn text(&self)->Result<&String,ValueError>{match self{Self::String(value)=>Ok(value),_=>Err(invalid())}}
 fn record_len(&self)->Result<usize,ValueError>{match self{Self::Object(entries)=>Ok(entries.len()),_=>Err(invalid())}}
 fn record_key(&self,index:usize)->Result<crate::record::NativeRecordKey<'_>,ValueError>{match self{Self::Object(entries)=>entries.get(index).map(|(key,_)|crate::record::NativeRecordKey::Name(key.as_str())).ok_or_else(invalid),_=>Err(invalid())}}
 fn record_entry(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match self{Self::Object(entries)=>entries.get(index).map(|entry|entry as &dyn NativeValueSource).ok_or_else(invalid),_=>Err(invalid())}}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match self{Self::Object(entries)=>entries.get(index).map(|(_,value)|value as &dyn NativeValueSource).ok_or_else(invalid),_=>Err(invalid())}}
 fn sequence_len(&self)->Result<usize,ValueError>{match self{Self::Array(entries)=>Ok(entries.len()),_=>Err(invalid())}}
 fn sequence_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match self{Self::Array(entries)=>entries.get(index).map(|value|value as &dyn NativeValueSource).ok_or_else(invalid),_=>Err(invalid())}}
}
impl NativeValueSource for String{scalar_source_methods!(|source:&Self,query:NativeScalarQuery|->Result<NativeSourceScalar,ValueError>{match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Text))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),_=>Err(invalid())}});fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Text)}fn text(&self)->Result<&String,ValueError>{Ok(self)}}
impl NativeValueSource for bool{scalar_source_methods!(|source:&Self,query:NativeScalarQuery|->Result<NativeSourceScalar,ValueError>{match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Bool))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::Boolean=>Ok(NativeSourceScalar::Boolean(*source)),_=>Err(invalid())}});fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Bool)}fn boolean(&self)->Result<bool,ValueError>{Ok(*self)}}
impl NativeValueSource for u8{scalar_source_methods!(|source:&Self,query:NativeScalarQuery|->Result<NativeSourceScalar,ValueError>{match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Number))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::Number=>Ok(NativeSourceScalar::Number(Number::UInt(u64::from(*source)))),_=>Err(invalid())}});fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Number)}fn number(&self)->Result<Number,ValueError>{Ok(Number::UInt(u64::from(*self)))}}
impl NativeValueSource for Number{scalar_source_methods!(|source:&Self,query:NativeScalarQuery|->Result<NativeSourceScalar,ValueError>{match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Number))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::Number=>Ok(NativeSourceScalar::Number(*source)),_=>Err(invalid())}});fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Number)}fn number(&self)->Result<Number,ValueError>{Ok(*self)}}
impl<T:NativeValueSource> NativeValueSource for (String,T){scalar_source_methods!(|source:&Self,query:NativeScalarQuery|->Result<NativeSourceScalar,ValueError>{match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Array))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::SequenceLength=>Ok(NativeSourceScalar::Length(2)),_=>Err(invalid())}});fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Array)}fn variant_tag(&self)->Option<&str>{Some(&self.0)}fn variant_payload(&self)->Result<&dyn NativeValueSource,ValueError>{Ok(&self.1)}fn sequence_len(&self)->Result<usize,ValueError>{Ok(2)}fn sequence_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match index{0=>Ok(&self.0),1=>Ok(&self.1),_=>Err(invalid())}}}
impl<T:NativeValueSource> NativeValueSource for Vec<T>{scalar_source_methods!(|source:&Self,query:NativeScalarQuery|->Result<NativeSourceScalar,ValueError>{match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Array))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::SequenceLength=>Ok(NativeSourceScalar::Length(source.len())),_=>Err(invalid())}});fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Array)}fn sequence_len(&self)->Result<usize,ValueError>{Ok(self.len())}fn sequence_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{self.get(index).map(|value|value as &dyn NativeValueSource).ok_or_else(invalid)}}

#[cfg(test)]
#[path="🧪️tests/scalar-query.rs"]
mod scalar_query_tests;

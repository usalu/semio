//! 🍃️ A record of scalar leaves is received straight from its ordinal-keyed original: no leaf slot, projection alias or key matcher is ever born.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::owned::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedCloneSource,retained_clone_grant_admits};
pub use semio_framework_value_retirement::retirement::RetireOwned;
pub use semio_framework_value_derive::{RetireOwned,NativeReceive};
#[path="../../../../../🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
use crate::native_decoding::{OriginalEffectReply,OriginalEffectReplyComplete};
use crate::owned::tree::{NativeScalarQuery,NativeSourceScalar,NativeTreeKind,NativeValueSource};
use crate::owned::NativeRecordKey;
const BODY:usize=512;
type Reply<'r>=OriginalEffectReply<'r,NativeSourceScalar,ValueError>;
type Done<'r>=Result<OriginalEffectReplyComplete<'r,NativeSourceScalar,ValueError>,ValueError>;
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"test source answers only its own scalar queries")}
fn record<'r>(answer:Result<NativeSourceScalar,ValueError>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Done<'r>{if effect.maximum_work_units()==0{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar query lacks its original work grant"))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar query exceeded its original work authority"))))}Ok(reply.complete(answer))}
#[derive(crate::RetireOwned)]#[value(crate="crate",native_owned="crate")]struct Num(f64);
#[derive(crate::RetireOwned)]#[value(crate="crate",native_owned="crate")]struct Flag(bool);
#[derive(crate::RetireOwned)]#[value(crate="crate",native_owned="crate")]struct PointSource{x:Num,y:Num,on:Flag}
impl NativeValueSource for Num{
 fn query_scalar<'r>(&self,query:NativeScalarQuery,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Done<'r>{record(match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Number))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::Number=>Ok(NativeSourceScalar::Number(Number::Float(self.0))),_=>Err(refused())},effect,reply)}
 fn recover_scalar<'r>(&self,_:NativeScalarQuery,_:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Done<'r>{Ok(reply.pending())}
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Number)}
 fn number(&self)->Result<Number,ValueError>{Ok(Number::Float(self.0))}
}
impl NativeValueSource for Flag{
 fn query_scalar<'r>(&self,query:NativeScalarQuery,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Done<'r>{record(match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Bool))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::Boolean=>Ok(NativeSourceScalar::Boolean(self.0)),_=>Err(refused())},effect,reply)}
 fn recover_scalar<'r>(&self,_:NativeScalarQuery,_:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Done<'r>{Ok(reply.pending())}
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Bool)}
 fn boolean(&self)->Result<bool,ValueError>{Ok(self.0)}
}
impl NativeValueSource for PointSource{
 fn query_scalar<'r>(&self,query:NativeScalarQuery,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Done<'r>{record(match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Object))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::RecordLength=>Ok(NativeSourceScalar::Length(3)),_=>Err(refused())},effect,reply)}
 fn recover_scalar<'r>(&self,_:NativeScalarQuery,_:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Done<'r>{Ok(reply.pending())}
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Object)}
 fn record_len(&self)->Result<usize,ValueError>{Ok(3)}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{if index<3{Ok(NativeRecordKey::Ordinal(index as u16))}else{Err(refused())}}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match index{0=>Ok(&self.x),1=>Ok(&self.y),2=>Ok(&self.on),_=>Err(refused())}}
}
#[derive(Debug,PartialEq,crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate")]struct Point{x:f64,y:f64,on:bool}
fn source()->PointSource{PointSource{x:Num(1.5),y:Num(-2.),on:Flag(true)}}
fn finish(mut receiver:<Point as NativeReceive<PointSource>>::Receiver,bank:&mut semio_framework_value_testing_admission::EffectBank){driver::close(&mut receiver,bank,BODY);assert!(receiver.terminal_is_empty());}
#[test]fn ordinal_scalar_record_receives_without_a_slot_alias_or_matcher_per_field(){
 let mut bank=driver::bank();let original=driver::admit(&mut bank,source());
 let mut receiver=<Point as NativeReceive<PointSource>>::receiver();
 assert!(std::mem::size_of_val(&receiver)<8192,"a scalar leaf record owns no per-field receiving slot: {}",std::mem::size_of_val(&receiver));
 let mut trace=driver::Trace::default();
 let point=driver::receive_traced(&mut receiver,original.borrow(),&mut bank,BODY,None,None,&mut trace).unwrap().unwrap();
 assert_eq!(point,Point{x:1.5,y:-2.,on:true});
 assert!(trace.advance.len()<=40,"three ordinal scalar leaves need one funded action per key, read and placement: {}",trace.advance.len());
 finish(receiver,&mut bank);driver::retire(point,&mut bank);driver::close_source(original,&mut bank);
}
#[test]fn ordinal_scalar_record_closes_every_cancelled_prefix(){
 for cut in 0..40{
  let mut bank=driver::bank();let original=driver::admit(&mut bank,source());let mut receiver=<Point as NativeReceive<PointSource>>::receiver();
  assert!(driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,Some(cut)).unwrap().is_none(),"a cut receive publishes nothing");
  finish(receiver,&mut bank);driver::close_source(original,&mut bank);
 }
}
#[derive(crate::RetireOwned)]#[value(crate="crate",native_owned="crate")]struct PairSource{a:PointSource,b:PointSource}
impl NativeValueSource for PairSource{
 fn query_scalar<'r>(&self,query:NativeScalarQuery,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Done<'r>{record(match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Object))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::RecordLength=>Ok(NativeSourceScalar::Length(2)),_=>Err(refused())},effect,reply)}
 fn recover_scalar<'r>(&self,_:NativeScalarQuery,_:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Done<'r>{Ok(reply.pending())}
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Object)}
 fn record_len(&self)->Result<usize,ValueError>{Ok(2)}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{if index<2{Ok(NativeRecordKey::Ordinal(index as u16))}else{Err(refused())}}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match index{0=>Ok(&self.a),1=>Ok(&self.b),_=>Err(refused())}}
}
#[derive(Debug,PartialEq,crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate")]struct Pair{a:Point,b:Point}
/// 🔁️ Two same-typed nested fields reuse one receiving slot: the first field's closed slot is legitimately vacant for the second.
#[test]fn two_same_typed_nested_fields_share_one_receiving_slot_and_both_arrive(){
 let mut bank=driver::bank();
 let original=driver::admit(&mut bank,PairSource{a:source(),b:PointSource{x:Num(-7.),y:Num(0.25),on:Flag(false)}});
 let mut receiver=<Pair as NativeReceive<PairSource>>::receiver();
 let pair=driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,None).unwrap().unwrap();
 assert_eq!(pair,Pair{a:Point{x:1.5,y:-2.,on:true},b:Point{x:-7.,y:0.25,on:false}});
 driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());
 driver::retire(pair,&mut bank);driver::close_source(original,&mut bank);
}

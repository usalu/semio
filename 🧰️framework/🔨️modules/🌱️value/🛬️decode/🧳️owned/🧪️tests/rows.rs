//! 🌲️ Forest-shaped rows with ordinal keys receive and retire within a per-field step budget: text and scalar fields are leaf reads, so a row costs O(fields) funded steps.
use super::*;
use crate::driver;
type Reply<'r>=crate::native_decoding::OriginalEffectReply<'r,crate::owned::tree::NativeSourceScalar,ValueError>;
type Done<'r>=Result<crate::native_decoding::OriginalEffectReplyComplete<'r,crate::owned::tree::NativeSourceScalar,ValueError>,ValueError>;
use crate::owned::tree::{NativeScalarQuery,NativeSourceScalar,NativeTreeKind,NativeValueSource};
use crate::owned::NativeRecordKey;
#[derive(crate::RetireOwned)]#[value(crate="crate",native_owned="crate")]pub enum Ord{Bool(bool),Num(f64),Text(String),Rec(Vec<(u16,Ord)>),Seq(Vec<Ord>)}
fn bad()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"ordinal fixture source refuses this query")}
impl NativeValueSource for Ord{
 fn query_scalar<'r>(&self,query:NativeScalarQuery,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Done<'r>{
  if effect.maximum_work_units()==0{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar query lacks its original work grant"))))}
  if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar query exceeded its original work authority"))))}
  Ok(reply.complete(match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(self.tree_kind())),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::Boolean=>self.boolean().map(NativeSourceScalar::Boolean),NativeScalarQuery::Number=>self.number().map(NativeSourceScalar::Number),NativeScalarQuery::RecordLength=>self.record_len().map(NativeSourceScalar::Length),NativeScalarQuery::SequenceLength=>self.sequence_len().map(NativeSourceScalar::Length),_=>Err(bad())}))
 }
 fn recover_scalar<'r>(&self,_:NativeScalarQuery,_:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Done<'r>{Ok(reply.pending())}
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{Some(match self{Ord::Bool(_)=>NativeTreeKind::Bool,Ord::Num(_)=>NativeTreeKind::Number,Ord::Text(_)=>NativeTreeKind::Text,Ord::Rec(_)=>NativeTreeKind::Object,Ord::Seq(_)=>NativeTreeKind::Array})}
 fn boolean(&self)->Result<bool,ValueError>{match self{Ord::Bool(v)=>Ok(*v),_=>Err(bad())}}
 fn number(&self)->Result<Number,ValueError>{match self{Ord::Num(v)=>Ok(Number::Float(*v)),_=>Err(bad())}}
 fn text(&self)->Result<&String,ValueError>{match self{Ord::Text(v)=>Ok(v),_=>Err(bad())}}
 fn record_len(&self)->Result<usize,ValueError>{match self{Ord::Rec(v)=>Ok(v.len()),_=>Err(bad())}}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{match self{Ord::Rec(v)=>v.get(index).map(|(k,_)|NativeRecordKey::Ordinal(*k)).ok_or_else(bad),_=>Err(bad())}}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match self{Ord::Rec(v)=>v.get(index).map(|(_,c)|c as &dyn NativeValueSource).ok_or_else(bad),_=>Err(bad())}}
 fn sequence_len(&self)->Result<usize,ValueError>{match self{Ord::Seq(v)=>Ok(v.len()),_=>Err(bad())}}
 fn sequence_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match self{Ord::Seq(v)=>v.get(index).map(|c|c as &dyn NativeValueSource).ok_or_else(bad),_=>Err(bad())}}
}

#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate")]struct Pt{x:f64,y:f64,z:f64}
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate",rename_all="camelCase")]struct Vertex{id:String,point:Pt,tol:f64}
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate",rename_all="camelCase")]struct LoopEdge{edge:String,orientation:bool}
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate",rename_all="camelCase")]struct Coedge{id:String,edge:String,forward:bool,loop_id:String,next:String,prev:String}
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate",rename_all="camelCase")]struct CoedgeFull{id:String,edge:String,forward:bool,#[value(default)]pcurve:Option<Pt>,#[value(default)]prange:(f64,f64),loop_id:String}
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate",rename_all="camelCase")]struct Loop{id:String,#[value(default)]edges:Vec<LoopEdge>}
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate",rename_all="camelCase")]struct Face{id:String,outer:String,#[value(default)]inner:Vec<String>}
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate")]struct Nums{#[value(default)]knots:Vec<f64>,#[value(default)]points:Vec<Pt>}
fn t(v:&str)->Ord{Ord::Text(v.into())}
fn steps<T:NativeReceive<Ord>+RetireOwned>(value:Ord)->(u64,usize){
 let mut bank=bank();let source=driver::admit(&mut bank,value);let mut receiver=T::receiver();let start=bank.wallet().remaining().maximum_copy_bytes;let mut steps=0u64;
 while !receiver.output_ready(){driver::none(driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,Some(1))).unwrap();steps+=1;assert!(steps<10_000)}
 let row=driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,None).unwrap().expect("publishes");driver::retire(row,&mut bank);
 steps+=driver::close_traced::<Ord,T,_>(&mut receiver,&mut bank,BODY).len() as u64;let copied=start-bank.wallet().remaining().maximum_copy_bytes;
 drop(receiver);driver::close_source(source,&mut bank);(steps,copied)
}
/// 🔨️ LAW: a forest row of text, scalar and nested leaves costs a handful of funded steps per field and kilobytes, never hundreds of steps: text fields are leaf reads, absent pure defaults are one inline step, a finished record closes as one box. The pinned budget is shared with the TS twin.
#[test]fn forest_text_rows_cost_steps_proportional_to_their_fields(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/📐️.json")).unwrap();let budget=&law["forestRows"];
 let point=||Ord::Rec(vec![(0,Ord::Num(1.0)),(1,Ord::Num(2.0)),(2,Ord::Num(3.0))]);
 let cases:[(&str,(u64,usize));6]=[
  ("coedge",steps::<Coedge>(Ord::Rec(vec![(0,t("coedge-1")),(1,t("edge-2")),(2,Ord::Bool(true)),(3,t("loop-3")),(4,t("coedge-4")),(5,t("coedge-5"))]))),
  ("loopEdge",steps::<LoopEdge>(Ord::Rec(vec![(0,t("edge-17")),(1,Ord::Bool(true))]))),
  ("vertex",steps::<Vertex>(Ord::Rec(vec![(0,t("vertex-12")),(1,point()),(2,Ord::Num(0.001))]))),
  ("coedgeAbsentDefaults",steps::<CoedgeFull>(Ord::Rec(vec![(0,t("c")),(1,t("e")),(2,Ord::Bool(true)),(5,t("l"))]))),
  ("loopOfFourEdges",steps::<Loop>(Ord::Rec(vec![(0,t("loop-1")),(1,Ord::Seq((0..4).map(|k|Ord::Rec(vec![(0,t(&format!("edge-{k}"))),(1,Ord::Bool(true))])).collect()))]))),
  ("faceOfThreeInnerLoops",steps::<Face>(Ord::Rec(vec![(0,t("f")),(1,t("o")),(2,Ord::Seq((0..3).map(|k|t(&format!("loop-{k}"))).collect()))]))),
 ];
 for(name,(count,bytes)) in cases{assert!(count<=budget[name]["steps"].as_u64().unwrap(),"{name} {count} steps");assert!(bytes as u64<=budget[name]["copyBytes"].as_u64().unwrap(),"{name} {bytes} bytes")}
}

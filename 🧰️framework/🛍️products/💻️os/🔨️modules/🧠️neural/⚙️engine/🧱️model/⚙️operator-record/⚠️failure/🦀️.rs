//! 🪢️ Original evaluation failures lend their complete typed fields to the prepaid job-domain boundary.
use super::{EvalError,OperatorWorkFailure};
use semio_framework_job::{OriginalDomainFailure,OriginalDomainProjection,OriginalDomainView as V,project_original_turn_failure,key_original_turn_failure,project_original_value_error,key_original_value_error};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress,Number};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"operator failure projection ordinal changed")}
#[derive(Clone,Copy)]
enum Node<'a>{Work(&'a OperatorWorkFailure),Eval(&'a EvalError),Progress(RetainedCloneProgress),Value(&'a ValueError),Text(&'a str),UInt(u64)}
impl<'a> Node<'a>{
 fn key(self,index:usize)->Result<&'a str,ValueError>{match self{
  Self::Work(OperatorWorkFailure::Evaluation{receipt_refusal,..})=>["primary","retained","receiptRefusal"].get(index).copied().filter(|_|index<2||receipt_refusal.is_some()).ok_or_else(invalid),
  Self::Work(OperatorWorkFailure::Original(f))=>key_original_turn_failure(f,&[],index),
  Self::Eval(e)=>e.projection_key(&[],index),
  Self::Progress(_)=>["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"].get(index).copied().ok_or_else(invalid),
  Self::Value(v)=>key_original_value_error(v,&[],index),_=>Err(invalid())
 }}
 fn child(self,index:usize)->Result<Self,ValueError>{Ok(match self{
  Self::Work(OperatorWorkFailure::Evaluation{primary,retained,receipt_refusal})=>match index{0=>Self::Eval(primary),1=>Self::Progress(*retained),2=>Self::Value(receipt_refusal.as_ref().ok_or_else(invalid)?),_=>return Err(invalid())},
  Self::Progress(p)=>Self::UInt(*[p.copied_items,p.copied_bytes,p.retained_capacity_bytes,p.released_bytes].get(index).ok_or_else(invalid)? as u64),_=>return Err(invalid())
 })}
 fn view(self)->Result<V<'a>,ValueError>{Ok(match self{
  Self::Work(OperatorWorkFailure::Evaluation{receipt_refusal,..})=>V::IntrinsicObject(2+usize::from(receipt_refusal.is_some())),
  Self::Work(OperatorWorkFailure::Original(f))=>return project_original_turn_failure(f,&[]),
  Self::Eval(e)=>return e.projection_view(&[]),Self::Progress(_)=>V::IntrinsicObject(4),Self::Value(v)=>return project_original_value_error(v,&[]),Self::Text(s)=>V::IntrinsicText(s),Self::UInt(n)=>V::IntrinsicNumber(Number::UInt(n))
 })}
}
fn locate<'a>(mut node:Node<'a>,path:&[usize])->Result<(Node<'a>,usize),ValueError>{for(i,&index)in path.iter().enumerate(){if matches!(node,Node::Value(_)|Node::Eval(_)|Node::Work(OperatorWorkFailure::Original(_))){return Ok((node,i))}node=node.child(index)?}Ok((node,path.len()))}
impl OriginalDomainProjection for OperatorWorkFailure{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{let(n,p)=locate(Node::Work(self),path)?;match n{Node::Eval(e)=>e.projection_view(&path[p..]),Node::Value(v)=>project_original_value_error(v,&path[p..]),Node::Work(Self::Original(f))=>project_original_turn_failure(f,&path[p..]),n=>n.view()}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{let(n,p)=locate(Node::Work(self),path)?;match n{Node::Eval(e)=>e.projection_key(&path[p..],index),Node::Value(v)=>key_original_value_error(v,&path[p..],index),Node::Work(Self::Original(f))=>key_original_turn_failure(f,&path[p..],index),n=>n.key(index)}}
}
impl OriginalDomainFailure for OperatorWorkFailure{
 fn domain_code(&self)->&str{"neural.operator-evaluation"}
 fn domain_message(&self)->&str{match self{Self::Original(f)=>f.primary.message(),Self::Evaluation{primary,..}=>primary.domain_message()}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{self.retained_progress()}
}

fn absent()->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original neural failure projection path is absent")}
impl EvalError{
 fn original_variant(&self)->&'static str{match self{Self::Retained(_)=>"retained",Self::UnknownKind(_)=>"unknownKind",Self::MissingInput(_)=>"missingInput",Self::InvalidInput(_)=>"invalidInput",Self::CardinalityViolation(_)=>"cardinalityViolation",Self::HeterogeneousList(_)=>"heterogeneousList",Self::CycleDetected=>"cycleDetected",Self::PendingExtension{..}=>"pendingExtension"}}
 fn original_text(&self)->Option<&str>{match self{Self::UnknownKind(value)|Self::MissingInput(value)|Self::InvalidInput(value)|Self::CardinalityViolation(value)|Self::HeterogeneousList(value)=>Some(value),_=>None}}
}
impl OriginalDomainProjection for EvalError{
 fn projection_identity(&self)->usize{match self{Self::Retained(value)=>value.message.as_ptr()as usize,Self::PendingExtension{extension_id,..}=>extension_id.as_ptr()as usize,_=>self.original_text().map_or(self as*const _ as usize,|value|value.as_ptr()as usize)}}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{
  if path.is_empty(){return Ok(V::IntrinsicObject(match self{Self::CycleDetected=>1,Self::PendingExtension{..}=>4,_=>2}))}if path==[0]{return Ok(V::IntrinsicText(self.original_variant()))}
  if let Self::Retained(value)=self{if path.first()==Some(&1){return semio_framework_job::project_original_value_error(value,&path[1..])}}
  if path.len()!=1{return Err(absent())}if let Some(value)=self.original_text(){return if path[0]==1{Ok(V::IntrinsicText(value))}else{Err(absent())}}
  if let Self::PendingExtension{extension_id,operator_id,node_hash}=self{return match path[0]{1=>Ok(V::IntrinsicText(extension_id)),2=>Ok(V::IntrinsicText(operator_id)),3=>Ok(V::IntrinsicNumber(semio_framework_value::Number::UInt(*node_hash))),_=>Err(absent())}}Err(absent())
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
  if path.is_empty(){return match index{0=>Ok("variant"),1=>Ok(match self{Self::Retained(_)=>"retained",Self::PendingExtension{..}=>"extensionId",_ if self.original_text().is_some()=>"value",_=>return Err(absent())}),2 if matches!(self,Self::PendingExtension{..})=>Ok("operatorId"),3 if matches!(self,Self::PendingExtension{..})=>Ok("nodeHash"),_=>Err(absent())}}
  if let Self::Retained(value)=self{if path.first()==Some(&1){return semio_framework_job::key_original_value_error(value,&path[1..],index)}}Err(absent())
 }
}
impl OriginalDomainFailure for EvalError{
 fn domain_code(&self)->&str{match self{Self::Retained(_)=>"neural.retained",Self::UnknownKind(_)=>"neural.unknown-kind",Self::MissingInput(_)=>"neural.missing-input",Self::InvalidInput(_)=>"neural.invalid-input",Self::CardinalityViolation(_)=>"neural.cardinality-violation",Self::HeterogeneousList(_)=>"neural.heterogeneous-list",Self::CycleDetected=>"neural.cycle-detected",Self::PendingExtension{..}=>"neural.pending-extension"}}
 fn domain_message(&self)->&str{match self{Self::Retained(value)=>&value.message,Self::CycleDetected=>"cycle detected",Self::PendingExtension{extension_id,..}=>extension_id,_=>self.original_text().unwrap()}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{match self{Self::Retained(value)=>value.retained_progress(),_=>Default::default()}}
}

#[cfg(test)]
#[path="🧪️laws/🦀️.rs"]
mod laws;

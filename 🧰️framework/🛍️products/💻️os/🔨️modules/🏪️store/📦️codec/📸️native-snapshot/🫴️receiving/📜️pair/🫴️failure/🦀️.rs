//! 🧯️ Artifact pair failures lend every original History, JSON, receiving and receipt field.
use super::{OriginalArtifactPairFailure,OriginalArtifactPairCause,RetainedHistoryOpMetaFault};
use semio_framework_job_failure::{OriginalDomainFailure,OriginalDomainProjection,OriginalDomainView as V,project_original_value_error,key_original_value_error,project_original_receiving_primary,key_original_receiving_primary,project_original_retirement_failure,key_original_retirement_failure,project_original_json_failure,key_original_json_failure};
use semio_framework_pack_json::JsonError;
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress,Number,OriginalRetirementFailure};
use semio_framework_value_native_owned::OriginalReceivingPrimary;
#[derive(Clone,Copy)]enum Node<'a>{Pair(&'a OriginalArtifactPairFailure),Cause(&'a OriginalArtifactPairCause),History(&'a RetainedHistoryOpMetaFault),Json(&'a JsonError),Value(&'a ValueError),Receiving(&'a OriginalReceivingPrimary),Retirement(&'a OriginalRetirementFailure),Progress(RetainedCloneProgress),Text(&'a str),UInt(u64),Null}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original artifact pair failure projection ordinal changed")}
impl<'a> Node<'a>{
 fn keys(self)->Result<&'static[&'static str],ValueError>{Ok(match self{Self::Pair(_)=>&["cause","progress","receiptRefusal"],Self::Cause(_)|Self::History(_)=>&["kind","original"],Self::Progress(_)=>&["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"],_=>return Err(invalid())})}
 fn key(self,index:usize)->Result<&'a str,ValueError>{self.keys()?.get(index).copied().ok_or_else(invalid)}
 fn child(self,index:usize)->Result<Self,ValueError>{Ok(match self{
 Self::Pair(v)=>match index{0=>Self::Cause(&v.cause),1=>Self::Progress(v.progress),2=>v.receipt_refusal.as_ref().map_or(Self::Null,Self::Value),_=>return Err(invalid())},
 Self::Cause(v)=>match(index,v){(0,OriginalArtifactPairCause::Value(_))=>Self::Text("value"),(0,OriginalArtifactPairCause::History(_))=>Self::Text("history"),(0,OriginalArtifactPairCause::Native(_))=>Self::Text("native"),(1,OriginalArtifactPairCause::Value(v))=>Self::Value(v),(1,OriginalArtifactPairCause::History(v))=>Self::History(v),(1,OriginalArtifactPairCause::Native(v))=>Self::Receiving(v),_=>return Err(invalid())},
 Self::History(v)=>match(index,v){(0,RetainedHistoryOpMetaFault::Value(_))=>Self::Text("value"),(0,RetainedHistoryOpMetaFault::Json(_))=>Self::Text("json"),(0,RetainedHistoryOpMetaFault::Native(_))=>Self::Text("native"),(0,RetainedHistoryOpMetaFault::Retirement(_))=>Self::Text("retirement"),(1,RetainedHistoryOpMetaFault::Value(v))=>Self::Value(v),(1,RetainedHistoryOpMetaFault::Json(v))=>Self::Json(v),(1,RetainedHistoryOpMetaFault::Native(v))=>Self::Receiving(v),(1,RetainedHistoryOpMetaFault::Retirement(v))=>Self::Retirement(v),_=>return Err(invalid())},
 Self::Progress(v)=>Self::UInt(*[v.copied_items,v.copied_bytes,v.retained_capacity_bytes,v.released_bytes].get(index).ok_or_else(invalid)?as u64),_=>return Err(invalid())
 })}
 fn view(self)->Result<V<'a>,ValueError>{Ok(match self{Self::Null=>V::IntrinsicNull,Self::Text(v)=>V::IntrinsicText(v),Self::UInt(v)=>V::IntrinsicNumber(Number::UInt(v)),Self::Json(v)=>return project_original_json_failure(v,&[]),Self::Value(v)=>return project_original_value_error(v,&[]),Self::Receiving(v)=>return project_original_receiving_primary(v,&[]),Self::Retirement(v)=>return project_original_retirement_failure(v,&[]),_=>V::IntrinsicObject(self.keys()?.len())})}
}
fn locate<'a>(mut node:Node<'a>,path:&[usize])->Result<(Node<'a>,usize),ValueError>{for(position,&index)in path.iter().enumerate(){if matches!(node,Node::Json(_)|Node::Value(_)|Node::Receiving(_)|Node::Retirement(_)){return Ok((node,position))}node=node.child(index)?}Ok((node,path.len()))}
impl OriginalDomainProjection for OriginalArtifactPairFailure{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{let(node,position)=locate(Node::Pair(self),path)?;match node{Node::Json(v)=>project_original_json_failure(v,&path[position..]),Node::Value(v)=>project_original_value_error(v,&path[position..]),Node::Receiving(v)=>project_original_receiving_primary(v,&path[position..]),Node::Retirement(v)=>project_original_retirement_failure(v,&path[position..]),node=>node.view()}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{let(node,position)=locate(Node::Pair(self),path)?;match node{Node::Json(v)=>key_original_json_failure(v,&path[position..],index),Node::Value(v)=>key_original_value_error(v,&path[position..],index),Node::Receiving(v)=>key_original_receiving_primary(v,&path[position..],index),Node::Retirement(v)=>key_original_retirement_failure(v,&path[position..],index),node=>node.key(index)}}
}
impl OriginalDomainFailure for OriginalArtifactPairFailure{
 fn domain_code(&self)->&str{"original.artifactPair"}
 fn domain_message(&self)->&str{match &self.cause{OriginalArtifactPairCause::Value(v)=>&v.message,OriginalArtifactPairCause::Native(v)=>v.message(),OriginalArtifactPairCause::History(v)=>match v{RetainedHistoryOpMetaFault::Value(v)=>&v.message,RetainedHistoryOpMetaFault::Native(v)=>v.message(),RetainedHistoryOpMetaFault::Json(JsonError::Native(v))=>&v.message,RetainedHistoryOpMetaFault::Json(_)=>"original History JSON parsing refused",RetainedHistoryOpMetaFault::Retirement(v)=>{let mut node=Some(v);while let Some(v)=node{if let Some(error)=v.primary().or(v.work_refusal()).or(v.receipt_refusal()){return &error.message}node=v.child()}"original History retirement refused"}}}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{self.progress}
}

//! 📸️ Borrowed native failure paths retain exact independent owners and never construct a diagnostic mirror.
use super::{OriginalDomainFailure,OriginalDomainProjection,OriginalDomainView as V,OriginalReceivingPrimary,ValueError};
use semio_framework_io_control::{NativeSnapshotFailure,NativeSnapshotCause,NativeSnapshotAllocationFailure};
use semio_framework_value::{native_encoding::{NativeEncodeEffectFailure,NativeEncodeAllocationOutcome}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeAllocationOutcome;
use semio_framework_value_retained_clone::{Number,RetainedCloneProgress,ValueRefusalKind,NativeAllocationFailure,NativeAllocationProgress,NativeAllocationRefusal,OriginalRetirementFailure};
const SNAPSHOT:&[&str]=&["cause","allocation","encodeEffect","receiptRefusal","contextRefusal","workRefusal","settlementRefusal","progress"];
const RECEIVING:&[&str]=&["type","primary","quoteRefusal","allocationRefusal","workRefusal","retirementFailure","allocationFailure","nativeAllocation","receiptRefusal","progress"];
const RETIREMENT:&[&str]=&["primary","workRefusal","receiptRefusal","progressOverride","child"];
const ENCODE:&[&str]=&["type","request","admitted","primary","forwardedQuote","scopedQuote","extentRefusal","forwardedRefusal","scopedRefusal","actual"];
const DECODE:&[&str]=&["type","request","admitted","primary","quoteRefusal","extentRefusal","settlementRefusal","actual"];
const EFFECT:&[&str]=&["primary","invariant","physicalInvariant","capacityInvariant","work","local","physical","allocation"];
const ALLOCATION:&[&str]=&["primary","admission","forwardedAdmission","scopedAdmission","forwardedQuote","scopedQuote","forwardedPost","scopedPost","forwardedCancel","scopedCancel","receipt","extent","progress"];
const ALLOCATION_PROGRESS:&[&str]=&["priorAdmittedBytes","requestedBytes","maximumBytes","admittedBytes","actualCapacityBytes"];
const REQUEST:&[&str]=&["bytes","ownedBytes","nextOwnedBytes","maximumBytes"];
const PROGRESS:&[&str]=&["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"];
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original native failure projection ordinal changed")}
fn receiving_keys(r:&OriginalReceivingPrimary)->[Option<&'static str>;9]{[r.primary.as_ref().map(|_|"primary"),r.quote_refusal.as_ref().map(|_|"quoteRefusal"),r.allocation_refusal.as_ref().map(|_|"allocationRefusal"),r.work_refusal.as_ref().map(|_|"workRefusal"),r.retirement_failure.as_ref().map(|_|"retirementFailure"),r.allocation_failure.as_ref().map(|_|"allocationFailure"),r.native_allocation.as_ref().map(|_|"nativeAllocation"),r.receipt_refusal.as_ref().map(|_|"receiptRefusal"),Some("retainedProgress")]}
#[derive(Clone,Copy)]
enum Node<'a>{Snapshot(&'a NativeSnapshotFailure),Cause(&'a NativeSnapshotCause),SnapshotRetirement(&'a OriginalRetirementFailure),Receiving(&'a OriginalReceivingPrimary),Encode(&'a NativeEncodeAllocationOutcome),Decode(&'a NativeDecodeAllocationOutcome),Effect(&'a NativeEncodeEffectFailure),Allocation(&'a NativeAllocationFailure),AllocationProgress(&'a NativeAllocationProgress),Request([usize;4]),Progress(RetainedCloneProgress),Value(&'a ValueError),Retirement(&'a OriginalRetirementFailure),Null,Text(&'a str),Bool(bool),UInt(usize)}
impl<'a> Node<'a>{
 fn value(value:Option<&'a ValueError>)->Self{value.map_or(Self::Null,Self::Value)}
 fn refusal(value:Option<&'a NativeAllocationRefusal>)->Self{Self::value(value.and_then(NativeAllocationRefusal::owned))}
 fn uint(value:Option<usize>)->Self{value.map_or(Self::Null,Self::UInt)}
 fn allocation(value:&'a NativeSnapshotAllocationFailure)->Self{match value{NativeSnapshotAllocationFailure::Encode(value)=>Self::Encode(value),NativeSnapshotAllocationFailure::Decode(value)=>Self::Decode(value)}}
 fn keys(self)->Result<&'static[&'static str],ValueError>{Ok(match self{Self::Snapshot(_)=>SNAPSHOT,Self::Cause(NativeSnapshotCause::Receiving(_))=>RECEIVING,Self::Cause(_)=>&["type","value"],Self::SnapshotRetirement(_)=>RETIREMENT,Self::Encode(_)=>ENCODE,Self::Decode(_)=>DECODE,Self::Effect(_)=>EFFECT,Self::Allocation(_)=>ALLOCATION,Self::AllocationProgress(_)=>ALLOCATION_PROGRESS,Self::Request(_)=>REQUEST,Self::Progress(_)=>PROGRESS,_=>return Err(invalid())})}
 fn key(self,index:usize)->Result<&'a str,ValueError>{if let Self::Receiving(r)=self{return receiving_keys(r).into_iter().flatten().nth(index).ok_or_else(invalid)}self.keys()?.get(index).copied().ok_or_else(invalid)}
 fn child(self,index:usize)->Result<Self,ValueError>{Ok(match self{
 Self::Snapshot(s)=>{let f=s.original_fields();match self.key(index)?{"cause"=>f.cause.map_or(Self::Null,Self::Cause),"allocation"=>f.allocation.map_or(Self::Null,Self::allocation),"encodeEffect"=>f.encode_effect.map_or(Self::Null,Self::Effect),"receiptRefusal"=>Self::value(f.receipt_refusal),"contextRefusal"=>Self::value(f.context_refusal),"workRefusal"=>Self::value(f.work_refusal),"settlementRefusal"=>Self::value(f.settlement_refusal),"progress"=>Self::Progress(*f.progress),_=>return Err(invalid())}},
 Self::Cause(NativeSnapshotCause::Receiving(r))=>if index==0{Self::Text("receiving")}else{match self.key(index)?{"primary"=>Self::value(r.primary.as_ref()),"quoteRefusal"=>Self::value(r.quote_refusal.as_ref()),"allocationRefusal"=>Self::value(r.allocation_refusal.as_ref()),"workRefusal"=>Self::value(r.work_refusal.as_ref()),"retirementFailure"=>r.retirement_failure.as_ref().map_or(Self::Null,Self::SnapshotRetirement),"allocationFailure"=>r.allocation_failure.as_ref().map_or(Self::Null,Self::Allocation),"nativeAllocation"=>r.native_allocation.as_ref().map_or(Self::Null,Self::Decode),"receiptRefusal"=>Self::value(r.receipt_refusal.as_ref()),"progress"=>Self::Progress(r.progress),_=>return Err(invalid())}},
 Self::SnapshotRetirement(r)=>match index{0=>Self::value(r.primary()),1=>Self::value(r.work_refusal()),2=>Self::value(r.receipt_refusal()),3=>r.progress_override().map_or(Self::Null,Self::Progress),4=>r.child().map_or(Self::Null,Self::SnapshotRetirement),_=>return Err(invalid())},
 Self::Cause(c)=>match(index,c){(0,NativeSnapshotCause::Value(_))=>Self::Text("value"),(0,NativeSnapshotCause::Retirement(_))=>Self::Text("retirement"),(1,NativeSnapshotCause::Value(v))=>Self::Value(v),(1,NativeSnapshotCause::Retirement(v))=>Self::SnapshotRetirement(v),_=>return Err(invalid())},
 Self::Receiving(r)=>match self.key(index)?{"primary"=>Self::value(r.primary.as_ref()),"quoteRefusal"=>Self::value(r.quote_refusal.as_ref()),"allocationRefusal"=>Self::value(r.allocation_refusal.as_ref()),"workRefusal"=>Self::value(r.work_refusal.as_ref()),"retirementFailure"=>r.retirement_failure.as_ref().map_or(Self::Null,Self::Retirement),"allocationFailure"=>r.allocation_failure.as_ref().map_or(Self::Null,Self::Allocation),"nativeAllocation"=>r.native_allocation.as_ref().map_or(Self::Null,Self::Decode),"receiptRefusal"=>Self::value(r.receipt_refusal.as_ref()),"retainedProgress"=>Self::Progress(r.progress),_=>return Err(invalid())},
 Self::Encode(v)=>match self.key(index)?{"type"=>Self::Text("encode"),"request"=>v.request.map_or(Self::Null,|r|Self::Request([r.bytes,r.owned_bytes,r.next_owned_bytes,r.maximum_bytes])),"admitted"=>Self::Bool(v.admitted),"primary"=>Self::value(v.primary.as_ref()),"forwardedQuote"=>Self::refusal(v.forwarded_quote.as_ref()),"scopedQuote"=>Self::refusal(v.scoped_quote.as_ref()),"extentRefusal"=>Self::value(v.extent_refusal.as_ref()),"forwardedRefusal"=>Self::refusal(v.forwarded_refusal.as_ref()),"scopedRefusal"=>Self::refusal(v.scoped_refusal.as_ref()),"actual"=>Self::uint(v.actual),_=>return Err(invalid())},
 Self::Decode(v)=>match self.key(index)?{"type"=>Self::Text("decode"),"request"=>v.request.map_or(Self::Null,|r|Self::Request([r.bytes,r.owned_bytes,r.next_owned_bytes,r.maximum_bytes])),"admitted"=>Self::Bool(v.admitted),"primary"=>Self::value(v.primary.as_ref()),"quoteRefusal"=>Self::refusal(v.quote_refusal.as_ref()),"extentRefusal"=>Self::value(v.extent_refusal.as_ref()),"settlementRefusal"=>Self::refusal(v.settlement_refusal.as_ref()),"actual"=>Self::uint(v.actual),_=>return Err(invalid())},
 Self::Effect(v)=>{if index<7{Self::value([v.primary.as_ref(),v.invariant.as_ref(),v.physical_invariant.as_ref(),v.capacity_invariant.as_ref(),v.work.as_ref(),v.local.as_ref(),v.physical.as_ref()][index])}else if index==7{v.allocation.as_ref().map_or(Self::Null,Self::Encode)}else{return Err(invalid())}},
 Self::Allocation(v)=>{if index<12{[Self::value(v.primary.as_ref()),Self::value(v.admission.as_ref()),Self::refusal(v.forwarded_admission.as_ref()),Self::refusal(v.scoped_admission.as_ref()),Self::refusal(v.forwarded_quote.as_ref()),Self::refusal(v.scoped_quote.as_ref()),Self::refusal(v.forwarded_post.as_ref()),Self::refusal(v.scoped_post.as_ref()),Self::refusal(v.forwarded_cancel.as_ref()),Self::refusal(v.scoped_cancel.as_ref()),Self::value(v.receipt.as_ref()),Self::value(v.extent.as_ref())][index]}else if index==12{Self::AllocationProgress(&v.progress)}else{return Err(invalid())}},
 Self::AllocationProgress(p)=>match index{0=>Self::uint(p.prior_admitted_bytes),1=>Self::uint(p.requested_bytes),2=>Self::uint(p.maximum_bytes),3=>Self::UInt(p.admitted_bytes),4=>Self::uint(p.actual_capacity_bytes),_=>return Err(invalid())},
 Self::Request(v)=>Self::UInt(*v.get(index).ok_or_else(invalid)?),
 Self::Progress(p)=>Self::UInt(*[p.copied_items,p.copied_bytes,p.retained_capacity_bytes,p.released_bytes].get(index).ok_or_else(invalid)?),
 _=>return Err(invalid())
 })}
 fn view(self)->Result<V<'a>,ValueError>{Ok(match self{Self::Value(v)=>return super::project_original_value_error(v,&[]),Self::Retirement(v)=>return super::project_original_retirement_failure(v,&[]),Self::Receiving(r)=>V::IntrinsicObject(receiving_keys(r).into_iter().flatten().count()),Self::Null=>V::IntrinsicNull,Self::Text(v)=>V::IntrinsicText(v),Self::Bool(v)=>V::IntrinsicBool(v),Self::UInt(v)=>V::IntrinsicNumber(Number::UInt(v as u64)),_=>V::IntrinsicObject(self.keys()?.len())})}
}
fn locate<'a>(mut node:Node<'a>,path:&[usize])->Result<(Node<'a>,usize),ValueError>{for(position,&index)in path.iter().enumerate(){if matches!(node,Node::Value(_)|Node::Retirement(_)){return Ok((node,position))}node=node.child(index)?}Ok((node,path.len()))}
fn view<'a>(node:Node<'a>,path:&[usize])->Result<V<'a>,ValueError>{let(node,position)=locate(node,path)?;match node{Node::Value(v)=>super::project_original_value_error(v,&path[position..]),Node::Retirement(v)=>super::project_original_retirement_failure(v,&path[position..]),node=>node.view()}}
fn key<'a>(node:Node<'a>,path:&[usize],index:usize)->Result<&'a str,ValueError>{let(node,position)=locate(node,path)?;match node{Node::Value(v)=>super::key_original_value_error(v,&path[position..],index),Node::Retirement(v)=>super::key_original_retirement_failure(v,&path[position..],index),node=>node.key(index)}}
/// 📸️ Owns the original native refusal through the domain interface without copying its cause tree.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
pub struct OriginalNativeSnapshotFailure{pub original:NativeSnapshotFailure}
impl From<NativeSnapshotFailure> for OriginalNativeSnapshotFailure{fn from(original:NativeSnapshotFailure)->Self{Self{original}}}
impl OriginalDomainFailure for OriginalNativeSnapshotFailure{
 fn domain_code(&self)->&str{self.original.kind().as_str()}
 fn domain_message(&self)->&str{self.original.message()}
 fn domain_retained_progress(&self)->RetainedCloneProgress{self.original.retained_progress()}
}
impl OriginalDomainProjection for OriginalNativeSnapshotFailure{
 fn projection_identity(&self)->usize{&self.original as*const NativeSnapshotFailure as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{project_original_native_snapshot_failure(&self.original,path)}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{key_original_native_snapshot_failure(&self.original,path,index)}
}
/// 🪞️ Lends every original snapshot cause, allocation outcome, effect and enclosing refusal.
pub fn project_original_native_snapshot_failure<'a>(error:&'a NativeSnapshotFailure,path:&[usize])->Result<V<'a>,ValueError>{view(Node::Snapshot(error),path)}
/// 🔑️ Lends a closed native failure field name at its original relative path.
pub fn key_original_native_snapshot_failure<'a>(error:&'a NativeSnapshotFailure,path:&[usize],index:usize)->Result<&'a str,ValueError>{key(Node::Snapshot(error),path,index)}
/// 🛬️ Lends the complete original receiving subtree including its independent retirement and allocation causes.
pub fn project_complete_original_receiving<'a>(error:&'a OriginalReceivingPrimary,path:&[usize])->Result<V<'a>,ValueError>{view(Node::Receiving(error),path)}
/// 🔑️ Lends every present receiving role without constructing a path or diagnostic owner.
pub fn key_complete_original_receiving<'a>(error:&'a OriginalReceivingPrimary,path:&[usize],index:usize)->Result<&'a str,ValueError>{key(Node::Receiving(error),path,index)}

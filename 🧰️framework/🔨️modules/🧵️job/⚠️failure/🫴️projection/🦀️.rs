//! 🫴️ Common borrowed failure projections retain every original field without constructing a diagnostic mirror.
use super::{OriginalDomainPrimary,OriginalDomainView as V,OriginalTurnFailure,OriginalTurnPrimary,ValueError};
use semio_framework_diagnostic::{Fault,FaultCause,FaultOrigin,FaultParams,FaultScope,FaultSource,Severity,TextSpan};
use semio_framework_value::{Number,RetainedCloneProgress,ValueRefusalKind};
#[derive(Clone,Copy)]
enum Node<'a>{Turn(&'a OriginalTurnFailure),Primary(&'a OriginalTurnPrimary),Value(&'a ValueError),Fault(&'a Fault),Domain(&'a OriginalDomainPrimary),Intrinsic(&'a semio_framework_value::DslValue),Progress(RetainedCloneProgress),Scope(&'a FaultScope),Source(FaultSource),Span(&'a TextSpan),Causes(&'a [FaultCause]),Cause(&'a FaultCause),Params(&'a FaultParams),Text(&'a str),Bool(bool),UInt(u64)}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original failure projection ordinal changed")}
fn fault_key(f:&Fault,index:usize)->Result<&'static str,ValueError>{if index<6{return Ok(["origin","code","severity","message","scope","retainedProgress"][index])}let mut position=6;for(present,key)in[(f.source.is_some(),"source"),(f.span.is_some(),"span"),(!f.causes.is_empty(),"causes"),(f.params.as_deref().is_some_and(|p|!p.0.is_empty()),"params"),(f.domain.is_some(),"domain")]{if present{if index==position{return Ok(key)}position+=1}}if index==position{Ok("retryable")}else{Err(invalid())}}
impl<'a> Node<'a>{
 fn key(self,index:usize)->Result<&'a str,ValueError>{match self{
  Self::Turn(f)=>if index==0{Ok("primary")}else{[f.receipt_refusal.as_ref().map(|_|"receiptRefusal"),f.context_refusal.as_ref().map(|_|"contextRefusal")].into_iter().flatten().nth(index-1).ok_or_else(invalid)},
  Self::Primary(_)=>["kind","original"].get(index).copied().ok_or_else(invalid),Self::Value(_)=>["kind","message","retainedProgress"].get(index).copied().ok_or_else(invalid),Self::Fault(f)=>fault_key(f,index),
  Self::Progress(_)=>["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"].get(index).copied().ok_or_else(invalid),
  Self::Scope(s)=>[(&s.plugin_id,"pluginId"),(&s.app_id,"appId"),(&s.instance_id,"instanceId"),(&s.module,"module"),(&s.body_key,"bodyKey")].into_iter().filter(|(v,_)|v.is_some()).nth(index).map(|(_,k)|k).ok_or_else(invalid),
  Self::Source(s)=>[(s.offset.is_some(),"offset"),(s.byte.is_some(),"byte"),(s.depth.is_some(),"depth")].into_iter().filter(|(present,_)|*present).nth(index).map(|(_,k)|k).ok_or_else(invalid),Self::Span(_)=>["line","column","length"].get(index).copied().ok_or_else(invalid),
  Self::Cause(c)=>if index==0{Ok("message")}else{[c.code.as_ref().map(|_|"code"),c.retained_progress.map(|_|"retainedProgress")].into_iter().flatten().nth(index-1).ok_or_else(invalid)},
  Self::Params(p)=>p.0.get(index).map(|(k,_)|k.as_str()).ok_or_else(invalid),_=>Err(invalid())
 }}
 fn child(self,index:usize)->Result<Self,ValueError>{Ok(match self{
  Self::Turn(f)=>if index==0{Self::Primary(&f.primary)}else{Self::Value([f.receipt_refusal.as_ref(),f.context_refusal.as_ref()].into_iter().flatten().nth(index-1).ok_or_else(invalid)?)},
  Self::Primary(p)=>match(index,p){(0,OriginalTurnPrimary::Value(_))=>Self::Text("value"),(0,OriginalTurnPrimary::Fault(_))=>Self::Text("fault"),(0,OriginalTurnPrimary::Domain(_))=>Self::Text("domain"),(1,OriginalTurnPrimary::Value(v))=>Self::Value(v),(1,OriginalTurnPrimary::Fault(v))=>Self::Fault(v),(1,OriginalTurnPrimary::Domain(v))=>Self::Domain(v),_=>return Err(invalid())},
  Self::Value(v)=>match index{0=>Self::Text(v.kind.as_str()),1=>Self::Text(&v.message),2=>Self::Progress(v.retained_progress()),_=>return Err(invalid())},
  Self::Fault(f)=>match fault_key(f,index)?{"origin"=>Self::Text(match f.origin{FaultOrigin::Edge=>"edge",FaultOrigin::Renderer=>"renderer",FaultOrigin::Os=>"os",FaultOrigin::Module=>"module",FaultOrigin::Plugin=>"plugin",FaultOrigin::App=>"app",FaultOrigin::Extension=>"extension",FaultOrigin::Framework=>"framework"}),"code"=>Self::Text(&f.code.0),"severity"=>Self::Text(match f.severity{Severity::Info=>"info",Severity::Warning=>"warning",Severity::Error=>"error",Severity::Fatal=>"fatal"}),"message"=>Self::Text(&f.message),"scope"=>Self::Scope(&f.scope),"retainedProgress"=>Self::Progress(f.retained_progress()),"source"=>Self::Source(f.source.ok_or_else(invalid)?),"span"=>Self::Span(f.span.as_ref().ok_or_else(invalid)?),"causes"=>Self::Causes(&f.causes),"params"=>Self::Params(f.params.as_deref().ok_or_else(invalid)?),"domain"=>Self::Intrinsic(f.domain.as_ref().ok_or_else(invalid)?),"retryable"=>Self::Bool(f.retryable),_=>return Err(invalid())},
  Self::Progress(p)=>Self::UInt(*[p.copied_items,p.copied_bytes,p.retained_capacity_bytes,p.released_bytes].get(index).ok_or_else(invalid)? as u64),
  Self::Scope(s)=>Self::Text([&s.plugin_id,&s.app_id,&s.instance_id,&s.module,&s.body_key].into_iter().flatten().nth(index).ok_or_else(invalid)?),Self::Source(s)=>Self::UInt([s.offset,s.byte.map(u64::from),s.depth].into_iter().flatten().nth(index).ok_or_else(invalid)?),Self::Span(s)=>Self::UInt(u64::from(*[s.line,s.column,s.length].get(index).ok_or_else(invalid)?)),
  Self::Causes(c)=>Self::Cause(c.get(index).ok_or_else(invalid)?),Self::Cause(c)=>match self.key(index)?{"message"=>Self::Text(&c.message),"code"=>Self::Text(&c.code.as_ref().ok_or_else(invalid)?.0),"retainedProgress"=>Self::Progress(c.retained_progress.ok_or_else(invalid)?),_=>return Err(invalid())},Self::Params(p)=>Self::Text(&p.0.get(index).ok_or_else(invalid)?.1),_=>return Err(invalid())
 })}
 fn view(self)->Result<V<'a>,ValueError>{Ok(match self{Self::Turn(f)=>V::IntrinsicObject(1+usize::from(f.receipt_refusal.is_some())+usize::from(f.context_refusal.is_some())),Self::Primary(_)=>V::IntrinsicObject(2),Self::Value(_)=>V::IntrinsicObject(3),Self::Fault(f)=>V::IntrinsicObject(7+usize::from(f.source.is_some())+usize::from(f.span.is_some())+usize::from(!f.causes.is_empty())+usize::from(f.params.as_deref().is_some_and(|p|!p.0.is_empty()))+usize::from(f.domain.is_some())),Self::Progress(_)=>V::IntrinsicObject(4),Self::Scope(s)=>V::IntrinsicObject([&s.plugin_id,&s.app_id,&s.instance_id,&s.module,&s.body_key].into_iter().flatten().count()),Self::Source(s)=>V::IntrinsicObject(usize::from(s.offset.is_some())+usize::from(s.byte.is_some())+usize::from(s.depth.is_some())),Self::Span(_)=>V::IntrinsicObject(3),Self::Causes(c)=>V::IntrinsicArray(c.len()),Self::Cause(c)=>V::IntrinsicObject(1+usize::from(c.code.is_some())+usize::from(c.retained_progress.is_some())),Self::Params(p)=>V::IntrinsicObject(p.0.len()),Self::Text(t)=>V::IntrinsicText(t),Self::Bool(b)=>V::IntrinsicBool(b),Self::UInt(n)=>V::IntrinsicNumber(Number::UInt(n)),Self::Domain(d)=>return d.projection().projection_view(&[]),Self::Intrinsic(d)=>return intrinsic_view(d,&[])})}
}
fn locate<'a>(mut node:Node<'a>,path:&[usize])->Result<(Node<'a>,usize),ValueError>{for(position,&index)in path.iter().enumerate(){if matches!(node,Node::Domain(_)|Node::Intrinsic(_)){return Ok((node,position))}node=node.child(index)?}Ok((node,path.len()))}
fn view<'a>(node:Node<'a>,path:&[usize])->Result<V<'a>,ValueError>{let(node,position)=locate(node,path)?;match node{Node::Domain(d)=>d.projection().projection_view(&path[position..]),Node::Intrinsic(d)=>intrinsic_view(d,&path[position..]),node=>node.view()}}
fn key<'a>(node:Node<'a>,path:&[usize],index:usize)->Result<&'a str,ValueError>{let(node,position)=locate(node,path)?;match node{Node::Domain(d)=>d.projection().projection_key(&path[position..],index),Node::Intrinsic(d)=>super::OriginalDomainProjection::projection_key(d,&path[position..],index),node=>node.key(index)}}
/// 🪢️ Borrows primary, receipt and context owners including complete nested domain fields.
pub fn project_original_turn_failure<'a>(error:&'a OriginalTurnFailure,path:&[usize])->Result<V<'a>,ValueError>{view(Node::Turn(error),path)}
pub fn key_original_turn_failure<'a>(error:&'a OriginalTurnFailure,path:&[usize],index:usize)->Result<&'a str,ValueError>{key(Node::Turn(error),path,index)}
/// 🧯️ Borrows exact native refusal kind, original text and independent retained effects.
pub fn project_original_value_error<'a>(error:&'a ValueError,path:&[usize])->Result<V<'a>,ValueError>{view(Node::Value(error),path)}
pub fn key_original_value_error<'a>(error:&'a ValueError,path:&[usize],index:usize)->Result<&'a str,ValueError>{key(Node::Value(error),path,index)}

/// 🚨️ Borrows every original diagnostic field through the same canonical ordinal projection.
pub fn project_original_fault<'a>(error:&'a Fault,path:&[usize])->Result<V<'a>,ValueError>{view(Node::Fault(error),path)}
/// 🔑️ Borrows the original diagnostic key without allocating a representation.
pub fn key_original_fault<'a>(error:&'a Fault,path:&[usize],index:usize)->Result<&'a str,ValueError>{key(Node::Fault(error),path,index)}
impl super::OriginalDomainProjection for OriginalTurnFailure{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{project_original_turn_failure(self,path)}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{key_original_turn_failure(self,path,index)}
}

fn intrinsic_view<'a>(source:&'a semio_framework_value::DslValue,path:&[usize])->Result<V<'a>,ValueError>{super::OriginalDomainProjection::projection_view(source,path)}

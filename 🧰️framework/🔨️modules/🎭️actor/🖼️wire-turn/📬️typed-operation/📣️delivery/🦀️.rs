//! 📣️ Original page delivery retains a fixed wire header and its genuine immutable body projection.
use super::{TypedOperationResultPage,TypedOperationResultToken,TypedOperationResultLane,TYPED_OPERATION_RESULT_HEADER_BYTES};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;

/// 🧷️ A portable wire descriptor owns the exact receiver alias until funded closure before output.
pub struct ResultPageDelivery{source:Option<TypedOperationResultPage>,header:[u8;TYPED_OPERATION_RESULT_HEADER_BYTES],ready:bool,closing:bool}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original page delivery has no admitted wire descriptor")}
impl ResultPageDelivery{
 /// 🎟️ Quotes the bounded original slot transfer before any source custody changes.
 pub fn capture_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()*2+size_of::<Option<TypedOperationResultPage>>(),depth:1,..Default::default()}}
 pub fn capture(slot:&mut Option<TypedOperationResultPage>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{let d=Self::capture_demand();if !fits(d,grant){return Ok(None)}if slot.is_none(){return Err(invalid())}Ok(Some((Self{source:slot.take(),header:[0;TYPED_OPERATION_RESULT_HEADER_BYTES],ready:false,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
 pub fn ready(&self)->bool{self.source.is_some()&&self.ready&&!self.closing}
 pub fn token(&self)->Option<TypedOperationResultToken>{self.source.as_ref().map(|page|page.token)}
 pub fn lane(&self)->Option<TypedOperationResultLane>{self.source.as_ref().map(|page|page.lane)}
 /// 🧾️ The header is the same magic and thirty token/lane/length bytes as the original actor ABI.
 pub fn header(&self)->Result<&[u8],ValueError>{if !self.ready(){return Err(invalid())}Ok(&self.header)}
 /// 📖️ Body reads borrow the exact retained original page allocation without any copy.
 pub fn body(&self)->Result<&[u8],ValueError>{if !self.ready(){return Err(invalid())}Ok(self.source.as_ref().ok_or_else(invalid)?.bytes())}
 pub fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closing{return self.next_close_demand(body)}Ok(if self.terminal_is_empty()||self.ready{Default::default()}else{RetirementDemand{copy_bytes:TYPED_OPERATION_RESULT_HEADER_BYTES+size_of::<bool>(),depth:1,..Default::default()}})}
 /// 🧵️ Framing is one bounded write into the fixed descriptor destination.
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.closing{return self.close_step(grant)}let d=self.next_demand(grant.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if self.ready||!fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}self.source.as_ref().ok_or_else(invalid)?.renderer_header(&mut self.header);self.ready=true;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 /// 📤️ Transfers the original descriptor without reconstructing either header or payload owners.
 pub fn descriptor_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(RetirementDemand{copy_bytes:size_of::<Self>()*2+size_of::<Option<TypedOperationResultPage>>(),depth:1,..Default::default()})}
 pub fn take_descriptor(&mut self,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{let d=self.descriptor_demand()?;if !fits(d,grant){return Ok(None)}Ok(Some((Self{source:self.source.take(),header:self.header,ready:self.ready,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn next_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{let Some(page)=self.source.as_ref()else{return Ok(Default::default())};let mut d=page.next_close_demand(body.saturating_sub(size_of::<Option<TypedOperationResultPage>>()))?;d.copy_bytes+=size_of::<Option<TypedOperationResultPage>>();Ok(d)}
 /// ♻️ The source receiver alias closes before the receiving actor releases its framed output.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.next_close_demand(grant.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<TypedOperationResultPage>>(),..grant};let page=self.source.as_mut().unwrap();let mut p=page.close_step(child)?.progress();if page.terminal_is_empty(){self.source=None;p.copied_bytes+=size_of::<Option<TypedOperationResultPage>>();return Ok(RetainedCloneStep::Complete(p))}Ok(RetainedCloneStep::Progress(p))}
}
impl Drop for ResultPageDelivery{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original wire descriptor reached Drop before funded receiver closure")}}

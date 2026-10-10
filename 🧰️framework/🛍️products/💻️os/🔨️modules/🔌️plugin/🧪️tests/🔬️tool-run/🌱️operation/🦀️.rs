//! 🌱️ Original inline Toy fields retain canonical codec and wire custody through caller-funded turns.
use semio_framework_dsl_record::{BorrowedDslVariants,native_encoding::{FieldProjectionSource,FieldProjectionView}};
use semio_framework_job::StepContext;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use store::{ArtifactPreparedOperationCursor,ArtifactPreparedOperationSource};
use std::mem::ManuallyDrop;
use super::TestMutation;

struct OriginalToyFields{count:i32,label:[u8;15],length:usize,tag:usize}
impl FieldProjectionSource for OriginalToyFields{
 fn projection_view(&self,path:&[usize])->Result<FieldProjectionView<'_>,ValueError>{match path{[]=>Ok(FieldProjectionView::Record(&[0])),[0]if self.tag==0=>Ok(FieldProjectionView::Int(i64::from(self.count))),[0]=>Ok(FieldProjectionView::Text(std::str::from_utf8(&self.label[..self.length]).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original Toy label is not UTF8"))?)),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original Toy field path is absent"))}}
}

/// 🧵️ Stable fields, original codec and actual native backing remain owned until separate handoff and closure.
pub(super) struct OriginalToyOperationSource{fields:OriginalToyFields,codec:ArtifactPreparedOperationCursor,wire:ManuallyDrop<Option<Vec<u8>>>,chunk:[u8;64],length:usize,position:usize,encoded:bool,codec_closed:bool,closed:bool}
impl OriginalToyOperationSource{
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 fn from_fields(fields:OriginalToyFields)->Self{Self{fields,codec:Default::default(),wire:ManuallyDrop::new(None),chunk:[0;64],length:0,position:0,encoded:false,codec_closed:false,closed:false}}
 pub(super) fn admit_count(count:i32,cx:&mut StepContext<'_>)->Result<Option<Self>,ValueError>{if !Self::permits(cx.retained_grant(),RetirementDemand{depth:1,..Default::default()}){return Ok(None)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(Some(Self::from_fields(OriginalToyFields{count,label:[0;15],length:0,tag:0})))}
 pub(super) fn admit_label(unit:u32,cx:&mut StepContext<'_>)->Result<Option<Self>,ValueError>{let mut word=unit;let mut digits=1;while word>=10{word/=10;digits+=1;}let length=5+digits;if !Self::permits(cx.retained_grant(),RetirementDemand{copy_bytes:length,depth:1,..Default::default()}){return Ok(None)}let mut fields=OriginalToyFields{count:0,label:[0;15],length,tag:1};fields.label[..5].copy_from_slice(b"unit-");word=unit;for index in(5..length).rev(){fields.label[index]=b'0'+(word%10)as u8;word/=10;}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:length,..Default::default()})?;Ok(Some(Self::from_fields(fields)))}
 pub(super) fn original_wire(&self)->Option<&[u8]>{self.wire.as_ref().map(Vec::as_slice)}
 pub(super) fn is_ready(&self)->bool{self.encoded&&self.codec_closed&&self.position==self.length}
 pub(super) fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
  if self.closed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Toy operation is closed"))}if self.is_ready(){return Ok(true)}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  if self.wire.is_none(){let demand=RetirementDemand{capacity_bytes:64,depth:1,..Default::default()};if !Self::permits(grant,demand){return Ok(false)}*self.wire=Some(Vec::with_capacity(64));cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:64,..Default::default()})?;return Ok(false)}
  if self.position<self.length{if !Self::permits(grant,RetirementDemand{copy_bytes:1,depth:1,..Default::default()}){return Ok(false)}let count=(self.length-self.position).min(grant.maximum_copy_bytes);let wire=self.wire.as_mut().unwrap();if wire.len()+count>wire.capacity(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Toy wire exceeds its paid backing"))}wire.extend_from_slice(&self.chunk[self.position..self.position+count]);self.position+=count;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;return Ok(false)}
  if !self.encoded{let source=ArtifactPreparedOperationSource::Pack{tag:self.fields.tag as u64,body:&self.fields,spec:(<TestMutation as BorrowedDslVariants>::VARIANTS[self.fields.tag].1)()};match self.codec.advance(source,&mut self.chunk,grant){Ok(step)=>{self.length=step.written_bytes;self.position=0;self.encoded=step.complete;cx.consume_retained(RetainedCloneProgress{copied_items:step.processed_items,copied_bytes:step.copied_bytes,retained_capacity_bytes:step.retained_capacity_bytes,released_bytes:step.released_bytes})?;},Err(original)=>{self.length=original.written_bytes;self.position=0;cx.consume_retained(original.reason.retained_progress())?;return Err(original.reason)}}return Ok(false)}
  match self.codec.close(grant){Ok(step)=>{cx.consume_retained(RetainedCloneProgress{copied_items:step.processed_items,copied_bytes:step.copied_bytes,retained_capacity_bytes:step.retained_capacity_bytes,released_bytes:step.released_bytes})?;self.codec_closed=step.complete;Ok(false)},Err(original)=>{cx.consume_retained(original.reason.retained_progress())?;Err(original.reason)}}
 }
 pub(super) fn original_wire_owner(&mut self)->Result<&mut Option<Vec<u8>>,ValueError>{if !self.is_ready(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Toy wire handoff precedes codec closure"))}Ok(&mut self.wire)}
 pub(super) fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if self.closed{return Ok(Default::default())}if !self.codec_closed{return self.codec.retirement_demands()}Ok(RetirementDemand{release_bytes:self.wire.as_ref().map_or(0,Vec::capacity),depth:1,..Default::default()})}
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.closed{return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.retirement_demands()?;if !Self::permits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.codec_closed{return match self.codec.close(grant){Ok(step)=>{self.codec_closed=step.complete;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:step.processed_items,copied_bytes:step.copied_bytes,retained_capacity_bytes:step.retained_capacity_bytes,released_bytes:step.released_bytes}))},Err(original)=>Err(original.reason)}}if let Some(wire)=self.wire.take(){drop(wire);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))}self.closed=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
 pub(super) fn terminal_is_empty(&self)->bool{self.closed&&self.wire.is_none()}
}
impl Drop for OriginalToyOperationSource{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Toy fields/codec/wire reached Drop before paid closure");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

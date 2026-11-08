//! 📝️ Paged UTF-8 receives through the funded append cursor, so a large text never needs one unbounded copy.
use super::*;
use semio_framework_value::{ErasedSnapshotRetirement,paged::{PagedUtf8,PagedUtf8AppendCursor}};
/// 📝️ The source text stays resident until its last chunk is appended and its own paid retirement completes.
pub struct TextReceiving<const N:usize>{source:OwnedSlot<FieldValue>,output:OwnedSlot<PagedUtf8<N>>,append:PagedUtf8AppendCursor,phase:u8,closing:bool,failed:bool}
impl<const N:usize>Default for TextReceiving<N>{fn default()->Self{Self{source:Default::default(),output:Default::default(),append:Default::default(),phase:0,closing:false,failed:false}}}
impl<const N:usize>DslOwnedReceivingField for PagedUtf8<N>{type Cursor=TextReceiving<N>;fn receiving_cursor()->Self::Cursor{Self::Cursor::default()}}
impl<const N:usize>TextReceiving<N>{
 fn text(&self)->Result<&str,ValueError>{match self.source.value.as_ref(){Some(FieldValue::Text(text))=>Ok(text.as_str()),_=>Err(invariant())}}
 fn append_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.append.next_advance_demand(self.text()?,self.output.value.as_ref().ok_or_else(invariant)?,body)}
}
impl<const N:usize>OwnedFieldReceiving<PagedUtf8<N>>for TextReceiving<N>{
 fn demands(&self,_:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.failed||self.closing{return Err(invariant())}match self.phase{1=>self.append_demand(body),2=>ErasedSnapshotRetirement::next_demand(&self.append,body),3 if !self.source.terminal_is_empty()=>self.source.close_demands(body),4=>Ok(Default::default()),_=>Ok(metadata::<()>())}}
 fn select_advance(&self,_:&Option<FieldValue>,body:usize,maximum_work_units:u64)->Result<OwnedAdvanceSelection,ValueError>{if self.failed||self.closing{return Err(invariant())}Ok(OwnedAdvanceSelection::Local{work:match self.phase{1=>maximum_work_units.clamp(1,4),2=>ErasedSnapshotRetirement::next_computational_work(&self.append,body)?,3 if !self.source.terminal_is_empty()=>self.source.close_work_demand(body)?,4=>0,_=>1}})}
 fn advance_local(&mut self,source:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{
  if self.failed||self.closing{return done(effect,Err(invariant().into()))}if self.phase==4{return idle(effect)}
  let d=match self.demands(source,body){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  let poll=match self.phase{
   0=>local(effect,d,||{let Some(FieldValue::Text(text))=source.as_ref()else{return Err(invalid())};if text.len()>N{return Err(invalid())}*self.source.value=source.take();*self.output.value=Some(PagedUtf8::default());self.phase=1;Ok(())}),
   1=>{
    let grant=effect.grant();let turn=match(self.source.value.as_ref(),self.output.value.as_mut()){(Some(FieldValue::Text(text)),Some(output))=>self.append.advance(text.as_str(),output,body,grant),_=>return done(effect,Err(invariant().into()))};
    let result=owe(effect,turn.work).and_then(|()|turn.result).map(|step|step.progress()).map_err(Into::into);
    if result.is_ok()&&self.append.complete{self.append.begin_close();self.phase=2}done(effect,result)
   },
   2=>{let poll=self.append.close_step(effect);if self.append.terminal_is_empty(){self.phase=3}settle(effect,receiving(physical(poll)))},
   _=>{let poll=self.source.close_step(effect);if self.source.terminal_is_empty(){self.phase=4}settle(effect,receiving(poll))},
  };
  poison(&mut self.failed,&poll);poll
 }
 fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==4&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<PagedUtf8<N>>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{if !self.is_ready(){return done(effect,Err(invariant()))}move_into(&mut self.output.value,output,effect)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(metadata::<()>())}if !self.append.terminal_is_empty(){return ErasedSnapshotRetirement::next_demand(&self.append,body)}if !self.output.terminal_is_empty(){return self.output.close_demands(body)}self.source.close_demands(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}if !self.closing{return Ok(1)}if !self.append.terminal_is_empty(){return ErasedSnapshotRetirement::next_computational_work(&self.append,body)}if !self.output.terminal_is_empty(){return self.output.close_work_demand(body)}self.source.close_work_demand(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{if self.terminal_is_empty(){return idle(effect)}let d=match self.close_demands(effect.grant().maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}if !self.closing{return local_close(effect,d,||{self.closing=true;self.append.begin_close();Ok(())})}if !self.append.terminal_is_empty(){let poll=self.append.close_step(effect);return settle(effect,physical(poll))}if !self.output.terminal_is_empty(){return self.output.close_step(effect)}self.source.close_step(effect)}
 fn terminal_is_empty(&self)->bool{self.append.terminal_is_empty()&&self.output.terminal_is_empty()&&self.source.terminal_is_empty()}
}
impl<const N:usize>crate::DslReceivingField for PagedUtf8<N>{
 fn receive_value(value:&FieldValue,output:&mut Option<Self>,owner:&mut super::super::NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),super::super::NativeSnapshotFailure>{
  let FieldValue::Text(text)=value else{return RetainedWorkOutcome{result:Err(invalid().into()),work:0}};
  owner.receive::<PagedUtf8<N>,PagedUtf8<N>,super::super::NativeSnapshotFailure>(output,&mut |slot,result,marker,failure,native,body|super::super::receiving_turn(|meter|{
   *slot=Some(PagedUtf8::default());body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:text.len(),maximum_capacity_bytes:text.len(),maximum_depth:1,..Default::default()})?;
   native.checkpoint()?;meter.performed();let mut offset=0;while offset<text.len(){let end=chunk_end(text,offset);slot.as_mut().ok_or_else(invariant)?.try_push_str_controlled(&text[offset..end],native)?;offset=end}
   let quote=super::super::NativeSnapshotDecodeEffectQuote::decode(body,native,RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()})?;meter.take(quote.run(marker,failure,&mut |performed,physical|{*result=slot.take();*performed=true;*physical=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(())}))?;Ok(())
  }))
 }
}
/// ✂️ The next chunk never splits a character and never exceeds the paged chunk size.
fn chunk_end(text:&str,offset:usize)->usize{let mut end=(offset+semio_framework_value::paged::PAGED_UTF8_CHUNK_BYTES).min(text.len());while !text.is_char_boundary(end){end-=1}end}

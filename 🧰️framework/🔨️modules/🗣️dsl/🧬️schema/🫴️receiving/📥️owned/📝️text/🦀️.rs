//! 📝️ Paged UTF-8 receives one authored text chunk per funded turn, so a large text never needs one unbounded copy.
use super::*;
use semio_framework_value::paged::{PagedUtf8,PAGED_UTF8_CHUNK_BYTES};
/// 📝️ The source text stays resident until its last chunk is appended and its own paid retirement completes.
pub struct TextReceiving<const N:usize>{source:OwnedSlot<FieldValue>,output:OwnedSlot<PagedUtf8<N>>,offset:usize,length:usize,phase:u8,closing:bool,failed:bool}
impl<const N:usize>Default for TextReceiving<N>{fn default()->Self{Self{source:Default::default(),output:Default::default(),offset:0,length:0,phase:0,closing:false,failed:false}}}
impl<const N:usize>DslOwnedReceivingField for PagedUtf8<N>{type Cursor=TextReceiving<N>;fn receiving_cursor()->Self::Cursor{Self::Cursor::default()}}
/// ✂️ The next chunk never splits a character and never exceeds the paged chunk size.
fn chunk_end(text:&str,offset:usize)->usize{let mut end=(offset+PAGED_UTF8_CHUNK_BYTES).min(text.len());while !text.is_char_boundary(end){end-=1}end}
impl<const N:usize>TextReceiving<N>{
 fn text(&self)->Result<&str,ValueError>{match self.source.value.as_ref(){Some(FieldValue::Text(text))=>Ok(text.as_str()),_=>Err(invariant())}}
 fn selected(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  if self.failed||self.closing{return RetainedWorkOutcome{result:Err(invariant().into()),work:0}}
  if self.phase==3{return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  let d=match self.demands(source,body){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e.into()),work:0}};
  if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  if self.phase==2&&!self.source.terminal_is_empty(){return close_source(&mut self.source,native,g)}
  if let Err(e)=native.checkpoint(){return RetainedWorkOutcome{result:Err(e.into()),work:0}}
  let result=(||{match self.phase{
   0=>{let Some(FieldValue::Text(text))=source.as_ref()else{return Err(invalid())};if text.len()>N{return Err(invalid())}self.length=text.len();*self.source.value=source.take();*self.output.value=Some(PagedUtf8::default());self.phase=1;},
   1 if self.offset<self.length=>{let (start,end)={let text=self.text()?;(self.offset,chunk_end(text,self.offset))};let chunk=match self.source.value.as_ref(){Some(FieldValue::Text(text))=>&text[start..end],_=>return Err(invariant())};let output=self.output.value.as_mut().ok_or_else(invariant)?;output.try_push_str_controlled(chunk,native)?;self.offset=end;},
   1=>self.phase=2,
   2=>self.phase=3,
   _=>return Err(invariant())}Ok(receipt(d))})();
  RetainedWorkOutcome{result:result.map_err(|error:ValueError|OriginalReceivingPrimary::from(error.with_retained_progress(receipt(metadata::<()>())))),work:1}
 }
}
impl<const N:usize>OwnedFieldReceiving<PagedUtf8<N>>for TextReceiving<N>{
 fn demands(&self,_:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.failed||self.closing{return Err(invariant())}match self.phase{1 if self.offset<self.length=>{let text=self.text()?;Ok(RetirementDemand{copy_bytes:chunk_end(text,self.offset)-self.offset,depth:1,..Default::default()})},2 if !self.source.terminal_is_empty()=>self.source.close_demands(body),3=>Ok(Default::default()),_=>Ok(metadata::<()>())}}
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=self.selected(source,native,body,g);if turn.result.is_err()&&turn.work>0{self.failed=true}turn}
 fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==3&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<PagedUtf8<N>>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if !self.is_ready(){return RetainedWorkOutcome{result:Err(invariant()),work:0}}move_into(&mut self.output.value,output,g)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(metadata::<()>())}if !self.output.terminal_is_empty(){return self.output.close_demands(body)}self.source.close_demands(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e),work:0}};if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}if !self.closing{self.closing=true;return RetainedWorkOutcome{result:Ok(receipt(d)),work:1}}if !self.output.terminal_is_empty(){return self.output.close_step(g)}self.source.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.output.terminal_is_empty()&&self.source.terminal_is_empty()}
}
impl<const N:usize>crate::DslReceivingField for PagedUtf8<N>{
 fn receive_value(value:&FieldValue,owner:&mut super::super::NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,super::super::NativeSnapshotFailure>{
  let FieldValue::Text(text)=value else{return RetainedWorkOutcome{result:Err(invalid().into()),work:0}};
  owner.receive::<PagedUtf8<N>,PagedUtf8<N>,super::super::NativeSnapshotFailure>(|slot,native,body|super::super::receiving_turn(|meter|{
   *slot=Some(PagedUtf8::default());body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:text.len(),maximum_capacity_bytes:text.len(),maximum_depth:1,..Default::default()})?;
   native.checkpoint()?;meter.performed();let mut offset=0;while offset<text.len(){let end=chunk_end(text,offset);slot.as_mut().ok_or_else(invariant)?.try_push_str_controlled(&text[offset..end],native)?;offset=end}Ok(slot.take().ok_or_else(invariant)?)
  }))
 }
}

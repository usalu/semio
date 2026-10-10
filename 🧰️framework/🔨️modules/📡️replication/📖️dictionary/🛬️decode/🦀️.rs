//! 📖️ Original dictionary strings remain borrowed source ranges while every genuine backing stays under its caller receipt.
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,list::{PagedList,PagedListError}};
use crate::codec::{RetainedWireUtf8Validation,RetainedWireByteSource,OriginalWireSourceIdentity};
use std::mem::ManuallyDrop;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedSourceDictionaryStep{Blocked,Progress,Ready,Closed}

pub struct RetainedSourceDictionary{
 source_identity:OriginalWireSourceIdentity,
 exact_record_end:bool,
 original_source_ranges:ManuallyDrop<PagedList<(usize,usize),{usize::MAX}>>,
 receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,
 phase:u8,
 closing:bool,
 cursor:usize,
 record_end:usize,
 remaining:usize,
 range:Option<(usize,usize)>,
 validation_cursor:usize,
 validation:RetainedWireUtf8Validation,
}
impl RetainedSourceDictionary{
 pub fn new<S:RetainedWireByteSource+?Sized>(source:&S)->Self{Self{source_identity:source.original_source_identity(),exact_record_end:true,original_source_ranges:ManuallyDrop::new(PagedList::empty()),receipt:None,phase:0,closing:false,cursor:0,record_end:0,remaining:0,range:None,validation_cursor:0,validation:RetainedWireUtf8Validation::new()}}
 pub fn source_identity(&self)->OriginalWireSourceIdentity{self.source_identity}
 pub fn position(&self)->usize{self.cursor}
 fn validate_source<S:RetainedWireByteSource+?Sized>(&self,source:&S)->Result<(),ValueError>{if self.source_identity!=source.original_source_identity(){return Err(Self::fault("original dictionary source identity or extent changed"))}Ok(())}
 fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
 fn list_error(error:PagedListError)->ValueError{ValueError::literal(match error.kind{semio_framework_value::list::PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,semio_framework_value::list::PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,_=>ValueRefusalKind::InvariantViolated},error.reason)}
 fn varint<S:RetainedWireByteSource+?Sized>(source:&S,cursor:&mut usize,end:usize)->Result<usize,ValueError>{
  let mut result=0u64;
  for index in 0..10{let byte=source.byte_at(*cursor).filter(|_|*cursor<end).ok_or_else(||Self::fault("original dictionary scalar is truncated"))?;*cursor+=1;if index==9&&byte>1{return Err(Self::fault("original dictionary scalar overflows"))}result|=((byte&127)as u64)<<(index*7);if byte&128==0{return usize::try_from(result).map_err(|_|Self::fault("original dictionary extent exceeds this platform"))}}
  Err(Self::fault("original dictionary scalar is not bounded"))
 }
 fn covers(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_depth>=demand.depth&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes}
 pub fn receipt(&self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt}
 pub fn take_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt.take()}
 pub fn len(&self)->usize{self.original_source_ranges.len()}
 pub fn is_ready(&self)->bool{!self.closing&&self.phase==0&&self.receipt.is_none()}
 pub fn resolve_range<S:RetainedWireByteSource+?Sized>(&self,source:&S,index:usize)->Result<(usize,usize),ValueError>{self.validate_source(source)?;let range=*self.original_source_ranges.get(index).ok_or_else(||Self::fault("original dictionary index is absent"))?;if range.0>range.1||range.1>self.source_identity.extent{return Err(Self::fault("original dictionary source range escaped its original extent"))}Ok(range)}
 pub fn begin_record<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,start:usize,end:usize,grant:RetainedCloneGrant)->Result<RetainedSourceDictionaryStep,ValueError>{
  self.validate_source(source)?;
  if !self.is_ready()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedSourceDictionaryStep::Blocked)}
  if start>=end||end>self.source_identity.extent{return Err(Self::fault("original dictionary record escaped its original source"))}
  if source.byte_at(start).ok_or_else(||Self::fault("original dictionary version is truncated"))?>1{return Err(Self::fault("original dictionary record version is unsupported"))}
  let mut cursor=start+1;let base=Self::varint(source,&mut cursor,end)?;let count=Self::varint(source,&mut cursor,end)?;
  if base!=self.len()||count>end-cursor{return Err(Self::fault("original dictionary record base or count is invalid"))}
  if count==0&&cursor!=end{return Err(Self::fault("original empty dictionary record has trailing bytes"))}
  self.exact_record_end=true;self.cursor=cursor;self.record_end=end;self.remaining=count;self.phase=if count==0{0}else{1};self.receipt=Some((grant,RetainedCloneProgress{copied_items:1,..Default::default()}));Ok(if self.phase==0{RetainedSourceDictionaryStep::Ready}else{RetainedSourceDictionaryStep::Progress})
 }
 pub fn begin_inline_catalog<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,start:usize,grant:RetainedCloneGrant)->Result<RetainedSourceDictionaryStep,ValueError>{
  self.validate_source(source)?;if !self.is_ready()||self.len()!=0||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedSourceDictionaryStep::Blocked)}
  let end=self.source_identity.extent;if start>=end{return Err(Self::fault("original inline catalog escaped its source"))}let mut cursor=start;let count=Self::varint(source,&mut cursor,end)?;if count>end-cursor{return Err(Self::fault("original inline catalog count exceeds its source"))}
  self.exact_record_end=false;self.cursor=cursor;self.record_end=end;self.remaining=count;self.phase=if count==0{0}else{1};self.receipt=Some((grant,RetainedCloneProgress{copied_items:1,..Default::default()}));Ok(if self.phase==0{RetainedSourceDictionaryStep::Ready}else{RetainedSourceDictionaryStep::Progress})
 }
 pub fn demands<S:RetainedWireByteSource+?Sized>(&self,source:&S)->Result<RetirementDemand,ValueError>{
  self.validate_source(source)?;
  if self.receipt.is_some()||self.phase==4{return Ok(Default::default())}
  if self.closing{
   if !self.original_source_ranges.is_empty(){return Ok(RetirementDemand{depth:self.original_source_ranges.next_pop_depth_demand().map_err(Self::list_error)?,..Default::default()})}
   return Ok(RetirementDemand{release_bytes:self.original_source_ranges.next_release_allocation_bytes().map_err(Self::list_error)?,depth:self.original_source_ranges.next_release_depth_demand().map_err(Self::list_error)?.max(1),..Default::default()})
  }
  match self.phase{
   0=>Ok(Default::default()),
   3 if !self.original_source_ranges.has_reserved_slot()=>Ok(RetirementDemand{capacity_bytes:self.original_source_ranges.next_allocation_bytes().map_err(Self::list_error)?,depth:self.original_source_ranges.next_reserve_depth_demand().map_err(Self::list_error)?,..Default::default()}),
   3=>Ok(RetirementDemand{depth:self.original_source_ranges.next_push_depth_demand().map_err(Self::list_error)?,..Default::default()}),
   _=>Ok(RetirementDemand{depth:1,..Default::default()}),
  }
 }
 pub fn advance<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,grant:RetainedCloneGrant)->Result<RetainedSourceDictionaryStep,ValueError>{
  self.validate_source(source)?;
  if self.receipt.is_some()||self.closing||self.phase==4{return Ok(RetainedSourceDictionaryStep::Blocked)}
  if self.phase==0{return Ok(RetainedSourceDictionaryStep::Ready)}
  if !Self::covers(grant,self.demands(source)?){return Ok(RetainedSourceDictionaryStep::Blocked)}
  let progress=match self.phase{
   1=>{let mut cursor=self.cursor;let length=Self::varint(source,&mut cursor,self.record_end)?;let end=cursor.checked_add(length).filter(|end|*end<=self.record_end).ok_or_else(||Self::fault("original dictionary field escaped its record"))?;self.range=Some((cursor,end));self.cursor=end;self.validation_cursor=cursor;self.validation=RetainedWireUtf8Validation::new();self.phase=2;RetainedCloneProgress{copied_items:1,..Default::default()}},
   2=>{let(_,end)=self.range.ok_or_else(||Self::fault("original dictionary validation range is absent"))?;let next=self.validation_cursor.saturating_add(4096).min(end);let mut validation=self.validation;for position in self.validation_cursor..next{let byte=source.byte_at(position).ok_or_else(||Self::fault("original dictionary UTF8 range contains absent source byte"))?;validation=validation.consume(&[byte])?;}if next==end&&!validation.is_complete(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original dictionary text is truncated UTF8"))}self.validation=validation;self.validation_cursor=next;if next==end{self.phase=3}RetainedCloneProgress{copied_items:1,..Default::default()}},
   3 if !self.original_source_ranges.has_reserved_slot()=>{
    match self.original_source_ranges.reserve_one(grant.maximum_capacity_bytes){
     Ok(piece)=>RetainedCloneProgress{copied_items:usize::from(piece.progressed),retained_capacity_bytes:piece.allocated_bytes,released_bytes:piece.released_allocation_bytes,..Default::default()},
     Err(error)=>{let progress=RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0),retained_capacity_bytes:error.allocated_bytes,..Default::default()};self.receipt=Some((grant,progress));return Err(Self::list_error(error.refusal()).with_retained_progress(progress))}
    }
   },
   3=>{let range=self.range.take().ok_or_else(||Self::fault("original dictionary publish range is absent"))?;if let Err(range)=self.original_source_ranges.push_reserved(range){self.range=Some(range);return Err(Self::fault("original reserved dictionary range was refused"))}self.remaining-=1;self.phase=if self.remaining==0{0}else{1};RetainedCloneProgress{copied_items:1,..Default::default()}},
   _=>return Err(Self::fault("original dictionary phase is invalid")),
  };
  self.receipt=Some((grant,progress));
  if !progress.fits(grant){return Err(Self::fault("original dictionary backing exceeded its original grant").with_retained_progress(progress))}
  if self.phase==0&&self.exact_record_end&&self.cursor!=self.record_end{return Err(Self::fault("original dictionary record has trailing bytes").with_retained_progress(progress))}
  Ok(if self.phase==0{RetainedSourceDictionaryStep::Ready}else{RetainedSourceDictionaryStep::Progress})
 }
 pub fn cancel(&mut self){self.closing=true}
 pub fn close_step<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,grant:RetainedCloneGrant)->Result<RetainedSourceDictionaryStep,ValueError>{
  self.validate_source(source)?;
  if self.receipt.is_some()||!self.closing{return Ok(RetainedSourceDictionaryStep::Blocked)}
  if self.phase==4{return Ok(RetainedSourceDictionaryStep::Closed)}
  if !Self::covers(grant,self.demands(source)?){return Ok(RetainedSourceDictionaryStep::Blocked)}
  let progress=if !self.original_source_ranges.is_empty(){self.original_source_ranges.pop().ok_or_else(||Self::fault("original dictionary range disappeared before paid close"))?;RetainedCloneProgress{copied_items:1,..Default::default()}}
  else if !self.original_source_ranges.terminal_is_empty(){let piece=self.original_source_ranges.release_empty_page(grant.maximum_release_bytes).map_err(Self::list_error)?;RetainedCloneProgress{copied_items:usize::from(piece.progressed),released_bytes:piece.released_allocation_bytes,..Default::default()}}
  else{self.range=None;self.remaining=0;self.phase=4;RetainedCloneProgress{copied_items:1,..Default::default()}};
  self.receipt=Some((grant,progress));if !progress.fits(grant){return Err(Self::fault("original dictionary close exceeded its original grant").with_retained_progress(progress))}Ok(if self.phase==4{RetainedSourceDictionaryStep::Closed}else{RetainedSourceDictionaryStep::Progress})
 }
 pub fn terminal_is_empty(&self)->bool{self.phase==4&&self.original_source_ranges.terminal_is_empty()&&self.range.is_none()&&self.receipt.is_none()}
}
impl Drop for RetainedSourceDictionary{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original dictionary abandoned borrowed source or paid backing custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original_source_ranges)}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🧾️source/🧪️tests/🦀️.rs"]
mod paged_source_tests;

//! 📦️ Record materialization borrows a Body or a staged, fallible verified Document driver.
use super::*;
use semio_framework_value::RetainedWorkOutcome;
use std::mem::MaybeUninit;
pub(super) struct DocumentOwner{storage:Option<Box<MaybeUninit<RetainedDocumentDriver>>>,initialized:bool,limits:PackLimits,length:usize,maximum:usize,closed:bool}
impl DocumentOwner{
 pub(super) fn new(limits:PackLimits,length:usize,maximum:usize)->Self{Self{storage:None,initialized:false,limits,length,maximum,closed:false}}
 fn driver(&self)->Option<&RetainedDocumentDriver>{self.initialized.then(||unsafe{self.storage.as_ref().unwrap().assume_init_ref()})}
 fn driver_mut(&mut self)->Option<&mut RetainedDocumentDriver>{if self.initialized{Some(unsafe{self.storage.as_mut().unwrap().assume_init_mut()})}else{None}}
 fn advance<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,range:(usize,usize),maximum:usize)->RetainedWorkOutcome<Option<Token>,PackRefusal>{
  if self.closed{return RetainedWorkOutcome{result:Err(PackRefusal::from(invalid("closed Document driver cannot advance"))),work:0}}
  if self.storage.is_none(){
   if maximum<size_of::<RetainedDocumentDriver>(){return RetainedWorkOutcome{result:Ok(None),work:0}}
   let raw=unsafe{std::alloc::alloc(std::alloc::Layout::new::<RetainedDocumentDriver>())}.cast::<MaybeUninit<RetainedDocumentDriver>>();
   let result=if raw.is_null(){Err(PackRefusal::from(allocation()))}else{self.storage=Some(unsafe{Box::from_raw(raw)});Ok(None)};return RetainedWorkOutcome{result,work:1}
  }
  if !self.initialized{let result=RetainedDocumentDriver::record(self.limits.clone(),self.length,self.maximum).map(|driver|{self.storage.as_mut().unwrap().write(driver);self.initialized=true;None});return RetainedWorkOutcome{result,work:1}}
  let driver=self.driver_mut().unwrap();let result=driver.one(source,range,maximum);RetainedWorkOutcome{result,work:driver.last_turn_work()}
 }
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,PackRefusal>{
  if self.closed{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if let Some(driver)=self.driver_mut(){if !driver.terminal_is_empty(){let result=driver.close_step(grant);return RetainedWorkOutcome{result,work:driver.last_turn_work()}}}
  let demand=RetirementDemand{release_bytes:usize::from(self.storage.is_some())*size_of::<RetainedDocumentDriver>(),depth:1,..Default::default()};
  if !funded(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  self.initialized=false;drop(self.storage.take());self.closed=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})),work:1}
 }
}
pub(super) enum RecordCursor{Body(RetainedRecordBodyCursor),Document(DocumentOwner)}
impl RecordCursor{
 pub(super) fn source_step_demands(&self)->Result<RetirementDemand,PackRefusal>{match self{Self::Body(cursor)if cursor.ingress_ready()=>Ok(RetirementDemand{copy_bytes:1,depth:1,..Default::default()}),Self::Body(cursor)=>cursor.source_step_demands(),Self::Document(owner)=>if let Some(cursor)=owner.driver(){cursor.normal_demands()}else{Ok(RetirementDemand{capacity_bytes:usize::from(owner.storage.is_none())*size_of::<RetainedDocumentDriver>(),depth:1,..Default::default()})}}}
 pub(super) fn allocated_bytes(&self)->usize{match self{Self::Body(cursor)=>cursor.allocated_bytes(),Self::Document(owner)=>usize::from(owner.storage.is_some())*size_of::<RetainedDocumentDriver>()+owner.driver().map_or(0,RetainedDocumentDriver::allocated_bytes)}}
 pub(super) fn symbol_chars(&self,symbol:u64)->Result<usize,PackRefusal>{match self{Self::Body(cursor)=>cursor.symbol_chars(symbol),Self::Document(owner)=>owner.driver().ok_or_else(||PackRefusal::from(invalid("uninitialized Document driver has no symbols")))?.symbol_chars(symbol)}}
 pub(super) fn symbol_utf8_bytes(&self,symbol:u64)->Result<usize,PackRefusal>{match self{Self::Body(cursor)=>cursor.symbol_utf8_bytes(symbol),Self::Document(owner)=>owner.driver().ok_or_else(||PackRefusal::from(invalid("uninitialized Document driver has no symbols")))?.symbol_utf8_bytes(symbol)}}
 pub(super) fn symbol_char(&self,symbol:u64,index:usize)->Result<Option<char>,PackRefusal>{match self{Self::Body(cursor)=>cursor.symbol_char(symbol,index),Self::Document(owner)=>owner.driver().ok_or_else(||PackRefusal::from(invalid("uninitialized Document driver has no symbols")))?.symbol_char(symbol,index)}}
 pub(super) fn advance<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,range:(usize,usize),position:&mut usize,sealed:&mut bool,maximum:usize)->RetainedWorkOutcome<Option<Token>,PackRefusal>{match self{
  Self::Document(owner)=>owner.advance(source,range,maximum),
  Self::Body(cursor)=>{
   let mut work=0;let result=(||{
    if cursor.ingress_ready(){if *position<range.1-range.0{work=1;let byte=source.byte_at(range.0+*position).ok_or_else(||PackRefusal::from(invalid("original record source byte is absent")))?;cursor.admit_byte(*position as u64,byte).map_err(|_|PackRefusal::from(invalid("original Record ingress handback refused")))?;*position+=1;}else if !*sealed{work=1;cursor.seal(*position as u64)?;*sealed=true;}else{return Err(PackRefusal::from(invalid("original Record lexer did not complete")))}}
    else if maximum>0{work=1;cursor.reserve_allocation(maximum).map_err(|error|error.fault)?;}
    else{if cursor.fault.is_some(){return cursor.grant().map(|_|None)}work=1;if let Some(RetainedRecordBodyToken::Value(token))=cursor.grant()?{return Ok(Some(token))}}
    Ok(None)
   })();RetainedWorkOutcome{result,work}
  },
 }}
 pub(super) fn source_close_demands(&self)->Result<RetirementDemand,PackRefusal>{match self{Self::Body(cursor)=>cursor.source_close_demands(),Self::Document(owner)=>if let Some(cursor)=owner.driver(){if !cursor.terminal_is_empty(){return cursor.close_demands(usize::MAX)}Ok(RetirementDemand{release_bytes:size_of::<RetainedDocumentDriver>(),depth:1,..Default::default()})}else{Ok(RetirementDemand{release_bytes:usize::from(owner.storage.is_some())*size_of::<RetainedDocumentDriver>(),depth:1,..Default::default()})}}}
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,PackRefusal>{match self{
  Self::Document(owner)=>owner.close_step(grant),
  Self::Body(cursor)=>{
   if cursor.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
   let demand=match cursor.source_close_demands(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
   if !funded(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
   let result=cursor.close_step(grant.maximum_items,grant.maximum_release_bytes).map(|step|match step{semio_framework_pack_format_retained::RetainedPackCloseStep::Complete=>RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),semio_framework_pack_format_retained::RetainedPackCloseStep::Pending{released_items,released_bytes}=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()})});RetainedWorkOutcome{result,work:1}
  }
 }}
 pub(super) fn terminal_is_empty(&self)->bool{match self{Self::Body(cursor)=>cursor.terminal_is_empty(),Self::Document(owner)=>owner.closed&&owner.storage.is_none()}}
}

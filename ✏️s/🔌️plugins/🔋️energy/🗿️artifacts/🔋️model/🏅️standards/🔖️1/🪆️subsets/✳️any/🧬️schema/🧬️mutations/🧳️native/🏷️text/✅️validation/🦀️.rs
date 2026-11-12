//! ✅️ Name policies scan original Unicode text and apply only the authored collection uniqueness rule.
use super::*;
use crate::{owned::OwnedRowValidation,row::RowOutcome,custody::{self,Cursor,Bound},scalar::{Quote,bind}};
use crate::retained::{Done,Flight,Reply};
use semio_framework_value::{RetainedAllocationEffect};
pub struct TextValidation<S:TextRowPath>{input:Bound,key:Bound,original:Bound,names:Option<Box<NameLookup<S>>>,offset:usize,phase:u8,result:Option<Option<RowOutcome>>,closing:bool,flight:Flight}
impl<S:TextRowPath> TextValidation<S>{
 pub fn new()->Self{Self{input:Default::default(),key:Default::default(),original:Default::default(),names:None,offset:0,phase:0,result:None,closing:false,flight:Default::default()}}
 fn end(&self,text:&str,body:usize)->usize{let mut end=self.offset+(body.saturating_sub(2*size_of::<usize>()+3)/2).min(text.len().saturating_sub(self.offset));while end>self.offset&&!text.is_char_boundary(end){end-=1;}end}
 pub fn result(&self)->Option<Option<RowOutcome>>{self.result}
 pub fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(names,input,key,original);
}
crate::custody::std_cursor!([S:TextRowPath] TextValidation<S>,[],|s|s.begin_close());
impl<S:TextRowPath> TextValidation<S>{
 pub fn quote_original(&self,input:RetainedCloneRef<'_,String>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<Quote,ValueError>{if self.closing||self.result.is_some()||!self.flight.idle(){return Err(invalid());}input.check_binding(&self.input.0)?;key.check_binding(&self.key.0)?;original.check_binding(&self.original.0)?;match self.phase{
  0=>Ok((nested(scalar(input.binding_copy_bytes()),1)?,input.binding_work_demand(&self.input.0)?)),1=>Ok((nested(scalar(key.binding_copy_bytes()),1)?,key.binding_work_demand(&self.key.0)?)),2=>Ok((nested(scalar(original.binding_copy_bytes()),1)?,original.binding_work_demand(&self.original.0)?)),
  3=>{let n=self.end(input.get(),body)-self.offset;Ok((scalar(2*size_of::<usize>()+3+2*n),1+u64::try_from(n).map_err(|_|invalid())?))},
  4=>Ok((birth::<NameLookup<S>>(),1)),
  5=>{let(d,w)=self.names.as_ref().ok_or_else(invalid)?.quote(input,key,original,body.saturating_sub(1),max)?;Ok((nested(d,1)?,w))},
  6=>Ok((Cursor::demand(&self.names,body)?,Cursor::work(&self.names,body)?)),
  _=>Err(invalid())}}
 pub fn advance_original_reply<'r>(&mut self,input:RetainedCloneRef<'_,String>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let quote=if recover{None}else{Some(self.quote_original(input,key,original,body,max))};let g=effect.grant();
  custody::gated(effect,reply,quote,max,|effect,work|{let mut p=receipt(scalar(1));match self.phase{
   0=>{p=add(bind(input,&mut self.input,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=1;},1=>{p=add(bind(key,&mut self.key,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=2;},2=>{p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=3;},
   3=>{let end=self.end(input.get(),body);if input.get()[self.offset..end].chars().any(|c|!c.is_whitespace()){if S::UNIQUE{self.phase=4;}else{self.result=Some(None);}}else if end==input.get().len(){self.result=Some(Some(RowOutcome::Invalid));}self.offset=end;},
   4=>{self.names=Some(Box::new(NameLookup::new()));self.phase=5;p=receipt(birth::<NameLookup<S>>());},
   5=>{let names=self.names.as_mut().ok_or_else(invalid)?;let nb=body.saturating_sub(1);
    let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,*work,|r| names.advance_reply(input,key,original,nb,max,&mut effect.nested(child(g,1)),r,false),names.advance_reply(input,key,original,nb,max,&mut effect.nested(child(g,1)),r,true));
    p=add(step?.progress(),1)?;if names.result().is_some(){names.begin_close();self.phase=6;}},
   6=>{let duplicate=self.names.as_ref().ok_or_else(invalid)?.result().ok_or_else(invalid)?;let step=custody::finish(&mut self.flight,&mut self.names,body,effect,work,recover)?;p=step.progress();if self.names.is_none(){self.result=Some(duplicate.then_some(RowOutcome::Duplicate));}},
   _=>return Err(invalid().into())}
   Ok(if self.result.is_some(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
}
impl<S:TextRowPath> OwnedRowValidation<String,S::Row> for TextValidation<S>{
 fn quote(&self,input:RetainedCloneRef<'_,String>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,_:RetainedCloneRef<'_,S::Row>,body:usize,max:u64)->Result<Quote,ValueError>{self.quote_original(input,key,original,body,max)}
 fn advance_reply<'r>(&mut self,input:RetainedCloneRef<'_,String>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,_:RetainedCloneRef<'_,S::Row>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.advance_original_reply(input,key,original,body,max,effect,reply,recover)}
 fn result(&self)->Option<Option<RowOutcome>>{Self::result(self)}
 fn begin_close(&mut self){Self::begin_close(self)}
}

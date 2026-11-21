//! 📦️ Completed coverage streams one pending run into the shared packed-page builder.
use super::*;
use crate::selection::{RasterSelectionSpans,RasterSelectionSpansBuilder};
use semio_framework_value::retained_clone::RetainedCloneClose;
use std::mem::ManuallyDrop;

pub(super) struct SelectionSpanWriter{builder:RasterSelectionSpansBuilder,pending:Option<RasterSelectionSpan>,output:ManuallyDrop<Option<RasterSelectionSpans>>,retirement:RetainedCloneClose,total_pixels:usize,cursor:usize,complete_span:bool,finishing:bool}
impl SelectionSpanWriter{
    pub fn new(rows:usize,pixels:usize)->Result<Self,ValueError>{Ok(Self{builder:RasterSelectionSpansBuilder::new(rows)?,pending:None,output:ManuallyDrop::new(None),retirement:Default::default(),total_pixels:pixels,cursor:0,complete_span:false,finishing:false})}
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.builder.output_ready(){return self.builder.take_demands();}
        if self.complete_span{return if self.builder.accepts_row(){self.builder.push_demands()}else{self.builder.demands()};}
        if self.cursor==self.total_pixels&&self.pending.is_none(){return if self.finishing{self.builder.demands()}else{self.builder.finish_demands()};}
        Ok(RetirementDemand{copy_bytes:body.min(32768),depth:1,..Default::default()})
    }
    pub fn advance(&mut self,coverage:&[u8],effect:&mut RetainedAllocationEffect<'_>,continue_work:&mut dyn FnMut()->bool)->Result<(bool,RetainedCloneProgress),ValueError>{
        let grant=effect.grant();
        let d=self.demands(grant.maximum_copy_bytes)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok((false,Default::default()));}
        if self.builder.output_ready(){
            if let Some(output)=self.builder.take(effect)?{*self.output=Some(output);return Ok((true,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
            return Ok((false,Default::default()));
        }
        if self.complete_span&&self.builder.accepts_row(){
            let step=self.builder.push(&mut self.pending,effect)?;
            if self.pending.is_none(){self.complete_span=false;}
            return Ok((false,step.progress()));
        }
        if self.cursor==coverage.len()&&!self.complete_span{
            if self.pending.is_some(){
                if !effect.performed(1)?{return Ok((false,Default::default()));}
                self.complete_span=true;return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}));
            }
            if !self.finishing{
                let step=self.builder.finish(effect)?;self.finishing=self.builder.finishing();return Ok((false,step.progress()));
            }
        }
        if self.complete_span||self.finishing{
            if effect.remaining_work_units()?<self.builder.advance_work_demand()?{return Ok((false,Default::default()));}
            let step=self.builder.advance(effect)?.transpose()?;
            return Ok((false,step.map_or_else(Default::default,|step|step.progress())));
        }
        let maximum=(effect.remaining_work_units()? as usize).min(grant.maximum_copy_bytes/32).min(1024);
        let mut work=0;
        while self.cursor<coverage.len()&&work<maximum&&continue_work()&&effect.performed(1)?{
            let alpha=coverage[self.cursor];work+=1;
            if let Some(span)=self.pending.as_mut(){if span.coverage==u32::from(alpha)&&span.start as usize+span.length as usize==self.cursor{span.length+=1;}else{self.complete_span=true;break;}}
            else if alpha!=0{self.pending=Some(RasterSelectionSpan{start:self.cursor as u32,length:1,coverage:u32::from(alpha)});}
            self.cursor+=1;
        }
        Ok((false,RetainedCloneProgress{copied_items:usize::from(work!=0),copied_bytes:work*32,..Default::default()}))
    }
    pub fn take(&mut self)->Option<RasterSelectionSpans>{self.output.take()}
    pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.builder.terminal_is_empty(){return if self.builder.is_closing(){self.builder.close_demands(body)}else{Ok(self.builder.begin_close_demand())};}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(false,&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(true,&None,body);}Ok(RetirementDemand{copy_bytes:usize::from(self.pending.is_some())*size_of::<Option<RasterSelectionSpan>>(),depth:1,..Default::default()})}
    pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
        use semio_framework_value::{retained_clone::OriginalEffectReply,RetirementPollExit};
        let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
            let d=self.close_demands(body)?;
            if !semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()));}
            if !self.builder.terminal_is_empty(){
                if !self.builder.is_closing(){return Ok(self.builder.begin_close_paid(effect)?);}
                let mut slot=None;
                self.builder.close_into(body,effect,OriginalEffectReply::borrow(&mut slot)?)?;
                return slot.map_or_else(||Err(RetirementPollExit::Pending(effect.performed_work())),|settled|settled.map_err(Into::into));
            }
            if !self.retirement.is_empty(){return RetirementPollExit::resume(self.retirement.step_granted(body,effect));}
            if self.output.is_some(){return Ok(self.retirement.begin_granted(&mut self.output,effect).result?.unwrap_or(RetainedCloneStep::Progress(Default::default())));}
            if self.pending.is_some()&&!effect.performed(1)?{return Ok(RetainedCloneStep::Progress(Default::default()));}
            self.pending=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:usize::from(d.copy_bytes!=0),copied_bytes:d.copy_bytes,..Default::default()}))
        })();
        RetirementPollExit::settle(effect,result)
    }
    pub fn empty(&self)->bool{self.builder.terminal_is_empty()&&self.pending.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl Drop for SelectionSpanWriter{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"selection page writer retains original rows and roots until funded closure");}}

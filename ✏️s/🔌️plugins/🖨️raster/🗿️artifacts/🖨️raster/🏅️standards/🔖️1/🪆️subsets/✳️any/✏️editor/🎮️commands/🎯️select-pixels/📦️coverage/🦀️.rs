//! 📦️ Completed coverage streams one pending run into the shared packed-page builder.
use super::*;
use crate::selection::{RasterSelectionSpans,RasterSelectionSpansBuilder};
use semio_framework_value::retained_clone::RetainedCloneClose;
use std::mem::ManuallyDrop;

pub(super) struct SelectionSpanWriter{builder:RasterSelectionSpansBuilder,pending:Option<RasterSelectionSpan>,output:ManuallyDrop<Option<RasterSelectionSpans>>,retirement:RetainedCloneClose,total_pixels:usize,cursor:usize,complete_span:bool,finishing:bool,closing:bool,pub work:usize}
impl SelectionSpanWriter{
    pub fn new(rows:usize,pixels:usize)->Result<Self,ValueError>{Ok(Self{builder:RasterSelectionSpansBuilder::new(rows)?,pending:None,output:ManuallyDrop::new(None),retirement:Default::default(),total_pixels:pixels,cursor:0,complete_span:false,finishing:false,closing:false,work:0})}
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.builder.output_ready(){return self.builder.take_demands();}
        if self.complete_span{return if self.builder.accepts_row(){self.builder.push_demands()}else{self.builder.demands()};}
        if self.cursor==self.total_pixels&&self.pending.is_none(){return if self.finishing{self.builder.demands()}else{self.builder.finish_demands()};}
        Ok(RetirementDemand{copy_bytes:body.min(32768),depth:1,..Default::default()})
    }
    pub fn advance(&mut self,coverage:&[u8],grant:semio_framework_value::RetainedCloneGrant,maximum_work:usize,continue_work:&mut dyn FnMut()->bool)->Result<(bool,RetainedCloneProgress),ValueError>{
        self.work=0;if maximum_work<2{return Ok((false,Default::default()));}
        let d=self.demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok((false,Default::default()));}
        if self.builder.output_ready(){let turn=self.builder.take(grant);self.work=turn.work as usize;if let Some((output,p))=turn.result?{*self.output=Some(output);return Ok((true,p));}return Ok((false,Default::default()));}
        if self.complete_span{
            if self.builder.accepts_row(){let turn=self.builder.push(&mut self.pending,grant);self.work=turn.work as usize;let p=turn.result?;if self.pending.is_none(){self.complete_span=false;}return Ok((false,p));}
            let turn=self.builder.advance(grant);self.work=turn.work as usize;return Ok((false,turn.result?.progress()));
        }
        if self.cursor==coverage.len(){
            if self.pending.is_some(){self.complete_span=true;self.work=1;return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}));}
            if !self.finishing{let turn=self.builder.finish(grant);self.work=turn.work as usize;let p=turn.result?;self.finishing=p.copied_items!=0;return Ok((false,p));}
            let turn=self.builder.advance(grant);self.work=turn.work as usize;return Ok((false,turn.result?.progress()));
        }
        let maximum=maximum_work.min(grant.maximum_copy_bytes/32).min(1024);
        while self.cursor<coverage.len()&&self.work<maximum&&continue_work(){let alpha=coverage[self.cursor];if let Some(span)=self.pending.as_mut(){if span.coverage==u32::from(alpha)&&span.start as usize+span.length as usize==self.cursor{span.length+=1;}else{self.complete_span=true;self.work+=1;break;}}else if alpha!=0{self.pending=Some(RasterSelectionSpan{start:self.cursor as u32,length:1,coverage:u32::from(alpha)});}self.cursor+=1;self.work+=1;}
        Ok((false,RetainedCloneProgress{copied_items:1,copied_bytes:self.work*32,..Default::default()}))
    }
    pub fn take(&mut self)->Option<RasterSelectionSpans>{self.output.take()}
    pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.builder.terminal_is_empty(){return self.builder.close_demands(body);}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(false,&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(true,&None,body);}Ok(RetirementDemand{copy_bytes:usize::from(self.pending.is_some())*size_of::<Option<RasterSelectionSpan>>(),depth:1,..Default::default()})}
    pub fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{
        let mut work=0;let result=(||{if !self.closing{self.closing=true;self.builder.begin_close();}let d=self.close_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.builder.terminal_is_empty(){let turn=self.builder.close_step(grant);work=turn.work;return turn.result;}if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);work=turn.work;return turn.result;}if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,grant);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default())));}work=u64::from(self.pending.is_some());self.pending=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:usize::from(d.copy_bytes!=0),copied_bytes:d.copy_bytes,..Default::default()}))})();semio_framework_value::RetainedWorkOutcome{result,work}
    }
    pub fn empty(&self)->bool{self.builder.terminal_is_empty()&&self.pending.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl Drop for SelectionSpanWriter{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"selection page writer retains original rows and roots until funded closure");}}

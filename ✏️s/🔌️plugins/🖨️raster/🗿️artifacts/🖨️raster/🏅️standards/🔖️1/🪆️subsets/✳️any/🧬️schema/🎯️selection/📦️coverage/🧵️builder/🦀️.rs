//! 🧵️ One granted builder proves full packed prefixes and the exact final remainder while retaining all unfinished custody.
use super::*;
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneClose,retained_clone_grant_admits},shared_issuance::{SealedShared,SharedIssuer}};
use semio_framework_value::persistent::PersistentListBuilder;
use std::mem::ManuallyDrop;

pub struct RasterSelectionSpansBuilder{
    builder:PersistentListBuilder<RasterSpanPage>,page:Option<RasterSpanPage>,sealed:ManuallyDrop<Option<SealedShared<RasterSpanPage>>>,output:ManuallyDrop<Option<RasterSelectionSpans>>,refused:ManuallyDrop<Option<PersistentList<RasterSpanPage>>>,retirement:RetainedCloneClose,expected:usize,accepted:usize,maximum_end:u64,finishing:bool,closing:bool,
}
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"packed selection builder differs from its original declared rows")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
/// 🪢️ Settles one reply-shaped persistent builder action into its recorded result.
fn local<E>(effect:&mut RetainedAllocationEffect<'_>,action:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->Result<Option<Result<RetainedCloneStep,E>>,ValueError>{let mut slot=None;let reply=OriginalEffectReply::borrow(&mut slot)?;action(&mut *effect,reply)?;Ok(slot.take())}
fn posted(copy_bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()}}

impl RasterSelectionSpansBuilder{
    pub fn new(expected:usize)->Result<Self,ValueError>{
        if expected>16_777_216||SealedShared::<RasterSpanPage>::birth_bytes()>RASTER_SELECTION_PAGE_BYTES{return Err(fault());}
        Ok(Self{builder:PersistentListBuilder::new(RasterSelectionSpans::page_count(expected)),page:None,sealed:ManuallyDrop::new(None),output:ManuallyDrop::new(None),refused:ManuallyDrop::new(None),retirement:Default::default(),expected,accepted:0,maximum_end:0,finishing:false,closing:false})
    }
    pub fn accepts_row(&self)->bool{!self.closing&&!self.finishing&&self.accepted<self.expected&&self.builder.accepts_row()&&self.sealed.is_none()&&self.page.as_ref().is_none_or(|page|!page.is_full())}
    pub fn push_demands(&self)->Result<RetirementDemand,ValueError>{if !self.accepts_row(){return Err(fault());}Ok(fixed(if self.page.is_none(){size_of::<RasterSpanPage>()}else{size_of::<RasterSelectionSpan>()+2*size_of::<usize>()}))}
    pub fn push(&mut self,row:&mut Option<RasterSelectionSpan>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        let d=self.push_demands()?;if !retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"packed selection push exceeded its original work"));}
        let span=row.as_ref().ok_or_else(fault)?;if span.length==0||span.coverage>255||u64::from(span.start)<self.maximum_end{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"selection spans must be nonempty, sorted, disjoint and cover at most 255"));}
        let end=u64::from(span.start)+u64::from(span.length);if self.page.is_none(){self.page=Some(RasterSpanPage::new());}else{self.page.as_mut().unwrap().push(row.take().unwrap())?;self.accepted+=1;self.maximum_end=end;}
        Ok(RetainedCloneStep::Progress(posted(d.copy_bytes)))
    }
    pub fn finish_demands(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.accepted!=self.expected{return Err(fault());}Ok(fixed(size_of::<bool>()))}
    pub fn finish(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        if self.finishing{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let d=self.finish_demands()?;if !retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"packed selection finish exceeded its original work"));}
        self.finishing=true;Ok(RetainedCloneStep::Progress(posted(d.copy_bytes)))
    }
    pub fn finishing(&self)->bool{self.finishing}
    pub fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
    pub fn demands(&self)->Result<RetirementDemand,ValueError>{
        if self.closing||self.output_ready(){return Err(fault());}
        if self.builder.output_ready(){return Ok(fixed(size_of::<RasterSelectionSpans>()));}
        if !self.builder.accepts_row(){return self.builder.demand();}
        if self.sealed.is_some(){return self.builder.adopt_demand();}
        if self.page.as_ref().is_some_and(|page|page.is_full()||self.finishing&&page.len()>0){return Ok(RetirementDemand{capacity_bytes:SealedShared::<RasterSpanPage>::birth_bytes(),..fixed(size_of::<RasterSpanPage>())});}
        if self.finishing{return self.builder.finish_demand();}Err(fault())
    }
    pub fn advance_work_demand(&self)->Result<u64,ValueError>{self.demands()?;Ok(if self.builder.output_ready(){2}else if !self.builder.accepts_row(){self.builder.advance_work_demand()?}else{4})}
    pub fn advance(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<Result<RetainedCloneStep,ValueError>>,ValueError>{
        let d=self.demands()?;if !retained_clone_grant_admits(effect.grant(),d){return Ok(Some(Ok(RetainedCloneStep::Progress(Default::default()))));}
        if self.builder.output_ready(){
            let mut pages=None;let Some(settled)=local(effect,|effect,reply|self.builder.take_into(&mut pages,effect,reply))?else{return Ok(None)};
            return Ok(Some(settled.and_then(|step|{let Some(pages)=pages else{return Ok(RetainedCloneStep::Progress(step.progress()))};let mut p=step.progress();match RasterSelectionSpans::from_pages(pages,self.expected,self.maximum_end){Ok(output)=>*self.output=Some(output),Err((error,pages))=>{*self.refused=Some(pages);return Err(error.with_retained_progress(p));}}p.copied_bytes+=size_of::<RasterSelectionSpans>();Ok(RetainedCloneStep::Complete(p))})));
        }
        if !self.builder.accepts_row(){return local(effect,|effect,reply|self.builder.advance_into(effect,reply));}
        if self.sealed.is_some(){return local(effect,|effect,reply|self.builder.adopt_into(&mut self.sealed,effect,reply));}
        if self.page.is_some(){let mut page=self.page.take();let turn=SealedShared::admit_from(&mut page,SharedIssuer::owned(),&mut self.sealed,effect);self.page=page;return Ok(Some(turn.result.map(|step|{let mut p=step.progress();p.copied_bytes+=size_of::<RasterSpanPage>();RetainedCloneStep::Progress(p)})));}
        local(effect,|effect,reply|self.builder.finish_into(effect,reply))
    }
    pub fn take_demands(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(fault());}Ok(fixed(size_of::<Option<RasterSelectionSpans>>()))}
    pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RasterSelectionSpans>,ValueError>{let d=self.take_demands()?;if !retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return Ok(None);}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"packed selection take exceeded its original work"));}Ok(self.output.take())}
    pub fn is_closing(&self)->bool{self.closing}
    pub fn begin_close_demand(&self)->RetirementDemand{if self.closing{Default::default()}else{fixed(size_of::<bool>())}}
    pub fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
    pub fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{if self.closing{return Ok(RetainedCloneStep::Complete(Default::default()));}let d=self.begin_close_demand();if !retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return Ok(RetainedCloneStep::Progress(Default::default()));}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"packed selection close start exceeded its original work"));}self.closing=true;Ok(RetainedCloneStep::Complete(posted(d.copy_bytes)))}
    pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.builder.terminal_is_empty(){return if self.builder.is_closing(){self.builder.close_demand(body)}else{Ok(self.builder.begin_close_demand())};}
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(false,&None,body);}
        if self.sealed.is_some(){return self.retirement.next_owner_demands::<SealedShared<RasterSpanPage>>(true,&None,body);}
        if self.output.is_some(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(true,&None,body);}
        if self.refused.is_some(){return self.retirement.next_owner_demands::<PersistentList<RasterSpanPage>>(true,&None,body);}
        Ok(if self.page.is_some(){fixed(size_of::<Option<RasterSpanPage>>())}else{Default::default()})
    }
    pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.builder.terminal_is_empty(){return if self.builder.is_closing(){self.builder.close_work_demand(body)}else{Ok(self.builder.begin_close_work_demand())}}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<RasterSelectionSpans>(false,&None,body)}if self.sealed.is_some(){return self.retirement.next_owner_work::<SealedShared<RasterSpanPage>>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<RasterSelectionSpans>(true,&None,body)}if self.refused.is_some(){return self.retirement.next_owner_work::<PersistentList<RasterSpanPage>>(true,&None,body)}Ok(u64::from(self.page.is_some()))}
    pub fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_routed(None,body,effect,reply,false)}
    pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_routed(None,body,effect,reply,true)}
    pub fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_routed(Some(original),body,effect,reply,false)}
    pub fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_routed(Some(original),body,effect,reply,true)}
    fn close_routed<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
        if !self.closing{return Ok(reply.complete(Err(fault().into())))}
        let g=effect.grant();let d=self.close_demands(g.maximum_copy_bytes)?;if !recover&&!retained_clone_grant_admits(g,d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
        if !self.builder.terminal_is_empty(){
            if !self.builder.is_closing(){let turn=self.builder.begin_close_paid(effect);return Ok(reply.complete(turn.result.map_err(Into::into)))}
            return match(original,recover){(Some(loan),false)=>self.builder.close_original_into(loan,body,effect,reply),(Some(loan),true)=>self.builder.recover_close_original_into(loan,body,effect,reply),(None,false)=>self.builder.close_into(body,effect,reply),(None,true)=>self.builder.recover_close_into(body,effect,reply)};
        }
        if !self.retirement.is_empty(){return match(original,recover){(Some(loan),false)=>self.retirement.step_original_into(loan,body,effect,reply),(Some(loan),true)=>self.retirement.recover_step_original_into(loan,body,effect,reply),(None,false)=>self.retirement.step_into(body,effect,reply),(None,true)=>self.retirement.recover_step_into(body,effect,reply)}}
        if recover{return Ok(reply.pending())}
        macro_rules! close{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}}}
        close!(sealed);close!(output);close!(refused);
        if self.page.is_some(){if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"packed selection page release exceeded its original work").into())))}self.page=None;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(posted(d.copy_bytes)))))}
        Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))
    }
    pub fn requires_original_birth(&self)->bool{if !self.builder.terminal_is_empty(){return self.builder.requires_original_birth()}self.retirement.requires_original_birth()}
    pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{if !self.builder.terminal_is_empty(){return self.builder.close_original_birth_step(original,grant)}self.retirement.step_original_birth(original,grant)}
    pub fn requires_original_control(&self)->bool{if !self.builder.terminal_is_empty(){return self.builder.requires_original_control()}self.retirement.requires_original_control()}
    pub fn terminal_is_empty(&self)->bool{self.builder.terminal_is_empty()&&self.page.is_none()&&self.sealed.is_none()&&self.output.is_none()&&self.refused.is_none()&&self.retirement.is_empty()}
}
impl Drop for RasterSelectionSpansBuilder{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"packed selection builder retains unfinished pages until funded closure");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

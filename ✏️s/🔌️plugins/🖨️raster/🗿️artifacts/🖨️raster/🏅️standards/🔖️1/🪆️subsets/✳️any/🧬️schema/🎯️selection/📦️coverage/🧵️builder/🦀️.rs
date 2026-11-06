//! 🧵️ One granted builder proves full packed prefixes and the exact final remainder while retaining all unfinished custody.
use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,retained_clone::{RetainedCloneClose,retained_clone_grant_admits},shared_issuance::{SealedShared,SharedIssuer}};
use semio_framework_value::persistent::PersistentListBuilder;
use std::mem::ManuallyDrop;

pub struct RasterSelectionSpansBuilder{
    builder:PersistentListBuilder<RasterSpanPage>,page:Option<RasterSpanPage>,sealed:ManuallyDrop<Option<SealedShared<RasterSpanPage>>>,output:ManuallyDrop<Option<RasterSelectionSpans>>,refused:ManuallyDrop<Option<PersistentList<RasterSpanPage>>>,retirement:RetainedCloneClose,expected:usize,accepted:usize,maximum_end:u64,finishing:bool,closing:bool,
}
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"packed selection builder differs from its original declared rows")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn posted(copy_bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}}

impl RasterSelectionSpansBuilder{
    pub fn new(expected:usize)->Result<Self,ValueError>{
        if expected>16_777_216||SealedShared::<RasterSpanPage>::birth_bytes()>RASTER_SELECTION_PAGE_BYTES{return Err(fault());}
        Ok(Self{builder:PersistentListBuilder::new(RasterSelectionSpans::page_count(expected)),page:None,sealed:ManuallyDrop::new(None),output:ManuallyDrop::new(None),refused:ManuallyDrop::new(None),retirement:Default::default(),expected,accepted:0,maximum_end:0,finishing:false,closing:false})
    }
    pub fn accepts_row(&self)->bool{!self.closing&&!self.finishing&&self.accepted<self.expected&&self.builder.accepts_row()&&self.sealed.is_none()&&self.page.as_ref().is_none_or(|page|!page.is_full())}
    pub fn push_demands(&self)->Result<RetirementDemand,ValueError>{if !self.accepts_row(){return Err(fault());}Ok(fixed(if self.page.is_none(){size_of::<RasterSpanPage>()}else{size_of::<RasterSelectionSpan>()+2*size_of::<usize>()}))}
    pub fn push(&mut self,row:&mut Option<RasterSelectionSpan>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{
        let mut work=0;let result=(||{let d=self.push_demands()?;if !retained_clone_grant_admits(grant,d){return Ok(Default::default());}work=1;let span=row.as_ref().ok_or_else(fault)?;if span.length==0||span.coverage>255||u64::from(span.start)<self.maximum_end{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"selection spans must be nonempty, sorted, disjoint and cover at most 255"));}let end=u64::from(span.start)+u64::from(span.length);if self.page.is_none(){self.page=Some(RasterSpanPage::new());}else{self.page.as_mut().unwrap().push(row.take().unwrap())?;self.accepted+=1;self.maximum_end=end;}Ok(posted(d.copy_bytes))})();RetainedWorkOutcome{result,work}
    }
    pub fn finish_demands(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.accepted!=self.expected{return Err(fault());}Ok(fixed(size_of::<bool>()))}
    pub fn finish(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{
        if self.finishing{return RetainedWorkOutcome{result:Ok(Default::default()),work:0};}
        let result=(||{let d=self.finish_demands()?;if !retained_clone_grant_admits(grant,d){return Ok(Default::default());}self.finishing=true;Ok(posted(d.copy_bytes))})();let work=u64::from(result.as_ref().is_ok_and(|p|p.copied_items!=0));RetainedWorkOutcome{result,work}
    }
    pub fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
    pub fn demands(&self)->Result<RetirementDemand,ValueError>{
        if self.closing||self.output_ready(){return Err(fault());}
        if self.builder.output_ready(){return Ok(fixed(size_of::<RasterSelectionSpans>()));}
        if !self.builder.accepts_row(){return self.builder.demand();}
        if self.sealed.is_some(){return self.builder.adopt_demand();}
        if self.page.as_ref().is_some_and(|page|page.is_full()||self.finishing&&page.len()>0){return Ok(RetirementDemand{capacity_bytes:SealedShared::<RasterSpanPage>::birth_bytes(),..fixed(size_of::<RasterSpanPage>())});}
        if self.finishing{return self.builder.finish_demand();}Err(fault())
    }
    pub fn advance(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
        let mut work=0;let result=(||{
            let d=self.demands()?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
            if self.builder.output_ready(){let turn=self.builder.take(grant);work=turn.work;if let Some((pages,mut p))=turn.result?{match RasterSelectionSpans::from_pages(pages,self.expected,self.maximum_end){Ok(output)=>*self.output=Some(output),Err((error,pages))=>{*self.refused=Some(pages);return Err(error.with_retained_progress(p));}}p.copied_bytes+=size_of::<RasterSelectionSpans>();return Ok(RetainedCloneStep::Complete(p));}return Ok(RetainedCloneStep::Progress(Default::default()));}
            if !self.builder.accepts_row(){let turn=self.builder.advance(grant);work=turn.work;return turn.result;}
            if self.sealed.is_some(){let turn=self.builder.adopt(&mut self.sealed,grant);work=turn.work;return turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default()));}
            if self.page.is_some(){let page=self.page.take().unwrap();let turn=SealedShared::admit(page,SharedIssuer::owned(),grant);work=turn.work;return match turn.result{Ok((sealed,mut p))=>{*self.sealed=Some(sealed);p.copied_bytes+=size_of::<RasterSpanPage>();Ok(RetainedCloneStep::Progress(p))},Err((error,page))=>{self.page=Some(page);Err(error)}};}
            let turn=self.builder.finish(grant);work=turn.work;turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default()))
        })();RetainedWorkOutcome{result,work}
    }
    pub fn take_demands(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(fault());}Ok(fixed(size_of::<Option<RasterSelectionSpans>>()))}
    pub fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(RasterSelectionSpans,RetainedCloneProgress)>>{let result=(||{let d=self.take_demands()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}Ok(Some((self.output.take().unwrap(),posted(d.copy_bytes))))})();let work=u64::from(result.as_ref().is_ok_and(Option::is_some));RetainedWorkOutcome{result,work}}
    pub fn begin_close(&mut self){self.closing=true;self.builder.begin_close();}
    pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.builder.terminal_is_empty(){return self.builder.close_demand(body);}
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(false,&None,body);}
        if self.sealed.is_some(){return self.retirement.next_owner_demands::<SealedShared<RasterSpanPage>>(true,&None,body);}
        if self.output.is_some(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(true,&None,body);}
        if self.refused.is_some(){return self.retirement.next_owner_demands::<PersistentList<RasterSpanPage>>(true,&None,body);}
        Ok(if self.page.is_some(){fixed(size_of::<Option<RasterSpanPage>>())}else{Default::default()})
    }
    pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.builder.terminal_is_empty(){return self.builder.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<RasterSelectionSpans>(false,&None,body)}if self.sealed.is_some(){return self.retirement.next_owner_work::<SealedShared<RasterSpanPage>>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<RasterSelectionSpans>(true,&None,body)}if self.refused.is_some(){return self.retirement.next_owner_work::<PersistentList<RasterSpanPage>>(true,&None,body)}Ok(u64::from(self.page.is_some()))}
    pub fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementOutcome{let grant=effect.grant();
        let mut work=0;let result:Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>=(||{if !self.closing{return Err(fault().into());}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
            if !self.builder.terminal_is_empty(){let turn=self.builder.close_step(effect);work=turn.work;return turn.result;}
            if !self.retirement.is_empty(){let turn=self.retirement.step_granted(effect);work=turn.work;return turn.result;}
            if self.sealed.is_some(){let turn=self.retirement.begin_granted(&mut self.sealed,effect);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into);}
            if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into);}
            if self.refused.is_some(){let turn=self.retirement.begin_granted(&mut self.refused,effect);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into);}
            work=u64::from(self.page.is_some());self.page=None;Ok(RetainedCloneStep::Complete(if work==0{Default::default()}else{posted(d.copy_bytes)}))})();RetainedWorkOutcome{result,work}
    }
    pub fn requires_original_birth(&self)->bool{if !self.builder.terminal_is_empty(){return self.builder.requires_original_birth()}self.retirement.requires_original_birth()}
    pub fn close_original_birth_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{if !self.builder.terminal_is_empty(){return self.builder.close_original_birth_step(original,grant)}self.retirement.step_original_birth(original,grant)}
    pub fn requires_original_control(&self)->bool{if !self.builder.terminal_is_empty(){return self.builder.requires_original_control()}self.retirement.requires_original_control()}
    pub fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementOutcome{if self.requires_original_birth(){return RetainedWorkOutcome{result:Err(fault().into()),work:0}}if !self.builder.terminal_is_empty()&&self.builder.requires_original_control(){return self.builder.close_original_step(original,effect)}if self.builder.terminal_is_empty()&&self.retirement.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}
    pub fn terminal_is_empty(&self)->bool{self.builder.terminal_is_empty()&&self.page.is_none()&&self.sealed.is_none()&&self.output.is_none()&&self.refused.is_none()&&self.retirement.is_empty()}
}
impl Drop for RasterSelectionSpansBuilder{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"packed selection builder retains unfinished pages until funded closure");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

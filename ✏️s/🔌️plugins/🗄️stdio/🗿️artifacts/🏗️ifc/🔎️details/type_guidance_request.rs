//! 📨️ Captures only original donated request args and returns the exact donor after projection closure.
use super::DslValue;
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_plugin::action_staging::StagedFormRequest;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneSourceTake,RetainedCloneStep,retained_clone_grant_admits},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::mem::{ManuallyDrop,size_of};
type Source=RetainedCloneSource<StagedFormRequest>;
type Cursor=<DslValue as RetainedClone>::Cursor;
pub(super) struct RequestCopy{
    source:ManuallyDrop<Option<Source>>,args:ManuallyDrop<Option<DslValue>>,cursor:Cursor,retirement:RetainedCloneClose,binding:Option<OriginalSourceBinding>,phase:u8,closing:bool,
}
impl Default for RequestCopy{fn default()->Self{Self{source:ManuallyDrop::new(None),args:ManuallyDrop::new(None),cursor:Default::default(),retirement:Default::default(),binding:None,phase:0,closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"staging.original-request-copy")}
impl RequestCopy{
    fn metadata()->usize{2*size_of::<Self>()+2*size_of::<StagedFormRequest>()+2*size_of::<DslValue>()}
    pub(super) fn request(&self)->Option<&StagedFormRequest>{self.source.as_ref().and_then(Source::original_authority)}
    pub(super) fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let bytes=Self::metadata();let mut demand=RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()};
        let child=match self.phase{
            0=>{let child=Source::owned_constructor_demand::<()>();RetirementDemand{capacity_bytes:child.capacity_bytes,depth:child.depth,..Default::default()}},
            1=>self.cursor.advance_demands(self.source.as_ref().ok_or_else(invalid)?.borrow().project(0,|request|&request.args),body.saturating_sub(bytes))?,
            3 if !self.cursor.terminal_is_empty()=>self.cursor.close_demands(body.saturating_sub(bytes))?,
            4=>self.source.as_ref().ok_or_else(invalid)?.next_take_demand(body.saturating_sub(bytes))?,
            _=>Default::default(),
        };
        demand.copy_bytes=demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(invalid)?;demand.capacity_bytes=child.capacity_bytes;demand.release_bytes=child.release_bytes;demand.depth=demand.depth.max(child.depth);Ok(demand)
    }
    fn post(cx:&mut StepContext<'_>,mut progress:RetainedCloneProgress)->Result<(),ValueError>{progress.copied_bytes=progress.copied_bytes.checked_add(Self::metadata()).ok_or_else(invalid)?;progress.copied_items=progress.copied_items.max(1);let posted=cx.consume_retained(progress);cx.consume_fuel(progress.copied_items as u64);posted}
    pub(super) fn advance(&mut self,donor:&mut Option<StagedFormRequest>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.closing||self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(invalid().into());}
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"staging request copy cancelled").into());}
        if self.phase==6{return Ok(true);}
        let grant=cx.retained_grant();let demand=self.demands(grant.maximum_copy_bytes.min(4096))?;
        if cx.should_yield()||!retained_clone_grant_admits(grant,demand){return Ok(false);}
        let child_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.min(4096).saturating_sub(Self::metadata()),..grant};
        let progress=match self.phase{
            0=>{
                let original=donor.take().ok_or_else(invalid)?;
                match Source::admit_owned(original,(),child_grant){
                    Ok((source,progress))=>{*self.source=Some(source);self.binding=Some(OriginalSourceBinding::borrow(cx));self.phase=1;progress},
                    Err((error,original,()))=>{*donor=Some(original);let posted=Self::post(cx,Default::default());let mut failure=OriginalTurnFailure::from(error);if let Err(error)=posted{failure.context_refusal=Some(error);}return Err(failure);},
                }
            },
            1=>{let step=self.cursor.advance(self.source.as_ref().ok_or_else(invalid)?.borrow().project(0,|request|&request.args),child_grant)?;if self.cursor.output_ready(){self.phase=2;}step.progress()},
            2=>{if self.args.is_some(){return Err(invalid().into());}*self.args=self.cursor.take();if self.args.is_none(){return Err(invalid().into());}self.cursor.begin_close();self.phase=3;Default::default()},
            3=>{if self.cursor.terminal_is_empty(){self.phase=4;Default::default()}else{self.cursor.close_step(child_grant)?.progress()}},
            4=>{
                if donor.is_some(){return Err(invalid().into());}
                match self.source.as_mut().ok_or_else(invalid)?.take_authority(child_grant)?{
                    RetainedCloneSourceTake::Pending(progress)=>progress,
                    RetainedCloneSourceTake::Ready(original,progress)=>{*donor=Some(original);self.phase=5;progress},
                }
            },
            5=>{if self.source.as_ref().is_none_or(|source|!source.terminal_is_empty()){return Err(invalid().into());}drop(self.source.take());self.phase=6;Default::default()},
            _=>return Err(invalid().into()),
        };
        Self::post(cx,progress)?;Ok(self.phase==6)
    }
    pub(super) fn take_args(&mut self,output:&mut Option<DslValue>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.phase!=6||self.closing||output.is_some()||self.args.is_none()||self.binding.is_none_or(|binding|!binding.matches(cx)){return Err(invalid().into());}
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"staging request args cancelled").into());}
        if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),RetirementDemand{copy_bytes:Self::metadata(),depth:1,..Default::default()}){return Ok(false);}
        *output=self.args.take();Self::post(cx,Default::default())?;Ok(true)
    }
    pub(super) fn begin_close(&mut self){self.closing=true;self.cursor.begin_close();}
    pub(super) fn empty(&self)->bool{self.cursor.terminal_is_empty()&&self.args.is_none()&&self.source.is_none()&&self.retirement.is_empty()}
    pub(super) fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.cursor.terminal_is_empty(){return self.cursor.close_demands(body);}
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,body);}
        if self.args.is_some(){return self.retirement.next_owner_demands::<DslValue>(true,&None,body);}
        if self.source.is_some(){return self.retirement.next_owner_demands::<Source>(true,&None,body);}
        Ok(Default::default())
    }
    pub(super) fn close_owned(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if !self.cursor.terminal_is_empty(){return self.cursor.close_step(grant);}
        if !self.retirement.is_empty(){return self.retirement.step_granted(grant);}
        if self.args.is_some(){return self.retirement.begin_granted(&mut self.args,grant)?.ok_or_else(invalid);}
        if self.source.is_some(){return self.retirement.begin_granted(&mut self.source,grant)?.ok_or_else(invalid);}
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}
impl RetireOwned for RequestCopy{fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn controlled_retirement_supported()->bool{true}}
impl RetirementCursor for RequestCopy{
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if !self.closing{return RetirementStep::BudgetExhausted;}match self.close_owned(grant){Ok(RetainedCloneStep::Complete(progress))if progress==Default::default()&&self.empty()=>RetirementStep::Complete,Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
    fn terminal_is_empty(&self)->bool{self.empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.copy_bytes)}
    fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.copy_bytes)}
    fn allows_admitted_narrow_work(&self)->bool{!self.empty()}
    fn next_birth_bytes(&self,body:usize)->Option<usize>{self.close_demand(body).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.close_demand(4096).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.empty().then_some(size_of::<Self>())}
}
impl Drop for RequestCopy{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"guided request copy requires original paid retirement");if self.empty(){unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.args);}}}}

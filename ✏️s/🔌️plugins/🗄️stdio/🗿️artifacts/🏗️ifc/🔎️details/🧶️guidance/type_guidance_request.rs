//! 📨️ Captures only original donated request args and returns the exact donor after projection closure.
use super::DslValue;
use super::{RetainedWorkOutcome,idle_step,owned_step,settled,effected,retire_self};
use super::source_loan::{advance_cursor,close_cursor,closed_cursor,funded_turn,replied,settle_cursor,take_cursor};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_plugin::action_staging::StagedFormRequest;
use semio_framework_value::{RetainedAllocationEffect,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneClose,RetainedCloneProgress,RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneStep,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
type Source=RetainedCloneSource<StagedFormRequest>;
type Admission=RetainedCloneSourceAdmission<StagedFormRequest,StagedFormRequest>;
type Cursor=<DslValue as RetainedClone>::Cursor;
pub(super) struct RequestCopy{
    admission:ManuallyDrop<Option<Admission>>,returned:ManuallyDrop<Option<StagedFormRequest>>,source:ManuallyDrop<Option<Source>>,args:ManuallyDrop<Option<DslValue>>,cursor:Cursor,retirement:RetainedCloneClose,binding:Option<OriginalSourceBinding>,phase:u8,closing:bool,recover:bool,fuel:u64,
}
impl Default for RequestCopy{fn default()->Self{Self{admission:ManuallyDrop::new(None),returned:ManuallyDrop::new(None),source:ManuallyDrop::new(None),args:ManuallyDrop::new(None),cursor:Default::default(),retirement:Default::default(),binding:None,phase:0,closing:false,recover:false,fuel:1}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"staging.original-request-copy")}
impl RequestCopy{
    fn metadata()->usize{2*size_of::<Self>()+2*size_of::<StagedFormRequest>()+2*size_of::<DslValue>()}
    pub(super) fn request(&self)->Option<&StagedFormRequest>{self.source.as_ref().and_then(Source::original_authority)}
    pub(super) fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let bytes=Self::metadata();let mut demand=RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()};
        let child=match self.phase{
            0=>RetirementDemand{depth:1,..Default::default()},
            7=>self.admission.as_ref().ok_or_else(invalid)?.next_demand()?,
            1=>self.cursor.advance_demands(self.source.as_ref().ok_or_else(invalid)?.borrow().project(0,|request|&request.args),body.saturating_sub(bytes),self.fuel)?,
            2=>self.cursor.take_demand()?,
            3 if !self.cursor.terminal_is_empty()=>self.cursor.close_demands(body.saturating_sub(bytes))?,
            4=>self.source.as_ref().ok_or_else(invalid)?.next_take_demand(body.saturating_sub(bytes))?,
            _=>Default::default(),
        };
        demand.copy_bytes=demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(invalid)?;demand.capacity_bytes=child.capacity_bytes;demand.release_bytes=child.release_bytes;demand.depth=demand.depth.max(child.depth);Ok(demand)
    }
    fn post(cx:&mut StepContext<'_>,fuel:u64)->Result<(),ValueError>{let progress=RetainedCloneProgress{copied_bytes:Self::metadata(),copied_items:1,..Default::default()};let posted=cx.consume_retained(progress);cx.consume_fuel(fuel);posted}
    pub(super) fn advance(&mut self,donor:&mut Option<StagedFormRequest>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.closing||self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(invalid().into());}
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"staging request copy cancelled").into());}
        if self.phase==6{return Ok(true);}
        self.fuel=cx.fuel_remaining();
        let grant=cx.retained_grant();let body=grant.maximum_copy_bytes.min(4096);let demand=self.demands(body)?;
        if cx.should_yield()||!retained_clone_grant_admits(grant,demand){return Ok(false);}
        let child_body=body.saturating_sub(Self::metadata());
        match self.phase{
            0=>{
                let original=donor.take().ok_or_else(invalid)?;
                *self.admission=Some(Source::admission_owned(original,()));self.binding=Some(OriginalSourceBinding::borrow(cx));self.phase=7;Self::post(cx,1)?;
            },
            7=>{
                let admission=self.admission.as_mut().ok_or_else(invalid)?;
                let source=&mut self.source;
                if funded_turn(cx,|effect|if admission.phase()=="ready"{admission.take_output(source,effect)}else{admission.advance(effect)})?.is_none(){return Ok(false);}
                if self.source.is_some(){*self.admission=None;self.phase=1;}
            },
            1=>{advance_cursor(cx,&mut self.cursor,self.source.as_ref().ok_or_else(invalid)?.borrow().project(0,|request|&request.args),child_body,&mut self.recover)?;if self.cursor.output_ready(){self.phase=2;}},
            2=>{
                if self.args.is_some(){return Err(invalid().into());}
                if !take_cursor(cx,&mut self.cursor,&mut self.args)?{return Ok(false);}
                self.phase=3;
            },
            3=>{if self.cursor.terminal_is_empty(){self.phase=4;Self::post(cx,0)?;}else{close_cursor(cx,&mut self.cursor,child_body,&mut self.recover)?;}},
            4=>{
                if donor.is_some(){return Err(invalid().into());}
                let source=self.source.as_mut().ok_or_else(invalid)?;let again=self.recover;
                let Some(reply)=effected(cx,|effect|if again{replied(effect,|effect,reply|source.recover_take_authority_into(donor,child_body,effect,reply))}else{replied(effect,|effect,reply|source.take_authority_into(donor,child_body,effect,reply))})? else{return Ok(false)};
                settle_cursor(cx,reply?,&mut self.recover)?;
                if donor.is_some(){self.phase=5;}
            },
            5=>{if self.source.as_ref().is_none_or(|source|!source.terminal_is_empty()){return Err(invalid().into());}drop(self.source.take());self.phase=6;Self::post(cx,0)?;},
            _=>return Err(invalid().into()),
        }
        Ok(self.phase==6)
    }
    pub(super) fn take_args(&mut self,output:&mut Option<DslValue>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.phase!=6||self.closing||output.is_some()||self.args.is_none()||self.binding.is_none_or(|binding|!binding.matches(cx)){return Err(invalid().into());}
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"staging request args cancelled").into());}
        if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),RetirementDemand{copy_bytes:Self::metadata(),depth:1,..Default::default()}){return Ok(false);}
        *output=self.args.take();Self::post(cx,1)?;Ok(true)
    }
    pub(super) fn begin_close(&mut self){self.closing=true;}
    pub(super) fn empty(&self)->bool{self.cursor.terminal_is_empty()&&self.args.is_none()&&self.source.is_none()&&self.admission.is_none()&&self.returned.is_none()&&self.retirement.is_empty()}
    pub(super) fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.cursor.terminal_is_empty(){return self.cursor.close_demands(body);}
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,body);}
        if let Some(admission)=self.admission.as_ref(){return if admission.phase()=="ready"{admission.next_demand()}else{admission.next_return_demand()};}
        if self.returned.is_some(){return self.retirement.next_owner_demands::<StagedFormRequest>(true,&None,body);}
        if self.args.is_some(){return self.retirement.next_owner_demands::<DslValue>(true,&None,body);}
        if self.source.is_some(){return self.retirement.next_owner_demands::<Source>(true,&None,body);}
        Ok(Default::default())
    }
    pub(super) fn close_owned(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
        if !self.cursor.terminal_is_empty(){return closed_cursor(effect,&mut self.cursor,body,&mut self.recover);}
        if !self.retirement.is_empty(){return settled(self.retirement.step_granted(body,effect));}
        if let Some(admission)=self.admission.as_mut(){
            let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source,effect)}else{let mut inputs=None;let turn=admission.return_inputs(&mut inputs,effect);if let Some((original,()))=inputs{*self.returned=Some(original);}turn};
            if admission.terminal_is_empty()||self.source.is_some(){*self.admission=None;}
            return turn;
        }
        if self.returned.is_some(){return owned_step(self.retirement.begin_granted(&mut self.returned,effect),"guided owner omitted its retirement");}
        if self.args.is_some(){return owned_step(self.retirement.begin_granted(&mut self.args,effect),"guided owner omitted its retirement");}
        if self.source.is_some(){return owned_step(self.retirement.begin_granted(&mut self.source,effect),"guided owner omitted its retirement");}
        idle_step(RetainedCloneStep::Complete(Default::default()))
    }
}
retire_self!([] RequestCopy);
impl Drop for RequestCopy{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"guided request copy requires original paid retirement");if self.empty(){unsafe{ManuallyDrop::drop(&mut self.admission);ManuallyDrop::drop(&mut self.returned);ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.args);}}}}

//! 🪜️ A borrowed source admission keeps its projection frame and unsettled action resident across turns and cancellation.
use semio_framework_value::{OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError};
use semio_framework_value::retained_clone::{OriginalEffectAction,OriginalEffectReply,OriginalEffectReplyComplete,OriginalProjectionFrame,OriginalSourceProjection,RetainedCloneSource,StagedProjectionSelection,StagedSourceAdmission,retained_clone_grant_admits};
use semio_framework_value::retirement::{admit_retirement_cursor,retirement_cursor_birth_demand,retirement_cursor_birth_work,RetireOwned,RetirementCursor,RetirementStep};
use std::mem::size_of;

/// 🧳️ One borrowed original and its resident projection stage.
pub struct SourceStaging<T:RetireOwned+Sync+'static,A:OriginalSourceProjection<T>+RetireOwned+Sync+Send>{admission:StagedSourceAdmission<T,A>,projection:Option<OriginalProjectionFrame<T,A>>,action:Option<OriginalEffectAction<RetainedCloneStep,ValueError>>}
fn fail(error:ValueError)->RetainedWorkOutcome{RetainedWorkOutcome{result:Err(error),work:0}}
fn idle()->RetainedWorkOutcome{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
fn settled(poll:RetainedWorkPoll<RetainedCloneStep,ValueError>)->RetainedWorkOutcome{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}}}
fn funded(effect:&RetainedAllocationEffect<'_>,demand:Result<RetirementDemand,ValueError>,work:Result<u64,ValueError>)->Result<bool,ValueError>{Ok(retained_clone_grant_admits(effect.grant(),demand?)&&effect.remaining_work_units()?>=work?)}
impl<T:RetireOwned+Sync+'static,A:OriginalSourceProjection<T>+RetireOwned+Sync+Send> SourceStaging<T,A>{
    /// 🔗️ Borrows the projection from the original authority.
    pub fn borrowed(original:A)->Self{Self{admission:StagedSourceAdmission::borrowed(original),projection:None,action:None}}
    /// 👓️ Borrows the original authority wherever the staging currently keeps it.
    pub fn original_authority(&self)->Option<&A>{self.admission.original_authority().or_else(||self.projection.as_ref().and_then(OriginalProjectionFrame::original_authority))}
    pub fn terminal_is_empty(&self)->bool{self.admission.terminal_is_empty()&&self.projection.is_none()&&self.action.is_none()}
    fn selection(&self,cancel:bool)->Result<Option<StagedProjectionSelection>,ValueError>{
        Ok(self.admission.projection_selection(&self.projection)?.filter(|selection|!(cancel&&matches!(selection,StagedProjectionSelection::Begin))))
    }
    fn pending_drive(&self,selection:&StagedProjectionSelection)->bool{matches!(selection,StagedProjectionSelection::Drive)&&self.projection.as_ref().is_some_and(OriginalProjectionFrame::has_pending_action)}
    /// 📐️ Quotes the selected projection stage or the plain admission transition.
    pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{
        if self.selection(false)?.is_some(){return self.admission.projection_demand(&self.projection,false)}
        if self.admission.projection_result()==Some(false){return self.admission.projection_result_demand()}
        self.admission.next_demand()
    }
    pub fn next_work_demand(&self)->Result<u64,ValueError>{
        if self.selection(false)?.is_some(){return self.admission.projection_work_demand(&self.projection,false)}
        if self.admission.projection_result()==Some(false){return self.admission.projection_result_work_demand()}
        self.admission.next_work_demand()
    }
    fn drive(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
        let before=effect.performed_work();
        let driven=(||->Result<Option<Result<RetainedCloneStep,ValueError>>,ValueError>{
            let selected=self.projection.as_mut().ok_or_else(||ValueError::invariant("original projection frame absent"))?;
            let mut context=selected.context()?;
            if !selected.authority()?.drive(&mut context,effect,&mut self.action)?{return Ok(None)}
            let mut recorded=None;
            self.action.as_mut().ok_or_else(||ValueError::invariant("original projection action absent"))?.transfer_reply(&mut recorded)?;
            selected.authority()?.finish(&mut self.action)?;
            Ok(Some(recorded.take().ok_or_else(||ValueError::invariant("original projection reply absent"))?))
        })();
        let work=effect.performed_work().saturating_sub(before);
        match driven{
            Ok(None)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work},
            Ok(Some(answer))=>RetainedWorkOutcome{result:answer.map(|step|RetainedCloneStep::Progress(step.progress())),work},
            Err(error)=>RetainedWorkOutcome{result:Err(error),work},
        }
    }
    fn refusal(&mut self,effect:&mut RetainedAllocationEffect<'_>,forward:bool)->RetainedWorkOutcome{
        let mut refusal=None;
        let turn=self.admission.take_projection_failure(&mut refusal,effect);
        RetainedWorkOutcome{result:turn.result.and_then(|step|match(forward,refusal){(true,Some(error))=>Err(error.with_retained_progress(step.progress())),(true,None)=>Err(ValueError::invariant("original projection refusal is absent")),(false,_)=>Ok(step)}),work:turn.work}
    }
    /// 🪡️ Drives one projection stage or advances the plain admission under the turn's original Source effect.
    pub fn advance(&mut self,output:&mut Option<RetainedCloneSource<T,A>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
        match funded(effect,self.next_demand(),self.next_work_demand()){Ok(true)=>{},Ok(false)=>return idle(),Err(error)=>return fail(error)}
        match self.selection(false){
            Err(error)=>fail(error),
            Ok(Some(StagedProjectionSelection::Drive))=>self.drive(effect),
            Ok(Some(_))=>settled(self.admission.projection_step(&mut self.projection,false,effect)),
            Ok(None) if self.admission.projection_result()==Some(false)=>self.refusal(effect,true),
            Ok(None)=>self.admission.advance(output,effect),
        }
    }
    /// 📏️ Quotes the next cancellation transition.
    pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{
        match self.selection(true)?{
            Some(selection) if self.pending_drive(&selection)=>self.admission.projection_demand(&self.projection,false),
            Some(_)=>self.admission.projection_demand(&self.projection,true),
            None if self.admission.projection_result().is_some()=>self.admission.projection_result_demand(),
            None=>self.admission.close_demand(),
        }
    }
    pub fn close_work_demand(&self)->Result<u64,ValueError>{
        match self.selection(true)?{
            Some(selection) if self.pending_drive(&selection)=>self.admission.projection_work_demand(&self.projection,false),
            Some(_)=>self.admission.projection_work_demand(&self.projection,true),
            None if self.admission.projection_result().is_some()=>self.admission.projection_result_work_demand(),
            None=>self.admission.close_work_demand(),
        }
    }
    /// ↩️ Settles the open projection, discards its answer, then returns the inputs or the published source.
    pub fn close_into(&mut self,input:&mut Option<A>,source:&mut Option<RetainedCloneSource<T,A>>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
        match funded(effect,self.close_demand(),self.close_work_demand()){Ok(true)=>{},Ok(false)=>return idle(),Err(error)=>return fail(error)}
        match self.selection(true){
            Err(error)=>fail(error),
            Ok(Some(selection)) if self.pending_drive(&selection)=>self.drive(effect),
            Ok(Some(_))=>settled(self.admission.projection_step(&mut self.projection,true,effect)),
            Ok(None)=>match self.admission.projection_result(){
                Some(true)=>self.admission.discard_projection(effect),
                Some(false)=>self.refusal(effect,false),
                None=>self.admission.close_into(input,source,body,effect),
            },
        }
    }
}
#[doc(hidden)]
pub struct SourceStagingCursor<T:RetireOwned+Sync+'static,A:OriginalSourceProjection<T>+RetireOwned+Sync+Send>{staging:SourceStaging<T,A>,base:Option<A>,source:Option<RetainedCloneSource<T,A>>}
impl<T:RetireOwned+Sync+'static,A:OriginalSourceProjection<T>+RetireOwned+Sync+Send> RetireOwned for SourceStaging<T,A>{
    type RetirementCursor=SourceStagingCursor<T,A>;
    fn retirement_seed(self)->Self::RetirementCursor{SourceStagingCursor{staging:self,base:None,source:None}}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SourceStagingCursor<T,A>>())}
    fn controlled_retirement_supported()->bool{true}
}
fn admit_child<R:RetireOwned>(slot:&mut Option<R>,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{
    let turn=admit_retirement_cursor(slot,effect);
    match turn.result{Ok(Some((cursor,progress)))=>RetirementStep::ChildProgress(cursor,progress,turn.work),Ok(None)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error.into(),turn.work)}
}
impl<T:RetireOwned+Sync+'static,A:OriginalSourceProjection<T>+RetireOwned+Sync+Send> SourceStagingCursor<T,A>{
    fn close_sync(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{
        if !self.staging.terminal_is_empty(){
            let turn=self.staging.close_into(&mut self.base,&mut self.source,body,effect);
            return match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error.into(),turn.work)};
        }
        if effect.grant().maximum_items==0||effect.maximum_work_units()==0{return RetirementStep::BudgetExhausted}
        if self.source.is_some(){return admit_child(&mut self.source,effect)}
        if self.base.is_some(){return admit_child(&mut self.base,effect)}
        RetirementStep::Complete(0)
    }
}
impl<T:RetireOwned+Sync+'static,A:OriginalSourceProjection<T>+RetireOwned+Sync+Send> RetirementCursor for SourceStagingCursor<T,A>{
    fn next_demand(&self,_body:usize)->Result<RetirementDemand,ValueError>{
        if !self.staging.terminal_is_empty(){return self.staging.close_demand()}
        if let Some(source)=self.source.as_ref(){return retirement_cursor_birth_demand(source)}
        if let Some(base)=self.base.as_ref(){return retirement_cursor_birth_demand(base)}
        Ok(Default::default())
    }
    fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{Ok(reply.complete(Ok(self.close_sync(body,effect))))}
    fn recover_close_into<'r>(&mut self,_body:usize,_:&mut RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{Err(ValueError::invariant("synchronous close has no unrecorded callback"))}
    fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{if !self.staging.terminal_is_empty(){return self.staging.close_work_demand()}Ok(if self.source.is_some()||self.base.is_some(){retirement_cursor_birth_work()}else{0})}
    fn terminal_is_empty(&self)->bool{self.staging.terminal_is_empty()&&self.base.is_none()&&self.source.is_none()}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
    fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
}

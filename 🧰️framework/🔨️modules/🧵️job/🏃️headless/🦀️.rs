//! 🏃️ Explicit headless driving retains one finite original source through admission and paid teardown.
use semio_framework_job::{CancelToken,Generation,InteractiveJob,InteractiveJobCloseStep,InteractiveStage,JobOutcomeView,Operation,OperationId,OriginalTurnFailure,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,StepBudget,StepContext};
use semio_framework_job_payload::RetainedJobPayload;
use semio_framework_job_worker::{BatchJobSession,WorkerJobDemandError,WorkerJobPoll,WorkerJobSessionAdmission};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams,WorkerJobCloseStep};
use semio_framework_value::{RetainedWorkOutcome,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement};

#[path="🎟️caller/🦀️.rs"]
mod caller;
use caller::HeadlessCaller;
#[path="📥️capture/🦀️.rs"]
mod capture;
use capture::PayloadCapture;

/// 🎟️ Each business turn narrows the caller's explicit lifetime source.
pub const HEADLESS_GRANT:RetainedCloneGrant=RetainedCloneGrant { maximum_items:64,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:2<<20,maximum_depth:128 };

/// 🧭️ The original caller declares its complete lifetime limits before any job is admitted.
#[derive(Clone,Copy)]
pub struct HeadlessSite {
    pub site:&'static str,
    pub label:&'static str,
    pub stage:InteractiveStage,
    pub fuel_per_step:u64,
    pub step_budget_us:u64,
    pub retained_stock:RetainedCloneGrant,
    pub lifetime_fuel:u64,
    pub lifetime_budget_us:u64,
}

/// 📦️ Reads the completed immutable payload while its original owner remains checked out.
pub fn payload_bytes(payload:&RetainedJobPayload)->Vec<u8> {
    (0..payload.page_count()).flat_map(|index|payload.page(index).expect("retained payload page").iter().copied()).collect()
}

/// ⚖️ Compares an immutable original payload without allocating a temporary copy.
pub fn payload_equals(payload:&RetainedJobPayload,expected:&[u8])->bool {
    if payload.len()!=expected.len() {return false;}
    let mut offset:usize=0;
    for index in 0..payload.page_count() {
        let Some(page)=payload.page(index) else {return false};
        let Some(end)=offset.checked_add(page.len()) else {return false};
        if expected.get(offset..end)!=Some(page) {return false;}
        offset=end;
    }
    offset==expected.len()
}

/// 🪜️ Every failed headless run remains an owner until these same-source turns reach terminal emptiness.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HeadlessCloseStep {
    Progress { progress:RetainedCloneProgress,work:u64 },
    Blocked,
    Complete { progress:RetainedCloneProgress,work:u64 },
    Refused,
}

/// 🧳️ Retains the failed run and any successful output until physical teardown is paid.
pub struct HeadlessFailure<J:InteractiveJob+Send+'static,R=()> {
    message:String,
    state:Option<HeadlessState<J>>,
    output:Option<R>,
}

impl<J:InteractiveJob+Send+'static,R> HeadlessFailure<J,R> {
    pub fn rejected(message:String)->Self { Self {message,state:None,output:None} }
    pub fn message(&self)->&str { &self.message }
    pub fn terminal_is_empty(&self)->bool { self.state.is_none() }
    pub fn original_failure(&self)->Option<&OriginalTurnFailure> { let state=self.state.as_ref()?;state.failures.iter().flatten().next().or_else(||state.session.as_ref()?.checked_out_continuation().and_then(|view|view.callback.or(view.qualification).or(view.receipt_qualification))) }
    pub fn original_continuation(&self)->Option<semio_framework_job::OriginalJobContinuationView<'_>> { self.state.as_ref()?.session.as_ref()?.checked_out_continuation() }
    pub fn take_original_failure(&mut self)->Result<Option<OriginalTurnFailure>,ValueError> {
        let Some(state)=self.state.as_mut() else {return Ok(None)};
        let Some(slot)=state.failures.iter_mut().find(|slot|slot.is_some()) else {return Ok(None)};
        let Some(caller)=state.caller.as_mut() else {return Ok(None)};
        let grant=caller.retirement_grant();
        if grant.maximum_items==0||grant.maximum_depth==0||caller.fuel_remaining()==0 {return Ok(None)}
        caller.pay(RetainedCloneProgress {copied_items:1,..Default::default()},1)?;
        Ok(slot.take())
    }
    pub fn close_step(&mut self)->HeadlessCloseStep {
        let Some(state)=self.state.as_mut() else { return HeadlessCloseStep::Complete {progress:Default::default(),work:0}; };
        let result=state.close_step();
        if state.terminal_is_empty() { self.state=None; }
        result
    }
    pub fn into_result(self)->Result<Result<R,String>,Self> {
        if self.state.is_some() { return Err(self); }
        Ok(match self.output { Some(output)=>Ok(output),None=>Err(self.message) })
    }
    pub fn into_message(self)->Result<String,Self> {
        if self.state.is_some()||self.output.is_some() { return Err(self); }
        Ok(self.message)
    }
    pub fn into_unadmitted_job(mut self)->Result<(J,String),Self> {
        let Some(state)=self.state.as_mut() else { return Err(self); };
        if self.output.is_some()||!state.capture.terminal_is_empty()||state.caller.is_some()||state.params.is_some()||state.session.is_some()||!state.admission.terminal_is_empty()||state.failures.iter().any(Option::is_some)||state.failure_cursor.is_some()||state.unbound_cancel.is_some()||state.cancel_return.is_some() { return Err(self); }
        let Some(job)=state.job.take() else { return Err(self); };
        self.state=None;
        Ok((job,self.message))
    }
}
impl<J:InteractiveJob+Send+'static,R> std::fmt::Debug for HeadlessFailure<J,R> {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { f.debug_struct("HeadlessFailure").field("message",&self.message).field("terminal_is_empty",&self.terminal_is_empty()).field("has_output",&self.output.is_some()).finish() }
}
impl<J:InteractiveJob+Send+'static,R> std::fmt::Display for HeadlessFailure<J,R> {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { self.message.fmt(f) }
}
impl<J:InteractiveJob+Send+'static,R> std::error::Error for HeadlessFailure<J,R> {}

#[derive(Clone,Copy)]
enum Seen { Empty,Pending,Complete,Cancelled,Fault }
struct HeadlessState<J:InteractiveJob+Send+'static> {
    site:HeadlessSite,
    caller:Option<HeadlessCaller>,
    job:Option<J>,
    params:Option<BatchJobParams>,
    admission:WorkerJobSessionAdmission<J,BatchJobSession<J>>,
    session:Option<BatchJobSession<J>>,
    cancel_return:Option<semio_framework_async::CancelTokenRetirement>,
    unbound_cancel:Option<CancelToken>,
    job_closing:bool,
    capture:PayloadCapture,
    failures:[Option<OriginalTurnFailure>;2],
    failure_cursor:Option<ControlledRetirement<OriginalTurnFailure>>,
}

impl<J:InteractiveJob+Send+'static> HeadlessState<J> {
    fn new(job:J,site:HeadlessSite)->Self {
        Self {site,caller:None,job:Some(job),params:None,admission:WorkerJobSessionAdmission::new(),session:None,cancel_return:None,unbound_cancel:None,job_closing:false,capture:PayloadCapture::new(),failures:[None,None],failure_cursor:None}
    }
    fn terminal_is_empty(&self)->bool {
        self.capture.terminal_is_empty()&&self.job.is_none()&&self.params.is_none()&&self.admission.terminal_is_empty()&&self.session.is_none()&&self.cancel_return.is_none()&&self.unbound_cancel.is_none()&&self.failures.iter().all(Option::is_none)&&self.failure_cursor.is_none()&&self.caller.as_ref().is_none_or(HeadlessCaller::terminal_is_empty)
    }
    fn remember(&mut self,failure:OriginalTurnFailure) {
        let slot=self.failures.iter_mut().find(|slot|slot.is_none()).expect("a headless operation reserves a receiving slot before work");
        *slot=Some(failure);
    }
    fn turn<T:Copy>(&mut self,run:impl FnOnce(&mut StepContext<'_>,&mut WorkerJobSessionAdmission<J,BatchJobSession<J>>,&mut Option<J>,&mut Option<BatchJobParams>,&mut Option<BatchJobSession<J>>)->Result<T,OriginalTurnFailure>)->Result<T,String> {
        let Self {caller,admission,job,params,session,site,..}=self;
        let result=caller.as_mut().expect("original headless caller").turn(site,false,|cx|run(cx,admission,job,params,session));
        match result {Ok(value)=>Ok(value),Err(failure)=>{self.remember(failure);Err(format!("{}-original-turn-refused",self.site.label))}}
    }
    fn prepare<R>(&mut self,operation:Operation,now_us:fn()->Option<u64>,supervise:&mut impl FnMut(&mut J)->Option<R>,early:&mut Option<R>)->Result<(),String> {
        self.caller=Some(HeadlessCaller::new(operation,now_us,&self.site)?);
        for _ in 0..16 {
            match self.caller.as_mut().expect("original headless caller").admit_step() {
                Ok(true)=>break,
                Ok(false)=>{},
                Err(error)=>{self.remember(error.into());return Err(format!("{}-source-admission-refused",self.site.label));}
            }
        }
        if !self.caller.as_ref().is_some_and(HeadlessCaller::is_ready) { return Err(format!("{}-source-admission-pending",self.site.label)); }
        *early=supervise(self.job.as_mut().expect("original unadmitted job"));
        if early.is_some() {return Ok(());}
        for _ in 0..16 {
            let Self {caller,site,unbound_cancel,..}=self;
            let result=caller.as_mut().expect("original headless caller").turn(site,false,|cx|cx.admit_original_cancel_alias(unbound_cancel));
            match result {Ok(true)=>break,Ok(false)=>{},Err(failure)=>{self.remember(failure);return Err(format!("{}-cancellation-alias-refused",self.site.label));}}
        }
        if self.unbound_cancel.is_none() { return Err(format!("{}-cancellation-alias-pending",self.site.label)); }
        {
            let Self {caller,site,params,unbound_cancel,..}=self;
            let result=caller.as_mut().expect("original headless caller").turn(site,false,|cx|{
                semio_framework_job::close_original_granted_step(cx,|_|{
                    let alias=unbound_cancel.take().expect("original headless cancellation alias");
                    *params=Some(BatchJobParams {operation:operation.operation,generation:operation.generation,cancel:alias,config:BatchDriveConfig {retained:HEADLESS_GRANT,site:site.site,stage:site.stage,fuel_per_step:site.fuel_per_step,step_budget_us:site.step_budget_us},now_us});
                    RetainedWorkOutcome {result:Ok(InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress {copied_items:1,..Default::default()}}),work:1}
                })?;
                Ok(())
            });
            if let Err(failure)=result {self.remember(failure);return Err(format!("{}-parameter-staging-refused",self.site.label));}
        }
        for _ in 0..4096 {
            if self.turn(|cx,admission,job,params,session|admission.advance(job,params,session,cx))? { return Ok(()); }
        }
        Err(format!("{}-worker-admission-pending",self.site.label))
    }
    fn capture_step(&mut self)->Result<bool,String> {
        let Self {caller,session,capture,site,..}=self;
        let result=caller.as_mut().expect("original headless caller").turn(site,false,|cx|{
            let payload=match session.as_ref().expect("checked out headless worker").checked_out_outcome()? {
                Some(JobOutcomeView::Complete {output,..})=>output,
                Some(JobOutcomeView::Fault {detail,..})=>Some(detail),
                _=>None,
            };
            capture.advance(payload,cx)
        });
        match result {Ok(done)=>Ok(done),Err(failure)=>{self.remember(failure);Err(format!("{}-terminal-capture-refused",self.site.label))}}
    }
    fn drive<R>(&mut self,mut supervise:impl FnMut(&mut J)->Option<R>,finish:impl FnOnce(&mut J,HeadlessOutcome<'_>)->Result<R,String>)->Result<R,String> {
        let mut finish=Some(finish);
        let mut stagnant=0;
        loop {
            let before=self.caller.as_ref().expect("original headless caller").receipt();
            let fuel_before=self.caller.as_ref().expect("original headless caller").fuel_remaining();
            let poll=self.turn(|cx,_,_,_,session|session.as_mut().expect("admitted headless worker").step_on_original_caller(cx))?;
            let caller=self.caller.as_ref().expect("original headless caller");
            if caller.receipt()==before&&caller.fuel_remaining()==fuel_before {stagnant+=1;} else {stagnant=0;}
            if stagnant>=16 {return Err(format!("{}-original-work-blocked",self.site.label));}
            if !matches!(poll,WorkerJobPoll::Outcome|WorkerJobPoll::Terminal) {continue;}
            let session=self.session.as_mut().expect("admitted headless worker");
            if !session.checkout_outcome() {continue;}
            let Some((issued,progress))=session.take_checked_out_retained_step_receipt() else {return Err(format!("{}-omitted-step-receipt",self.site.label));};
            if !progress.fits(issued) {return Err(format!("{}-step-receipt-exceeded-grant",self.site.label));}
            if session.checked_out_continuation().is_some() {return Err(format!("{}-original-continuation-owned",self.site.label));}
            let seen=match session.checked_out_outcome() {
                Ok(None)=>Seen::Empty,
                Ok(Some(JobOutcomeView::Yield {..}|JobOutcomeView::PreviewReady {..}|JobOutcomeView::CheckpointReady {..}))=>Seen::Pending,
                Ok(Some(JobOutcomeView::Complete {..}))=>Seen::Complete,
                Ok(Some(JobOutcomeView::Cancelled {..}))=>Seen::Cancelled,
                Ok(Some(JobOutcomeView::Fault {..}))=>Seen::Fault,
                Err(error)=>{self.remember(error.into());return Err(format!("{}-outcome-refused",self.site.label));}
            };
            if matches!(seen,Seen::Complete|Seen::Fault) {
                let mut blocked=0;
                loop {
                    let caller=self.caller.as_ref().unwrap();
                    let before=(caller.receipt(),caller.fuel_remaining());
                    if self.capture_step()? {break;}
                    let caller=self.caller.as_ref().unwrap();
                    if before==(caller.receipt(),caller.fuel_remaining()) {blocked+=1;} else {blocked=0;}
                    if blocked>=16 {return Err(format!("{}-terminal-capture-blocked",self.site.label));}
                }
            }
            let mut acknowledged=false;
            for _ in 0..4096 {
                if self.turn(|cx,_,_,_,session|session.as_mut().expect("admitted headless worker").acknowledge_outcome(cx))? {acknowledged=true;break;}
            }
            if !acknowledged {return Err(format!("{}-outcome-acknowledgement-pending",self.site.label));}
            let job=self.session.as_mut().expect("admitted headless worker").checked_out_job_mut().ok_or_else(||format!("{}-acknowledged-without-job",self.site.label))?;
            match seen {
                Seen::Pending|Seen::Empty=>{
                    if matches!(seen,Seen::Pending) {if let Some(result)=supervise(job) {return Ok(result);}}
                    self.session.as_mut().expect("admitted headless worker").resume().map_err(|_|format!("{}-resume-refused",self.site.label))?;
                },
                terminal=>{
                    let bytes=self.capture.bytes.as_deref().unwrap_or_default();
                    let outcome=match terminal {Seen::Complete=>HeadlessOutcome::Complete(bytes),Seen::Fault=>HeadlessOutcome::Fault(bytes),Seen::Cancelled=>HeadlessOutcome::Cancelled,Seen::Pending|Seen::Empty=>unreachable!()};
                    return finish.take().expect("one headless completion")(job,outcome);
                }
            }
        }
    }
    fn close_business_step(&mut self)->Result<(),OriginalTurnFailure> {
        let Self {caller,admission,job,params,session,cancel_return,unbound_cancel,job_closing,capture,site,..}=self;
        let caller=caller.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"unadmitted headless job still belongs to its original caller"))?;
        if !caller.is_ready() {
            caller.admit_step_retiring()?;
            return Ok(());
        }
        caller.turn(site,true,|cx|{
            if !capture.terminal_is_empty() {capture.close_step(cx)?;return Ok(());}
            if !admission.terminal_is_empty() { admission.close_step(cx)?;return Ok(()); }
            if let Some(owner)=session.as_mut() {
                if owner.terminal_is_empty() {
                    semio_framework_job::close_original_granted_step(cx,|_|{*session=None;RetainedWorkOutcome {result:Ok(InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress {copied_items:1,..Default::default()}}),work:1}})?;
                    return Ok(());
                }
                owner.begin_close();
                let witness=owner.has_original_cancel_alias_witness(cx.original_cancel_token()).map_err(|error|match error {WorkerJobDemandError::Refused(error)=>OriginalTurnFailure::from(error),WorkerJobDemandError::Contention(_)=>ValueError::literal(ValueRefusalKind::WorkLimit,"headless worker close is contended").into()})?;
                if witness {
                    let turn=owner.return_original_cancel_alias_step(cx.original_cancel_token(),cx.retained_grant());
                    let result=match turn.result {Ok(Some(step))=>Ok(step),Ok(None)|Err(WorkerJobDemandError::Contention(_))=>Ok(RetainedCloneStep::Progress(Default::default())),Err(WorkerJobDemandError::Refused(error))=>Err(error)};
                    semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome {result,work:turn.work})?;
                } else { owner.close_step(cx)?; }
                return Ok(());
            }
            if let Some(owner)=job.as_mut() {
                if owner.terminal_is_empty() {
                    semio_framework_job::close_original_granted_step(cx,|_|{*job=None;RetainedWorkOutcome {result:Ok(InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress {copied_items:1,..Default::default()}}),work:1}})?;
                } else if !*job_closing {
                    semio_framework_job::close_original_granted_step(cx,|_|{owner.begin_close();*job_closing=true;RetainedWorkOutcome {result:Ok(InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress {copied_items:1,..Default::default()}}),work:1}})?;
                } else { owner.close_step(cx)?; }
                return Ok(());
            }
            if params.is_some() {
                semio_framework_job::close_original_granted_step(cx,|_|{let params=params.take().expect("original headless parameters");*cancel_return=Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));RetainedWorkOutcome {result:Ok(InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress {copied_items:1,..Default::default()}}),work:1}})?;
                return Ok(());
            }
            if unbound_cancel.is_some() {
                semio_framework_job::close_original_granted_step(cx,|_|{*cancel_return=Some(semio_framework_async::CancelTokenRetirement::from_token(unbound_cancel.take().expect("unadmitted original cancellation alias")));RetainedWorkOutcome {result:Ok(InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress {copied_items:1,..Default::default()}}),work:1}})?;
                return Ok(());
            }
            if let Some(owner)=cancel_return.as_mut() {
                if owner.terminal_is_empty() {
                    semio_framework_job::close_original_granted_step(cx,|_|{*cancel_return=None;RetainedWorkOutcome {result:Ok(InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress {copied_items:1,..Default::default()}}),work:1}})?;
                } else {
                    let turn=owner.return_alias_step(cx.original_cancel_token(),cx.retained_grant());
                    let result=match turn.result {Ok(step)=>Ok(step),Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>Ok(RetainedCloneStep::Progress(Default::default())),Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>Err(error)};
                    semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome {result,work:turn.work})?;
                }
            }
            Ok(())
        })
    }
    fn close_failure_step(&mut self)->Result<bool,ValueError> {
        let Some(caller)=self.caller.as_mut() else { return Ok(false); };
        let grant=caller.retirement_grant();
        if grant.maximum_items==0||grant.maximum_depth==0||caller.fuel_remaining()==0 { return Ok(false); }
        if let Some(cursor)=self.failure_cursor.as_mut() {
            if cursor.terminal_is_empty() {
                self.failure_cursor=None;
                caller.pay(RetainedCloneProgress {copied_items:1,..Default::default()},1)?;
                return Ok(true);
            }
            let result=cursor.step(grant);let work=cursor.last_turn_work();
            match result {
                Ok(step)=>{let progress=step.progress();caller.pay(progress,work)?;Ok(progress!=RetainedCloneProgress::default()||work!=0)}
                Err(error)=>{caller.pay(error.retained_progress(),work)?;Err(error)}
            }
        } else if let Some(slot)=self.failures.iter_mut().find(|slot|slot.is_some()) {
            self.failure_cursor=Some(ControlledRetirement::retain(slot.take().expect("retained original failure")));
            caller.pay(RetainedCloneProgress {copied_items:1,..Default::default()},1)?;
            Ok(true)
        } else { Ok(false) }
    }
    fn close_step(&mut self)->HeadlessCloseStep {
        if self.terminal_is_empty() { return HeadlessCloseStep::Complete {progress:Default::default(),work:0}; }
        if self.caller.is_none() { return HeadlessCloseStep::Blocked; }
        if self.failure_cursor.is_some()&&self.failures.iter().all(Option::is_some) {return HeadlessCloseStep::Refused;}
        let before=self.caller.as_ref().expect("original headless caller").receipt();
        let fuel=self.caller.as_ref().expect("original headless caller").fuel_remaining();
        let business=self.job.is_some()||self.params.is_some()||!self.admission.terminal_is_empty()||self.session.is_some()||self.cancel_return.is_some()||self.unbound_cancel.is_some();
        let result=if self.failures.iter().any(Option::is_some)||self.failure_cursor.is_some() {self.close_failure_step().map(|_|()).map_err(Into::into)} else if business {self.close_business_step()} else {self.caller.as_mut().expect("original headless caller").close_step().map(|_|()).map_err(Into::into)};
        if let Err(failure)=result {self.remember(failure);return HeadlessCloseStep::Refused;}
        let caller=self.caller.as_ref().expect("original headless caller");
        let after=caller.receipt();
        let progress=RetainedCloneProgress {copied_items:after.copied_items.saturating_sub(before.copied_items),copied_bytes:after.copied_bytes.saturating_sub(before.copied_bytes),retained_capacity_bytes:after.retained_capacity_bytes.saturating_sub(before.retained_capacity_bytes),released_bytes:after.released_bytes.saturating_sub(before.released_bytes)};
        let work=fuel.saturating_sub(caller.fuel_remaining());
        if self.terminal_is_empty() {HeadlessCloseStep::Complete {progress,work}} else if progress==RetainedCloneProgress::default()&&work==0 {HeadlessCloseStep::Blocked} else {HeadlessCloseStep::Progress {progress,work}}
    }
}
impl<J:InteractiveJob+Send+'static> Drop for HeadlessState<J> {
    fn drop(&mut self) { assert!(std::thread::panicking()||self.terminal_is_empty(),"headless run retains its genuine original source or partial job"); }
}

/// 📬️ Terminal kind and captured bytes remain borrowed from the same original run.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HeadlessOutcome<'a> {Complete(&'a [u8]),Cancelled,Fault(&'a [u8])}

/// 🏁️ Drives complete output while retaining any unfinished original teardown.
pub fn run_headless<J,R>(job:J,operation:Operation,now_us:fn()->Option<u64>,site:HeadlessSite,finish:impl FnOnce(&mut J,&[u8])->Result<R,String>)->Result<R,HeadlessFailure<J,R>>
where J:InteractiveJob+Send+'static {
    run_headless_supervised(job,operation,now_us,site,|_|None,|job,outcome|match outcome {
        HeadlessOutcome::Complete(bytes)=>finish(job,bytes),
        HeadlessOutcome::Cancelled=>Err(format!("{}-cancelled",site.label)),
        HeadlessOutcome::Fault(_)=>Err(format!("{}-fault",site.label)),
    })
}

/// 🎛️ Supervises genuine acknowledged yields and preserves terminal outcomes before later stop requests.
pub fn run_headless_supervised<J,R>(job:J,operation:Operation,now_us:fn()->Option<u64>,site:HeadlessSite,mut supervise:impl FnMut(&mut J)->Option<R>,finish:impl FnOnce(&mut J,HeadlessOutcome<'_>)->Result<R,String>)->Result<R,HeadlessFailure<J,R>>
where J:InteractiveJob+Send+'static {
    let mut state=HeadlessState::new(job,site);
    let mut early=None;
    let result=state.prepare(operation,now_us,&mut supervise,&mut early).and_then(|()|match early.take(){Some(result)=>Ok(result),None=>state.drive(supervise,finish)});
    for _ in 0..2_000_000 {
        if state.failures.iter().any(Option::is_some)||state.session.as_ref().is_some_and(|session|session.checked_out_continuation().is_some()) {break;}
        if state.terminal_is_empty() {break;}
        match state.close_step() {HeadlessCloseStep::Progress {..}|HeadlessCloseStep::Complete {..}=>{},HeadlessCloseStep::Blocked|HeadlessCloseStep::Refused=>break}
    }
    if state.terminal_is_empty() {return result.map_err(|message|HeadlessFailure {message,state:None,output:early});}
    let (message,output)=match result {Ok(output)=>(format!("{}-teardown-retains-original-owner",site.label),Some(output)),Err(message)=>(message,early)};
    Err(HeadlessFailure {message,state:Some(state),output})
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

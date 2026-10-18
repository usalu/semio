//! 🔬️ Lifecycle probe and refused source: the relay lifecycle probe control and job and the refused source with its interactive job implementation. See [`super`].

use super::*;

//#region 🔀️PostTurnRelay
pub(super) struct GuestRelayLifecycleProbeControl {
    awake: AtomicBool,
    release_permits: std::sync::atomic::AtomicUsize,
    pub(super) waker: Mutex<Option<std::task::Waker>>,
    pub(super) close_admissions: std::sync::atomic::AtomicUsize,
    pub(super) releases: std::sync::atomic::AtomicUsize,
}

impl GuestRelayLifecycleProbeControl {
    pub(crate) fn new() -> Self {
        Self { awake: AtomicBool::new(false), release_permits: std::sync::atomic::AtomicUsize::new(0), waker: Mutex::new(None), close_admissions: std::sync::atomic::AtomicUsize::new(0), releases: std::sync::atomic::AtomicUsize::new(0) }
    }

    pub(crate) fn wake(&self) {
        self.awake.store(true, std::sync::atomic::Ordering::Release);
        self.wake_registered();
    }

    pub(super) fn release_one(&self) {
        self.release_permits.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.wake_registered();
    }

    pub(super) fn wake_registered(&self) {
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }
}

pub(super) struct GuestRelayLifecycleProbeJob {
    pub(super) control: Arc<GuestRelayLifecycleProbeControl>,
    pub(super) remaining: usize,
    pub(super) terminal_output: Option<Vec<u8>>,
    pub(super) publication: Option<GuestRelayPublication>,
    pub(super) closing: bool,
}

impl GuestRelayLifecycleProbeJob {
    fn publication_demand(&self)->Option<Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>>{
        let publication=self.publication.as_ref()?;
        if publication.terminal_is_empty(){return Some(Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<GuestRelayPublication>>(),depth:1,..Default::default()}));}
        Some(publication.retirement_demands().and_then(|mut demand|{demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"lifecycle probe publication depth overflow"))?;Ok(demand)}))
    }
}

impl semio_framework_job::InteractiveJob for GuestRelayLifecycleProbeJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if self.publication.is_none() {
            let Some(output) = self.terminal_output.take() else { return semio_framework_job::JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from) };
            self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Commit, output));
            return Ok(None);
        }
        let publication = self.publication.as_mut().expect("lifecycle probe publication was just observed");
        if publication.delivered {
            return semio_framework_job::JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        publication.step(context)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            _ => self.publication.as_ref().ok_or_else(|| GuestRelayPublication::invalid("lifecycle probe descriptor requires its publication"))?.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        if !self.closing {
            self.closing = true;
            self.control.close_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as S;
        let grant=cx.retained_grant();
        if cx.should_yield(){return Ok(S::Pending{progress:Default::default()})}
        if let Some(publication)=self.publication.as_mut(){
            if publication.terminal_is_empty(){
                let bytes=size_of::<Option<GuestRelayPublication>>();
                if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth<1{return Ok(S::Pending{progress:Default::default()});}
                let progress=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};cx.consume_retained(progress)?;
                drop(self.publication.take());cx.consume_fuel(1);return Ok(S::Pending{progress});
            }
            if grant.maximum_depth==0{return Ok(S::Pending{progress:Default::default()});}
            return receive_relay_close(publication.close_step(semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|match step{S::Complete{progress}=>S::Pending{progress},step=>step}),cx);
        }
        if self.remaining==0{return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original Host lifecycle control retirement is undeclared"),cx))}
        if !self.control.awake.load(std::sync::atomic::Ordering::Acquire){return Ok(S::Blocked);}
        let bytes=size_of::<usize>();
        if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth<1{return Ok(S::Pending{progress:Default::default()});}
        if self.control.release_permits.try_update(std::sync::atomic::Ordering::AcqRel,std::sync::atomic::Ordering::Acquire,|permits|permits.checked_sub(1)).is_err(){return Ok(S::Blocked);}
        self.remaining-=1;self.control.releases.fetch_add(1,std::sync::atomic::Ordering::SeqCst);
        receive_relay_close(Ok(S::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}),cx)
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{if let Some(demand)=self.publication_demand(){return Ok(demand?.copy_bytes);}if self.remaining==0{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original lifecycle control Arc, wake and output owners require native retirement"));}Ok(size_of::<usize>())}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{if let Some(demand)=self.publication_demand(){return Ok(demand?.capacity_bytes);}self.next_close_copy_byte_demand().map(|_|0)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{if let Some(demand)=self.publication_demand(){return Ok(demand?.release_bytes);}self.next_close_copy_byte_demand().map(|_|0)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{if let Some(demand)=self.publication_demand(){return Ok(demand?.depth);}self.next_close_copy_byte_demand().map(|_|1)}

    fn terminal_is_empty(&self)->bool{false}
}

/// 🧾️ The original job and parameters a refused worker admission leaves with the registry until their paid close.
pub(super) struct GuestRelayRefusedSource<J: semio_framework_job::InteractiveJob> {
    pub(super) job: Option<J>,
    pub(super) params: Option<semio_framework_job_worker_authority::BatchJobParams>,
    pub(super) closing: bool,
}

impl<J: semio_framework_job::InteractiveJob> GuestRelayRefusedSource<J> {
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.params.is_none()
    }

    pub(crate) fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(job) = self.job.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: job.next_close_copy_byte_demand()?, capacity_bytes: job.next_close_capacity_byte_demand(maximum_copy_bytes)?, release_bytes: job.next_close_release_byte_demand()?, depth: job.next_close_depth_demand()? });
        }
        Ok(semio_framework_value::RetirementDemand { depth: usize::from(self.params.is_some()), ..Default::default() })
    }

    pub(crate) fn begin_close(&mut self) {
        if !self.closing {
            self.closing = true;
            if let Some(job) = self.job.as_mut() {
                job.begin_close();
            }
        }
    }

    pub(crate) fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as S;
        let grant=cx.retained_grant();
        self.begin_close();
        if cx.should_yield(){return Ok(S::Pending{progress:Default::default()})}
        let item = semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if let Some(job) = self.job.as_mut() {
            if job.terminal_is_empty() {
                if grant.maximum_items == 0 {
                    return Ok(S::Pending { progress: Default::default() });
                }
                cx.consume_retained(item)?;drop(self.job.take());cx.consume_fuel(1);
                return Ok(S::Pending { progress: item });
            }
            let step=job.close_step(cx)?.admit(grant,job.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
            return Ok(match step {
                S::Complete { progress } => S::Pending { progress },
                step => step,
            });
        }
        if self.params.is_some() {
            if grant.maximum_items == 0 {
                return Ok(S::Pending { progress: Default::default() });
            }
            cx.consume_retained(item)?;drop(self.params.take());cx.consume_fuel(1);
            return Ok(S::Pending { progress: item });
        }
        Ok(S::Complete { progress: Default::default() })
    }
}
//#endregion 🔀️PostTurnRelay

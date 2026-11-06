//! 🎟️ One explicit headless source owns every birth, bounded turn and paid retirement.
use super::*;
use semio_framework_async::CancelTokenRetirement;
use semio_framework_job::{ClockStride,OriginalCallerSourceParent};
use std::sync::atomic::{AtomicU64,Ordering};

static NEXT_CLOCK_BINDING:AtomicU64=AtomicU64::new(1);

pub(super) struct HeadlessCaller {
    authority_actual:Option<usize>,
    parent:Option<OriginalCallerSourceParent>,
    authority:Option<semio_framework_job_payload::JobPayloadAuthority>,
    cancel:Option<CancelToken>,
    cancel_retirement:Option<CancelTokenRetirement>,
    operation:Operation,
    now_us:fn()->Option<u64>,
    deadline:u64,
    binding:u64,
    sequence:u64,
    stride:ClockStride,
    stock:RetainedCloneGrant,
    spent:RetainedCloneProgress,
    fuel:u64,
    reserve_fuel:u64,
    reserve:RetainedCloneGrant,
}

fn refusal(message:&'static str)->ValueError { ValueError::literal(ValueRefusalKind::WorkLimit,message) }
fn remaining(stock:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant {
    RetainedCloneGrant { maximum_items:stock.maximum_items.saturating_sub(spent.copied_items), maximum_copy_bytes:stock.maximum_copy_bytes.saturating_sub(spent.copied_bytes), maximum_capacity_bytes:stock.maximum_capacity_bytes.saturating_sub(spent.retained_capacity_bytes), maximum_release_bytes:stock.maximum_release_bytes.saturating_sub(spent.released_bytes), ..stock }
}
fn narrow(a:RetainedCloneGrant,b:RetainedCloneGrant)->RetainedCloneGrant {
    RetainedCloneGrant { maximum_items:a.maximum_items.min(b.maximum_items), maximum_copy_bytes:a.maximum_copy_bytes.min(b.maximum_copy_bytes), maximum_capacity_bytes:a.maximum_capacity_bytes.min(b.maximum_capacity_bytes), maximum_release_bytes:a.maximum_release_bytes.min(b.maximum_release_bytes), maximum_depth:a.maximum_depth.min(b.maximum_depth) }
}

impl HeadlessCaller {
    pub(super) fn new(operation:Operation,now_us:fn()->Option<u64>,site:&HeadlessSite)->Result<Self,String> {
        let now=now_us().ok_or_else(||format!("{}-clock-unavailable",site.label))?;
        let deadline=now.checked_add(site.lifetime_budget_us).ok_or_else(||format!("{}-deadline-overflow",site.label))?;
        if site.lifetime_budget_us==0||site.lifetime_fuel<4||site.retained_stock.maximum_items<4||site.retained_stock.maximum_depth==0 {
            return Err(format!("{}-source-budget-refused",site.label));
        }
        let binding=NEXT_CLOCK_BINDING.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).map_err(|_|format!("{}-clock-binding-exhausted",site.label))?;
        let stock=site.retained_stock;
        let reserve=RetainedCloneGrant { maximum_items:stock.maximum_items/2, maximum_copy_bytes:stock.maximum_copy_bytes/2, maximum_capacity_bytes:stock.maximum_capacity_bytes/2, maximum_release_bytes:stock.maximum_release_bytes/2, ..stock };
        Ok(Self { authority_actual:None,parent:None,authority:None,cancel:None,cancel_retirement:None,operation,now_us,deadline,binding,sequence:operation.preview_sequence,stride:ClockStride::new(),stock,spent:Default::default(),fuel:site.lifetime_fuel,reserve_fuel:site.lifetime_fuel/2,reserve })
    }

    pub(super) fn receipt(&self)->RetainedCloneProgress { self.spent }
    pub(super) fn fuel_remaining(&self)->u64 { self.fuel }
    pub(super) fn is_ready(&self)->bool { self.parent.is_some() }
    pub(super) fn terminal_is_empty(&self)->bool { self.parent.is_none()&&self.authority.is_none()&&self.cancel.is_none()&&self.cancel_retirement.is_none() }
    pub(super) fn retirement_grant(&self)->RetainedCloneGrant {
        let left=remaining(self.stock,self.spent);
        RetainedCloneGrant { maximum_items:left.maximum_items.min(1),maximum_copy_bytes:left.maximum_copy_bytes.min(HEADLESS_GRANT.maximum_copy_bytes),..left }
    }
    pub(super) fn pay(&mut self,progress:RetainedCloneProgress,work:u64)->Result<(),ValueError> {
        if self.authority_actual.is_some_and(|actual|actual!=progress.retained_capacity_bytes){return Err(refusal("original headless authority attempt differs from its receipt"))}let enough=self.fuel>=work;
        self.fuel=self.fuel.saturating_sub(work);
        self.spent=self.spent.checked_add(progress)?;
        if !self.spent.fits(self.stock)||!enough { return Err(refusal("headless original lifetime receipt exceeded its source")); }
        self.authority_actual=None;Ok(())
    }

    pub(super) fn admit_step(&mut self)->Result<bool,ValueError> { self.admit_step_inner(false) }
    pub(super) fn admit_step_retiring(&mut self)->Result<bool,ValueError> { self.admit_step_inner(true) }
    fn admit_step_inner(&mut self,retiring:bool)->Result<bool,ValueError> {
        if self.parent.is_some() { return Ok(true); }
        let grant=self.retirement_grant();
        if (!retiring&&self.fuel<=self.reserve_fuel)||self.fuel==0||grant.maximum_items==0||grant.maximum_depth==0 { return Err(refusal("headless original source birth exhausted")); }
        if self.cancel.is_none() {
            let turn=CancelToken::admit_root(grant);
            match turn.result {
                Ok(Some((cancel,progress)))=>{self.cancel=Some(cancel);self.pay(progress,turn.work)?;}
                Ok(None)=>{self.pay(Default::default(),turn.work)?;return Err(refusal("headless cancellation root birth refused"));}
                Err(error)=>{self.pay(error.retained_progress(),turn.work)?;return Err(error);}
            }
            return Ok(false);
        }
        if self.authority.is_none() {
            let turn=semio_framework_job_payload::JobPayloadAuthority::admit(self.operation.operation,self.operation.generation,grant,&mut self.authority_actual);
            match turn.result {
                Ok(Some((authority,progress)))=>{self.authority=Some(authority);self.pay(progress,turn.work)?;}
                Ok(None)=>{self.pay(Default::default(),turn.work)?;return Err(refusal("headless payload source birth refused"));}
                Err(error)=>{self.pay(error.retained_progress(),turn.work)?;return Err(error);}
            }
            return Ok(false);
        }
        let authority=self.authority.take().expect("retained original payload source");
        let cancel=self.cancel.take().expect("retained original cancellation root");
        let turn=OriginalCallerSourceParent::admit(authority,cancel,grant,&mut self.authority_actual);
        match turn.result {
            Ok((parent,progress))=>{self.parent=Some(parent);self.pay(progress,turn.work)?;Ok(true)}
            Err((error,authority,cancel))=>{self.authority=Some(authority);self.cancel=Some(cancel);self.pay(error.retained_progress(),turn.work)?;Err(error)}
        }
    }

    pub(super) fn turn<R:Copy>(&mut self,site:&HeadlessSite,retiring:bool,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure> {
        let left=remaining(self.stock,self.spent);
        let available=if retiring { self.fuel } else { self.fuel.saturating_sub(self.reserve_fuel) };
        let grant=if retiring { self.retirement_grant() } else {
            narrow(HEADLESS_GRANT,RetainedCloneGrant { maximum_items:left.maximum_items.saturating_sub(self.reserve.maximum_items),maximum_copy_bytes:left.maximum_copy_bytes.saturating_sub(self.reserve.maximum_copy_bytes),maximum_capacity_bytes:left.maximum_capacity_bytes.saturating_sub(self.reserve.maximum_capacity_bytes),maximum_release_bytes:left.maximum_release_bytes.saturating_sub(self.reserve.maximum_release_bytes),..left })
        };
        if available==0||grant.maximum_items==0||grant.maximum_depth==0 { return Err(refusal("headless original business or retirement budget exhausted").into()); }
        let now=(self.now_us)().ok_or_else(||refusal("headless original clock unavailable"))?;
        if !retiring&&now>=self.deadline { return Err(refusal("headless original lifetime deadline exhausted").into()); }
        let deadline=if retiring { self.deadline } else { now.saturating_add(site.step_budget_us).min(self.deadline) };
        let fuel=if retiring { available.min(1024) } else { available.min(site.fuel_per_step) };
        if fuel==0 { return Err(refusal("headless original turn fuel exhausted").into()); }
        self.stride.begin(Some(now));
        let parent=self.parent.as_ref().ok_or_else(||refusal("headless original parent is not admitted"))?;
        let mut cx=parent.borrow_context(StepBudget::new(self.fuel,self.deadline,self.stock),self.now_us,&mut self.sequence,&mut self.spent)?;
        cx.bind_original_clock_binding(self.binding)?;
        cx.restore_clock_stride(self.stride);
        let result=if retiring {
            cx.with_original_retirement(|cx|cx.with_original_work_loan(fuel,self.deadline,|cx|cx.with_retained_loan(grant,run)))
        } else {
            cx.with_original_work_loan(fuel,deadline,|cx|cx.with_retained_loan(grant,run))
        };
        self.fuel=cx.original_source_fuel_remaining().min(self.fuel);
        self.stride=cx.clock_stride();
        result
    }

    pub(super) fn close_step(&mut self)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let grant=self.retirement_grant();
        if self.fuel==0||grant.maximum_items==0||grant.maximum_depth==0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(parent)=self.parent.as_mut() {
            if parent.terminal_is_empty() { self.parent=None;let progress=RetainedCloneProgress {copied_items:1,..Default::default()};self.pay(progress,1)?;return Ok(RetainedCloneStep::Progress(progress)); }
            let turn=parent.close_step(grant);
            return match turn.result {Ok(step)=>{self.pay(step.progress(),turn.work)?;Ok(step)},Err(error)=>{self.pay(error.retained_progress(),turn.work)?;Err(error)}};
        }
        if let Some(authority)=self.authority.as_mut() {
            if authority.terminal_is_empty() { self.authority=None;let progress=RetainedCloneProgress {copied_items:1,..Default::default()};self.pay(progress,1)?;return Ok(RetainedCloneStep::Progress(progress)); }
            let turn=authority.close_step(grant);
            return match turn.result {Ok(step)=>{self.pay(step.progress(),turn.work)?;Ok(step)},Err(error)=>{self.pay(error.retained_progress(),turn.work)?;Err(error)}};
        }
        if let Some(cancel)=self.cancel.take() {
            self.cancel_retirement=Some(CancelTokenRetirement::from_token(cancel));
            let progress=RetainedCloneProgress {copied_items:1,..Default::default()};
            self.pay(progress,1)?;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        let owner=self.cancel_retirement.as_mut().expect("retained original cancellation retirement");
        if owner.terminal_is_empty() { self.cancel_retirement=None;let progress=RetainedCloneProgress {copied_items:1,..Default::default()};self.pay(progress,1)?;return Ok(RetainedCloneStep::Progress(progress)); }
        let turn=owner.close_step(grant);
        match turn.result {
            Ok(step)=>{self.pay(step.progress(),turn.work)?;Ok(step)}
            Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>{self.pay(error.retained_progress(),turn.work)?;Err(error)}
            Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>{self.pay(Default::default(),turn.work)?;Ok(RetainedCloneStep::Progress(Default::default()))}
        }
    }
}

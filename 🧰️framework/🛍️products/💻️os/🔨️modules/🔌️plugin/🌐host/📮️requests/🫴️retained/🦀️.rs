//! 🫴️ Send-safe reply custody from one actor registry into its original retained job.
use semio_framework::Fault;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::size_of;
use std::sync::{Arc, Mutex};
use std::task::Waker;

struct Reply {
    result: Option<Result<Vec<u8>, Fault>>,
    ended: bool,
    wake: Option<Waker>,
}

impl Drop for Reply {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.result.is_none(), "retained host reply abandoned its original response"); }
}

pub(super) struct SharedRequestSender(Arc<Mutex<Reply>>);

/// 📬️ One movable reply receiver; the actor-local Host and request registry never cross threads.
pub struct SharedRequestReceiver {
    source: Option<Arc<Mutex<Reply>>>,
    closing: Option<ControlledRetirement<Result<Vec<u8>, Fault>>>,
}

pub(super) fn pair(wake: Waker) -> (SharedRequestSender, SharedRequestReceiver) {
    let source = Arc::new(Mutex::new(Reply { result: None, ended: false, wake: Some(wake) }));
    (SharedRequestSender(source.clone()), SharedRequestReceiver { source: Some(source), closing: None })
}

impl SharedRequestSender {
    pub(super) fn receive(&self, original: &mut Option<Result<Vec<u8>, Fault>>) -> bool {
        let wake = {
            let Ok(mut reply) = self.0.try_lock() else { return false };
            if reply.ended || reply.result.is_some() || original.is_none() { return false; }
            reply.result = original.take();
            reply.ended = true;
            reply.wake.take()
        };
        if let Some(wake) = wake { wake.wake(); }
        true
    }
}

impl Drop for SharedRequestSender {
    fn drop(&mut self) {
        let wake = { let mut reply = self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner); reply.ended = true; reply.wake.take() };
        if let Some(wake) = wake { wake.wake(); }
    }
}

impl SharedRequestReceiver {
    /// 📥️ Moves exactly the funded response owner without cloning its body or consuming a refused slot.
    pub fn take_original(&mut self, cx: &mut StepContext<'_>) -> Result<Option<Result<Vec<u8>, Fault>>, OriginalTurnFailure> {
        let grant = cx.retained_grant();
        let bytes = size_of::<Option<Result<Vec<u8>, Fault>>>();
        if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 { return Ok(None); }
        let Some(source) = self.source.as_ref() else { return Ok(None) };
        let mut reply = source.try_lock().map_err(|_| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "retained host reply is currently borrowed"))?;
        if reply.result.is_none() { return Ok(None); }
        let output = reply.result.take();
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(output)
    }

    pub fn is_disconnected(&self) -> bool { self.source.as_ref().is_none_or(|source| source.lock().is_ok_and(|reply| reply.ended && reply.result.is_none())) }

    pub fn is_pending(&self) -> bool { self.source.as_ref().is_some_and(|source| source.lock().map_or(true, |reply| !reply.ended)) }

    /// ⏰️ Replaces only a wake lease; no response body or actor-local handle escapes custody.
    pub fn register_wake(&self, wake: &Waker) -> bool {
        let Some(source) = self.source.as_ref() else { return false };
        let Ok(mut reply) = source.try_lock() else { return false };
        if reply.ended { return false; }
        reply.wake = Some(wake.clone());
        true
    }

    pub fn close_demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(close) = self.closing.as_ref() {
            if close.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<Option<ControlledRetirement<Result<Vec<u8>, Fault>>>>(), depth: 1, ..Default::default() }); }
            return close.selected_demand(maximum_body_bytes);
        }
        let Some(source) = self.source.as_ref() else { return Ok(Default::default()) };
        let reply = source.try_lock().map_err(|_| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "retained host reply is currently borrowed"))?;
        if reply.result.is_some() { return Ok(RetirementDemand { copy_bytes: 2 * size_of::<ControlledRetirement<Result<Vec<u8>, Fault>>>() + size_of::<Option<Result<Vec<u8>, Fault>>>(), depth: 1, ..Default::default() }); }
        Ok(RetirementDemand { copy_bytes: size_of::<Option<Arc<Mutex<Reply>>>>(), release_bytes: if Arc::strong_count(source) == 1 { semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<Mutex<Reply>>() } else { 0 }, depth: 1, ..Default::default() })
    }

    /// ♻️ Keeps pending actor custody until completion, then retires every original body under the same caller.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.closing.as_ref().is_some_and(|close| close.terminal_is_empty()) {
            let bytes = size_of::<Option<ControlledRetirement<Result<Vec<u8>, Fault>>>>();
            if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.closing = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }));
        }
        if let Some(close) = self.closing.as_mut() { return close.step(grant); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let source = self.source.as_ref().unwrap();
        let mut reply = source.try_lock().map_err(|_| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "retained host reply is currently borrowed"))?;
        if let Some(result) = reply.result.take() {
            match ControlledRetirement::new(result) {
                Ok(close) => self.closing = Some(close),
                Err((error, original)) => { reply.result = Some(original); return Err(error); }
            }
        } else {
            if !reply.ended || Arc::strong_count(source) != 1 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            drop(reply);
            drop(self.source.take());
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() }))
    }

    pub fn terminal_is_empty(&self) -> bool { self.source.is_none() && self.closing.is_none() }
}

impl Drop for SharedRequestReceiver {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "retained host receiver requires original funded closure"); }
}

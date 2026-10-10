//! 📡️ One live query slot retains its page and all three captured Store leases until exact closure.

use super::{
    authority::inputs::LocalInteractionInputReads,
    capture::LocalInteractionCaptureCursor,
    query::{LocalInteractionQuery, LocalInteractionQueryCapture, LocalInteractionQueryStep},
};
use protocol::{LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryRejection, LocalInteractionQueryReply, LocalInteractionQueryToken};
use std::mem::ManuallyDrop;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use store::{ArtifactStoreOneItemGrant, SnapshotRead};

//#region 🔢️RuntimeGeneration
/// 🔢️ This allocator belongs to the runtime, never to a reusable app instance slot.
#[derive(Default)]
pub(crate) struct LocalInteractionQueryGeneration(std::cell::Cell<u64>);

impl LocalInteractionQueryGeneration {
    pub(crate) fn next(&self) -> Option<u64> {
        let next = self.0.get().checked_add(1)?;
        self.0.set(next);
        Some(next)
    }
}
//#endregion 🔢️RuntimeGeneration

//#region 📡️LiveOwner
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LocalInteractionLiveStep {
    Blocked,
    Advanced { emitted_bytes: usize, retired_bytes: usize, ownership: RetainedCloneProgress },
}
impl LocalInteractionLiveStep{
    pub(crate) fn progress(self)->RetainedCloneProgress{match self{Self::Advanced{ownership,..}=>ownership,Self::Blocked=>Default::default()}}
}

struct LiveState<D, C, Q: LocalInteractionQueryCapture> {
    query: Option<LocalInteractionQuery<Q>>,
    inputs: LocalInteractionInputReads<D, C>,
    error: Option<ValueError>,
    error_close: Option<Box<dyn store::ErasedSnapshotRetirement>>,
}

/// 🔒️ The app captures these roots under one exclusive owner; transport receives fixed pages only.
pub(crate) struct LocalInteractionLiveQuery<D, C, Q: LocalInteractionQueryCapture = LocalInteractionCaptureCursor> {
    owned: ManuallyDrop<LiveState<D, C, Q>>,
    request_id: u64,
    started: bool,
    page_sent: bool,
    closing: bool,
    cancelled: bool,
    failed: bool,
    terminal_sent: bool,
}

impl<D, C> LocalInteractionLiveQuery<D, C> {
    pub(crate) fn new(request_id: u64, query_generation: u64, identity: LocalInteractionIdentity, document: Option<SnapshotRead<D>>, config: Option<SnapshotRead<C>>, interaction: Option<SnapshotRead<protocol::InteractionState>>) -> Self {
        let failed = document.is_none() || config.is_none() || interaction.is_none();
        let inputs = LocalInteractionInputReads::from_optional(document, config);
        let query = interaction.map(|read| LocalInteractionQuery::new(LocalInteractionCaptureCursor::new(read, identity), request_id, query_generation));
        let mut owner = Self { owned: ManuallyDrop::new(LiveState { query, inputs, error: None, error_close:None }), request_id, started: false, page_sent: false, closing: false, cancelled: false, failed, terminal_sent: false };
        if failed {
            owner.begin_close();
        }
        owner
    }
}

impl<D, C, Q: LocalInteractionQueryCapture> LocalInteractionLiveQuery<D, C, Q> {
    pub(crate) fn acknowledge(&mut self, token: &LocalInteractionQueryToken) -> bool {
        if !self.started || !self.page_sent || self.closing {
            return false;
        }
        let terminal = self.owned.query.as_ref().and_then(LocalInteractionQuery::page).is_some_and(|page| page.terminal);
        if !self.owned.query.as_mut().is_some_and(|query| query.acknowledge(token)) {
            return false;
        }
        self.page_sent = false;
        if terminal {
            self.closing = true;
            self.owned.inputs.begin_close();
        }
        true
    }

    pub(crate) fn cancel_authorized(&mut self, token: &LocalInteractionQueryToken) -> bool {
        if self.closing || !self.owned.query.as_mut().is_some_and(|query| query.cancel_authorized(token)) {
            return false;
        }
        self.cancelled = true;
        self.closing = true;
        self.page_sent = false;
        self.owned.inputs.begin_close();
        true
    }

    pub(crate) fn begin_close(&mut self) {
        self.closing = true;
        self.cancelled = true;
        self.page_sent = false;
        if let Some(query) = self.owned.query.as_mut() {
            query.cancel();
        }
        self.owned.inputs.begin_close();
    }

    pub(crate) fn advance(&mut self, grant: ArtifactStoreOneItemGrant, output_width: usize) -> Result<LocalInteractionLiveStep, ValueError> {
        if grant.maximum_items == 0 || grant.maximum_depth<2 || self.closing {
            return Ok(LocalInteractionLiveStep::Blocked);
        }
        if !self.started {
            return Ok(LocalInteractionLiveStep::Blocked);
        }
        let query = self.owned.query.as_mut().expect("admitted query has its immutable interaction lease");
        let before_emitted = query.completed_bytes();
        let before_retired = query.retired_bytes();
        match query.advance(ArtifactStoreOneItemGrant{maximum_depth:grant.maximum_depth-1,..grant}, output_width) {
            Ok(LocalInteractionQueryStep::Blocked | LocalInteractionQueryStep::PageReady) => Ok(LocalInteractionLiveStep::Blocked),
            Ok(LocalInteractionQueryStep::Advanced { emitted_bytes, retired_bytes, ownership }) => Ok(LocalInteractionLiveStep::Advanced { emitted_bytes, retired_bytes, ownership }),
            Ok(LocalInteractionQueryStep::Closing) => Ok(LocalInteractionLiveStep::Blocked),
            Err(reason) => {
                let emitted_bytes = (query.completed_bytes() - before_emitted) as usize;
                let retired_bytes = (query.retired_bytes() - before_retired) as usize;
                let ownership=reason.retained_progress();
                self.owned.error=Some(reason);
                self.failed = true;
                self.begin_close();
                Ok(LocalInteractionLiveStep::Advanced { emitted_bytes, retired_bytes, ownership })
            }
        }
    }

    pub(crate) fn retired_bytes(&self)->u64 {
        self.owned.query.as_ref().map_or(0,LocalInteractionQuery::retired_bytes)
    }

    pub(crate) fn is_closing(&self) -> bool {
        self.closing
    }

    /// 📏️ Exact per-axis quote of the next closing turn: error bytes, then the query, then the input returns.
    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "live local interaction depth overflow"))?;
            Ok(demand)
        };
        if let Some(owner)=self.owned.error_close.as_ref(){return nested(store::artifact_retirement_box_demands(owner,body)?)}
        if self.owned.error.is_some(){return nested(store::artifact_retirement_owned_birth_demands(&self.owned.error)?)}
        if let Some(query) = self.owned.query.as_ref().filter(|query| !query.terminal_is_empty()) {
            return nested(query.retirement_demands(body)?);
        }
        nested(self.owned.inputs.retirement_demands())
    }

    /// ♻️ Closes one original owner under the complete caller grant; nested owners receive one item and one less depth.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.owners_are_empty() {
            return Ok(RetainedCloneStep::Complete(idle));
        }
        if !self.closing || grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "live local interaction close exceeds admitted depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = if self.owned.error_close.is_some(){store::artifact_retirement_box_close_step(&mut self.owned.error_close,child)?}
        else if self.owned.error.is_some(){let owned=&mut *self.owned;store::artifact_retirement_admit_owned(&mut owned.error,&mut owned.error_close,child)?}
        else if let Some(query) = self.owned.query.as_mut().filter(|query| !query.terminal_is_empty()) {
            semio_framework_value::retained_clone::admit_retained_clone_close(child, query.close_step(child)?, query.terminal_is_empty(), "live local interaction query")?
        } else {
            let inputs = &mut self.owned.inputs;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, inputs.close_step(child)?, inputs.terminal_is_empty(), "live local interaction inputs")?
        };
        let progress = step.progress();
        Ok(if self.owners_are_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub(crate) fn take_reply(&mut self) -> Option<LocalInteractionQueryReply> {
        self.take_reply_admitted(|_| true)
    }

    /// 📬️ State advances only after the exact fixed reply has entered the caller's admitted output slot.
    pub(crate) fn take_reply_admitted(&mut self, mut admit: impl FnMut(&LocalInteractionQueryReply) -> bool) -> Option<LocalInteractionQueryReply> {
        if self.terminal_sent {
            return None;
        }
        if self.closing {
            if !self.owners_are_empty() {
                return None;
            }
            let reply = if self.failed {
                LocalInteractionQueryReply::Rejected { request_id: self.request_id, code: LocalInteractionQueryRejection::SourceFailed }
            } else {
                LocalInteractionQueryReply::Closed { token: self.owned.query.as_ref().expect("accepted query retains fixed terminal token").token().clone(), cancelled: self.cancelled }
            };
            if !admit(&reply) {
                return None;
            }
            self.terminal_sent = true;
            return Some(reply);
        }
        let query = self.owned.query.as_ref()?;
        if !self.started {
            let reply = LocalInteractionQueryReply::Started { token: query.token().clone() };
            if !admit(&reply) {
                return None;
            }
            self.started = true;
            return Some(reply);
        }
        if self.page_sent {
            return None;
        }
        let page = query.page()?;
        let token = page.token;
        let reply = LocalInteractionQueryReply::Page {
            page: LocalInteractionPage { request_id: token.request_id, query_generation: token.query_generation, identity: token.identity.clone(), ordinal: token.ordinal, terminal: page.terminal, bytes: page.bytes.to_vec() },
        };
        if !admit(&reply) {
            return None;
        }
        self.page_sent = true;
        Some(reply)
    }

    pub(crate) fn reply_ready(&self) -> bool {
        !self.terminal_sent && if self.closing { self.owners_are_empty() } else { !self.started || (!self.page_sent && self.owned.query.as_ref().is_some_and(|query| query.page().is_some())) }
    }

    pub(crate) fn has_pending_work(&self) -> bool {
        !self.terminal_sent && (self.closing || !self.started || !self.page_sent || self.owned.query.as_ref().is_some_and(LocalInteractionQuery::has_pending_work))
    }

    pub(crate) fn owners_are_empty(&self) -> bool {
        self.closing && self.owned.error.is_none() && self.owned.error_close.is_none() && self.owned.query.as_ref().is_none_or(LocalInteractionQuery::terminal_is_empty) && self.owned.inputs.terminal_is_empty()
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.owners_are_empty() && self.terminal_sent
    }
}

impl<D, C, Q: LocalInteractionQueryCapture> Drop for LocalInteractionLiveQuery<D, C, Q> {
    fn drop(&mut self) {
        if !self.owners_are_empty() {
            if !std::thread::panicking() {
                panic!("live local interaction query dropped before all exact captured roots returned");
            }
            return;
        }
        unsafe {
            ManuallyDrop::drop(&mut self.owned);
        }
    }
}
//#endregion 📡️LiveOwner

#[cfg(test)]
#[path = "🧪️tests/📡️live/🦀️.rs"]
mod tests;

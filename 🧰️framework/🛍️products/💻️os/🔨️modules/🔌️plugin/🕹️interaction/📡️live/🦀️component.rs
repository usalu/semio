//! 📡️ One live query slot retains its page and all three captured Store leases until exact closure.

use std::mem::ManuallyDrop;
use protocol::{LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryReply, LocalInteractionQueryRejection, LocalInteractionQueryToken};
use store::{ArtifactStoreOneItemGrant, SnapshotRead, SnapshotRetirementStep};
use super::{authority::inputs::LocalInteractionInputReads, capture::LocalInteractionCaptureCursor, query::{LocalInteractionQuery, LocalInteractionQueryCapture, LocalInteractionQueryStep}};

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
    Advanced { emitted_bytes: usize, retired_bytes: usize, released_items: usize },
    Complete,
}

impl LocalInteractionLiveStep {
    fn retirement(step: SnapshotRetirementStep) -> Self {
        match step {
            SnapshotRetirementStep::Blocked => Self::Blocked,
            SnapshotRetirementStep::Complete => Self::Complete,
            SnapshotRetirementStep::Pending { released_items, released_bytes } => Self::Advanced { emitted_bytes: 0, retired_bytes: released_bytes, released_items },
        }
    }
}

struct LiveState<D, C, Q: LocalInteractionQueryCapture> {
    query: Option<LocalInteractionQuery<Q>>,
    inputs: LocalInteractionInputReads<D, C>,
    error_bytes: Option<Vec<u8>>,
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
    pub(crate) fn new(request_id: u64, query_generation: u64, identity: LocalInteractionIdentity, document: Option<SnapshotRead<D>>, document_generation: u64, config: Option<SnapshotRead<C>>, config_generation: u64, config_revision: [u8; 32], interaction: Option<SnapshotRead<protocol::InteractionState>>) -> Self {
        let failed = document.is_none() || config.is_none() || interaction.is_none();
        let inputs = LocalInteractionInputReads::from_optional(document, document_generation, identity.document_revision, config, config_generation, config_revision);
        let query = interaction.map(|read| LocalInteractionQuery::new(LocalInteractionCaptureCursor::new(read, identity), request_id, query_generation));
        let mut owner = Self { owned: ManuallyDrop::new(LiveState { query, inputs, error_bytes: None }), request_id, started: false, page_sent: false, closing: false, cancelled: false, failed, terminal_sent: false };
        if failed { owner.begin_close(); }
        owner
    }
}

impl<D, C, Q: LocalInteractionQueryCapture> LocalInteractionLiveQuery<D, C, Q> {

    pub(crate) fn acknowledge(&mut self, token: &LocalInteractionQueryToken) -> bool {
        if !self.started || !self.page_sent || self.closing { return false; }
        let terminal = self.owned.query.as_ref().and_then(LocalInteractionQuery::page).is_some_and(|page| page.terminal);
        if !self.owned.query.as_mut().is_some_and(|query| query.acknowledge(token)) { return false; }
        self.page_sent = false;
        if terminal { self.closing = true; self.owned.inputs.begin_close(); }
        true
    }

    pub(crate) fn cancel_authorized(&mut self, token: &LocalInteractionQueryToken) -> bool {
        if self.closing || !self.owned.query.as_mut().is_some_and(|query| query.cancel_authorized(token)) { return false; }
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
        if let Some(query) = self.owned.query.as_mut() { query.cancel(); }
        self.owned.inputs.begin_close();
    }

    pub(crate) fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<LocalInteractionLiveStep, String> {
        if grant.maximum_items == 0 { return Ok(LocalInteractionLiveStep::Blocked); }
        if self.closing {
            if let Some(bytes) = self.owned.error_bytes.as_mut() {
                if !bytes.is_empty() {
                    let released_bytes = bytes.len().min(grant.maximum_bytes);
                    if released_bytes == 0 { return Ok(LocalInteractionLiveStep::Blocked); }
                    bytes.truncate(bytes.len() - released_bytes);
                    return Ok(LocalInteractionLiveStep::Advanced { emitted_bytes: 0, retired_bytes: released_bytes, released_items: 0 });
                }
                self.owned.error_bytes = None;
                return Ok(LocalInteractionLiveStep::Advanced { emitted_bytes: 0, retired_bytes: 0, released_items: 1 });
            }
            if let Some(query) = self.owned.query.as_mut() {
                if !query.terminal_is_empty() {
                    return query.close_step(grant).map(|step| LocalInteractionLiveStep::retirement(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 } } else { step }));
                }
            }
            return self.owned.inputs.close_step(grant).map(LocalInteractionLiveStep::retirement);
        }
        if !self.started { return Ok(LocalInteractionLiveStep::Blocked); }
        let query = self.owned.query.as_mut().expect("admitted query has its immutable interaction lease");
        let before_emitted = query.completed_bytes();
        let before_retired = query.retired_bytes();
        match query.advance(grant) {
            Ok(LocalInteractionQueryStep::Blocked | LocalInteractionQueryStep::PageReady) => Ok(LocalInteractionLiveStep::Blocked),
            Ok(LocalInteractionQueryStep::Advanced { emitted_bytes, retired_bytes }) => Ok(LocalInteractionLiveStep::Advanced { emitted_bytes, retired_bytes, released_items: 0 }),
            Ok(LocalInteractionQueryStep::Closing) => Ok(LocalInteractionLiveStep::Blocked),
            Err(reason) => {
                let emitted_bytes = (query.completed_bytes() - before_emitted) as usize;
                let retired_bytes = (query.retired_bytes() - before_retired) as usize;
                self.owned.error_bytes = Some(reason.into_bytes());
                self.failed = true;
                self.begin_close();
                Ok(LocalInteractionLiveStep::Advanced { emitted_bytes, retired_bytes, released_items: 0 })
            },
        }
    }

    pub(crate) fn take_reply(&mut self) -> Option<LocalInteractionQueryReply> {
        self.take_reply_admitted(|_| true)
    }

    /// 📬️ State advances only after the exact fixed reply has entered the caller's admitted output slot.
    pub(crate) fn take_reply_admitted(&mut self, mut admit: impl FnMut(&LocalInteractionQueryReply) -> bool) -> Option<LocalInteractionQueryReply> {
        if self.terminal_sent { return None; }
        if self.closing {
            if !self.owners_are_empty() { return None; }
            let reply = if self.failed {
                LocalInteractionQueryReply::Rejected { request_id: self.request_id, code: LocalInteractionQueryRejection::SourceFailed }
            } else {
                LocalInteractionQueryReply::Closed { token: self.owned.query.as_ref().expect("accepted query retains fixed terminal token").token().clone(), cancelled: self.cancelled }
            };
            if !admit(&reply) { return None; }
            self.terminal_sent = true;
            return Some(reply);
        }
        let query = self.owned.query.as_ref()?;
        if !self.started {
            let reply = LocalInteractionQueryReply::Started { token: query.token().clone() };
            if !admit(&reply) { return None; }
            self.started = true;
            return Some(reply);
        }
        if self.page_sent { return None; }
        let page = query.page()?;
        let token = page.token;
        let reply = LocalInteractionQueryReply::Page { page: LocalInteractionPage { request_id: token.request_id, query_generation: token.query_generation, identity: token.identity.clone(), ordinal: token.ordinal, terminal: page.terminal, bytes: page.bytes.to_vec() } };
        if !admit(&reply) { return None; }
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
        self.closing && self.owned.error_bytes.is_none() && self.owned.query.as_ref().is_none_or(LocalInteractionQuery::terminal_is_empty) && self.owned.inputs.terminal_is_empty()
    }

    pub(crate) fn is_closing(&self) -> bool { self.closing }

    pub(crate) fn terminal_is_empty(&self) -> bool { self.owners_are_empty() && self.terminal_sent }
}

impl<D, C, Q: LocalInteractionQueryCapture> Drop for LocalInteractionLiveQuery<D, C, Q> {
    fn drop(&mut self) {
        if !self.owners_are_empty() {
            if !std::thread::panicking() { panic!("live local interaction query dropped before all exact captured roots returned"); }
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.owned); }
    }
}
//#endregion 📡️LiveOwner

#[cfg(test)]
#[path = "🧪️component.rs"]
mod tests;

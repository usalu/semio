//! 📃️ One ACK-owned fixed response page over an exact immutable local interaction capture.

use protocol::LocalInteractionIdentity;
use store::{ArtifactStoreOneItemGrant, SnapshotRetirementStep};
use super::capture::LocalInteractionCaptureCursor;

//#region 📃️PageAuthority
pub(crate) const LOCAL_INTERACTION_QUERY_PAGE_BYTES: usize = 256;

/// 🔐️ Fixed-width acknowledgement authority; no historical tutorial identity authorizes a new query.
pub(crate) use protocol::LocalInteractionQueryToken as LocalInteractionPageToken;

/// 👁️ A borrowed page cannot outlive or mutate its exact query owner.
pub(crate) struct LocalInteractionPageView<'a> {
    pub token: &'a LocalInteractionPageToken,
    pub terminal: bool,
    pub bytes: &'a [u8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LocalInteractionQueryStep { Blocked, Advanced { emitted_bytes: usize, retired_bytes: usize }, PageReady, Closing }
//#endregion 📃️PageAuthority

//#region 📖️QueryOwner
/// 🔒️ Exact frozen source ownership; output bytes and close authority stay with the same owner.
pub(crate) trait LocalInteractionQueryCapture {
    fn identity(&self) -> &LocalInteractionIdentity;
    fn write_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<usize, store::ArtifactCanonicalJsonEncodeError>;
    fn complete(&self) -> bool;
    fn completed_bytes(&self) -> u64;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

impl LocalInteractionQueryCapture for LocalInteractionCaptureCursor {
    fn identity(&self) -> &LocalInteractionIdentity { self.identity() }
    fn write_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<usize, store::ArtifactCanonicalJsonEncodeError> { self.write_chunk(grant, output) }
    fn complete(&self) -> bool { self.complete() }
    fn completed_bytes(&self) -> u64 { self.completed_bytes() }
    fn cancel(&mut self) { self.cancel(); }
    fn begin_close(&mut self) { self.begin_close(); }
    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> { self.close_step(grant) }
    fn terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
}

/// 📖️ Retains one page until an exact ACK; cancellation hides it before bytewise retirement.
pub(crate) struct LocalInteractionQuery<C: LocalInteractionQueryCapture = LocalInteractionCaptureCursor> {
    capture: C,
    token: LocalInteractionPageToken,
    page: [u8; LOCAL_INTERACTION_QUERY_PAGE_BYTES],
    length: usize,
    ready: bool,
    terminal_page: bool,
    retiring_page: bool,
    closing: bool,
    retired_bytes: u64,
}

impl<C: LocalInteractionQueryCapture> LocalInteractionQuery<C> {
    pub(crate) fn new(capture: C, request_id: u64, query_generation: u64) -> Self {
        let identity = capture.identity().clone();
        Self { capture, token: LocalInteractionPageToken { request_id, query_generation, identity, ordinal: 0 }, page: [0; LOCAL_INTERACTION_QUERY_PAGE_BYTES], length: 0, ready: false, terminal_page: false, retiring_page: false, closing: false, retired_bytes: 0 }
    }

    pub(crate) fn page(&self) -> Option<LocalInteractionPageView<'_>> {
        (self.ready && !self.closing).then(|| LocalInteractionPageView { token: &self.token, terminal: self.terminal_page, bytes: &self.page[..self.length] })
    }

    pub(crate) fn token(&self) -> &LocalInteractionPageToken { &self.token }
    pub(crate) fn has_pending_work(&self) -> bool { !self.ready && !self.terminal_is_empty() }

    pub(crate) fn cancel_authorized(&mut self, token: &LocalInteractionPageToken) -> bool {
        if self.closing || token.request_id != self.token.request_id || token.query_generation != self.token.query_generation || token.identity != self.token.identity { return false; }
        self.cancel();
        true
    }

    pub(crate) fn acknowledge(&mut self, token: &LocalInteractionPageToken) -> bool {
        if self.closing || !self.ready || token != &self.token { return false; }
        self.ready = false;
        self.retiring_page = true;
        if self.terminal_page { self.closing = true; self.capture.begin_close(); }
        true
    }

    pub(crate) fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<LocalInteractionQueryStep, String> {
        if self.closing { return Ok(LocalInteractionQueryStep::Closing); }
        if grant.maximum_items == 0 || grant.maximum_bytes == 0 { return Ok(LocalInteractionQueryStep::Blocked); }
        if self.ready { return Ok(LocalInteractionQueryStep::PageReady); }
        if self.retiring_page {
            if self.length != 0 {
                let retired_bytes = self.retire_page(grant.maximum_bytes);
                return Ok(LocalInteractionQueryStep::Advanced { emitted_bytes: 0, retired_bytes });
            }
            let Some(ordinal) = self.token.ordinal.checked_add(1) else { self.cancel(); return Err("local-interaction.query-ordinal-exhausted".into()); };
            self.token.ordinal = ordinal;
            self.retiring_page = false;
            return Ok(LocalInteractionQueryStep::Advanced { emitted_bytes: 0, retired_bytes: 0 });
        }
        let maximum = grant.maximum_bytes.min(LOCAL_INTERACTION_QUERY_PAGE_BYTES);
        match self.capture.write_chunk(grant, &mut self.page[..maximum]) {
            Ok(count) => self.length = count,
            Err(error) => {
                if error.written_bytes > maximum { self.cancel(); return Err("local-interaction.query-error-byte-grant".into()); }
                self.length = error.written_bytes;
                self.cancel();
                return Err(error.reason);
            }
        }
        self.terminal_page = self.capture.complete();
        self.ready = self.length != 0 || self.terminal_page;
        Ok(LocalInteractionQueryStep::Advanced { emitted_bytes: self.length, retired_bytes: 0 })
    }

    pub(crate) fn cancel(&mut self) {
        self.ready = false;
        self.closing = true;
        self.retiring_page = true;
        self.capture.cancel();
    }

    pub(crate) fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        if self.terminal_is_empty() { return Ok(SnapshotRetirementStep::Complete); }
        if !self.closing || grant.maximum_items == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        if self.length != 0 {
            if grant.maximum_bytes == 0 { return Ok(SnapshotRetirementStep::Blocked); }
            let released_bytes = self.retire_page(grant.maximum_bytes);
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
        }
        self.capture.close_step(grant)
    }

    fn retire_page(&mut self, maximum_bytes: usize) -> usize {
        let retired = self.length.min(maximum_bytes);
        let remaining = self.length - retired;
        self.page[remaining..self.length].fill(0);
        self.length = remaining;
        self.retired_bytes += retired as u64;
        retired
    }

    pub(crate) fn completed_bytes(&self) -> u64 { self.capture.completed_bytes() }
    pub(crate) fn retired_bytes(&self) -> u64 { self.retired_bytes }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.closing && !self.ready && self.length == 0 && self.capture.terminal_is_empty() }
}
//#endregion 📖️QueryOwner

#[cfg(test)]
#[path = "🧪️component.rs"]
pub(crate) mod tests;

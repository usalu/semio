//! 📃️ One ACK-owned fixed response page over an exact immutable local interaction capture.

use super::capture::LocalInteractionCaptureCursor;
use protocol::LocalInteractionIdentity;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use store::ArtifactStoreOneItemGrant;

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
pub(crate) enum LocalInteractionQueryStep {
    Blocked,
    Advanced { emitted_bytes: usize, retired_bytes: usize, ownership: RetainedCloneProgress },
    PageReady,
    Closing,
}
impl LocalInteractionQueryStep {
    pub(crate) fn progress(self)->RetainedCloneProgress{match self{Self::Advanced{ownership,..}=>ownership,_=>Default::default()}}
}
//#endregion 📃️PageAuthority

//#region 📖️QueryOwner
/// 🔒️ Exact frozen source ownership; output bytes and close authority stay with the same owner.
pub(crate) trait LocalInteractionQueryCapture {
    fn identity(&self) -> &LocalInteractionIdentity;
    fn write_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<store::ArtifactCanonicalJsonTreeStep, store::ArtifactCanonicalJsonEncodeError>;
    fn complete(&self) -> bool;
    fn completed_bytes(&self) -> u64;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

impl LocalInteractionQueryCapture for LocalInteractionCaptureCursor {
    fn identity(&self) -> &LocalInteractionIdentity {
        self.identity()
    }
    fn write_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<store::ArtifactCanonicalJsonTreeStep, store::ArtifactCanonicalJsonEncodeError> {
        self.write_chunk(grant, output)
    }
    fn complete(&self) -> bool {
        self.complete()
    }
    fn completed_bytes(&self) -> u64 {
        self.completed_bytes()
    }
    fn cancel(&mut self) {
        self.cancel();
    }
    fn begin_close(&mut self) {
        self.begin_close();
    }
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.retirement_demands(body)
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.close_step(grant)
    }
    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty()
    }
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
        Self {
            capture,
            token: LocalInteractionPageToken { request_id, query_generation, identity, ordinal: 0 },
            page: [0; LOCAL_INTERACTION_QUERY_PAGE_BYTES],
            length: 0,
            ready: false,
            terminal_page: false,
            retiring_page: false,
            closing: false,
            retired_bytes: 0,
        }
    }

    pub(crate) fn page(&self) -> Option<LocalInteractionPageView<'_>> {
        (self.ready && !self.closing).then(|| LocalInteractionPageView { token: &self.token, terminal: self.terminal_page, bytes: &self.page[..self.length] })
    }

    pub(crate) fn token(&self) -> &LocalInteractionPageToken {
        &self.token
    }
    pub(crate) fn has_pending_work(&self) -> bool {
        !self.ready && !self.terminal_is_empty()
    }

    pub(crate) fn cancel_authorized(&mut self, token: &LocalInteractionPageToken) -> bool {
        if self.closing || token.request_id != self.token.request_id || token.query_generation != self.token.query_generation || token.identity != self.token.identity {
            return false;
        }
        self.cancel();
        true
    }

    pub(crate) fn acknowledge(&mut self, token: &LocalInteractionPageToken) -> bool {
        if self.closing || !self.ready || token != &self.token {
            return false;
        }
        self.ready = false;
        self.retiring_page = true;
        if self.terminal_page {
            self.closing = true;
            self.capture.begin_close();
        }
        true
    }

    pub(crate) fn advance(&mut self, grant: ArtifactStoreOneItemGrant, output_width: usize) -> Result<LocalInteractionQueryStep, ValueError> {
        if self.closing {
            return Ok(LocalInteractionQueryStep::Closing);
        }
        if !grant.permits_one() || grant.maximum_copy_bytes == 0 || output_width == 0 {
            return Ok(LocalInteractionQueryStep::Blocked);
        }
        if self.ready {
            return Ok(LocalInteractionQueryStep::PageReady);
        }
        if self.retiring_page {
            if self.length != 0 {
                let retired_bytes = self.retire_page(grant.maximum_copy_bytes.min(output_width));
                return Ok(LocalInteractionQueryStep::Advanced { emitted_bytes: 0, retired_bytes, ownership:RetainedCloneProgress{copied_items:1,copied_bytes:retired_bytes,..Default::default()} });
            }
            let Some(ordinal) = self.token.ordinal.checked_add(1) else {
                self.cancel();
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"local-interaction.query-ordinal-exhausted"));
            };
            self.token.ordinal = ordinal;
            self.retiring_page = false;
            return Ok(LocalInteractionQueryStep::Advanced { emitted_bytes: 0, retired_bytes: 0, ownership:RetainedCloneProgress{copied_items:1,..Default::default()} });
        }
        let maximum = output_width.min(LOCAL_INTERACTION_QUERY_PAGE_BYTES);
        if grant.maximum_depth<2{return Ok(LocalInteractionQueryStep::Blocked)}
        let child=ArtifactStoreOneItemGrant{maximum_depth:grant.maximum_depth-1,..grant};
        let ownership=match self.capture.write_chunk(child, &mut self.page[..maximum]) {
            Ok(step) => {let progress=step.ownership.progress();self.length=step.written_bytes.min(maximum);if step.written_bytes>maximum||!progress.fits(grant.retained_grant()){self.cancel();return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"local-interaction.query-original-receipt").with_retained_progress(progress))}progress},
            Err(error) => {
                if error.written_bytes > maximum {
                    self.cancel();
                    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"local-interaction.query-error-byte-grant").with_retained_progress(error.reason.retained_progress()));
                }
                self.length = error.written_bytes;
                self.cancel();
                return Err(error.reason);
            }
        };
        self.terminal_page = self.capture.complete();
        self.ready = self.length != 0 || self.terminal_page;
        Ok(LocalInteractionQueryStep::Advanced { emitted_bytes: self.length, retired_bytes: 0, ownership })
    }

    pub(crate) fn cancel(&mut self) {
        self.ready = false;
        self.closing = true;
        self.retiring_page = true;
        self.capture.cancel();
    }

    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.length != 0 {
            return Ok(RetirementDemand { copy_bytes: 1, depth: 1, ..Default::default() });
        }
        let mut demand = self.capture.retirement_demands(body)?;
        demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "local interaction query depth overflow"))?;
        Ok(demand)
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(idle));
        }
        if !self.closing || grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "local interaction query close exceeds admitted depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        if self.length != 0 {
            let copied_bytes = self.retire_page(grant.maximum_copy_bytes);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes, ..idle }));
        }
        let step = self.capture.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?;
        Ok(match step {
            RetainedCloneStep::Complete(progress) if !self.terminal_is_empty() => RetainedCloneStep::Progress(progress),
            step => step,
        })
    }

    fn retire_page(&mut self, maximum_bytes: usize) -> usize {
        let retired = self.length.min(maximum_bytes);
        let remaining = self.length - retired;
        self.page[remaining..self.length].fill(0);
        self.length = remaining;
        self.retired_bytes += retired as u64;
        retired
    }

    pub(crate) fn completed_bytes(&self) -> u64 {
        self.capture.completed_bytes()
    }
    pub(crate) fn retired_bytes(&self) -> u64 {
        self.retired_bytes
    }
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.closing && !self.ready && self.length == 0 && self.capture.terminal_is_empty()
    }
}
//#endregion 📖️QueryOwner

#[cfg(test)]
#[path = "🧪️tests/📃️query/🦀️.rs"]
pub(crate) mod tests;

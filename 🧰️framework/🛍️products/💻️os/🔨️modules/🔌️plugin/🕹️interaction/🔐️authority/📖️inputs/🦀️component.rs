//! 🔒️ Exact immutable document/config read ownership retained alongside a local interaction query.

use std::mem::ManuallyDrop;
use store::{ArtifactStoreOneItemGrant, SnapshotRead, SnapshotReadReturn, SnapshotRetirementStep};

//#region 🔒️FrozenInputRoots
struct InputReadState<D, C> {
    document: Option<SnapshotRead<D>>,
    config: Option<SnapshotRead<C>>,
    returned: [Option<SnapshotReadReturn>; 2],
    closing: bool,
}

/// 🔒️ Publication checks use exact Store lease authority; returning roots waits for both registries.
pub(crate) struct LocalInteractionInputReads<D, C> {
    owned: ManuallyDrop<InputReadState<D, C>>,
    document_generation: u64,
    document_revision: [u8; 32],
    config_generation: u64,
    config_revision: [u8; 32],
}

impl<D, C> LocalInteractionInputReads<D, C> {
    /// 📥️ Called under the app's exclusive owner with the read and fixed identity captured together.
    pub(crate) fn new(document: SnapshotRead<D>, document_generation: u64, document_revision: [u8; 32], config: SnapshotRead<C>, config_generation: u64, config_revision: [u8; 32]) -> Self {
        Self::from_optional(Some(document), document_generation, document_revision, Some(config), config_generation, config_revision)
    }

    /// 🧯️ Failed capture still retains every successfully issued lease until exact registry return.
    pub(crate) fn from_optional(document: Option<SnapshotRead<D>>, document_generation: u64, document_revision: [u8; 32], config: Option<SnapshotRead<C>>, config_generation: u64, config_revision: [u8; 32]) -> Self {
        Self { owned: ManuallyDrop::new(InputReadState { document, config, returned: [None, None], closing: false }), document_generation, document_revision, config_generation, config_revision }
    }

    pub(crate) fn document_revision(&self) -> [u8; 32] { self.document_revision }
    pub(crate) fn config_revision(&self) -> [u8; 32] { self.config_revision }

    pub(crate) fn authority_is_current(&self) -> bool {
        !self.owned.closing
            && self.owned.document.as_ref().is_some_and(|read| read.commit_authority_matches(self.document_generation, self.document_revision))
            && self.owned.config.as_ref().is_some_and(|read| read.commit_authority_matches(self.config_generation, self.config_revision))
    }

    pub(crate) fn begin_close(&mut self) { self.owned.closing = true; }

    pub(crate) fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        if self.terminal_is_empty() { return Ok(SnapshotRetirementStep::Complete); }
        if !self.owned.closing || grant.maximum_items == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        if let Some(read) = self.owned.document.take() {
            self.owned.returned[0] = read.return_to_registry_witness();
            if self.owned.returned[0].is_none() { return Err("local-interaction.document-read-return".into()); }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(read) = self.owned.config.take() {
            self.owned.returned[1] = read.return_to_registry_witness();
            if self.owned.returned[1].is_none() { return Err("local-interaction.config-read-return".into()); }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        for returned in &mut self.owned.returned {
            if returned.as_ref().is_some_and(SnapshotReadReturn::terminal_is_empty) {
                *returned = None;
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
        }
        Ok(SnapshotRetirementStep::Blocked)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool { self.owned.closing && self.owned.document.is_none() && self.owned.config.is_none() && self.owned.returned.iter().all(Option::is_none) }
}

impl<D, C> Drop for LocalInteractionInputReads<D, C> {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() { panic!("local interaction input reads dropped before both exact Store returns completed"); }
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.owned); }
    }
}
//#endregion 🔒️FrozenInputRoots

//! 🎛️ Retained VCS state and begin: the operation slot, the fixed owners, the document record, the retained VCS owner and the constructors, accessors and `begin_*` entry points of every authoring operation. See [`super`].

use super::*;

//#region 🌊️RetainedVcs
pub(super) struct FlowVcsOperation {
    pub(super) handle: FlowVcsHandle,
    pub(super) authority: FlowVcsAuthority,
    pub(super) source: FlowVcsCensus,
    pub(super) action: Option<FlowVcsAction>,
    pub(super) rollback_owner: Option<FlowVcsAction>,
    pub(super) layout_update: Option<LayoutUpdate<WidgetLayout>>,
    pub(super) retirement: FlowVcsRetirement,
    pub(super) cursor: FlowVcsCursor,
    pub(super) page: Option<FlowVcsPage>,
    pub page_leased: bool,
    pub(super) delivery_held: bool,
    pub(super) stage: FlowVcsStage,
    pub close_phase: u8,
}

pub struct FlowFixedOwners<T, const N: usize> {
    pub slots: [Option<T>; N],
    pub length: usize,
}

impl<T, const N: usize> FlowFixedOwners<T, N> {
    pub(super) fn new() -> Self {
        Self { slots: [const { None }; N], length: 0 }
    }

    pub(super) fn len(&self) -> usize {
        self.length
    }

    pub(super) fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub(super) fn is_full(&self) -> bool {
        self.length == N
    }

    pub fn remaining(&self) -> usize {
        N - self.length
    }

    pub(super) fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        self.slots[self.length] = Some(value);
        self.length += 1;
        Ok(())
    }

    pub(super) fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.length -= 1;
        self.slots[self.length].take()
    }

    pub(super) fn last_mut(&mut self) -> Option<&mut T> {
        self.length.checked_sub(1).and_then(|index| self.slots[index].as_mut())
    }

    pub(super) fn get(&self, index: usize) -> Option<&T> {
        (index < self.length).then(|| self.slots[index].as_ref()).flatten()
    }

    pub(super) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (index < self.length).then(|| self.slots[index].as_mut()).flatten()
    }
}

pub(super) struct FlowVcsDocument {
    pub versions: FlowFixedOwners<FlowHostSnapshot, FLOW_VCS_MAX_HISTORY>,
    pub(super) active: usize,
    pub(super) revision: u64,
    pub(super) parent_revision: u64,
    pub(super) generation: u64,
    pub(super) committed_digest: u64,
    pub(super) edit_owner: Option<u64>,
    pub surface: Option<FlowSurfaceOwner>,
}

impl FlowVcsDocument {
    pub(super) fn new(host_snapshot: FlowHostSnapshot, revision: u64, parent_revision: u64) -> Self {
        let committed_digest = flow_vcs_host_snapshot_scalar_digest(&host_snapshot);
        let mut versions = FlowFixedOwners::new();
        let _ = versions.push(host_snapshot);
        Self { versions, active: 0, revision, parent_revision, generation: 1, committed_digest, edit_owner: None, surface: None }
    }

    pub fn host_snapshot(&self) -> &FlowHostSnapshot {
        self.versions.get(self.active).expect("active Flow VCS document version")
    }

    pub(super) fn host_snapshot_mut(&mut self) -> &mut FlowHostSnapshot {
        self.versions.get_mut(self.active).expect("active Flow VCS document version")
    }
}

pub struct FlowRetainedVcs {
    pub session_generation: u32,
    pub document: Option<FlowVcsDocument>,
    pub operations: [Option<FlowVcsOperation>; FLOW_VCS_MAX_OPERATIONS],
    pub(super) slot_generations: [u32; FLOW_VCS_MAX_OPERATIONS],
    pub(super) next_operation: u64,
    pub(super) next_page: u64,
    pub credits: FlowVcsCredits,
    pub undo: FlowFixedOwners<FlowVcsAction, FLOW_VCS_MAX_HISTORY>,
    pub redo: FlowFixedOwners<FlowVcsAction, FLOW_VCS_MAX_HISTORY>,
    pub retired_actions: FlowFixedOwners<FlowVcsAction, FLOW_VCS_MAX_HISTORY>,
    pub retired_surfaces: FlowFixedOwners<FlowSurfaceOwner, FLOW_VCS_MAX_HISTORY>,
    pub(super) retirement: FlowVcsRetirement,
    pub closing: bool,
}

impl FlowRetainedVcs {
    pub fn new(document: FlowHostSnapshot, session_generation: u32, revision: u64, parent_revision: u64) -> Self {
        Self {
            session_generation,
            document: Some(FlowVcsDocument::new(document, revision, parent_revision)),
            operations: [const { None }; FLOW_VCS_MAX_OPERATIONS],
            slot_generations: [1; FLOW_VCS_MAX_OPERATIONS],
            next_operation: 1,
            next_page: 1,
            credits: FlowVcsCredits::default(),
            undo: FlowFixedOwners::new(),
            redo: FlowFixedOwners::new(),
            retired_actions: FlowFixedOwners::new(),
            retired_surfaces: FlowFixedOwners::new(),
            retirement: None,
            closing: false,
        }
    }

    pub fn authority(&self) -> FlowVcsAuthority {
        let document = self.document.as_ref().expect("open Flow VCS document");
        FlowVcsAuthority { session_generation: self.session_generation, base_revision: document.revision, parent_revision: document.parent_revision }
    }

    pub fn credits(&self) -> FlowVcsCredits {
        self.credits
    }

    pub fn resource_fingerprint(&self) -> FlowVcsResourceFingerprint {
        let active_operations = self.credits.operations;
        let leased_pages = usize::from(self.operations[0].as_ref().is_some_and(|operation| operation.page_leased))
            + usize::from(self.operations[1].as_ref().is_some_and(|operation| operation.page_leased))
            + usize::from(self.operations[2].as_ref().is_some_and(|operation| operation.page_leased))
            + usize::from(self.operations[3].as_ref().is_some_and(|operation| operation.page_leased));
        let (revision, parent_revision, document_generation, document_digest, document_versions, active_document_version, edit_owner, document_retained) = match &self.document {
            Some(document) => (document.revision, document.parent_revision, document.generation, document.committed_digest, document.versions.len(), document.active, document.edit_owner, true),
            None => (0, 0, 0, 0, 0, 0, None, false),
        };
        FlowVcsResourceFingerprint {
            credits: self.credits,
            active_operations,
            leased_pages,
            undo_owners: self.undo.len(),
            redo_owners: self.redo.len(),
            retired_action_owners: self.retired_actions.len(),
            retired_surface_owners: self.retired_surfaces.len(),
            revision,
            parent_revision,
            document_generation,
            document_digest,
            document_versions,
            active_document_version,
            edit_owner,
            document_retained,
            closing: self.closing,
        }
    }

    pub fn bind_surface(&mut self, surface: u64, host: u64, generation: u64) -> Result<(), FlowVcsFault> {
        if self.closing {
            return Err(FlowVcsFault::Closed);
        }
        let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
        if document.surface.is_some() || self.retired_surfaces.len() == FLOW_VCS_MAX_HISTORY {
            return Err(FlowVcsFault::Full);
        }
        document.surface = Some(FlowSurfaceOwner::from_host_snapshot(surface, host, generation, document.host_snapshot()));
        Ok(())
    }

    pub fn begin_add_widget(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<Widget>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = flow_vcs_widget_census(source.get()?);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::InsertWidget { index, item: source.take() })
    }

    pub fn begin_remove_widget(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::RemoveWidget { id: source.take() })
    }

    pub fn begin_move_widget(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::MoveWidget { id: source.take(), index })
    }

    pub fn begin_patch_widget(&mut self, authority: FlowVcsAuthority, id: &mut FlowVcsSource<String>, source: &mut FlowVcsSource<Widget>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let mut census = flow_vcs_widget_census(source.get()?);
        census.include(FlowVcsCensus::leaf(id.get()?.len()));
        if widget_id_for(source.get()?) != id.get()?.as_str() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::PatchWidget { id: id.take(), item: source.take() })
    }

    pub fn begin_add_synapse(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<SynapseSpec>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = flow_vcs_synapse_census(source.get()?);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::InsertSynapse { index, item: source.take() })
    }

    pub fn begin_remove_synapse(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::RemoveSynapse { id: source.take() })
    }

    pub fn begin_move_synapse(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::MoveSynapse { id: source.take(), index })
    }

    pub fn begin_patch_synapse(&mut self, authority: FlowVcsAuthority, id: &mut FlowVcsSource<String>, source: &mut FlowVcsSource<SynapseSpec>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let mut census = flow_vcs_synapse_census(source.get()?);
        census.include(FlowVcsCensus::leaf(id.get()?.len()));
        if source.get()?.id.as_str() != id.get()?.as_str() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::PatchSynapse { id: id.take(), item: source.take() })
    }

    pub fn begin_set_layout(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<FlowLayoutEntry>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.id.len() + 16);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::SetLayout(source.take()))
    }

    /// 📥️ Document load: checks a fresh document out of `source` as the active version. It is a base reset, never a history step, so it needs a pristine history and writes no undo row.
    pub fn begin_replace_document(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<FlowHostSnapshot>) -> Result<FlowVcsHandle, FlowVcsFault> {
        if !self.undo.is_empty() || !self.redo.is_empty() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        let census = flow_vcs_fixture_census(source.get()?);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::ReplaceDocument(source.take()))
    }

    pub fn begin_undo(&mut self, authority: FlowVcsAuthority) -> Result<FlowVcsHandle, FlowVcsFault> {
        self.preflight(FlowVcsCensus::leaf(0))?;
        if self.undo.is_empty() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.admit(authority, FlowVcsCensus::leaf(0), FlowVcsAction::Undo)
    }

    pub fn begin_redo(&mut self, authority: FlowVcsAuthority) -> Result<FlowVcsHandle, FlowVcsFault> {
        self.preflight(FlowVcsCensus::leaf(0))?;
        if self.redo.is_empty() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.admit(authority, FlowVcsCensus::leaf(0), FlowVcsAction::Redo)
    }

    pub fn begin_checkpoint(&mut self, authority: FlowVcsAuthority) -> Result<FlowVcsHandle, FlowVcsFault> {
        self.preflight(FlowVcsCensus::leaf(0))?;
        self.admit(authority, FlowVcsCensus::leaf(0), FlowVcsAction::Checkpoint)
    }
}
//#endregion 🌊️RetainedVcs

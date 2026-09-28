//! 📬️ Retained one-item Store preparation shared by Semio native editors.

use crate::protocol;
use semio_framework_plugin::plugin_app_close_prelude::store as app_store;
use std::sync::Arc;

pub(crate) const PAGE_BYTES: usize = 4_096;

pub(crate) enum StructuralCopyStep {
    Progress { items: usize, bytes: usize },
    Complete,
}

pub(crate) trait StructuralMutationCopy<S, M>: Send {
    fn advance(&mut self, base: &S, mutation: &M, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String>;
    fn take_result(&mut self) -> Option<(S, M)>;
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

pub(crate) struct StructuralPreparationFactory<S, M> {
    prefix: &'static str,
    recognizes: fn(&M) -> bool,
    preflight: fn(&M) -> Result<usize, String>,
    copy: fn() -> Box<dyn StructuralMutationCopy<S, M>>,
    mutation_retirement: Arc<dyn app_store::ArtifactOwnedValueRetirementFactory<M>>,
    snapshot_retirement: Arc<dyn app_store::SnapshotRetirementFactory<S>>,
}

impl<S, M> StructuralPreparationFactory<S, M> {
    pub(crate) fn new(
        prefix: &'static str,
        recognizes: fn(&M) -> bool,
        preflight: fn(&M) -> Result<usize, String>,
        copy: fn() -> Box<dyn StructuralMutationCopy<S, M>>,
        mutation_retirement: Arc<dyn app_store::ArtifactOwnedValueRetirementFactory<M>>,
        snapshot_retirement: Arc<dyn app_store::SnapshotRetirementFactory<S>>,
    ) -> Self {
        Self { prefix, recognizes, preflight, copy, mutation_retirement, snapshot_retirement }
    }
}

impl<S, M> app_store::ArtifactStoreOneItemPreparationFactory<S, M> for StructuralPreparationFactory<S, M>
where
    S: Send + Sync + 'static,
    M: app_store::ArtifactCanonicalJson + Send + Sync + 'static,
{
    fn preflight(&self, mutation: &M, description: Option<&str>, lane: app_store::HistoryLane) -> Result<app_store::ArtifactStoreOneItemFootprint, String> {
        if !(self.recognizes)(mutation) || lane != app_store::HistoryLane::Document || description.is_some_and(|value| value.len() > app_store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err(format!("{}-admission", self.prefix));
        }
        let retained_bytes = (self.preflight)(mutation)?;
        if retained_bytes > app_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES {
            return Err(format!("{}-payload", self.prefix));
        }
        Ok(app_store::ArtifactStoreOneItemFootprint::for_one_item(1, retained_bytes.max(1)))
    }

    fn begin(&self, request: app_store::ArtifactStoreOneItemPreparationRequest<S, M>) -> Result<Box<dyn app_store::ArtifactStoreOneItemPreparation<S, M>>, app_store::ArtifactStoreOneItemPreparationRequest<S, M>> {
        let admitted = request.lane == app_store::HistoryLane::Document
            && request.operation == request.authority.operation()
            && request.generation == request.authority.generation()
            && request.base_revision == request.authority.base_revision()
            && request.authority.actor().len() <= app_store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            && request.description.as_ref().is_none_or(|value| value.len() <= app_store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES)
            && (self.recognizes)(&request.mutation)
            && (self.preflight)(&request.mutation).is_ok_and(|bytes| bytes <= app_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
        if !admitted {
            return Err(request);
        }
        Ok(Box::new(StructuralPreparation {
            prefix: self.prefix,
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            copy: Some((self.copy)()),
            mutation_retirement: Arc::clone(&self.mutation_retirement),
            snapshot_retirement: Arc::clone(&self.snapshot_retirement),
            sealer: None,
            external_retirement: None,
            checkpoint: Default::default(),
            seal_base_checkpoint: None,
            phase: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

struct StructuralPreparation<S, M> {
    prefix: &'static str,
    base: Option<app_store::SnapshotRead<S>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<Arc<app_store::ArtifactStoreOneItemLiveAuthority>>,
    copy: Option<Box<dyn StructuralMutationCopy<S, M>>>,
    mutation_retirement: Arc<dyn app_store::ArtifactOwnedValueRetirementFactory<M>>,
    snapshot_retirement: Arc<dyn app_store::SnapshotRetirementFactory<S>>,
    sealer: Option<app_store::ArtifactStoreOneItemSealer<S, M>>,
    external_retirement: Option<Box<dyn app_store::ErasedSnapshotRetirement>>,
    checkpoint: app_store::ArtifactStoreOneItemCheckpoint,
    seal_base_checkpoint: Option<app_store::ArtifactStoreOneItemCheckpoint>,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

impl<S, M> StructuralPreparation<S, M> {
    fn progress(&mut self, items: usize, bytes: usize) -> app_store::ArtifactStoreOneItemPreparationStep {
        self.checkpoint.cursor = self.checkpoint.cursor.saturating_add(1);
        self.checkpoint.completed_items = self.checkpoint.completed_items.saturating_add(items as u32);
        self.checkpoint.completed_bytes = self.checkpoint.completed_bytes.saturating_add(bytes as u64);
        app_store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint)
    }

    fn is_terminal_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.copy.is_none() && self.sealer.is_none() && self.external_retirement.is_none()
    }
}

impl<S, M> app_store::ArtifactStoreOneItemPreparation<S, M> for StructuralPreparation<S, M>
where
    S: Send + Sync + 'static,
    M: app_store::ArtifactCanonicalJson + Send + 'static,
{
    fn advance(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(app_store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if let Some(sealer) = self.sealer.as_mut() {
            let step = sealer.advance(grant)?;
            let checkpoint = match step {
                app_store::ArtifactStoreOneItemPreparationStep::Progress(checkpoint) | app_store::ArtifactStoreOneItemPreparationStep::Prepared(checkpoint) => checkpoint,
                app_store::ArtifactStoreOneItemPreparationStep::Blocked => return Ok(app_store::ArtifactStoreOneItemPreparationStep::Blocked),
            };
            let base = self.seal_base_checkpoint.ok_or_else(|| format!("{}-seal-checkpoint-owner", self.prefix))?;
            self.checkpoint = app_store::ArtifactStoreOneItemCheckpoint {
                cursor: base.cursor.saturating_add(checkpoint.cursor),
                completed_items: base.completed_items.saturating_add(checkpoint.completed_items),
                completed_bytes: base.completed_bytes.saturating_add(checkpoint.completed_bytes),
                digest: checkpoint.digest,
            };
            if matches!(step, app_store::ArtifactStoreOneItemPreparationStep::Prepared(_)) {
                self.checkpoint.digest = sealer.prepared().ok_or_else(|| format!("{}-prepared-owner", self.prefix))?.edit_digest();
                return Ok(app_store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
            }
            return Ok(app_store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        match self.phase {
            0 => {
                let step = self.copy.as_mut().ok_or_else(|| format!("{}-copy-owner", self.prefix))?.advance(
                    self.base.as_ref().ok_or_else(|| format!("{}-base-owner", self.prefix))?.get(),
                    self.mutation.as_ref().ok_or_else(|| format!("{}-mutation-owner", self.prefix))?,
                    grant.maximum_items,
                    grant.maximum_bytes.min(PAGE_BYTES),
                )?;
                match step {
                    StructuralCopyStep::Progress { items, bytes } => Ok(self.progress(items, bytes)),
                    StructuralCopyStep::Complete => {
                        self.phase = 1;
                        Ok(self.progress(1, 0))
                    }
                }
            }
            1 => {
                let (post, inverse) = self.copy.as_mut().ok_or_else(|| format!("{}-copy-owner", self.prefix))?.take_result().ok_or_else(|| format!("{}-copy-result", self.prefix))?;
                let mutation = self.mutation.take().ok_or_else(|| format!("{}-mutation-owner", self.prefix))?;
                let authority = self.authority.as_ref().ok_or_else(|| format!("{}-authority-owner", self.prefix))?;
                let id = authority.edit_id();
                let edit = protocol::Edit {
                    id: id.clone(),
                    actor: Some(authority.actor().to_string()),
                    forwards: vec![mutation],
                    inverse: vec![inverse],
                    mutation_meta: vec![protocol::MutationMeta {
                        mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
                        dependencies: Vec::new(),
                        base_version: authority.base_applied_edit_count() as u64,
                        author_id: Some(protocol::ActorId(authority.actor().to_string())),
                        timestamp: authority.next_clock(),
                        undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                        payload_hash: None,
                        semantic_kind: None,
                        label: None,
                        group_id: authority.group_id().map(str::to_owned),
                        origin: Default::default(),
                    }],
                    description: self.description.take(),
                    coalesce_key: None,
                    sequence_number: authority.next_sequence_number(),
                    started_at: String::new(),
                    finished_at: None,
                };
                self.sealer = Some(authority.begin_one_item_seal(edit, Arc::new(post), Arc::clone(&self.mutation_retirement), Arc::clone(&self.snapshot_retirement)));
                self.seal_base_checkpoint = Some(self.checkpoint);
                self.phase = 2;
                Ok(self.progress(1, 0))
            }
            _ => Err(format!("{}-preparation-state", self.prefix)),
        }
    }

    fn checkpoint(&self) -> app_store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&app_store::ArtifactStoreOneItemPrepared<S, M>> {
        self.sealer.as_ref().and_then(app_store::ArtifactStoreOneItemSealer::prepared)
    }

    fn take_prepared(&mut self) -> Option<app_store::ArtifactStoreOneItemPrepared<S, M>> {
        self.sealer.as_mut().and_then(app_store::ArtifactStoreOneItemSealer::take_prepared)
    }

    fn cancel(&mut self) {
        self.cancelled = true;
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.cancel();
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.begin_close();
        }
        if let Some(copy) = self.copy.as_mut() {
            copy.begin_close();
        }
    }

    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, String> {
        if !self.closing || !grant.permits_one() {
            return Ok(app_store::SnapshotRetirementStep::Blocked);
        }
        if let Some(active) = self.external_retirement.as_mut() {
            let step = active.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !active.terminal_is_empty() {
                    return Err(format!("{}-retirement-witness", self.prefix));
                }
                self.external_retirement = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(sealer) = self.sealer.as_mut() {
            let step = sealer.close_step(grant)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !sealer.terminal_is_empty() {
                    return Err(format!("{}-sealer-witness", self.prefix));
                }
                self.sealer = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(copy) = self.copy.as_mut() {
            let step = copy.close_step(grant)?;
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !copy.terminal_is_empty() {
                return Err(format!("{}-copy-witness", self.prefix));
            }
            self.copy = None;
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(mutation) = self.mutation.take() {
            self.external_retirement = Some(self.mutation_retirement.retire_owned(mutation));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(description) = self.description.take() {
            self.external_retirement = Some(app_store::retirement::owned_retirement(description));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err(format!("{}-base-return", self.prefix));
            }
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.take() {
            self.external_retirement = Some(authority.retire());
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(app_store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.is_terminal_empty()
    }
}

impl<S, M> Drop for StructuralPreparation<S, M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.is_terminal_empty(), "Semio structural preparation dropped with live owners");
    }
}

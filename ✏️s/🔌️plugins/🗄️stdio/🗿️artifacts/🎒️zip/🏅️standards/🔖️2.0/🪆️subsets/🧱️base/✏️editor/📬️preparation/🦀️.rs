//! 📬️ Bounded retained publication for archive text edits.

use crate::schema::mutations::{add_entry, remove_entry, rename_entry, set_archive_comment, set_entry_data};
use crate::schema::snapshot::{ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata};
use crate::{ZipMutation, ZipSnapshot};
use semio_framework_plugin::plugin_app_close_prelude::store as app_store;
use semio_s_artifact_stdio_contract::editing::{NativeEditPreparationRoute, RetainedBytesCopy, RetainedTextCopy};
use std::{mem::ManuallyDrop, sync::Arc};

const PAGE_BYTES: usize = 4_096;
const MAXIMUM_STRUCTURAL_ITEMS: usize = 4_096;
const MAXIMUM_TEXT_BYTES: usize = u16::MAX as usize;

fn structural_items(snapshot: &ZipSnapshot) -> Option<usize> {
    snapshot.entries.iter().try_fold(snapshot.entries.len(), |total, entry| total.checked_add(entry.metadata.local.extra_fields.len())?.checked_add(entry.metadata.central.extra_fields.len()))
}

pub(super) fn route(prefix: &'static str) -> Option<NativeEditPreparationRoute<ZipSnapshot, ZipMutation>> {
    Some(NativeEditPreparationRoute::new(|mutation| matches!(mutation, ZipMutation::RenameEntry(_) | ZipMutation::SetArchiveComment(_)), Arc::new(ZipPreparationFactory { prefix })))
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct ZipPreparationFactory {
    prefix: &'static str,
}

impl app_store::ArtifactStoreOneItemPreparationFactory<ZipSnapshot, ZipMutation> for ZipPreparationFactory {
    fn preflight(&self, mutation: &ZipMutation, lane: app_store::HistoryLane) -> Result<app_store::ArtifactStoreOneItemFootprint, String> {
        let admitted = lane == app_store::HistoryLane::Document
            && match mutation {
                ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) => !name.is_empty() && name != new_name && name.len() <= MAXIMUM_TEXT_BYTES && !new_name.is_empty() && new_name.len() <= MAXIMUM_TEXT_BYTES,
                ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment, .. }) => comment.len() <= MAXIMUM_TEXT_BYTES,
                _ => false,
            };
        admitted.then(|| app_store::ArtifactStoreOneItemFootprint::for_leaf(mutation, app_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)).ok_or_else(|| format!("{}-admission", self.prefix))
    }

    fn begin(
        &self,
        request: app_store::ArtifactStoreOneItemPreparationRequest<ZipSnapshot, ZipMutation>,
    ) -> Result<Box<dyn app_store::ArtifactStoreOneItemPreparation<ZipSnapshot, ZipMutation>>, app_store::ArtifactStoreOneItemPreparationRequest<ZipSnapshot, ZipMutation>> {
        let admitted = request.lane == app_store::HistoryLane::Document
            && request.operation == request.authority.operation()
            && request.generation == request.authority.generation()
            && request.base_revision == request.authority.base_revision()
            && request.authority.actor().len() <= app_store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            && structural_items(request.base.get()).is_some_and(|items| items <= MAXIMUM_STRUCTURAL_ITEMS)
            && request.base.get().comment.len() <= MAXIMUM_TEXT_BYTES
            && match &request.mutation {
                ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) => !name.is_empty() && name != new_name && name.len() <= MAXIMUM_TEXT_BYTES && !new_name.is_empty() && new_name.len() <= MAXIMUM_TEXT_BYTES,
                ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment, .. }) => comment.len() <= MAXIMUM_TEXT_BYTES,
                _ => false,
            };
        if !admitted {
            return Err(request);
        }
        Ok(Box::new(ZipPreparation {
            prefix: self.prefix,
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            inverse_copy: ZipInverseCopy::default(),
            inverse: None,
            post_copy: ZipPostCopy::default(),
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

struct ZipPreparation {
    prefix: &'static str,
    base: Option<app_store::SnapshotRead<ZipSnapshot>>,
    mutation: Option<ZipMutation>,
    authority: Option<Arc<app_store::ArtifactStoreOneItemLiveAuthority>>,
    inverse_copy: ZipInverseCopy,
    inverse: Option<ZipMutation>,
    post_copy: ZipPostCopy,
    sealer: Option<app_store::ArtifactStoreOneItemSealer<ZipSnapshot, ZipMutation>>,
    external_retirement: Option<Box<dyn app_store::ErasedSnapshotRetirement>>,
    checkpoint: app_store::ArtifactStoreOneItemCheckpoint,
    seal_base_checkpoint: Option<app_store::ArtifactStoreOneItemCheckpoint>,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

impl ZipPreparation {
    fn progress(&mut self, bytes: usize) -> app_store::ArtifactStoreOneItemPreparationStep {
        self.checkpoint.cursor = self.checkpoint.cursor.saturating_add(1);
        self.checkpoint.completed_items = self.checkpoint.completed_items.saturating_add(1);
        self.checkpoint.completed_bytes = self.checkpoint.completed_bytes.saturating_add(bytes as u64);
        app_store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint)
    }
}

impl app_store::ArtifactStoreOneItemPreparation<ZipSnapshot, ZipMutation> for ZipPreparation {
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
        if grant.maximum_bytes < PAGE_BYTES {
            return Ok(app_store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        match self.phase {
            0 => {
                let bytes =
                    self.inverse_copy.advance(self.base.as_ref().ok_or_else(|| format!("{}-base-owner", self.prefix))?.get(), self.mutation.as_ref().ok_or_else(|| format!("{}-mutation-owner", self.prefix))?, grant.maximum_bytes.min(PAGE_BYTES))?;
                if self.inverse_copy.is_complete() {
                    self.inverse = self.inverse_copy.take();
                    self.phase = 1;
                }
                Ok(self.progress(bytes))
            }
            1 => {
                let bytes =
                    self.post_copy.advance(self.base.as_ref().ok_or_else(|| format!("{}-base-owner", self.prefix))?.get(), self.mutation.as_ref().ok_or_else(|| format!("{}-mutation-owner", self.prefix))?, grant.maximum_bytes.min(PAGE_BYTES))?;
                if self.post_copy.is_complete() {
                    self.phase = 2;
                }
                Ok(self.progress(bytes))
            }
            2 => {
                let post = self.post_copy.take().ok_or_else(|| format!("{}-post-copy", self.prefix))?;
                crate::standards::v2_0::subsets::base::io::validate_zip_snapshot_serialization(&post).map_err(|error| format!("{}-post-validation-{error}", self.prefix))?;
                let post = Arc::new(post);
                let mutation = self.mutation.take().ok_or_else(|| format!("{}-mutation-owner", self.prefix))?;
                let inverse = self.inverse.take().ok_or_else(|| format!("{}-inverse-owner", self.prefix))?;
                let authority = self.authority.as_ref().ok_or_else(|| format!("{}-authority-owner", self.prefix))?;
                let edit = authority.next_edit(mutation, vec![inverse]);
                self.sealer = Some(authority.begin_one_item_seal(edit, post, Arc::new(ZipMutationRetirementFactory), Arc::new(ZipSnapshotRetirementFactory)));
                self.seal_base_checkpoint = Some(self.checkpoint);
                self.phase = 3;
                Ok(self.progress(0))
            }
            _ => Err(format!("{}-preparation-state", self.prefix)),
        }
    }

    fn checkpoint(&self) -> app_store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&app_store::ArtifactStoreOneItemPrepared<ZipSnapshot, ZipMutation>> {
        self.sealer.as_ref().and_then(app_store::ArtifactStoreOneItemSealer::prepared)
    }
    fn take_prepared(&mut self) -> Option<app_store::ArtifactStoreOneItemPrepared<ZipSnapshot, ZipMutation>> {
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
        self.inverse_copy.begin_close();
        self.post_copy.begin_close();
    }
    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || !grant.permits_one() {
            return Ok(app_store::SnapshotRetirementStep::Blocked);
        }
        if let Some(active) = self.external_retirement.as_mut() {
            let step = active.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !active.terminal_is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, format!("{}-retirement-witness", self.prefix)));
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
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, format!("{}-sealer-witness", self.prefix)));
                }
                self.sealer = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if !self.post_copy.terminal_is_empty() {
            return self.post_copy.close_step(grant);
        }
        if !self.inverse_copy.terminal_is_empty() {
            return self.inverse_copy.close_step(grant);
        }
        if let Some(value) = self.inverse.take().or_else(|| self.mutation.take()) {
            self.external_retirement = Some(app_store::ArtifactOwnedValueRetirementFactory::retire_owned(&ZipMutationRetirementFactory, value));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, format!("{}-base-return", self.prefix)));
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
        self.closing
            && self.base.is_none()
            && self.mutation.is_none()
           
            && self.authority.is_none()
            && self.inverse.is_none()
            && self.inverse_copy.terminal_is_empty()
            && self.post_copy.terminal_is_empty()
            && self.sealer.is_none()
            && self.external_retirement.is_none()
    }
}

impl Drop for ZipPreparation {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || app_store::ArtifactStoreOneItemPreparation::terminal_is_empty(self), "ZIP preparation dropped with live owners");
    }
}

#[derive(Default)]
struct ZipInverseCopy {
    phase: u8,
    first: RetainedTextCopy,
    second: RetainedTextCopy,
    first_value: Option<String>,
    inverse: Option<ZipMutation>,
    complete: bool,
    closing: bool,
    retirement: Option<Box<dyn app_store::ErasedSnapshotRetirement>>,
}

impl ZipInverseCopy {
    fn advance(&mut self, base: &ZipSnapshot, mutation: &ZipMutation, maximum_bytes: usize) -> Result<usize, String> {
        if self.complete || self.closing || maximum_bytes == 0 {
            return Ok(0);
        }
        match mutation {
            ZipMutation::SetArchiveComment(_) => {
                let bytes = self.first.advance(&base.comment, maximum_bytes)?.unwrap_or(0);
                if self.first.is_complete() {
                    self.inverse = Some(ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: self.first.take().ok_or("stdio-zip-base-snapshot-edit-inverse-comment")?, comment_utf8: base.comment_utf8 }));
                    self.complete = true;
                }
                Ok(bytes)
            }
            ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) => match self.phase {
                0 => {
                    let bytes = self.first.advance(new_name, maximum_bytes)?.unwrap_or(0);
                    if self.first.is_complete() {
                        self.first_value = self.first.take();
                        self.phase = 1;
                    }
                    Ok(bytes)
                }
                1 => {
                    let bytes = self.second.advance(name, maximum_bytes)?.unwrap_or(0);
                    if self.second.is_complete() {
                        self.inverse = Some(ZipMutation::RenameEntry(rename_entry::RenameEntry {
                            name: self.first_value.take().ok_or("stdio-zip-base-snapshot-edit-inverse-new-name")?,
                            new_name: self.second.take().ok_or("stdio-zip-base-snapshot-edit-inverse-old-name")?,
                        }));
                        self.complete = true;
                    }
                    Ok(bytes)
                }
                _ => Err("stdio-zip-base-snapshot-edit-inverse-state".into()),
            },
            _ => Err("stdio-zip-base-snapshot-edit-unsupported-mutation".into()),
        }
    }
    fn is_complete(&self) -> bool {
        self.complete
    }
    fn take(&mut self) -> Option<ZipMutation> {
        if !self.complete {
            return None;
        }
        self.complete = false;
        self.inverse.take()
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || !grant.permits_one() {
            return Ok(app_store::SnapshotRetirementStep::Blocked);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let step = retirement.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !retirement.terminal_is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "stdio-zip-base-snapshot-edit-inverse-retirement-witness"));
                }
                self.retirement = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        for copy in [&mut self.first, &mut self.second] {
            match copy.close_step(grant.maximum_items, grant.maximum_bytes) {
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => return Ok(app_store::SnapshotRetirementStep::Pending { released_items, released_bytes }),
                semio_framework_job::InteractiveJobCloseStep::Blocked => return Ok(app_store::SnapshotRetirementStep::Blocked),
                semio_framework_job::InteractiveJobCloseStep::Complete => {}
            }
        }
        if let Some(value) = self.inverse.take() {
            self.retirement = Some(app_store::ArtifactOwnedValueRetirementFactory::retire_owned(&ZipMutationRetirementFactory, value));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(value) = self.first_value.take() {
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(value));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.complete = false;
        Ok(app_store::SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.first.terminal_is_empty() && self.second.terminal_is_empty() && self.first_value.is_none() && self.inverse.is_none() && self.retirement.is_none()
    }
}

#[derive(Default)]
struct PartialEntry {
    name: Option<String>,
    data: Option<Vec<u8>>,
    metadata: PartialMetadata,
    phase: u8,
}

#[derive(Default)]
struct PartialMetadata {
    phase: u8,
    extra_index: usize,
    current_extra_id: Option<u16>,
    local_extra_fields: Vec<ZipExtraField>,
    local_unicode_path_legacy_name: Option<Vec<u8>>,
    central_extra_fields: Vec<ZipExtraField>,
    central_unicode_path_legacy_name: Option<Vec<u8>>,
    central_comment: Option<String>,
    central_unicode_comment_legacy: Option<Vec<u8>>,
}

#[derive(Default)]
struct ZipPostCopy {
    phase: u8,
    schema: Option<String>,
    entries: Vec<ZipEntry>,
    entry_index: usize,
    entry: Option<PartialEntry>,
    comment: Option<String>,
    comment_utf8: Option<bool>,
    text: RetainedTextCopy,
    bytes: RetainedBytesCopy,
    rename_matches: usize,
    rename_collision: bool,
    complete: bool,
    closing: bool,
    retirement: Option<Box<dyn app_store::ErasedSnapshotRetirement>>,
}

impl ZipPostCopy {
    fn advance(&mut self, source: &ZipSnapshot, mutation: &ZipMutation, maximum_bytes: usize) -> Result<usize, String> {
        if maximum_bytes == 0 || self.complete || self.closing {
            return Ok(0);
        }
        if structural_items(source).is_none_or(|items| items > MAXIMUM_STRUCTURAL_ITEMS) {
            return Err("stdio-zip-base-snapshot-edit-structural-envelope".into());
        }
        match self.phase {
            0 => {
                let bytes = self.text.advance(&source.schema, maximum_bytes)?.unwrap_or(0);
                if self.text.is_complete() {
                    self.schema = self.text.take();
                    self.phase = 1;
                }
                Ok(bytes)
            }
            1 => self.copy_entries(source, mutation, maximum_bytes),
            2 => {
                let comment = match mutation {
                    ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment, .. }) => comment,
                    ZipMutation::RenameEntry(_) => &source.comment,
                    _ => return Err("stdio-zip-base-snapshot-edit-unsupported-mutation".into()),
                };
                let bytes = self.text.advance(comment, maximum_bytes)?.unwrap_or(0);
                if self.text.is_complete() {
                    self.comment = self.text.take();
                    self.comment_utf8 = Some(match mutation {
                        ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment_utf8, .. }) => *comment_utf8,
                        ZipMutation::RenameEntry(_) => source.comment_utf8,
                        _ => return Err("stdio-zip-base-snapshot-edit-unsupported-mutation".into()),
                    });
                    self.phase = 3;
                }
                Ok(bytes)
            }
            3 => {
                self.complete = true;
                Ok(0)
            }
            _ => Err("stdio-zip-base-snapshot-edit-copy-state".into()),
        }
    }

    fn copy_entries(&mut self, source: &ZipSnapshot, mutation: &ZipMutation, maximum_bytes: usize) -> Result<usize, String> {
        if self.entry_index == source.entries.len() {
            if matches!(mutation, ZipMutation::RenameEntry(_)) && (self.rename_matches != 1 || self.rename_collision) {
                return Err("stdio-zip-base-snapshot-edit-stale-rename".into());
            }
            self.phase = 2;
            return Ok(0);
        }
        let source_entry = &source.entries[self.entry_index];
        if source_entry.name.len() > MAXIMUM_TEXT_BYTES {
            return Err("stdio-zip-base-snapshot-edit-entry-name-envelope".into());
        }
        if self.entry.is_none() {
            if let ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) = mutation {
                if source_entry.name == *name {
                    self.rename_matches = self.rename_matches.saturating_add(1);
                } else if source_entry.name == *new_name {
                    self.rename_collision = true;
                }
            }
            self.entry = Some(PartialEntry::default());
        }
        let entry = self.entry.as_mut().ok_or("stdio-zip-base-snapshot-edit-entry-owner")?;
        match entry.phase {
            0 => {
                let name = match mutation {
                    ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) if source_entry.name == *name => new_name,
                    ZipMutation::RenameEntry(_) | ZipMutation::SetArchiveComment(_) => &source_entry.name,
                    _ => return Err("stdio-zip-base-snapshot-edit-unsupported-mutation".into()),
                };
                let bytes = self.text.advance(name, maximum_bytes)?.unwrap_or(0);
                if self.text.is_complete() {
                    entry.name = self.text.take();
                    entry.phase = 1;
                }
                Ok(bytes)
            }
            1 => {
                let bytes = self.bytes.advance(&source_entry.data, maximum_bytes)?.unwrap_or(0);
                if self.bytes.is_complete() {
                    entry.data = self.bytes.take();
                    entry.phase = 2;
                }
                Ok(bytes)
            }
            2 => {
                let bytes = Self::copy_metadata(&mut self.text, &mut self.bytes, &source_entry.metadata, &mut entry.metadata, maximum_bytes)?;
                if entry.metadata.phase == 7 {
                    let entry = self.entry.take().ok_or("stdio-zip-base-snapshot-edit-entry-owner")?;
                    self.entries.push(ZipEntry {
                        name: entry.name.ok_or("stdio-zip-base-snapshot-edit-entry-name")?,
                        data: entry.data.ok_or("stdio-zip-base-snapshot-edit-entry-data")?,
                        metadata: Self::finish_metadata(&source_entry.metadata, entry.metadata),
                    });
                    self.entry_index += 1;
                }
                Ok(bytes)
            }
            _ => Err("stdio-zip-base-snapshot-edit-entry-state".into()),
        }
    }

    fn copy_metadata(text: &mut RetainedTextCopy, bytes: &mut RetainedBytesCopy, source: &ZipEntryMetadata, target: &mut PartialMetadata, maximum_bytes: usize) -> Result<usize, String> {
        match target.phase {
            0 => Self::copy_extra_fields(bytes, &source.local.extra_fields, &mut target.local_extra_fields, &mut target.extra_index, &mut target.current_extra_id, &mut target.phase, maximum_bytes),
            1 => Self::copy_optional_bytes(bytes, source.local.unicode_path_legacy_name.as_deref(), &mut target.local_unicode_path_legacy_name, &mut target.phase, maximum_bytes),
            2 => Self::copy_extra_fields(bytes, &source.central.extra_fields, &mut target.central_extra_fields, &mut target.extra_index, &mut target.current_extra_id, &mut target.phase, maximum_bytes),
            3 => Self::copy_optional_bytes(bytes, source.central.unicode_path_legacy_name.as_deref(), &mut target.central_unicode_path_legacy_name, &mut target.phase, maximum_bytes),
            4 => {
                let copied = text.advance(&source.central.comment, maximum_bytes)?.unwrap_or(0);
                if text.is_complete() {
                    target.central_comment = text.take();
                    target.phase += 1;
                }
                Ok(copied)
            }
            5 => Self::copy_optional_bytes(bytes, source.central.unicode_comment_legacy.as_deref(), &mut target.central_unicode_comment_legacy, &mut target.phase, maximum_bytes),
            6 => {
                target.phase = 7;
                Ok(0)
            }
            _ => Err("stdio-zip-base-snapshot-edit-metadata-state".into()),
        }
    }

    fn copy_extra_fields(bytes: &mut RetainedBytesCopy, source: &[ZipExtraField], target_fields: &mut Vec<ZipExtraField>, extra_index: &mut usize, current_extra_id: &mut Option<u16>, phase: &mut u8, maximum_bytes: usize) -> Result<usize, String> {
        if *extra_index == source.len() {
            *extra_index = 0;
            *phase += 1;
            return Ok(0);
        }
        let source_field = &source[*extra_index];
        current_extra_id.get_or_insert(source_field.id);
        let copied = bytes.advance(&source_field.data, maximum_bytes)?.unwrap_or(0);
        if bytes.is_complete() {
            target_fields.push(ZipExtraField { id: current_extra_id.take().ok_or("stdio-zip-base-snapshot-edit-extra-field-owner")?, data: bytes.take().ok_or("stdio-zip-base-snapshot-edit-extra-data-owner")? });
            *extra_index += 1;
        }
        Ok(copied)
    }

    fn copy_optional_bytes(bytes: &mut RetainedBytesCopy, source: Option<&[u8]>, target_value: &mut Option<Vec<u8>>, phase: &mut u8, maximum_bytes: usize) -> Result<usize, String> {
        let Some(source) = source else {
            *phase += 1;
            return Ok(0);
        };
        let copied = bytes.advance(source, maximum_bytes)?.unwrap_or(0);
        if bytes.is_complete() {
            *target_value = bytes.take();
            *phase += 1;
        }
        Ok(copied)
    }

    fn finish_metadata(source: &ZipEntryMetadata, target: PartialMetadata) -> ZipEntryMetadata {
        ZipEntryMetadata {
            compression_method: source.compression_method,
            local: ZipLocalHeaderMetadata {
                version_needed: source.local.version_needed,
                flags: source.local.flags,
                modified_time: source.local.modified_time,
                modified_date: source.local.modified_date,
                extra_fields: target.local_extra_fields,
                unicode_path_legacy_name: target.local_unicode_path_legacy_name,
            },
            central: ZipCentralHeaderMetadata {
                version_made_by: source.central.version_made_by,
                version_needed: source.central.version_needed,
                flags: source.central.flags,
                modified_time: source.central.modified_time,
                modified_date: source.central.modified_date,
                extra_fields: target.central_extra_fields,
                unicode_path_legacy_name: target.central_unicode_path_legacy_name,
                comment: target.central_comment.unwrap_or_default(),
                unicode_comment_legacy: target.central_unicode_comment_legacy,
                internal_attributes: source.central.internal_attributes,
                external_attributes: source.central.external_attributes,
            },
            data_descriptor_signature: source.data_descriptor_signature,
        }
    }

    fn is_complete(&self) -> bool {
        self.complete
    }
    fn take(&mut self) -> Option<ZipSnapshot> {
        if !self.complete {
            return None;
        }
        self.complete = false;
        Some(self.take_partial_snapshot())
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || !grant.permits_one() {
            return Ok(app_store::SnapshotRetirementStep::Blocked);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let step = retirement.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !retirement.terminal_is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "stdio-zip-base-snapshot-edit-copy-retirement-witness"));
                }
                self.retirement = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        match self.text.close_step(grant.maximum_items, grant.maximum_bytes) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => return Ok(app_store::SnapshotRetirementStep::Pending { released_items, released_bytes }),
            semio_framework_job::InteractiveJobCloseStep::Blocked => return Ok(app_store::SnapshotRetirementStep::Blocked),
            semio_framework_job::InteractiveJobCloseStep::Complete => {}
        }
        match self.bytes.close_step(grant.maximum_items, grant.maximum_bytes) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => return Ok(app_store::SnapshotRetirementStep::Pending { released_items, released_bytes }),
            semio_framework_job::InteractiveJobCloseStep::Blocked => return Ok(app_store::SnapshotRetirementStep::Blocked),
            semio_framework_job::InteractiveJobCloseStep::Complete => {}
        }
        if self.has_partial_owners() {
            self.retirement = Some(app_store::SnapshotRetirementFactory::retire(&ZipSnapshotRetirementFactory, Arc::new(self.take_partial_snapshot())));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.complete = false;
        Ok(app_store::SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && !self.has_partial_owners() && self.text.terminal_is_empty() && self.bytes.terminal_is_empty() && self.retirement.is_none()
    }
    fn has_partial_owners(&self) -> bool {
        self.schema.is_some() || !self.entries.is_empty() || self.entry.is_some() || self.comment.is_some() || self.comment_utf8.is_some()
    }
    fn take_partial_snapshot(&mut self) -> ZipSnapshot {
        if let Some(entry) = self.entry.take() {
            self.entries.push(ZipEntry { name: entry.name.unwrap_or_default(), data: entry.data.unwrap_or_default(), metadata: Self::finish_metadata(&ZipEntryMetadata::default(), entry.metadata) });
        }
        ZipSnapshot { schema: self.schema.take().unwrap_or_default(), entries: std::mem::take(&mut self.entries), comment: self.comment.take().unwrap_or_default(), comment_utf8: self.comment_utf8.take().unwrap_or(true) }
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct ZipMutationRetirementFactory;
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct ZipSnapshotRetirementFactory;

impl app_store::ArtifactOwnedValueRetirementFactory<ZipMutation> for ZipMutationRetirementFactory {
    fn retire_owned(&self, value: ZipMutation) -> Box<dyn app_store::ErasedSnapshotRetirement> {
        semio_framework_value::retirement::owned_retirement(value)
    }
}
impl app_store::SnapshotRetirementFactory<ZipSnapshot> for ZipSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &Arc<ZipSnapshot>) -> usize { semio_framework_value::retirement::shared_retirement_birth_bytes::<ZipSnapshot>() }

    fn retire(&self, value: Arc<ZipSnapshot>) -> Box<dyn app_store::ErasedSnapshotRetirement> {
        semio_framework_value::retirement::shared_retirement(value)
    }
}

enum RetiredOwner {
    Snapshot(ZipSnapshot),
    Entries(Vec<ZipEntry>),
    Entry(ZipEntry),
    EntryMetadata(ZipEntryMetadata),
    LocalMetadata(ZipLocalHeaderMetadata),
    CentralMetadata(ZipCentralHeaderMetadata),
    ExtraFields(Vec<ZipExtraField>),
    ExtraField(ZipExtraField),
    Mutation(ZipMutation),
    String(String),
    Bytes(Vec<u8>),
}

struct ZipRetirementCursor(ManuallyDrop<Vec<RetiredOwner>>);

impl ZipRetirementCursor {
    fn push(&mut self, owner: RetiredOwner) {
        self.0.push(owner);
    }
}

impl semio_framework_value::retirement::RetirementCursor for ZipRetirementCursor {
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep {
        use semio_framework_value::retirement::RetirementStep;
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        let maximum_bytes = grant.maximum_copy_bytes;
        let Some(owner) = self.0.pop() else { return RetirementStep::Complete };
        match owner {
            RetiredOwner::String(value) => {
                self.push(RetiredOwner::Bytes(value.into_bytes()));
                RetirementStep::Bytes(0)
            }
            RetiredOwner::Bytes(mut value) => {
                if value.is_empty() {
                    return RetirementStep::Bytes(0);
                }
                if maximum_bytes == 0 {
                    self.push(RetiredOwner::Bytes(value));
                    return RetirementStep::BudgetExhausted;
                }
                let bytes = maximum_bytes.min(value.len());
                value.truncate(value.len() - bytes);
                if !value.is_empty() {
                    self.push(RetiredOwner::Bytes(value));
                }
                RetirementStep::ProcessedBytes(bytes)
            }
            RetiredOwner::Snapshot(value) => {
                self.push(RetiredOwner::String(value.comment));
                self.push(RetiredOwner::Entries(value.entries));
                self.push(RetiredOwner::String(value.schema));
                RetirementStep::Bytes(0)
            }
            RetiredOwner::Entries(mut entries) => {
                if let Some(entry) = entries.pop() {
                    self.push(RetiredOwner::Entries(entries));
                    self.push(RetiredOwner::Entry(entry));
                }
                RetirementStep::Bytes(0)
            }
            RetiredOwner::Entry(entry) => {
                self.push(RetiredOwner::EntryMetadata(entry.metadata));
                self.push(RetiredOwner::Bytes(entry.data));
                self.push(RetiredOwner::String(entry.name));
                RetirementStep::Bytes(0)
            }
            RetiredOwner::EntryMetadata(metadata) => {
                self.push(RetiredOwner::CentralMetadata(metadata.central));
                self.push(RetiredOwner::LocalMetadata(metadata.local));
                RetirementStep::Bytes(0)
            }
            RetiredOwner::LocalMetadata(metadata) => {
                if let Some(value) = metadata.unicode_path_legacy_name {
                    self.push(RetiredOwner::Bytes(value));
                }
                self.push(RetiredOwner::ExtraFields(metadata.extra_fields));
                RetirementStep::Bytes(0)
            }
            RetiredOwner::CentralMetadata(metadata) => {
                if let Some(value) = metadata.unicode_comment_legacy {
                    self.push(RetiredOwner::Bytes(value));
                }
                self.push(RetiredOwner::String(metadata.comment));
                if let Some(value) = metadata.unicode_path_legacy_name {
                    self.push(RetiredOwner::Bytes(value));
                }
                self.push(RetiredOwner::ExtraFields(metadata.extra_fields));
                RetirementStep::Bytes(0)
            }
            RetiredOwner::ExtraFields(mut fields) => {
                if let Some(field) = fields.pop() {
                    self.push(RetiredOwner::ExtraFields(fields));
                    self.push(RetiredOwner::ExtraField(field));
                }
                RetirementStep::Bytes(0)
            }
            RetiredOwner::ExtraField(field) => {
                self.push(RetiredOwner::Bytes(field.data));
                RetirementStep::Bytes(0)
            }
            RetiredOwner::Mutation(mutation) => {
                retire_mutation(&mut self.0, mutation);
                RetirementStep::Bytes(0)
            }
        }
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Drop for ZipRetirementCursor {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.0.is_empty(), "ZIP retirement cursor dropped before terminal emptiness");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}

fn retire_mutation(stack: &mut Vec<RetiredOwner>, mutation: ZipMutation) {
    match mutation {
        ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment, .. }) => stack.push(RetiredOwner::String(comment)),
        ZipMutation::AddEntry(add_entry::AddEntry { entry, before }) => {
            if let Some(before) = before {
                stack.push(RetiredOwner::String(before));
            }
            stack.push(RetiredOwner::Entry(entry));
        }
        ZipMutation::RemoveEntry(remove_entry::RemoveEntry { name }) => stack.push(RetiredOwner::String(name)),
        ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) => {
            stack.push(RetiredOwner::String(new_name));
            stack.push(RetiredOwner::String(name));
        }
        ZipMutation::SetEntryData(set_entry_data::SetEntryData { name, data }) => {
            stack.push(RetiredOwner::Bytes(data));
            stack.push(RetiredOwner::String(name));
        }
    }
}

impl semio_framework_value::retirement::RetireOwned for ZipSnapshot {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        Box::new(ZipRetirementCursor(ManuallyDrop::new(vec![RetiredOwner::Snapshot(self)])))
    }
}
impl semio_framework_value::retirement::RetireOwned for ZipMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        Box::new(ZipRetirementCursor(ManuallyDrop::new(vec![RetiredOwner::Mutation(self)])))
    }
}

fn canonical_text(value: &str) -> app_store::ArtifactCanonicalJsonValue<'_> {
    app_store::ArtifactCanonicalJsonValue::Scalar(app_store::ArtifactCanonicalJsonNode::String(value))
}
fn canonical_static_text<'a>(value: &'static str) -> app_store::ArtifactCanonicalJsonValue<'a> {
    app_store::ArtifactCanonicalJsonValue::Scalar(app_store::ArtifactCanonicalJsonNode::String(value))
}
fn canonical_bool<'a>(value: bool) -> app_store::ArtifactCanonicalJsonValue<'a> {
    app_store::ArtifactCanonicalJsonValue::Scalar(app_store::ArtifactCanonicalJsonNode::Bool(value))
}
fn canonical_object<'a, const N: usize>(mut fields: [(&'a str, app_store::ArtifactCanonicalJsonValue<'a>); N]) -> app_store::ArtifactCanonicalJsonValue<'a> {
    fields.sort_unstable_by(|left, right| left.0.cmp(right.0));
    app_store::ArtifactCanonicalJsonValue::Object(app_store::ArtifactCanonicalJsonObject::new(fields.into_iter()))
}

impl app_store::ArtifactCanonicalJson for ZipMutation {
    fn canonical_json_borrowed_root(&self) -> Result<Option<app_store::ArtifactCanonicalJsonValue<'_>>, String> {
        Ok(Some(match self {
            ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment, comment_utf8 }) => {
                canonical_object([("comment", canonical_text(comment)), ("commentUtf8", canonical_bool(*comment_utf8)), ("mutation", canonical_static_text("setArchiveComment"))])
            }
            ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) => canonical_object([("mutation", canonical_static_text("renameEntry")), ("name", canonical_text(name)), ("newName", canonical_text(new_name))]),
            _ => return Err("stdio-zip-base-snapshot-edit-canonical-mutation".into()),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("🧫️fixtures/🧵️bounded-copy/🔣️.json")).expect("bounded ZIP preparation fixture")
    }

    fn large_snapshot(bytes: usize) -> ZipSnapshot {
        let metadata = ZipEntryMetadata {
            local: ZipLocalHeaderMetadata { extra_fields: vec![ZipExtraField { id: 0xCAFE, data: (0..8_193).map(|index| (index % 251) as u8).collect() }], ..Default::default() },
            central: ZipCentralHeaderMetadata { extra_fields: vec![ZipExtraField { id: 0xBEEF, data: (0..8_195).map(|index| (index % 247) as u8).collect() }], comment: "member comment".repeat(257), ..Default::default() },
            ..Default::default()
        };
        ZipSnapshot {
            schema: "stdio.zip".into(),
            entries: vec![ZipEntry { name: "before.bin".into(), data: (0..bytes).map(|index| (index % 251) as u8).collect(), metadata }, ZipEntry { name: "sibling.txt".into(), data: b"unchanged".to_vec(), ..Default::default() }],
            comment: "original comment".into(),
            ..Default::default()
        }
    }

    #[test]
    fn post_copy_pages_payload_and_constructs_exact_forward_and_inverse_rename() {
        let fixture = fixture();
        let source = large_snapshot(fixture["largeEntryBytes"].as_u64().expect("largeEntryBytes") as usize);
        let replacement = "renamed".repeat(fixture["replacementRepeats"].as_u64().expect("replacementRepeats") as usize);
        let mutation = ZipMutation::RenameEntry(rename_entry::RenameEntry { name: "before.bin".into(), new_name: replacement.clone() });
        let page_bytes = fixture["pageBytes"].as_u64().expect("pageBytes") as usize;
        let mut inverse = ZipInverseCopy::default();
        while !inverse.is_complete() {
            assert!(inverse.advance(&source, &mutation, page_bytes).expect("inverse copy advances") <= page_bytes);
        }
        let copied_inverse = inverse.take().expect("inverse copy owns the exact inverse");
        assert_eq!(copied_inverse, ZipMutation::RenameEntry(rename_entry::RenameEntry { name: replacement.clone(), new_name: "before.bin".into() }));
        assert_eq!(vec![copied_inverse], <ZipMutation as protocol::Mutation<ZipSnapshot>>::inverse(&mutation, &source).expect("valid retained mutation inverse fixture"));
        inverse.begin_close();
        assert!(inverse.terminal_is_empty());

        let mut post = ZipPostCopy::default();
        let mut turns = 0;
        while !post.is_complete() {
            assert!(post.advance(&source, &mutation, page_bytes).expect("post copy advances") <= page_bytes);
            turns += 1;
            assert!(turns < 1_000, "bounded post copy converges");
        }
        assert!(turns > fixture["minimumCompleteTurns"].as_u64().expect("minimumCompleteTurns") as usize);
        let copied = post.take().expect("post copy owns a snapshot");
        assert_eq!(copied.entries[0].name, replacement);
        assert_eq!(copied.entries[0].data, source.entries[0].data);
        assert_eq!(copied.entries[1], source.entries[1]);
        assert_eq!(copied.comment, source.comment);
        let outcome = <ZipMutation as protocol::Mutation<ZipSnapshot>>::diff(&mutation, &source);
        assert_eq!(copied, protocol::apply_diff(outcome.diff(), &source).expect("independent mutation algebra applies the rename"));
        post.begin_close();
        assert!(post.terminal_is_empty());
    }

    #[test]
    fn comment_copy_changes_only_comment_and_cancellation_retires_partial_bytes_in_pages() {
        let fixture = fixture();
        let source = large_snapshot(fixture["largeEntryBytes"].as_u64().expect("largeEntryBytes") as usize);
        let replacement = "updated".repeat(fixture["replacementRepeats"].as_u64().expect("replacementRepeats") as usize);
        let mutation = ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: replacement, comment_utf8: true });
        let page_bytes = fixture["pageBytes"].as_u64().expect("pageBytes") as usize;
        let mut post = ZipPostCopy::default();
        for _ in 0..fixture["cancelAfterTurns"].as_u64().expect("cancelAfterTurns") {
            assert!(post.advance(&source, &mutation, page_bytes).expect("partial post copy advances") <= page_bytes);
        }
        post.begin_close();
        let grant = app_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: page_bytes };
        for _ in 0..fixture["maximumCloseTurns"].as_u64().expect("maximumCloseTurns") {
            if post.close_step(grant).expect("partial post copy retires") == app_store::SnapshotRetirementStep::Complete {
                assert!(post.terminal_is_empty());
                return;
            }
        }
        panic!("partial post copy did not retire under bounded grants");
    }

    #[test]
    fn comment_copy_builds_the_exact_inverse_and_post_snapshot() {
        let fixture = fixture();
        let mut source = large_snapshot(32);
        source.comment = "é".into();
        source.comment_utf8 = false;
        let replacement = format!("{}🎒", "updated".repeat(fixture["replacementRepeats"].as_u64().expect("replacementRepeats") as usize));
        let mutation = ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: replacement.clone(), comment_utf8: true });
        let page_bytes = fixture["pageBytes"].as_u64().expect("pageBytes") as usize;
        let mut inverse = ZipInverseCopy::default();
        while !inverse.is_complete() {
            assert!(inverse.advance(&source, &mutation, page_bytes).expect("comment inverse copy advances") <= page_bytes);
        }
        assert_eq!(inverse.take(), Some(ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: source.comment.clone(), comment_utf8: source.comment_utf8 })));
        inverse.begin_close();
        assert!(inverse.terminal_is_empty());

        let mut post = ZipPostCopy::default();
        while !post.is_complete() {
            assert!(post.advance(&source, &mutation, page_bytes).expect("comment post copy advances") <= page_bytes);
        }
        let copied = post.take().expect("comment post copy owns a snapshot");
        assert_eq!(copied.entries, source.entries);
        assert_eq!(copied.comment, replacement);
        assert!(copied.comment_utf8);
        let bytes = crate::standards::v2_0::subsets::base::io::encode_zip(&copied).expect("prepared comment snapshot saves");
        assert_eq!(crate::standards::v2_0::subsets::base::io::decode_zip(&bytes).expect("prepared comment snapshot reopens"), copied);
        let outcome = <ZipMutation as protocol::Mutation<ZipSnapshot>>::diff(&mutation, &source);
        assert_eq!(copied, protocol::apply_diff(outcome.diff(), &source).expect("independent mutation algebra applies the comment"));
        post.begin_close();
        assert!(post.terminal_is_empty());
    }

    #[test]
    fn zip_retirement_batches_payload_bytes_under_the_exact_grant() {
        let fixture = fixture();
        let bytes = fixture["largeEntryBytes"].as_u64().expect("largeEntryBytes") as usize;
        let page_bytes = fixture["pageBytes"].as_u64().expect("pageBytes") as usize;
        let mut retirement = semio_framework_value::retirement::owned_retirement(large_snapshot(bytes));
        let mut turns = 0;
        loop {
            match retirement.close_step(1, page_bytes).expect("ZIP retirement advances") {
                app_store::SnapshotRetirementStep::Pending { released_bytes, .. } => assert!(released_bytes <= page_bytes),
                app_store::SnapshotRetirementStep::Blocked => panic!("owned ZIP retirement must not block"),
                app_store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    break;
                }
            }
            turns += 1;
            assert!(turns < fixture["maximumCloseTurns"].as_u64().expect("maximumCloseTurns") as usize);
        }
        assert!(turns > bytes / page_bytes);
    }
}

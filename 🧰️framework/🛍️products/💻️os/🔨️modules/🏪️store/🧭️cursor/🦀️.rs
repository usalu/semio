//! 🧭 Cursor roots and original retirement independent of Store execution.
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, artifact_retire_struct};
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, sequence};
use std::{mem::ManuallyDrop, sync::Arc};
#[cfg(any(test, feature = "testkit"))]
use serde::{Deserialize, Serialize};

/// 🎯️ Undo/redo/checkout position — derived by folding the event log, never persisted.
/// Carries the FULL applied-edit list (not just the tail edit id): an edit undone mid-history
/// precedes later-applied edits in file order, and the redo stack can contain edits in any order
/// relative to `applied_edit_ids` — a single marker id cannot represent that. `checkpoint_id`
/// mirrors `ArtifactStore::current_checkpoint_id`.
/// 🌱️ `serde` is TEST-ONLY (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01):
/// `BackboneDocument` in `💻️os/🖥️host` — the only production consumer that forced `ArtifactCursor`/
/// `ArtifactCursorOwners` to stay dual — moved to a hand-written `ToValue`/`FromValue` impl (see
/// `📓️os-kernel-serde-final.md`). Production already routes through `ToValue`/`FromValue`; this
/// wave's own test fixtures (`serde_json::from_value` oracle comparisons below) still need the
/// derive, hence `cfg_attr(test, …)` rather than deleting it outright.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ArtifactCursorOwners {
    #[value(default, skip_serializing_if = "semio_framework_os_vcs::history_id_stack_is_empty")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "semio_framework_os_vcs::history_id_stack_is_empty"))]
    pub applied_edit_ids: semio_framework_os_vcs::HistoryPageStack<String>,
    #[value(default, skip_serializing_if = "semio_framework_os_vcs::history_id_stack_is_empty")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "semio_framework_os_vcs::history_id_stack_is_empty"))]
    pub redo_edit_ids: semio_framework_os_vcs::HistoryPageStack<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub checkpoint_id: Option<String>,
}

/// 🧭 Read-visible cursor root; prepared group owners stay private until their shared decision.
pub struct ArtifactCursor {
    owners: std::mem::ManuallyDrop<ArtifactCursorOwners>,
    group: std::mem::ManuallyDrop<Option<ArtifactCursorGroupRoot>>,
}

struct ArtifactCursorGroupRoot {
    visibility: Arc<semio_framework_os_vcs::ArtifactGroupVisibility>,
    owners: ArtifactCursorOwners,
}

impl ArtifactCursor {
    pub fn new(applied_edit_ids: impl Into<semio_framework_os_vcs::HistoryPageStack<String>>, redo_edit_ids: impl Into<semio_framework_os_vcs::HistoryPageStack<String>>, checkpoint_id: Option<String>) -> Self {
        Self::from_owners(ArtifactCursorOwners { applied_edit_ids: applied_edit_ids.into(), redo_edit_ids: redo_edit_ids.into(), checkpoint_id })
    }

    pub fn from_owners(owners: ArtifactCursorOwners) -> Self {
        Self { owners: std::mem::ManuallyDrop::new(owners), group: std::mem::ManuallyDrop::new(None) }
    }

    pub fn stage_group_owned(&mut self, owners: ArtifactCursorOwners, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>) -> Result<(), ArtifactCursorOwners> {
        if self.group.is_some() || !visibility.pending() {
            return Err(owners);
        }
        *self.group = Some(ArtifactCursorGroupRoot { visibility: Arc::clone(visibility), owners });
        Ok(())
    }

    pub fn adopt_group_owned(&mut self, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>) -> Result<ArtifactCursorOwners, ()> {
        if !visibility.committed() || !self.group.as_ref().is_some_and(|group| Arc::ptr_eq(&group.visibility, visibility)) {
            return Err(());
        }
        let group = self.group.take().expect("validated committed cursor root remains owned");
        Ok(std::mem::replace(&mut *self.owners, group.owners))
    }

    pub fn abort_group_owned(&mut self, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>) -> Result<ArtifactCursorOwners, ()> {
        if visibility.pending() || visibility.committed() || !self.group.as_ref().is_some_and(|group| Arc::ptr_eq(&group.visibility, visibility)) {
            return Err(());
        }
        Ok(self.group.take().expect("validated aborted cursor root remains owned").owners)
    }

    pub fn group_visibility(&self) -> Option<&semio_framework_os_vcs::ArtifactGroupVisibility> {
        self.group.as_ref().map(|group| group.visibility.as_ref())
    }

    pub fn read_group(&self, decision: Option<&semio_framework_os_vcs::ArtifactGroupReadDecision<'_>>) -> Result<&ArtifactCursorOwners, ()> {
        match self.group.as_ref() {
            Some(group) if decision.ok_or(())?.committed_for(&group.visibility)? => Ok(&group.owners),
            _ => Ok(&*self.owners),
        }
    }
}

impl Default for ArtifactCursor {
    fn default() -> Self {
        Self::from_owners(ArtifactCursorOwners::default())
    }
}

impl std::ops::Deref for ArtifactCursor {
    type Target = ArtifactCursorOwners;

    fn deref(&self) -> &Self::Target {
        self.group.as_ref().filter(|group| group.visibility.committed()).map_or(&*self.owners, |group| &group.owners)
    }
}

impl std::ops::DerefMut for ArtifactCursor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(self.group.is_none(), "mutable cursor access requires its staged group to be adopted or aborted");
        &mut self.owners
    }
}

impl Clone for ArtifactCursor {
    fn clone(&self) -> Self {
        Self::from_owners((**self).clone())
    }
}

impl std::fmt::Debug for ArtifactCursor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(formatter)
    }
}

impl PartialEq for ArtifactCursor {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl ToValue for ArtifactCursor {
    fn to_value(&self) -> DslValue {
        (**self).to_value()
    }
}

impl FromValue for ArtifactCursor {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        Ok(Self::from_owners(ArtifactCursorOwners::from_value(value)?))
    }
}

/// 🌉️ serde twin of the `ToValue`/`FromValue` pair above — TEST-ONLY
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01): `BackboneDocument` in
/// `💻️os/🖥️host`, the only production consumer that forced this to stay unconditional, moved to a
/// hand-written `ToValue`/`FromValue` impl (see `📓️os-kernel-serde-final.md`). Delegates through
/// the same `ArtifactCursorOwners` view (itself now also `cfg_attr(test, …)`-gated) so both
/// encodings emit an identical shape. Hand-written rather than derived because the private fields
/// are `ManuallyDrop` (the staged group root must be adopted or retired before `Drop`), which no
/// derive can see through.
#[cfg(any(test, feature = "testkit"))]
impl Serialize for ArtifactCursor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Serialize::serialize(&**self, serializer)
    }
}

/// 🌉️ Mirror of the `Serialize` twin directly above — also test-only.
#[cfg(any(test, feature = "testkit"))]
impl<'de> Deserialize<'de> for ArtifactCursor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ArtifactCursorOwners::deserialize(deserializer).map(Self::from_owners)
    }
}

impl Drop for ArtifactCursor {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.group.is_none()), "artifact cursor reached Drop before its exact staged root was adopted or returned for retirement");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.owners) };
    }
}

artifact_retire_struct!(ArtifactCursorOwners { applied_edit_ids, redo_edit_ids, checkpoint_id });
artifact_retire_struct!(ArtifactCursorGroupRoot { visibility, owners });
impl RetireOwned for ArtifactCursor {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let mut original = ManuallyDrop::new(self);
        let owners = unsafe { ManuallyDrop::take(&mut original.owners) };
        let group = unsafe { ManuallyDrop::take(&mut original.group) };
        sequence(vec![semio_framework_value::retirement::deferred(owners), semio_framework_value::retirement::deferred(group)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&*self.owners), semio_framework_value::retirement::deferred_birth_bytes_for(&*self.group)]) }
    fn controlled_retirement_supported() -> bool { true }
}

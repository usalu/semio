//! 🗄️ Generic document version-graph algebra — Author/Change/Checkpoint/Alternative/DocumentVcs,
//! `VcsError`, content-addressed checkpoint ids, and the raw collection-diff/operation helpers. Pure
//! data plus pure functions: nothing here touches a live document (that's `store::DocumentStore`,
//! which depends on this crate — see `26/07/28/EXTRACT-STORE-INTO-ITS-OWN-TECHNOLOGY`).

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use thiserror::Error;

// This crate's own body spells the trait name bare (`crate::Operation<P>` in `apply_operation`
// below, disambiguating the trait from the same-named generic parameter) — a private (non-`pub`)
// import keeps that ergonomics without re-exposing `protocol::Operation` on `vcs`'s own public API
// (dependents import `protocol::Operation` directly). `OperationDiff` is imported for its `apply`
// method, called on `Operation::Diff` inside `apply_operation`.
use protocol::{Edit, Operation, OperationDiff};

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// @emoji 🆔️ Allocates stable ids for document VCS entities.
pub fn create_document_vcs_id(prefix: &str) -> String {
    let n = ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{n}")
}

//#region 🔖️Schemas
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

// 🎞️ `OperationMeta` lives in `protocol_command`; `Edit<Operation>` (imported above) is this
// crate's own field type for `DocumentVcs.edits` below.

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub id: String,
    pub edit_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub saved_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checkpoint {
    pub id: String,
    pub change_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub timestamp: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentVcs<P, Operation> {
    pub initial_projection: P,
    pub edits: Vec<Edit<Operation>>,
    pub changes: Vec<Change>,
    pub checkpoints: Vec<Checkpoint>,
    pub alternatives: Vec<Alternative>,
}
//#endregion 🔖️Schemas
//#region 🔖️Errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum VcsError {
    #[error("unknown edit id: {0}")]
    UnknownEdit(String),
    #[error("unknown change id: {0}")]
    UnknownChange(String),
    #[error("unknown alternative id: {0}")]
    UnknownAlternative(String),
    #[error("no checkpoint for alternative")]
    NoCheckpoint,
    #[error("empty apply command")]
    EmptyApply,
    #[error("nothing to undo")]
    NothingToUndo,
    #[error("cannot undo edit authored by another actor: {0}")]
    ForeignEdit(String),
    #[error("nothing to redo")]
    NothingToRedo,
    #[error("serialize error: {0}")]
    Serialize(String),
    #[error("deserialize error: {0}")]
    Deserialize(String),
    #[error("backbone error: {0}")]
    Backbone(String),
}

dsl_core::fault_from_thiserror!(VcsError, dsl_core::FaultOrigin::Module, "module.vcs");

//#endregion 🔖️Errors
//#region 🔖️CollectionDiff
/// @emoji 🧩️ Sparse collection patch entry (mirrors semio_compose_rs `XModified`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPatch<TId, TPatch> {
    pub id: TId,
    pub patch: TPatch,
}

/// @emoji 🧩️ Sparse collection diff (mirrors semio_compose_rs `XCollectionDiff`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDiff<TId, TPatch, TAdded> {
    pub removed: Vec<TId>,
    pub modified: Vec<ItemPatch<TId, TPatch>>,
    pub added: Vec<TAdded>,
}

impl<TId, TPatch, TAdded> Default for CollectionDiff<TId, TPatch, TAdded> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}
//#endregion 🔖️CollectionDiff

//#region 🔖️CollectionOperation
/// @emoji 🏷️ Identifies an item within a `Vec` by a stable id, for generic collection operations.
pub trait Identified<TId> {
    fn id(&self) -> &TId;
}

/// @emoji 🩹️ Applies a patch in place and returns the patch that undoes it (captured from prior state).
pub trait Patchable<TPatch> {
    fn apply_patch(&mut self, patch: &TPatch) -> TPatch;
}

/// @emoji 🧺️ Generic ordered-collection operation (add/remove/move/patch) with mechanical pre-state inverses.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CollectionOperation<TId, TItem, TPatch> {
    Add { index: usize, item: TItem },
    Remove { id: TId },
    Move { id: TId, to_index: usize },
    Patch { id: TId, patch: TPatch },
}

/// @emoji ▶️ Applies a `CollectionOperation` to a `Vec` in place.
pub fn apply_collection_operation<TId, TItem, TPatch>(items: &mut Vec<TItem>, operation: &CollectionOperation<TId, TItem, TPatch>)
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match operation {
        CollectionOperation::Add { index, item } => {
            let at = (*index).min(items.len());
            items.insert(at, item.clone());
        }
        CollectionOperation::Remove { id } => {
            items.retain(|item| item.id() != id);
        }
        CollectionOperation::Move { id, to_index } => {
            if let Some(from) = items.iter().position(|item| item.id() == id) {
                let item = items.remove(from);
                let at = (*to_index).min(items.len());
                items.insert(at, item);
            }
        }
        CollectionOperation::Patch { id, patch } => {
            if let Some(item) = items.iter_mut().find(|item| item.id() == id) {
                item.apply_patch(patch);
            }
        }
    }
}

/// @emoji ↩️ Computes the inverse `CollectionOperation` from the pre-state `items`. Panics if `operation` targets
/// an id absent from `items` (Remove/Move/Patch always target an existing item by construction).
pub fn invert_collection_operation<TId, TItem, TPatch>(items: &[TItem], operation: &CollectionOperation<TId, TItem, TPatch>) -> CollectionOperation<TId, TItem, TPatch>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match operation {
        CollectionOperation::Add { item, .. } => CollectionOperation::Remove { id: item.id().clone() },
        CollectionOperation::Remove { id } => {
            let index = items.iter().position(|item| item.id() == id).expect("remove target must exist in pre-state");
            CollectionOperation::Add { index, item: items[index].clone() }
        }
        CollectionOperation::Move { id, .. } => {
            let index = items.iter().position(|item| item.id() == id).expect("move target must exist in pre-state");
            CollectionOperation::Move { id: id.clone(), to_index: index }
        }
        CollectionOperation::Patch { id, patch } => {
            let mut prior = items.iter().find(|item| item.id() == id).cloned().expect("patch target must exist in pre-state");
            let inverse_patch = prior.apply_patch(patch);
            CollectionOperation::Patch { id: id.clone(), patch: inverse_patch }
        }
    }
}

/// @emoji 🧮️ Projects a `CollectionOperation` onto a sparse {@link CollectionDiff}, so a plugin's
/// `Operation::diff` can produce a diff in one call instead of hand-writing `removed`/`modified`/
/// `added`. `Add` → `added`, `Remove` → `removed`, `Patch` → `modified`. `CollectionDiff` has no
/// positional-move channel, so `Move` is encoded as `removed` + `added` (delete then re-add by
/// identity); a plugin that keeps items keyed by id reconstructs order from item identity.
pub fn collection_diff_from_operation<TId, TItem, TPatch>(items: &[TItem], operation: &CollectionOperation<TId, TItem, TPatch>) -> CollectionDiff<TId, TPatch, TItem>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone,
    TPatch: Clone,
{
    let mut diff = CollectionDiff::default();
    match operation {
        CollectionOperation::Add { item, .. } => diff.added.push(item.clone()),
        CollectionOperation::Remove { id } => diff.removed.push(id.clone()),
        CollectionOperation::Patch { id, patch } => diff.modified.push(ItemPatch { id: id.clone(), patch: patch.clone() }),
        CollectionOperation::Move { id, .. } => {
            if let Some(item) = items.iter().find(|item| item.id() == id) {
                diff.removed.push(id.clone());
                diff.added.push(item.clone());
            }
        }
    }
    diff
}
//#endregion 🔖️CollectionOperation
//#region 🔖️Operation
// 🎞️ `Operation`/`OperationDiff` live in `protocol_command`; this region just replays a projection
// through an operation's forward diff — the pure per-step transform every store-level replay uses.

pub fn apply_operation<P, Operation>(projection: &P, operation: &Operation) -> P
where
    Operation: crate::Operation<P>,
{
    operation.diff(projection).apply(projection)
}

//#endregion 🔖️Operation
//#region 🔖️MergeStrategy
// 🎞️ `merge_concurrent_diffs` (real per-`MergeStrategyKind` dispatch) lives in `protocol_crdt`. The
// checkpoint-ancestor/merge-base helpers that used to live in this region moved to `store` along
// with `DocumentEnvelope` (`checkpoint_ancestors`/`merge_base`/`reconcile_alternative` all take an
// envelope) — only the envelope-free id-minting primitive stays here.

/// @emoji 🔒️ Content-addressed checkpoint id: `ck-<hex16(blake3(parent_id || ordered_change_content_
/// hashes || message || authors || timestamp))>`, replacing the old fully-random counter-string
/// scheme (`create_document_vcs_id("checkpoint")`) — two peers that independently commit the
/// identical checkpoint content (same parent, same changes in the same order, same message/authors/
/// timestamp) now converge on the identical id instead of minting two different ones. `changes` must
/// already contain every entry `change_ids` references (including one freshly created by this same
/// commit, if any) — callers push a new `Change` before calling this.
pub fn content_addressed_checkpoint_id(parent_id: Option<&str>, change_ids: &[String], changes: &[Change], message: Option<&str>, authors: &[Author], timestamp: &str) -> String {
    let mut input = Vec::new();
    input.extend_from_slice(parent_id.unwrap_or("").as_bytes());
    input.push(0);
    for change_id in change_ids {
        let change_hash = changes.iter().find(|change| change.id == *change_id).map(|change| *blake3::hash(&serde_json::to_vec(change).unwrap_or_default()).as_bytes()).unwrap_or([0u8; 32]);
        input.extend_from_slice(&change_hash);
    }
    input.push(0);
    input.extend_from_slice(message.unwrap_or("").as_bytes());
    input.push(0);
    for author in authors {
        input.extend_from_slice(author.id.as_bytes());
        input.push(0);
    }
    input.push(0);
    input.extend_from_slice(timestamp.as_bytes());
    let digest = *blake3::hash(&input).as_bytes();
    let hex16: String = digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("ck-{hex16}")
}
//#endregion 🔖️MergeStrategy

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct DemoItem {
        id: String,
        value: i32,
    }

    impl Identified<String> for DemoItem {
        fn id(&self) -> &String {
            &self.id
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoItemPatch {
        value: Option<i32>,
    }

    impl Patchable<DemoItemPatch> for DemoItem {
        fn apply_patch(&mut self, patch: &DemoItemPatch) -> DemoItemPatch {
            let inverse = DemoItemPatch { value: Some(self.value) };
            if let Some(value) = patch.value {
                self.value = value;
            }
            inverse
        }
    }

    #[test]
    fn collection_diff_from_op_projects_each_variant() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }];
        let added = collection_diff_from_operation::<String, DemoItem, DemoItemPatch>(&items, &CollectionOperation::Add { index: 0, item: DemoItem { id: "c".into(), value: 3 } });
        assert_eq!(added.added.len(), 1);
        assert!(added.removed.is_empty() && added.modified.is_empty());

        let removed = collection_diff_from_operation::<String, DemoItem, DemoItemPatch>(&items, &CollectionOperation::Remove { id: "a".into() });
        assert_eq!(removed.removed, vec!["a".to_string()]);

        let patched = collection_diff_from_operation(&items, &CollectionOperation::Patch { id: "b".into(), patch: DemoItemPatch { value: Some(9) } });
        assert_eq!(patched.modified.len(), 1);
        assert_eq!(patched.modified[0].id, "b");

        let moved = collection_diff_from_operation::<String, DemoItem, DemoItemPatch>(&items, &CollectionOperation::Move { id: "a".into(), to_index: 1 });
        assert_eq!(moved.removed, vec!["a".to_string()], "move is encoded as remove + re-add by identity");
        assert_eq!(moved.added.len(), 1);
        assert_eq!(moved.added[0].id, "a");
    }

    #[test]
    fn collection_op_add_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
        let operation = CollectionOperation::Add { index: 1, item: DemoItem { id: "b".into(), value: 2 } };
        let mut applied = items.clone();
        apply_collection_operation(&mut applied, &operation);
        assert_eq!(applied.len(), 2);
        assert_eq!(applied[1].id, "b");
        let inverse = invert_collection_operation(&items, &operation);
        apply_collection_operation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_move_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }, DemoItem { id: "c".into(), value: 3 }];
        let operation = CollectionOperation::Move { id: "a".into(), to_index: 2 };
        let mut applied = items.clone();
        apply_collection_operation(&mut applied, &operation);
        assert_eq!(applied.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c", "a"]);
        let inverse = invert_collection_operation(&items, &operation);
        apply_collection_operation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_patch_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
        let operation = CollectionOperation::Patch { id: "a".into(), patch: DemoItemPatch { value: Some(9) } };
        let mut applied = items.clone();
        apply_collection_operation(&mut applied, &operation);
        assert_eq!(applied[0].value, 9);
        let inverse = invert_collection_operation(&items, &operation);
        apply_collection_operation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_remove_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }];
        let operation = CollectionOperation::Remove { id: "a".into() };
        let mut applied = items.clone();
        apply_collection_operation(&mut applied, &operation);
        assert_eq!(applied.len(), 1);
        let inverse = invert_collection_operation(&items, &operation);
        apply_collection_operation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    //#endregion 🔖️ReconcileAlternative

    //#region 🔖️ContentAddressedCheckpointAndMergeBase
    #[test]
    fn content_addressed_checkpoint_id_is_deterministic_and_content_sensitive() {
        let root_change = Change { id: "change-root".into(), edit_ids: vec!["edit-1".into()], description: Some("root".into()), saved_at: "2026-07-27T00:00:00Z".into() };
        let changes = vec![root_change];
        let change_ids = vec!["change-root".to_string()];
        let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];

        let id_a = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z");
        let id_b = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z");
        assert_eq!(id_a, id_b, "identical inputs converge on the identical id");
        assert!(id_a.starts_with("ck-"), "got {id_a}");

        let id_different_message = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("other message"), &authors, "2026-07-27T00:00:01Z");
        assert_ne!(id_a, id_different_message, "a different message must change the id");

        let id_different_parent = content_addressed_checkpoint_id(Some("ck-parent"), &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z");
        assert_ne!(id_a, id_different_parent, "a different parent must change the id");

        let id_different_timestamp = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:02Z");
        assert_ne!(id_a, id_different_timestamp, "a different timestamp must change the id");
    }
}
//#endregion 🧪️Tests

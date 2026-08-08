//! 🗄️ Generic document version-graph algebra — Author/Change/Checkpoint/Alternative/DocumentVcs,
//! `VcsError`, content-addressed checkpoint ids, and the raw collection-diff/operation helpers. Pure
//! data plus pure functions: nothing here touches a live document (that's `store::DocumentStore`,
//! which depends on this crate — see `26/07/28/EXTRACT-STORE-INTO-ITS-OWN-TECHNOLOGY`).

use serde::{Deserialize, Serialize};
use thiserror::Error;

// This crate's own body spells the trait name bare (`self::Mutation<P>` in `apply_mutation`
// below, disambiguating the trait from the same-named generic parameter) — a private (non-`pub`)
// import keeps that ergonomics without re-exposing `crate::os_spr::Mutation` on `vcs`'s own public API
// (dependents import `crate::os_spr::Mutation` directly). `MutationDiff` is imported for its `apply`
// method, called on `Mutation::Diff` inside `apply_mutation`.
use crate::os_spr::{Edit, Mutation, MutationDiff};

//#region 🆔️Ids
/// @emoji 🔑 Content-addressed entity id: `{prefix}-{hex16(blake3(prefix || 0 || payload))}`.
pub fn content_addressed_entity_id(prefix: &str, payload: &[u8]) -> String {
    let mut input = prefix.as_bytes().to_vec();
    input.push(0);
    input.extend_from_slice(payload);
    let digest = *blake3::hash(&input).as_bytes();
    let hex16: String = digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("{prefix}-{hex16}")
}

/// @emoji 🆔️ Deterministic child id scoped to an edit: blake3(`{edit_id}:{ordinal}`).
pub fn edit_scoped_id(edit_id: &str, ordinal: u32) -> String {
    let digest = blake3::hash(format!("{edit_id}:{ordinal}").as_bytes());
    let hex16: String = digest.as_bytes()[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("scoped-{hex16}")
}

/// @emoji ✏️ Content-addressed edit id from actor + sequence + forwards fingerprint (no global counter).
pub fn mint_edit_id(actor: Option<&str>, sequence: i32, forwards_fingerprint: &[u8]) -> String {
    let mut payload = Vec::new();
    payload.extend_from_slice(actor.unwrap_or("").as_bytes());
    payload.push(0);
    payload.extend_from_slice(&sequence.to_le_bytes());
    payload.push(0);
    payload.extend_from_slice(forwards_fingerprint);
    content_addressed_entity_id("edit", &payload)
}

/// @emoji 📦️ Content-addressed change id from ordered edit ids (+ optional description distinguisher).
pub fn mint_change_id(edit_ids: &[String], description: Option<&str>) -> String {
    let mut payload = edit_ids.join("\0").into_bytes();
    payload.push(0);
    payload.extend_from_slice(description.unwrap_or("").as_bytes());
    content_addressed_entity_id("change", &payload)
}

/// @emoji 🌿️ Content-addressed alternative id from name + ordered checkpoint ids.
pub fn mint_alternative_id(name: &str, checkpoint_ids: &[String]) -> String {
    let mut payload = name.as_bytes().to_vec();
    payload.push(0);
    payload.extend_from_slice(checkpoint_ids.join("\0").as_bytes());
    content_addressed_entity_id("alternative", &payload)
}

/// @emoji ⚙️ Content-addressed operation id from the operation's binary (or other) fingerprint bytes.
pub fn mint_mutation_id(mutation_bytes: &[u8]) -> String {
    content_addressed_entity_id("mutation", mutation_bytes)
}

/// @emoji 🆔️ Legacy-compatible prefix-only mint — identical inputs collide.
/// Prefer [`mint_edit_id`] / [`mint_change_id`] / [`mint_alternative_id`] / [`mint_mutation_id`] /
/// [`content_addressed_entity_id`] with a distinguishing payload.
pub fn create_document_vcs_id(prefix: &str) -> String {
    content_addressed_entity_id(prefix, prefix.as_bytes())
}
//#endregion 🆔️Ids

//#region 🔖️Schemas
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

// 🎞️ `MutationMeta` lives in `protocol_command`; `Edit<Mutation>` (imported above) is this
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
pub struct DocumentVcs<P, Mutation> {
    pub initial_projection: P,
    pub edits: Vec<Edit<Mutation>>,
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

crate::fault_from_thiserror!(VcsError, crate::os_dsl::FaultOrigin::Module, "module.vcs");

//#endregion 🔖️Errors
//#region 🔖️CollectionDiff
/// @emoji 🧩️ Sparse collection patch entry (mirrors semio_compose_rs `XModified`).
///
/// 🎞️ Field-identical to `crate::os_spr::command::ItemPatch`, but kept local because the surrounding
/// VCS `CollectionMutation` schema still diverges from spr (`index` vs `at`) — see that enum's note.
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

//#region 🔖️CollectionMutation
/// @emoji 🏷️ Identifies an item within a `Vec` by a stable id, for generic collection operations.
pub trait Identified<TId> {
    fn id(&self) -> &TId;
}

/// @emoji 🩹️ Applies a patch in place and returns the patch that undoes it (captured from prior state).
pub trait Patchable<TPatch>: Sized {
    fn apply_patch(&mut self, patch: &TPatch);
    fn diff_patch(&self, other: &Self) -> Option<TPatch>;
}

/// @emoji 🧺️ Generic ordered-collection operation (add/remove/move/patch) with mechanical pre-state inverses.
///
/// 🎞️ `crate::os_spr::command::CollectionMutation` is the frozen-contract twin (`Add { id, item, at }`,
/// `Move { id, to }`). This VCS shape keeps `index`/`to_index` for `apply_collection_mutation` below —
/// schemas differ, so these are NOT `pub use` aliases of spr.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CollectionMutation<TId, TItem, TPatch> {
    Add { index: usize, item: TItem },
    Remove { id: TId },
    Move { id: TId, to_index: usize },
    Patch { id: TId, patch: TPatch },
}

/// @emoji ▶️ Applies a `CollectionMutation` to a `Vec` in place.
pub fn apply_collection_mutation<TId, TItem, TPatch>(items: &mut Vec<TItem>, operation: &CollectionMutation<TId, TItem, TPatch>)
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match operation {
        CollectionMutation::Add { index, item } => {
            let at = (*index).min(items.len());
            items.insert(at, item.clone());
        }
        CollectionMutation::Remove { id } => {
            items.retain(|item| item.id() != id);
        }
        CollectionMutation::Move { id, to_index } => {
            if let Some(from) = items.iter().position(|item| item.id() == id) {
                let item = items.remove(from);
                let at = (*to_index).min(items.len());
                items.insert(at, item);
            }
        }
        CollectionMutation::Patch { id, patch } => {
            if let Some(item) = items.iter_mut().find(|item| item.id() == id) {
                item.apply_patch(patch);
            }
        }
    }
}

/// @emoji ↩️ Computes the inverse `CollectionMutation` from the pre-state `items`. Panics if `operation` targets
/// an id absent from `items` (Remove/Move/Patch always target an existing item by construction).
pub fn inverse_collection_mutation<TId, TItem, TPatch>(items: &[TItem], operation: &CollectionMutation<TId, TItem, TPatch>) -> CollectionMutation<TId, TItem, TPatch>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match operation {
        CollectionMutation::Add { item, .. } => CollectionMutation::Remove { id: item.id().clone() },
        CollectionMutation::Remove { id } => {
            let index = items.iter().position(|item| item.id() == id).expect("remove target must exist in pre-state");
            CollectionMutation::Add { index, item: items[index].clone() }
        }
        CollectionMutation::Move { id, .. } => {
            let index = items.iter().position(|item| item.id() == id).expect("move target must exist in pre-state");
            CollectionMutation::Move { id: id.clone(), to_index: index }
        }
        CollectionMutation::Patch { id, patch } => {
            let prior = items.iter().find(|item| item.id() == id).cloned().expect("patch target must exist in pre-state");
            let mut after = prior.clone();
            after.apply_patch(patch);
            let inverse_patch = after.diff_patch(&prior).expect("a patch that changed state must yield a computable inverse");
            CollectionMutation::Patch { id: id.clone(), patch: inverse_patch }
        }
    }
}

/// @emoji 🧮️ Projects a `CollectionMutation` onto a sparse {@link CollectionDiff}, so a plugin's
/// `Mutation::diff` can produce a diff in one call instead of hand-writing `removed`/`modified`/
/// `added`. `Add` → `added`, `Remove` → `removed`, `Patch` → `modified`. `CollectionDiff` has no
/// positional-move channel, so `Move` is encoded as `removed` + `added` (delete then re-add by
/// identity); a plugin that keeps items keyed by id reconstructs order from item identity.
pub fn collection_diff_from_mutation<TId, TItem, TPatch>(items: &[TItem], operation: &CollectionMutation<TId, TItem, TPatch>) -> CollectionDiff<TId, TPatch, TItem>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone,
    TPatch: Clone,
{
    let mut diff = CollectionDiff::default();
    match operation {
        CollectionMutation::Add { item, .. } => diff.added.push(item.clone()),
        CollectionMutation::Remove { id } => diff.removed.push(id.clone()),
        CollectionMutation::Patch { id, patch } => diff.modified.push(ItemPatch { id: id.clone(), patch: patch.clone() }),
        CollectionMutation::Move { id, .. } => {
            if let Some(item) = items.iter().find(|item| item.id() == id) {
                diff.removed.push(id.clone());
                diff.added.push(item.clone());
            }
        }
    }
    diff
}
//#endregion 🔖️CollectionMutation
//#region 🔖️Mutation
// 🎞️ `Mutation`/`MutationDiff` live in `protocol_command`; this region just replays a projection
// through an operation's forward diff — the pure per-step transform every store-level replay uses.

pub fn apply_mutation<P, Mutation>(projection: &P, operation: &Mutation) -> P
where
    Mutation: self::Mutation<P>,
{
    operation.diff(projection).apply(projection)
}

//#endregion 🔖️Mutation
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
        let added = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Add { index: 0, item: DemoItem { id: "c".into(), value: 3 } });
        assert_eq!(added.added.len(), 1);
        assert!(added.removed.is_empty() && added.modified.is_empty());

        let removed = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Remove { id: "a".into() });
        assert_eq!(removed.removed, vec!["a".to_string()]);

        let patched = collection_diff_from_mutation(&items, &CollectionMutation::Patch { id: "b".into(), patch: DemoItemPatch { value: Some(9) } });
        assert_eq!(patched.modified.len(), 1);
        assert_eq!(patched.modified[0].id, "b");

        let moved = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Move { id: "a".into(), to_index: 1 });
        assert_eq!(moved.removed, vec!["a".to_string()], "move is encoded as remove + re-add by identity");
        assert_eq!(moved.added.len(), 1);
        assert_eq!(moved.added[0].id, "a");
    }

    #[test]
    fn collection_op_add_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
        let operation = CollectionMutation::Add { index: 1, item: DemoItem { id: "b".into(), value: 2 } };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation);
        assert_eq!(applied.len(), 2);
        assert_eq!(applied[1].id, "b");
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_move_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }, DemoItem { id: "c".into(), value: 3 }];
        let operation = CollectionMutation::Move { id: "a".into(), to_index: 2 };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation);
        assert_eq!(applied.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c", "a"]);
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_patch_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
        let operation = CollectionMutation::Patch { id: "a".into(), patch: DemoItemPatch { value: Some(9) } };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation);
        assert_eq!(applied[0].value, 9);
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_remove_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }];
        let operation = CollectionMutation::Remove { id: "a".into() };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation);
        assert_eq!(applied.len(), 1);
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse);
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

    //#region 🆔️Ids
    #[test]
    fn content_addressed_entity_and_mint_helpers_are_deterministic() {
        assert_eq!(content_addressed_entity_id("x", b"payload"), content_addressed_entity_id("x", b"payload"));
        assert_ne!(content_addressed_entity_id("x", b"a"), content_addressed_entity_id("x", b"b"));
        assert_eq!(edit_scoped_id("edit-1", 0), edit_scoped_id("edit-1", 0));
        assert_ne!(edit_scoped_id("edit-1", 0), edit_scoped_id("edit-1", 1));
        assert!(edit_scoped_id("edit-1", 0).starts_with("scoped-"));
        assert_eq!(mint_edit_id(Some("alice"), 3, b"fwd"), mint_edit_id(Some("alice"), 3, b"fwd"));
        assert_ne!(mint_edit_id(Some("alice"), 3, b"fwd"), mint_edit_id(Some("bob"), 3, b"fwd"));
        assert_eq!(mint_change_id(&["e1".into(), "e2".into()], Some("msg")), mint_change_id(&["e1".into(), "e2".into()], Some("msg")));
        assert_eq!(mint_alternative_id("main", &["ck1".into()]), mint_alternative_id("main", &["ck1".into()]));
        assert_eq!(mint_mutation_id(b"op-bytes"), mint_mutation_id(b"op-bytes"));
        assert_eq!(create_document_vcs_id("draft"), create_document_vcs_id("draft"));
        assert!(create_document_vcs_id("draft").starts_with("draft-"));
    }
    //#endregion 🆔️Ids
}
//#endregion 🧪️Tests

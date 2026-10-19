//! 🧮️ Diff and mutation: the sparse collection diff, the identified and patchable item traits and the mutation replay step.
//!
//! Holds the `🔖️CollectionDiff`, `🔖️Identified`, `🔖️Mutation` regions of the module; the root re-exports its items.

use super::*;

//#region 🔖️CollectionDiff
/// 🧩️ Sparse collection patch entry (mirrors semio_compose_rs `XModified`).
///
/// 🎞️ Canonical collection patch entry for sparse collection diffs (re-exported by `crate::os_spr`).
#[derive(Clone, Debug, Default, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ItemPatch<TId, TPatch> {
    pub id: TId,
    pub patch: TPatch,
}

/// 🧩️ Sparse collection diff (mirrors semio_compose_rs `XCollectionDiff`).
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
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

//#region 🔖️Identified
/// 🏷️ Identifies an item within a `Vec` by a stable id, for generic collection operations.
pub trait Identified<TId> {
    fn id(&self) -> &TId;
}

/// 🩹️ Applies a patch in place.
pub trait Patchable<TPatch>: Sized {
    fn apply_patch(&mut self, patch: &TPatch);
}
//#endregion 🔖️Identified
//#region 🔖️Mutation
// 🎞️ `Mutation`/`MutationDiff`/`MutationMessage` live in `protocol_command`; this region just
// replays a snapshot through an operation's forward diff — the pure per-step transform every
// store-level replay uses.

/// ▶️ Computes `operation.diff(snapshot)`, applies the resulting diff, and returns the new
/// snapshot alongside every [`protocol::MutationMessage`] the outcome carried. Diff-apply
/// rejection is returned as its structured [`MutationApplyError`] before a snapshot is produced. A `Fatal`
/// message's diff is `D::default()` by construction (§C2 LAW 1), so applying it is always a no-op —
/// callers that must not silently apply a rejected op check `worst_level(&messages)` against their
/// `MergePolicy` themselves (this fn stays policy-agnostic, matching its old unconditional-apply
/// shape).
pub fn apply_mutation<P, Mutation>(snapshot: &P, operation: &Mutation) -> Result<(P, Vec<protocol::MutationMessage>), MutationApplyError>
where
    P: Clone,
    Mutation: self::Mutation<P> + ColdMutation<P>,
{
    let (diff, messages) = operation.diff(snapshot).into_parts();
    let applied = protocol::apply_diff_cold(&diff, snapshot);
    MutationDiff::retire_cold(diff);
    Ok((applied?, messages))
}

//#endregion 🔖️Mutation

//! 🪪️ Checkpoint records: the author, change, composition pin, checkpoint and alternative records of the version graph.
//!
//! Holds items of the module; the root re-exports them.

use super::*;

pub use protocol::Author;

// 🎞️ `MutationMeta` lives in `protocol_command`; `Edit<Mutation>` (imported above) is this
// crate's own field type for `ArtifactVcs.edits` below.

// 🎞️ `Change`'s two historical reasons to stay dual-derived are BOTH resolved now: the
// checkpoint-id-hash reason (`📓️float-format-parity.md` — `content_addressed_checkpoint_id_core`
// hashes `crate::os_pack::json::to_json_string(change)`, proven byte-identical to `serde_json` for
// every `f64`, and `Change` carries no float field regardless) and the
// `ArtifactRepositoryHistoryEntryAuthority<T>: DeserializeOwned` reason (`📓️history-decoder-serde.md`
// — that authority's bound is now `FromValue`, which `Change` already derives). `serde` stays
// `cfg_attr(test, …)` rather than dropping outright, though: this file's own
// `change_to_json_string_matches_serde_json_byte_for_byte` and
// `content_addressed_checkpoint_id_composition_pins_are_deterministic_and_backward_compatible`
// tests both compare `Change` against a real `serde_json` oracle, matching this ticket's own
// established oracle-preservation pattern (`crate::os_store::store_schema::ArtifactCursorOwners`,
// `📓️os-kernel-serde-final.md`). `ArtifactHistoryLedger`/`ArtifactHistoryIter`/`ArtifactVcs`/
// `ArtifactVcsRead` stay fully serde-free (dropped outright, prior wave);
// `Author`/`CompositionPin`/`Checkpoint`/`Alternative` now join them (see `Author`'s own docstring
// above). `crate::os_store::store_schema::ArtifactEnvelopeRead`'s hand-off reason was already stale before any of this
// (converted off `Serialize` onto `ToValue` outright, see `📓️store-serde-final.md`).
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Change {
    pub id: String,
    pub edit_ids: Vec<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,
    pub saved_at: String,
}

/// 🧩️ One owned child's checkpoint pin, captured on the parent's checkpoint so checking out
/// the parent can restore the whole composition. `child_ref` is the pinned child artifact's real
/// `semio_framework_artifact_reference::ArtifactRef` — **correction, `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/
/// b2-store-composition-report.md`**: the prior wave (`b1-spr-vcs-report.md`) believed `ArtifactRef`
/// (defined in `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs`) was unreachable from this crate and fell
/// back to the wire URI `String`. That was wrong: `io/🦀️.rs` is dual-mounted — the
/// `semio-framework` crate mounts it as `io`, and THIS crate (`semio-framework-os-kernel`) mounts the
/// very same source file as `io` (see `💻️os/📦️packages/🦀️rust/🦀️.rs:237-238`,
/// `pub mod io;`) — no cross-crate dependency-direction problem exists; `store` already reaches
/// `semio_framework_artifact_reference::ArtifactDialect` directly (`🏪️store/🦀️.rs:88/105/662`). Sorting for
/// [`content_addressed_checkpoint_id`] below is therefore by `child_ref.to_uri()` (the same
/// deterministic string this field used to store literally), not by any `Ord` on `ArtifactRef`
/// itself (which does not implement one).
// 🎞️ `Serialize`/`Deserialize` dropped outright — same reason as `Author` above:
// `Checkpoint.composition_pins: Vec<CompositionPin>` no longer needs it, and no test anywhere
// serializes `CompositionPin` directly either.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CompositionPin {
    pub child_ref: semio_framework_artifact_reference::ArtifactRef,
    pub checkpoint_id: String,
}

// 🎞️ `Serialize`/`Deserialize` dropped outright (see `Author`'s docstring above for the full
// reasoning): `🏪️store/🦀️.rs`'s `ArtifactRepositoryHistoryEntryAuthority<T>` — mounted at field
// ids 4/5 of the fresh-VCS decoder, feeding this type directly — converted its bound from
// `DeserializeOwned` to `FromValue`, which `Checkpoint` already derives. See
// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
// 🔍️research/📓️history-decoder-serde.md`.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Checkpoint {
    pub id: String,
    pub change_ids: Vec<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub authors: Vec<Author>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub timestamp: String,
    /// 🧩️ Which checkpoint each owned child was at when this checkpoint was committed —
    /// empty for a non-composite artifact (every checkpoint before this ticket, and every leaf
    /// artifact after it). Additive; see [`CompositionPin`].
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub composition_pins: Vec<CompositionPin>,
}

// 🎞️ `Serialize`/`Deserialize` dropped outright — same `ArtifactRepositoryHistoryEntryAuthority<T>`
// reason as `Checkpoint` above (see its docstring).
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Alternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

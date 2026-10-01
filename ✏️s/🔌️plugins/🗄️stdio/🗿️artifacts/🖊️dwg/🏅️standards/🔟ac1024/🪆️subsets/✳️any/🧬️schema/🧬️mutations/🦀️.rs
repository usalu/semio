//! 🧬️ Logical DWG document mutations.

use crate::schema::diff::{self, DwgDiff};
use crate::DwgSnapshot;
use protocol::Mutation;

//#region 🔖️Mutations
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷️set-version-info/🦀️.rs"]
pub mod set_version_info;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none, and `no` is not an
/// approved semantic verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = DwgSnapshot, diff = DwgDiff, schema = "DwgMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum DwgMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetVersionInfo(set_version_info::SetVersionInfo),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_dwg_mutation(snapshot: &mut DwgSnapshot, mutation: &DwgMutation) -> protocol::MutationOutcome<DwgDiff> {
    let outcome = mutation.diff(snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Mutations

//#region 🔖️Codecs
crate::impl_serde_op_codec!(DwgMutation, "dwg-mutation", protocol = include_str!("💾️binary/📡️.protocol.semio"));
//#endregion 🔖️Codecs

//#region 🔖️Kinds
impl DwgMutation {
    /// 🏷️ Kebab-case kind spelling — the exact vocabulary BOTH DWG catalogs declare
    /// (`../../🔣️oracle.json` and `../../../../4️⃣ac1018/🪆️subsets/✳️any/🔮️oracles/
    /// 🔣️.json`), and the row ids of both cases' Scenario Outlines. Hand-matched rather
    /// than derived, so [`KINDS`] is checked against something with its own reason to be right; and
    /// exhaustive, so a variant added to the enum is a COMPILE error here rather than a silently
    /// uncatalogued kind.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn kind(&self) -> &'static str {
        match self {
            DwgMutation::SetSnapshot(_) => "set-snapshot",
            DwgMutation::SetVersionInfo(_) => "set-version-info",
        }
    }
}

/// 🏷️ Every declared kind, kebab-case, in the enum's own declaration order. ⚠️ It mirrors TWO
/// catalogs, not one: `4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` is a `pub use`
/// of this module, so AC1018 declares this same vocabulary and both manifests must list it.
pub const KINDS: &[&str] = &["set-snapshot", "set-version-info"];
//#endregion 🔖️Kinds

//#region 🔖️MutationTrait
/// 🧮️ Snapshot assignment validates the owned version sentinel; version edits retain their declared writer conformance.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn agg_diff(this: &DwgMutation, base: &DwgSnapshot) -> protocol::MutationOutcome<DwgDiff> {
    let next = match this {
        DwgMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => std::borrow::Cow::Borrowed(snapshot.as_ref()),
        DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version, maintenance_version, codepage }) => std::borrow::Cow::Owned(diff::version_info_next(base, version, *maintenance_version, *codepage)),
    };
    let refusal=crate::schema::snapshot::unwritable_version(&next).filter(|(code,_)|!matches!(this,DwgMutation::SetSnapshot(_))||*code=="version-sentinel");
    match refusal {
        Some((_, message)) => protocol::MutationOutcome::fatal("mutation.invariant", message, Vec::<String>::new()),
        None => protocol::MutationOutcome::new(diff::diff_set_snapshot(base, &next)),
    }
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &DwgMutation, base: &DwgSnapshot) -> Vec<DwgMutation> {
    match this {
        DwgMutation::SetSnapshot(_) => vec![DwgMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(base.clone()) })],
        DwgMutation::SetVersionInfo(_) => vec![DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version: base.version.clone(), maintenance_version: base.maintenance_version, codepage: base.codepage })],
    }
}
//#endregion 🔖️MutationTrait

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<DwgMutation> {
    let base = crate::standards::v_ac1024::engine::demo_dwg_snapshot();
    vec![DwgMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(base) }), DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version: "AC1024".into(), maintenance_version: 9, codepage: 65001 })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/✏️retitles-the-summary-and-records-the-last-editor/🦀️.rs"]
mod set_snapshot_retitles_the_summary_and_records_the_last_editor;
//#endregion 🧪️FixtureCases

//! 🧬️ Logical DWG document mutations.

use crate::artifacts::dwg::schema::diff::{self, DwgDiff};
use crate::artifacts::dwg::DwgSnapshot;
use protocol::Mutation;

//#region 🔖️Mutations
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷set-version-info/🦀️.rs"]
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
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#region 🚪️Reachability
/// ▶️ [`apply_dwg_mutation`] in a signature that names only this subset's own public types, so an
/// external crate can drive the real production apply path and still SEE a rejection instead of
/// discarding it. `protocol` is a private `extern crate` alias in this plugin's glue: nothing
/// outside the crate can name `protocol::MutationOutcome` or `protocol::Mutation`, so without these
/// two wrappers a test host could only re-derive the semantics by hand and would then be testing its
/// own re-derivation. Same wall, same fix as the 🧿️semio ✳️kit subset's.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_dwg_mutation_checked(snapshot: &mut DwgSnapshot, mutation: &DwgMutation) -> Result<(), String> {
    let outcome = apply_dwg_mutation(snapshot, mutation);
    match outcome.messages().first() {
        None => Ok(()),
        Some(message) => Err(format!("{:?} was rejected: [{}] {}", mutation, message.code.0, message.message)),
    }
}

/// ↩️ `Mutation::inverse` for `DwgMutation`, reachable without naming the `protocol` alias — the
/// production inverse itself, never a copy of its rules.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_dwg_mutation(base: &DwgSnapshot, mutation: &DwgMutation) -> Vec<DwgMutation> {
    <DwgMutation as Mutation<DwgSnapshot>>::inverse(mutation, base)
}
//#endregion 🚪️Reachability
//#endregion 🔖️Mutations

//#region 🔖️Codecs
crate::impl_serde_op_codec!(DwgMutation, "dwg-mutation");
//#endregion 🔖️Codecs

//#region 🔖️Kinds
impl DwgMutation {
    /// 🏷️ Kebab-case kind spelling — the exact vocabulary BOTH DWG catalogs declare
    /// (`../../🔣️oracle.json` and `../../../../🔖️ac1018/🪆️subsets/✳️any/🧪️oracle/
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
/// catalogs, not one: `🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` is a `pub use`
/// of this module, so AC1018 declares this same vocabulary and both manifests must list it.
pub const KINDS: &[&str] = &["set-snapshot", "set-version-info"];
//#endregion 🔖️Kinds

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &DwgMutation, base: &DwgSnapshot) -> protocol::MutationOutcome<DwgDiff> {
    protocol::MutationOutcome::new(match this {
        DwgMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff::diff_set_snapshot(base, snapshot),
        DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version, maintenance_version, codepage }) => diff::diff_set_version_info(base, version, *maintenance_version, *codepage),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &DwgMutation, base: &DwgSnapshot) -> Vec<DwgMutation> {
    match this {
        DwgMutation::SetSnapshot(_) => vec![DwgMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        DwgMutation::SetVersionInfo(_) => vec![DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version: base.version.clone(), maintenance_version: base.maintenance_version, codepage: base.codepage })],
    }
}
//#endregion 🔖️MutationTrait

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<DwgMutation> {
    let base = crate::artifacts::dwg::standards::v_ac1024::engine::demo_dwg_snapshot();
    vec![DwgMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base }), DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version: "AC1024".into(), maintenance_version: 9, codepage: 65001 })]
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;
    use protocol::OpBinary;
    use protocol::OpText;

    /// 🧪️ Keeps the declaration honest, which nothing else can: the framework never parses Rust, so
    /// the CATALOGS are what the contract gate counts against, and this is the only check that ties
    /// them to the enum. BOTH are read, because AC1018 re-exports this vocabulary rather than
    /// declaring one — a kind added here and catalogued under only one standard fails here.
    #[test]
    fn kinds_matches_every_variant_and_both_catalogs() {
        let from_variants: std::collections::BTreeSet<&str> = demo_mutation_cases().iter().map(DwgMutation::kind).collect();
        let from_kinds: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
        assert_eq!(from_variants, from_kinds, "KINDS must equal every DwgMutation variant's kind()");
        assert_eq!(KINDS.len(), 2, "KINDS must list exactly the declared 2 kinds");
        for manifest in [include_str!("../../🔣️oracle.json"), include_str!("../../../../../🔖️ac1018/🪆️subsets/✳️any/🔣️oracle.json")] {
            for kind in KINDS {
                assert!(manifest.contains(&format!("\"{kind}\"")), "a committed DWG catalog is missing kind {kind:?}");
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn logical_mutations_obey_diff_and_inverse_laws() {
        let base = DwgSnapshot { version: "AC1024".into(), maintenance_version: 2, codepage: 30, ..Default::default() };
        for mutation in demo_mutation_cases() {
            let mut applied = base.clone();
            let diff = apply_dwg_mutation(&mut applied, &mutation);
            assert_eq!(diff.diff().apply(&base).expect("diff must apply to base"), applied);
            for inverse in mutation.inverse(&base) {
                apply_dwg_mutation(&mut applied, &inverse);
            }
            assert_eq!(applied, base);
        }
        assert!(DwgDiff::between(&base, &base).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn operation_codecs_retain_logical_mutations() {
        for mutation in demo_mutation_cases() {
            assert_eq!(DwgMutation::parse_op(&mutation.print_op()).expect("text mutation"), mutation);
            assert_eq!(DwgMutation::decode_op(&mutation.encode_op().expect("binary mutation")).expect("binary mutation"), mutation);
        }
    }
}

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/retitles-the-summary-and-records-the-last-editor/🦀️.rs"]
mod set_snapshot_retitles_the_summary_and_records_the_last_editor;
//#endregion 🧪️FixtureCases

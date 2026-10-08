//! 🧬️ Logical DWG document mutations.

use crate::schema::diff::{self, DwgDiff};
use crate::DwgSnapshot;
use protocol::Mutation;

//#region 🔖️Mutations
//#region 🔖️Leaves
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
    SetVersionInfo(set_version_info::SetVersionInfo),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_dwg_mutation(snapshot: &mut DwgSnapshot, mutation: &DwgMutation) -> protocol::MutationOutcome<DwgDiff> {
    let outcome = mutation.diff(snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Mutations

//#region 🔖️Codecs

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
            DwgMutation::SetVersionInfo(_) => "set-version-info",
        }
    }
}

/// 🏷️ Every declared kind, kebab-case, in the enum's own declaration order. ⚠️ It mirrors TWO
/// catalogs, not one: `4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` is a `pub use`
/// of this module, so AC1018 declares this same vocabulary and both manifests must list it.
pub const KINDS: &[&str] = &["set-version-info"];
//#endregion 🔖️Kinds

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to exactly `next`: the preamble triple if any of its three fields moved. Every other field lives in the proprietary
/// container, which no leaf can address, so a `next` that differs there answers nothing and the edit is refused rather than approximated.
pub fn net_mutations(base: &DwgSnapshot, next: &DwgSnapshot) -> Vec<DwgMutation> {
    let triple_moved = base.version != next.version || base.maintenance_version != next.maintenance_version || base.codepage != next.codepage;
    let addressable = DwgSnapshot { version: next.version.clone(), maintenance_version: next.maintenance_version, codepage: next.codepage, ..base.clone() };
    match (&addressable == next, triple_moved) {
        (true, true) => vec![DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version: next.version.clone(), maintenance_version: next.maintenance_version, codepage: next.codepage })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Net

//#endregion 🔖️MutationTrait

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<DwgMutation> {
    vec![DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version: "AC1024".into(), maintenance_version: 9, codepage: 65001 })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🧪️FixtureCases
//#endregion 🧪️FixtureCases

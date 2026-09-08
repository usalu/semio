//! ⚙️ S Home mutation codec bridge, catalog identity, and behavior tests. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! direct leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<SHomeSnapshot>`
//! and `impl protocol::SemanticMutation<SHomeSnapshot>` from that payload — no hand-written
//! apply/diff/inverse dispatch here. Whole-document replace (the old `SetSnapshot`) is banned; it
//! goes through `ArtifactStore::reset` (non-history), never through this enum.

use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::mutations::{change_catalog_generation, register_s_home_mutation_descriptors};
use crate::SHomeSnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `SHomeMutation` variant, in declaration order — the vocabulary the `s-home-1-any` mutation catalog
/// (`../../🔣️oracle.json`) declares and the `🌵️mutate-s-home-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &["change-catalog-generation"];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `SHomeMutation` nor
/// `SHomeSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔣️oracle.json — the catalog and the recorded no-oracle decision.
pub fn s_home_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<SHomeSnapshot, String> {
        let decoded: SHomeSnapshot = pack::from_json_str(text).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: SHomeMutation = pack::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = pack::json!({
        "base": pack::json_from_dsl_value(&dsl::ToValue::to_value(&base)),
        "expectedSnapshot": pack::json_from_dsl_value(&dsl::ToValue::to_value(&expected)),
        "snapshot": pack::json_from_dsl_value(&dsl::ToValue::to_value(&applied)),
        "diff": pack::json_from_dsl_value(&dsl::ToValue::to_value(forward.diff())),
        "messages": pack::json_from_dsl_value(&dsl::ToValue::to_value(&forward.messages().to_vec())),
        "inverseSteps": pack::json_from_dsl_value(&dsl::ToValue::to_value(&inverse)),
        "inverseSnapshot": pack::json_from_dsl_value(&dsl::ToValue::to_value(&undone)),
        "inverseMessages": pack::json_from_dsl_value(&dsl::ToValue::to_value(&inverse_messages)),
    });
    Ok(report.to_string())
}
//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance

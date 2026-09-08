//! ⚙️ S Space mutation bridge, laws, and behavior tests.

#[cfg(test)]
use crate::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::mutations::{create_artifact, delete_artifact, register_s_space_mutation_descriptors, rename_artifact, touch_artifact};
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `SSpaceMutation` nor
/// `SSpaceSnapshot` can be named there, and hand-transcribing either into a Rust literal
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
pub fn s_space_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<SSpaceSnapshot, String> {
        let decoded: SSpaceSnapshot = pack::from_json_str(text).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: SSpaceMutation = pack::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(step, &undone).apply_to(&mut undone);
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

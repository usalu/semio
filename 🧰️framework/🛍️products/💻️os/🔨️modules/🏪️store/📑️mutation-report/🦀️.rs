//! 📑️ Language-neutral mutation report over production dispatch, shared by every state lane's case adapter.

use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
use semio_os_spr::ColdMutation;
use semio_os_store_replay::apply_outcome;

/// 📑️ The report every language-neutral mutation case compares against its committed specification vector. The
/// base and expected-after snapshots and the mutation are decoded through the production JSON codec, the mutation is
/// applied through `protocol::apply_diff` (so an apply refusal surfaces as the fatal message production
/// dispatch records), and the mutation's own computed inverse steps are replayed in reverse order (as the engine does) onto the applied snapshot.
/// The forward half is `base`, `expectedSnapshot`, `snapshot`, `diff`, `messages`; the inverse half is
/// `inverseSteps`, `inverseSnapshot`, `inverseMessages`. `expectedSnapshot` is decoded through the same path as
/// `base`, so a caller compares like with like.
///
/// @see ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the laws case adapters assert over this report.
pub fn mutation_report_json<S, M>(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String>
where
    S: Clone + ToValue + FromValue,
        M: ColdMutation<S> + ToValue + FromValue,
{
    let decode_snapshot = |text: &str| -> Result<S, String> { semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: M = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let (applied, forward) = apply_outcome(&base, mutation.diff(&base));
    let inverse = mutation.inverse(&base).map_err(ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in inverse.iter().rev() {
        let (next, step_outcome) = apply_outcome(&undone, step.diff(&undone));
        undone = next;
        inverse_messages.extend(step_outcome.messages().iter().cloned());
    }
    let json = |value: DslValue| semio_framework_pack_json::from_dsl_value(&value);
    let report = semio_framework_pack_json::object([
        ("base".to_string(), json(base.to_value())),
        ("expectedSnapshot".to_string(), json(expected.to_value())),
        ("snapshot".to_string(), json(applied.to_value())),
        ("diff".to_string(), json(forward.diff().to_value())),
        ("messages".to_string(), json(forward.messages().to_vec().to_value())),
        ("inverseSteps".to_string(), json(inverse.to_value())),
        ("inverseSnapshot".to_string(), json(undone.to_value())),
        ("inverseMessages".to_string(), json(inverse_messages.to_value())),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}

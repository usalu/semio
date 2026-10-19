//! 📚️ Acceptance fixtures, cases and change buckets: the failure debit law, the fixture directories, records, operations and compositions, the acceptance cases, the example cases and the change buckets. See [`super`].

use super::*;

/// 🧵️ Original seed and annotation custody preserve all three causes until their same-context paid physical close.
#[cfg(test)]
#[test]
fn original_guest_failure_debit_history_acceptance_keeps_complete_seed_cause_and_paid_close(){
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
    let expected:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🧾️expected.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();let phase:RetainedCloneGrant=serde_json::from_value(policy["retirementLoan"].clone()).unwrap();let mut caller=artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let identity=cx.original_recipient_identity();
    let((failure,pointers),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let primary=ValueError::new(ValueRefusalKind::AllocationFailed,"original acceptance cause 源");let receipt=ValueError::new(ValueRefusalKind::InvariantViolated,"original acceptance receipt 拒");let context=ValueError::new(ValueRefusalKind::Canceled,"original acceptance context 断");let pointers=(primary.message.as_ptr(),receipt.message.as_ptr(),context.message.as_ptr());({let original=semio_framework_job::OriginalTurnFailure::from(semio_framework_job::OriginalTurnPrimary::Value(primary));semio_framework_job::OriginalTurnFailure{receipt_refusal:Some(receipt),context_refusal:Some(context),..original}},pointers)});let(mut born,mut released)=(heap.requested_bytes,heap.released_bytes);cx.consume_retained(RetainedCloneProgress{copied_items:3,retained_capacity_bytes:born,released_bytes:released,..Default::default()}).unwrap();
    let(error,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||AcceptanceSeedFault::Original(failure.into()).into_error().context("the seed does not land".into()).context("original reload annotation".into()));born+=heap.requested_bytes;released+=heap.released_bytes;cx.consume_retained(RetainedCloneProgress{copied_items:2,retained_capacity_bytes:heap.requested_bytes,released_bytes:heap.released_bytes,..Default::default()}).unwrap();let original=error.original.as_ref().unwrap();let semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Value(primary)=&original.primary else{panic!("original acceptance primary retains its variant")};assert_eq!((primary.message.as_ptr(),original.receipt_refusal.as_ref().unwrap().message.as_ptr(),original.context_refusal.as_ref().unwrap().message.as_ptr()),pointers);assert_eq!(serde_json::json!({"callerRetainsFailure":true})["callerRetainsFailure"],expected["callerRetainsFailure"]);
    let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_value::retirement::controlled::ControlledRetirement::new(error).unwrap_or_else(|_|panic!("complete acceptance error supports paid retirement")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    for turn in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if owner.terminal_is_empty(){break}let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(phase,|same|semio_framework_job::close_original_retained_step(same,owner.selected_demand(phase.maximum_copy_bytes)?,|grant|owner.step(grant))));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(cx.retained_progress(),before.checked_add(progress).unwrap());assert_eq!(cx.original_recipient_identity(),identity);born+=heap.requested_bytes;released+=heap.released_bytes;assert!(turn+1<artifact_app_laws::OriginalFixtureCaller::maximum_turns())}assert!(owner.terminal_is_empty());assert_eq!(born,released);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
}

/// 🔍️ Where two document texts first disagree (line number and both lines, clipped), or that only their values disagree.
pub(super) fn acceptance_text_difference(left: &str, right: &str) -> String {
    if left == right {
        return "the canonical value (the document texts agree)".into();
    }
    let clip = |line: Option<&str>| line.map(|line| line.chars().take(240).collect::<String>());
    let (mut lefts, mut rights) = (left.lines(), right.lines());
    let mut number = 1;
    loop {
        match (lefts.next(), rights.next()) {
            (Some(one), Some(other)) if one == other => number += 1,
            (one, other) => return format!("document text line {number}: {:?} vs {:?}", clip(one), clip(other)),
        }
    }
}

/// 🗂️ Every directory under `root` that commits a mutation wire (`🦠️mutation/🔣️.json`), in path order.
fn acceptance_fixture_directories(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    fn walk(directory: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(directory) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if !path.is_dir() || name.starts_with('.') || ["target", "dist", "node_modules", "🗑️generated"].contains(&name.as_str()) {
                continue;
            }
            if path.join("🦠️mutation").join("🔣️.json").is_file() {
                found.push(path.clone());
            }
            walk(&path, found);
        }
    }
    let mut directories = Vec::new();
    walk(root, &mut directories);
    directories.sort();
    directories
}

/// 📄️ The JSON document at `path` as a DSL value.
pub(super) fn acceptance_json(path: &std::path::Path) -> Option<DslValue> {
    let text = std::fs::read_to_string(path).ok()?;
    semio_framework_pack_json::parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().map(|json| semio_framework_pack_json::to_dsl_value(&json))
}

/// 📇️ Whether a committed wire is a `{mutation, before, after}` case record (which carries its own document).
fn acceptance_record(wire: &DslValue) -> bool {
    wire.get("before").is_some() && wire.get("after").is_some() && wire.get("mutation").and_then(DslValue::as_object).is_some()
}

/// 🦠️ The operation a committed wire names, when it decodes as `A::Mutation` and the generic editor edits it (an input schema
/// that shows at least one input row — design §22.20: an editor with zero rows never opens — and no foreign-step capability).
fn acceptance_operation<A: ArtifactApp>(wire: &DslValue) -> Option<A::Mutation> {
    let value = if acceptance_record(wire) { wire.get("mutation").cloned().unwrap_or(DslValue::Null) } else { wire.clone() };
    let op = <A::Mutation as semio_framework_value::FromValue>::from_value(value).ok()?;
    if ::protocol::Mutation::<A::Snapshot>::input_schema(&op).is_none_or(|schema| !time_travel::time_travel_schema_shows_inputs(schema)) || ::protocol::Mutation::<A::Snapshot>::may_emit_foreign_steps(&op) {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(op);
        return None;
    }
    Some(op)
}

/// 🪆️ Whether a document value names a composed child by its handle (`{childId, target}`): the child's owned content lives
/// outside the value, so a document decoded from the value alone is not materialized and cannot seed an edit.
fn acceptance_composed(value: &DslValue) -> bool {
    match value {
        DslValue::Object(entries) => (entries.len() == 2 && value.get("childId").is_some() && value.get("target").is_some()) || entries.iter().any(|(_, entry)| acceptance_composed(entry)),
        DslValue::Array(items) => items.iter().any(acceptance_composed),
        _ => false,
    }
}

/// 🗣️ The DSL text of a self-contained document given as a JSON value (none for a value that names a composed child — see
/// [`acceptance_composed`]); the snapshot is never dropped, so a fail-closed root never trips.
fn acceptance_dsl_of<A: ArtifactApp>(document: DslValue) -> Option<String> {
    if acceptance_composed(&document) {
        return None;
    }
    let snapshot = <A::Snapshot as semio_framework_value::FromValue>::from_value(document).ok()?;
    let text = pack::snapshot::ArtifactDsl::print_dsl(&snapshot);
    std::mem::forget(snapshot);
    Some(text)
}

/// 🗄️ Every committed fixture case under `root` whose outcome is no refusal (applied, or a no-op an edit can turn into a change),
/// whose document is given (`📸️snapshot/⬅️before/🗣️.dsl.semio` verbatim — the only form a composed parent's owned children
/// survive in — else its JSON twin decoded as `A::Snapshot`) and whose operation the generic editor edits — in path order. A
/// `{mutation, before, after}` case record carries its own document.
pub fn acceptance_cases<A: ArtifactApp>(root: &std::path::Path) -> Vec<AcceptanceCase<A::Mutation>> {
    let mut cases = Vec::new();
    for directory in acceptance_fixture_directories(root) {
        let Some(wire) = acceptance_json(&directory.join("🦠️mutation").join("🔣️.json")) else { continue };
        let record = acceptance_record(&wire);
        let applied = record || acceptance_json(&directory.join("🎯️outcome").join("🔣️.json")).is_some_and(|outcome| outcome.get("status").and_then(DslValue::as_str).is_some_and(|status| status != "rejected"));
        if !applied {
            continue;
        }
        let before = directory.join("📸️snapshot").join("⬅️before");
        let base = match std::fs::read_to_string(before.join("🗣️.dsl.semio")) {
            Ok(text) => Some(text),
            Err(_) => (if record { wire.get("before").cloned() } else { acceptance_json(&before.join("🔣️.json")) }).and_then(acceptance_dsl_of::<A>),
        };
        let Some(base) = base else { continue };
        let Some(op) = acceptance_operation::<A>(&wire) else { continue };
        cases.push(AcceptanceCase { directory: directory.to_string_lossy().into_owned(), base_key: base.clone(), base, op });
    }
    cases
}

/// 📚️ The derived cases the law falls back to when no committed case exercises the flow: every committed operation under `root`
/// (whatever its own outcome — a refusal on its fixture document may apply elsewhere) on every document the app itself ships —
/// its initial document and each of `examples` (a DSL body verbatim, a JSON body decoded as `A::Snapshot`). Cases on one
/// document share its key, so they can follow each other downstream.
pub async fn acceptance_example_cases<A: ArtifactApp>(root: &std::path::Path, examples: &[ExampleSource]) -> Vec<AcceptanceCase<A::Mutation>> {
    let initial = A::initial_snapshot().await;
    let mut documents = vec![("initial".to_string(), pack::snapshot::ArtifactDsl::print_dsl(&initial))];
    std::mem::forget(initial);
    for example in examples {
        let body = example.document();
        let text = if body.trim_start().starts_with('{') {
            semio_framework_pack_json::parse(&body, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|json| acceptance_dsl_of::<A>(semio_framework_pack_json::to_dsl_value(&json)))
        } else {
            Some(body)
        };
        if let Some(text) = text.filter(|text| documents.iter().all(|(_, known)| known != text)) {
            documents.push((format!("example {}", example.id()), text));
        }
    }
    let operations: Vec<(String, A::Mutation)> =
        acceptance_fixture_directories(root).into_iter().filter_map(|directory| Some((directory.to_string_lossy().into_owned(), acceptance_operation::<A>(&acceptance_json(&directory.join("🦠️mutation").join("🔣️.json"))?)?))).collect();
    let mut cases = Vec::new();
    for (name, base) in &documents {
        for (directory, op) in &operations {
            cases.push(AcceptanceCase { directory: format!("{directory} on the {name} document"), base: base.clone(), base_key: name.clone(), op: op.clone() });
        }
    }
    for (_, op) in operations {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(op);
    }
    cases
}

/// 🎛️ The schema-valid changes the law tries for one draft, most robust first ([`acceptance_change_buckets`] flattened): each a
/// JSON pointer and the value to draft there.
pub fn acceptance_changes(inputs: &[ActionArgDef], value: &DslValue) -> Vec<(String, DslValue)> {
    acceptance_change_buckets(inputs, value).into_iter().flatten().take(ACCEPTANCE_CHANGES_PER_LEAF).collect()
}

/// 🪣️ The schema-valid changes of one draft per input-control kind ([`ACCEPTANCE_CONTROL_KINDS`]): every number input moved by its
/// step (then to its bounds and their midpoint), every boolean flipped, every option switched, every vector's first axis moved,
/// every free text extended — each a JSON pointer and the value to draft there. Existing array slots expose their item controls;
/// reference and opaque inputs are never changed.
pub fn acceptance_change_buckets(inputs: &[ActionArgDef], value: &DslValue) -> [Vec<(String, DslValue)>; 5] {
    fn at<'a>(value: &'a DslValue, pointer: &str) -> Option<&'a DslValue> {
        pointer.split('/').skip(1).try_fold(value, |current, segment| current.get(&segment.replace("~1", "/").replace("~0", "~")))
    }
    fn numbers(current: Option<f64>, min: Option<f64>, max: Option<f64>, step: Option<f64>, integer: bool) -> Vec<f64> {
        let step = step.filter(|step| *step > 0.0).map_or(1.0, |step| if integer { step.round().max(1.0) } else { step });
        let base = current.unwrap_or_else(|| min.unwrap_or(0.0));
        let mut values = vec![base + step, base - step];
        values.extend(max);
        values.extend(min);
        values.extend(min.zip(max).map(|(min, max)| (min + max) / 2.0));
        let mut kept: Vec<f64> = Vec::new();
        for value in values.into_iter().map(|value| if integer { value.round() } else { value }) {
            if value.is_finite() && Some(value) != current && min.is_none_or(|min| value >= min) && max.is_none_or(|max| value <= max) && !kept.contains(&value) {
                kept.push(value);
            }
        }
        kept
    }
    fn collect(inputs: &[ActionArgDef], value: &DslValue, prefix: &str, buckets: &mut [Vec<(String, DslValue)>; 5]) {
        for input in inputs {
            if input.presentation == Some(semio_framework::ArgPresentation::Hidden) { continue; }
            let pointer = format!("{prefix}{}", input.id);
            let current = at(value, &pointer);
            match &input.schema {
                ArgSchema::Number { min, max, step, integer, .. } => {
                    let integer = *integer;
                    buckets[0].extend(numbers(current.and_then(DslValue::as_f64), *min, *max, *step, integer).into_iter().map(|number| (pointer.clone(), if integer { DslValue::int(number as i64) } else { DslValue::float(number) })));
                }
                ArgSchema::Boolean => buckets[1].push((pointer.clone(), DslValue::Bool(!current.and_then(DslValue::as_bool).unwrap_or(false)))),
                ArgSchema::String { options, .. } if !options.is_empty() => {
                    let selected = current.and_then(DslValue::as_str);
                    buckets[2].extend(options.iter().filter(|option| Some(option.value.as_str()) != selected).map(|option| (pointer.clone(), DslValue::String(option.value.clone()))));
                }
                ArgSchema::Vector { min, max, step, .. } => {
                    if let Some(axes) = current.and_then(DslValue::as_array).filter(|axes| !axes.is_empty()) {
                        for first in numbers(axes[0].as_f64(), *min, *max, *step, false) {
                            let mut moved = axes.to_vec();
                            moved[0] = DslValue::float(first);
                            buckets[3].push((pointer.clone(), DslValue::Array(moved)));
                        }
                    }
                }
                ArgSchema::String { pattern: None, format: None, .. } => {
                    if let Some(text) = current.and_then(DslValue::as_str) {
                        buckets[4].push((pointer.clone(), DslValue::String(format!("{text} edited"))));
                    }
                }
                ArgSchema::Object { fields } => collect(fields, value, &pointer, buckets),
                ArgSchema::Array { items, .. } => {
                    if let Some(values) = current.and_then(DslValue::as_array) {
                        for index in 0..values.len().min(ACCEPTANCE_CHANGES_PER_LEAF) {
                            let item = ActionArgDef { id: format!("{}/{index}", input.id), schema: items.as_ref().clone(), required: true, nullable: false, default: None, ..input.clone() };
                            collect(std::slice::from_ref(&item), value, prefix, buckets);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    let mut buckets: [Vec<(String, DslValue)>; 5] = Default::default();
    collect(inputs, value, "", &mut buckets);
    buckets
}

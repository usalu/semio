//! ✏️ The supersede-ledger law, driven by the language-agnostic fixture `🧫️fixtures/🧫️supersede-ledger/🔣️.json` whose
//! cases the TypeScript twin (`🟦️.ts` beside this file) validates with Ajv and classifies with its own implementation:
//! every `Supersede` transition's role and history edit, the history edits in effect for the final alternative, which
//! rows read as applied, every history edit each author may undo with the restore it authors, and each author's redo.

use super::*;
use serde_json::{json, Value};

const SUPERSEDE_LEDGER_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/🧫️supersede-ledger/🔣️.json");

fn replacement(value: &Value) -> protocol::InputReplacement {
    match value.as_str() {
        Some(text) => protocol::InputReplacement::Input { schema: "fixture".into(), payload: text.as_bytes().to_vec() },
        None => protocol::InputReplacement::Withdrawn,
    }
}

fn authored(scope: &Option<String>, inputs: &[protocol::SupersededInput]) -> Value {
    let inputs: Vec<Value> = inputs
        .iter()
        .map(|input| {
            let text = match &input.replacement {
                protocol::InputReplacement::Input { payload, .. } => Value::String(String::from_utf8(payload.clone()).expect("fixture payloads are text")),
                protocol::InputReplacement::Withdrawn => Value::Null,
            };
            json!({ "target": input.target.0, "input": text })
        })
        .collect();
    json!({ "scope": scope, "inputs": inputs })
}

fn role_name(role: SupersedeRole) -> &'static str {
    match role {
        SupersedeRole::Edit => "edit",
        SupersedeRole::Undo => "undo",
        SupersedeRole::Redo => "redo",
    }
}

/// ⚖️ LAW: the ledger reaches every fixture case's roles, applied history edits and rows, undo restores and redos.
#[test]
fn every_fixture_case_classifies_its_supersede_transitions() {
    let fixture: Value = serde_json::from_str(SUPERSEDE_LEDGER_FIXTURE_JSON).expect("supersede ledger fixture parses");
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let records = case["transitions"]
            .as_array()
            .expect("transitions")
            .iter()
            .map(|transition| SupersedeRecord {
                transition_id: transition["id"].as_str().expect("id").to_string(),
                actor: transition["actor"].as_str().expect("actor").to_string(),
                timestamp: HybridLogicalTimestamp { actor: 0, physical_ms: transition["at"].as_u64().expect("at"), logical: 0 },
                scope: transition["scope"].as_str().map(str::to_string),
                inputs: transition["inputs"].as_array().expect("inputs").iter().map(|input| protocol::SupersededInput { target: MutationId(input["target"].as_str().expect("target").to_string()), replacement: replacement(&input["input"]) }).collect(),
                role: SupersedeRole::Edit,
                entry: 0,
            })
            .collect();
        let originals = case["originals"].as_object().expect("originals").iter().map(|(target, input)| (MutationId(target.clone()), replacement(input))).collect();
        let ledger = SupersedeLedger::from_records(records, HashMap::new(), originals);
        let name = |index: usize| ledger.records[index].transition_id.clone();
        let classified: Vec<Value> = ledger.records.iter().map(|record| json!({ "id": record.transition_id, "role": role_name(record.role), "entry": name(record.entry) })).collect();
        assert_eq!(Value::Array(classified), case["records"], "{id}: records");
        let alternative = case["finalAlternative"].as_str();
        let mut effective: BTreeMap<MutationId, protocol::EffectiveSupersession> = BTreeMap::new();
        for record in ledger.records.iter().filter(|record| record.scope.is_none() || record.scope.as_deref() == alternative) {
            for input in &record.inputs {
                effective.insert(
                    input.target.clone(),
                    protocol::EffectiveSupersession { transition_id: record.transition_id.clone(), actor: record.actor.clone(), timestamp: record.timestamp, scope: record.scope.clone(), replacement: input.replacement.clone() },
                );
            }
        }
        let applied = ledger.applied_entries(effective.iter());
        let mut applied_names: Vec<String> = applied.iter().map(|&index| name(index)).collect();
        applied_names.sort();
        assert_eq!(json!(applied_names), case["applied"], "{id}: applied history edits");
        let rows: serde_json::Map<String, Value> = (0..ledger.records.len()).map(|index| (name(index), Value::Bool(ledger.row_applied(index, &applied)))).collect();
        assert_eq!(Value::Object(rows), case["rowsApplied"], "{id}: applied rows");
        let undo: Vec<Value> = ledger
            .records
            .iter()
            .enumerate()
            .filter(|(index, record)| record.role == SupersedeRole::Edit && ledger.undoable(&record.actor, *index, &applied))
            .map(|(index, record)| {
                let (scope, inputs) = ledger.restore(index);
                json!({ "actor": record.actor, "entry": name(index), "restore": authored(&scope, &inputs) })
            })
            .collect();
        assert_eq!(Value::Array(undo), case["undo"], "{id}: undo");
        for (actor, expected) in case["redo"].as_object().expect("redo") {
            let redo = ledger.redo_top(actor).map_or(Value::Null, |(entry, undo)| {
                let (scope, inputs) = ledger.redo(entry, undo);
                json!({ "entry": name(entry), "undo": name(undo), "redo": authored(&scope, &inputs) })
            });
            assert_eq!(&redo, expected, "{id}: redo of {actor}");
        }
    }
}

use semio_framework_pack_json::{from_json_str, object, parse, to_string, JsonMemberPolicy, Value};
use semio_os_kernel as protocol;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::{default_remodeling_scene, durable_remodeling_asset, remodeling_asset, ImageAsset, RemodelingSnapshot};
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::{bridge_step, inverse_remodeling_mutation, RemodelingMutation};
use std::collections::{BTreeMap, BTreeSet};

fn field<'a>(value: &'a Value, key: &str) -> &'a Value {
    value.get(key).unwrap_or_else(|| panic!("missing fixture field {key}"))
}

fn string<'a>(value: &'a Value, key: &str) -> &'a str {
    field(value, key).as_str().unwrap()
}

fn replay(mut snapshot: RemodelingSnapshot, steps: impl IntoIterator<Item = RemodelingMutation>, context: &str) -> RemodelingSnapshot {
    for step in steps {
        let (next, diagnostics) = bridge_step(&snapshot, &step).unwrap();
        assert!(diagnostics.is_empty(), "{context} inverse refused: {diagnostics:?}");
        for (_, handle) in next.assets.iter() {
            assert!(next.durable_artifacts.get(&handle.child_id).is_some(), "{context} inverse left a dangling asset");
        }
        snapshot = next;
    }
    snapshot
}

/// 🔗️ Imported keys sharing one leaf preserve it until the last binding leaves and restore both lanes on undo.
#[test]
fn shared_asset_custody_matches_neutral_sequences_and_undo() {
    let fixture = parse(include_str!("../../../../../../../🧪️tests/🧫️fixtures/🧫️asset-custody/🔣️.json"), JsonMemberPolicy::Reject).unwrap();
    let wire_payloads = field(&fixture, "payloads");
    let payloads: BTreeMap<_, ImageAsset> = wire_payloads.as_object().unwrap().iter()
        .map(|(name, value)| (name, from_json_str(&to_string(value), JsonMemberPolicy::Reject).unwrap()))
        .collect();
    let ids = field(&fixture, "contentIds");
    for case in field(&fixture, "cases").as_array().unwrap() {
        let mut snapshot = default_remodeling_scene();
        if let Some(unbound) = case.get("unbound") {
            for name in unbound.as_array().unwrap().iter().map(|name| name.as_str().unwrap()) {
                snapshot.durable_artifacts.cold_insert(string(ids, name).into(), durable_remodeling_asset(&payloads[name]).unwrap());
            }
        }
        for (index, step) in field(case, "steps").as_array().unwrap().iter().enumerate() {
            let context = format!("{} step {}", string(case, "id"), index + 1);
            let mut wire = vec![("mutation".into(), Value::from(string(step, "op").to_owned())), ("key".into(), Value::from(string(step, "key").to_owned()))];
            if let Some(name) = step.get("asset").and_then(Value::as_str) {
                wire.push(("asset".into(), field(wire_payloads, name).clone()));
            }
            let mutation: RemodelingMutation = from_json_str(&to_string(&object(wire)), JsonMemberPolicy::Reject).unwrap();
            let (next, diagnostic) = bridge_step(&snapshot, &mutation).unwrap();
            let expected = step.get("diagnostic").map(|code| code.as_str().unwrap()).into_iter().collect::<Vec<_>>();
            assert_eq!(diagnostic, expected, "{context}");
            let bindings = next.assets.iter().map(|(key, handle)| (key.as_str(), handle.child_id.as_str())).collect::<BTreeMap<_, _>>();
            let expected_bindings = field(step, "bindings").as_object().unwrap().iter()
                .map(|(key, name)| (key, string(ids, name.as_str().unwrap())))
                .collect::<BTreeMap<_, _>>();
            assert_eq!(bindings, expected_bindings, "{context}");
            for (key, name) in field(step, "bindings").as_object().unwrap().iter() {
                assert_eq!(remodeling_asset(&next, key).as_ref(), Some(&payloads[name.as_str().unwrap()]), "{context} imported frame {key}");
            }
            let leaves = next.durable_artifacts.iter().map(|(id, _)| id.as_str()).collect::<BTreeSet<_>>();
            let expected_leaves = field(step, "leaves").as_array().unwrap().iter()
                .map(|name| string(ids, name.as_str().unwrap()))
                .collect::<BTreeSet<_>>();
            assert_eq!(leaves, expected_leaves, "{context}");
            for name in field(step, "leaves").as_array().unwrap().iter().map(|name| name.as_str().unwrap()) {
                assert_eq!(next.durable_artifacts.get(string(ids, name)), durable_remodeling_asset(&payloads[name]).as_ref(), "{context}");
            }
            let public = inverse_remodeling_mutation(&snapshot, &mutation).unwrap();
            let original = protocol::ColdMutation::inverse(&mutation, &snapshot).unwrap();
            assert_eq!(replay(next.clone(), public, &context), snapshot, "{context} public inverse");
            assert_eq!(replay(next.clone(), original.into_iter().rev(), &context), snapshot, "{context} store inverse");
            snapshot = next;
        }
    }
}

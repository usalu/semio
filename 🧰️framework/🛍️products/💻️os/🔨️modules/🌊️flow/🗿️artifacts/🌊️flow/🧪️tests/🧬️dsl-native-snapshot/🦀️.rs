use crate::os_pack::snapshot::ArtifactDsl;
use crate::FlowHostSnapshot;
use semio_framework_value::ToValue;

fn corpus() -> Vec<(String, serde_json::Value)> {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧬️dsl-native-snapshot/🔣️.json")).unwrap();
    assert_eq!(fixture["schema"], "semio.flow.dsl-native-snapshot-law/v1");
    fixture["cases"].as_array().unwrap().iter().map(|row| (row["id"].as_str().unwrap().to_string(), row["snapshot"].clone())).collect()
}

fn numeric(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Number(number) => serde_json::json!(number.as_f64().unwrap()),
        serde_json::Value::Array(items) => serde_json::Value::Array(items.iter().map(numeric).collect()),
        serde_json::Value::Object(map) => serde_json::Value::Object(map.iter().map(|(key, inner)| (key.clone(), numeric(inner))).collect()),
        other => other.clone(),
    }
}

fn parse(snapshot: &serde_json::Value) -> FlowHostSnapshot {
    semio_framework_pack_json::from_json_str(&snapshot.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture snapshot decodes through the first-party contract")
}

fn project(snapshot: &FlowHostSnapshot) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&snapshot.to_value()))).expect("first-party JSON is valid JSON for the third-party reader")
}

#[test]
fn every_fixture_snapshot_round_trips_through_the_canonical_json_projection() {
    for (id, snapshot) in corpus() {
        let model = parse(&snapshot);
        assert_eq!(numeric(&project(&model)), numeric(&snapshot), "{id}");
        let reparsed = parse(&project(&model));
        assert_eq!(reparsed, model, "{id}");
        reparsed.retire_cold();
        model.retire_cold();
    }
}

#[test]
fn every_fixture_snapshot_round_trips_through_the_flow_document_text() {
    for (id, snapshot) in corpus() {
        let model = parse(&snapshot);
        let text = model.print_dsl();
        let reread = <FlowHostSnapshot as ArtifactDsl>::parse_dsl(&text).unwrap_or_else(|error| panic!("{id}: {error:?}\n{text}"));
        assert_eq!(reread, model, "{id}");
        assert_eq!(reread.print_dsl(), text, "{id}: printing is a fixpoint");
        assert_eq!(numeric(&project(&reread)), numeric(&snapshot), "{id}");
        reread.retire_cold();
        model.retire_cold();
    }
}

#[test]
fn dsl_native_text_prints_synapses_as_table_rows_without_the_wire_literal() {
    for (id, snapshot) in corpus() {
        let model = parse(&snapshot);
        let text = model.print_dsl();
        assert!(!text.contains(" -> "), "{id}: wire-literal sugar survived\n{text}");
        for synapse in &model.synapses {
            assert!(text.contains(&synapse.id), "{id}: synapse row {} missing", synapse.id);
        }
        model.retire_cold();
    }
}

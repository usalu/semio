//! 🧪️ Selected-face cuts preserve geometry, downstream measurements, and document history.
use super::*;
use semio_s_artifact_procedural_generation3d::central_apply::{apply_generation3d_mutation};

fn fixtures() -> serde_json::Value { serde_json::from_str(include_str!("../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔪️knife-mesh-selection/🧫️fixtures/🔣️.json")).unwrap() }
fn payload() -> KnifeMeshSelection { KnifeMeshSelection { start: [0.0, -1.0, 0.0], end: [0.0, 1.0, 0.0] } }
fn ids(value: &serde_json::Value) -> Vec<String> { value.as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_owned()).collect() }

#[test]
fn knife_selection_parameters_match_shared_fixtures() {
    let fixture = fixtures();
    for case in fixture["valid"].as_array().unwrap() {
        let actual = parameters(&payload(), &ids(&case["ids"])).unwrap();
        assert_eq!(actual.face, case["face"].as_u64().unwrap() as u32);
        assert_eq!(actual.start, payload().start);
        assert_eq!(actual.end, payload().end);
    }
    for selection in fixture["invalidSelections"].as_array().unwrap() { assert!(parameters(&payload(), &ids(selection)).is_err()); }
    for case in fixture["invalidPayloads"].as_array().unwrap() {
        if let (Ok(start), Ok(end)) = (serde_json::from_value(case["start"].clone()), serde_json::from_value(case["end"].clone())) {
            assert!(parameters(&KnifeMeshSelection { start, end }, &ids(&fixture["valid"][0]["ids"])).is_err());
        }
    }
    for value in [f64::NAN, f64::INFINITY] {
        assert!(parameters(&KnifeMeshSelection { start: [value, 0.0, 0.0], ..payload() }, &ids(&fixture["valid"][0]["ids"])).is_err());
    }
}

#[test]
fn knife_selection_splices_a_typed_widget_and_preserves_analysis() {
    let _serial = crate::test_serial::lock();
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::{Generation3dMutation};

    let snapshot = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::example_snapshot(semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
    let (id, rows) = cut_rows(&payload(), &snapshot.host_snapshot, &["extrude@meshOut#0.face.0".into()]).unwrap();
    let channels: Vec<&str> = rows.iter().filter_map(|row| match row { Generation3dMutation::ChangeWidgetInput(input) if input.id == id => Some(input.channel.as_str()), _ => None }).collect();
    assert_eq!(channels, ["face", "start", "end"], "the cut's inputs are absolute leaves after the default-params splice");
    let mut landed = snapshot.clone();
    for row in rows {
        apply_generation3d_mutation(&mut landed, &row).unwrap();
        row.retire_cold();
    }
    let before_snapshot = snapshot.clone();
    let before: serde_json::Value = serde_json::from_str(&with_host(&before_snapshot.host_snapshot, |host| host.evaluate()).unwrap()).unwrap();
    before_snapshot.retire_cold();
    let result = with_host(&landed.host_snapshot, |host| {
        assert!(host.host_snapshot.synapses.iter().any(|wire| wire.from == id && wire.to == "analysis" && wire.to_port == "mesh"));
        assert!(host.host_snapshot.synapses.iter().any(|wire| wire.from == "extrude" && wire.to == id && wire.to_port == "mesh"));
        let after: serde_json::Value = serde_json::from_str(&host.evaluate().map_err(|error| error.to_string())?).unwrap();
        assert_eq!(after[&id]["out"]["meshOut"]["$schema"], "mesh");
        for field in ["area", "volume"] {
            let expected = before["analysis"]["out"][field]["value"].as_f64().unwrap();
            let actual = after["analysis"]["out"][field]["value"].as_f64().unwrap();
            assert!((actual / expected - 1.0).abs() < 1e-6, "{field}: {actual} != {expected}");
        }
        Ok::<_, String>(())
    });
    landed.retire_cold();
    snapshot.retire_cold();
    result.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn knife_selection_undo_redo_restores_the_graph_and_analysis_connection() {
    use crate::editor_domain::editor_laws::context;
    use semio_s_artifact_procedural_generation3d::editor::generation3d::{Generation3dCommand, commands::set_active_example};
    let _serial = crate::test_serial::lock();
    let mut app = context::app().await;
    context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_MESH_WORKBENCH.into() })).await;
    context::drain_flow_eval_ticks(&mut app).await;
    context::select_domain(&mut app, DOMAIN, "face", &["extrude@meshOut#0.face.0"]).await;
    let probe = |app: &context::Generation3dApp| {
        let snapshot = context::snapshot(app);
        (snapshot.host_snapshot.widgets.len(), snapshot.host_snapshot.synapses.iter().find(|wire| wire.to == "analysis" && wire.to_port == "mesh").unwrap().from.clone())
    };
    let before = probe(&app);
    let after = (before.0 + 1, "extrude__knifeCut".into());
    semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip(&mut app, Generation3dCommand::KnifeMeshSelection(payload()), probe, before, after).await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

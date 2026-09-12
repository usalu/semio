use crate::standards::v1::subsets::object::schema::{SemioObjectArtifact, diff::SemioObjectDiff, snapshot::SemioObjectSnapshot};
use protocol::{DiffCodec, MutationDiff};
use store::{ArtifactDsl, ArtifactPack};

#[semio_framework_async_macros::async_test]
async fn stdio_document_contract_object_round_trips_exact_children() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")).expect("neutral Object vectors");
    for case in fixture["snapshotCases"].as_array().expect("snapshot vectors") {
        let input = &case["input"];
        let text = input.to_string();
        let snapshot = dsl::json::from_json_str::<SemioObjectSnapshot>(&text);
        let artifact = dsl::json::from_json_str::<SemioObjectArtifact>(&text);
        let valid = case["valid"].as_bool().expect("expected admission");
        assert_eq!(snapshot.is_ok(), valid, "snapshot: {input}");
        assert_eq!(artifact.is_ok(), valid, "artifact: {input}");
        if valid {
            let snapshot = snapshot.expect("valid snapshot");
            assert_eq!(artifact.expect("valid artifact").to_snapshot(), snapshot);
            assert_eq!(SemioObjectSnapshot::parse_dsl(&snapshot.print_dsl()).expect("Object text"), snapshot);
            assert_eq!(SemioObjectSnapshot::decode_pack(&snapshot.encode_pack()).expect("Object Pack decode"), snapshot);
            let encoded = dsl::json::to_json_string(&snapshot);
            let independent: serde_json::Value = serde_json::from_str(&encoded).expect("independent JSON decode");
            assert!(store::pack_rt::json_values_equal(&independent, input), "{independent} != {input}");
        }
    }
    let rich: SemioObjectSnapshot = dsl::json::from_json_str(&fixture["snapshotCases"][1]["input"].to_string()).expect("rich Object");
    for case in fixture["diffCases"].as_array().expect("diff vectors") {
        let input = &case["input"];
        let diff = dsl::json::from_json_str::<SemioObjectDiff>(&input.to_string());
        let valid = case["valid"].as_bool().expect("expected diff admission");
        assert_eq!(diff.is_ok(), valid, "diff: {input}");
        if valid {
            let diff = diff.expect("valid diff");
            assert_eq!(SemioObjectDiff::parse_diff(&diff.print_diff()).expect("diff text"), diff);
            assert_eq!(SemioObjectDiff::decode_diff(&diff.encode_diff().expect("diff Pack")).expect("diff Pack decode"), diff);
            let next = diff.apply(&rich).expect("valid diff applies");
            if diff.mesh.is_none() { assert_eq!(next.mesh, rich.mesh); }
            if diff.brep.is_none() { assert_eq!(next.brep, rich.brep); }
            if diff.properties.is_none() { assert_eq!(next.properties, rich.properties); }
        }
    }
    for case in fixture["patchCases"].as_array().expect("neutral parent edit vectors") {
        let before: SemioObjectSnapshot = dsl::json::from_json_str(&case["before"].to_string()).expect("edit before");
        let diff: SemioObjectDiff = dsl::json::from_json_str(&case["diff"].to_string()).expect("edit diff");
        let expected: SemioObjectSnapshot = dsl::json::from_json_str(&case["after"].to_string()).expect("independent expected edit");
        assert_eq!(diff.apply(&before).expect("edit applies"), expected);
    }
    let text = include_str!("../../../🖼️assets/📦️crate/🗣️.dsl.semio");
    let pack = include_bytes!("../../../🖼️assets/📦️crate/🎒️.pack.semio");
    assert_eq!(SemioObjectSnapshot::parse_dsl(text).expect("authored text asset"), SemioObjectSnapshot::decode_pack(pack).expect("authored Pack asset"));
    eprintln!("[DEBUG] Object snapshot/diff JSON, text and Pack preserve exact child references and reject foreign owners");
}

#[semio_framework_async_macros::async_test]
async fn stdio_document_contract_object_rejects_invalid_typed_mutations() {
    use protocol::Mutation;
    use crate::standards::v1::subsets::object::schema::mutations::{SemioObjectMutation, apply_semio_object_mutation, create_mesh::CreateMesh};
    let mut snapshot = SemioObjectSnapshot::default();
    let before = snapshot.clone();
    let mutation = SemioObjectMutation::CreateMesh(CreateMesh {
        child_id: "wrong-owner-child".into(),
        target: store::os_io::ArtifactRef { artifact_id: "mesh-1".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() } },
    });
    let outcome = apply_semio_object_mutation(&mut snapshot, &mutation);
    assert_eq!(snapshot, before);
    assert!(mutation.inverse(&before).is_empty(), "rejected creation has no inverse effect");
    assert_eq!(outcome.messages().len(), 1);
    assert_eq!(outcome.messages()[0].code.0.as_str(), "mutation.child-identity");
    let foreign = serde_json::json!({"CreateMesh": {"child_id": "mesh-1", "target": {"artifactId": "mesh-1", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "mesh"}}, "locale": "de"}});
    assert!(dsl::json::from_json_str::<SemioObjectMutation>(&foreign.to_string()).is_err(), "closed mutation payload rejects OS settings");
    eprintln!("[DEBUG] Object mutation rejects mismatched child identity before modifying the parent");
}

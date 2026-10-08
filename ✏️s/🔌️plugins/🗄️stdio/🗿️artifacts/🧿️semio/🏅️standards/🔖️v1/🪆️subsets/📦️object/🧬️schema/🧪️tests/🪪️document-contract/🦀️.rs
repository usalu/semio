use crate::standards::v1::subsets::object::schema::{SemioObjectArtifact, diff::SemioObjectDiff, snapshot::SemioObjectSnapshot};
use protocol::{DiffBinary,DiffCodec,DiffText, MutationDiff};
use store::{ArtifactDsl, ArtifactPack};

#[semio_framework_async_macros::async_test]
async fn stdio_document_contract_object_round_trips_exact_children() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")).expect("neutral Object vectors");
    for case in fixture["snapshotCases"].as_array().expect("snapshot vectors") {
        let input = &case["input"];
        let text = input.to_string();
        let snapshot = semio_framework_pack_json::from_json_str::<SemioObjectSnapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject);
        let artifact = semio_framework_pack_json::from_json_str::<SemioObjectArtifact>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject);
        let valid = case["valid"].as_bool().expect("expected admission");
        assert_eq!(snapshot.is_ok(), valid, "snapshot: {input}");
        assert_eq!(artifact.is_ok(), valid, "artifact: {input}");
        if valid {
            let snapshot = snapshot.expect("valid snapshot");
            assert_eq!(artifact.expect("valid artifact").to_snapshot(), snapshot);
            assert_eq!(SemioObjectSnapshot::parse_dsl(&snapshot.print_dsl()).expect("Object text"), snapshot);
            assert_eq!(SemioObjectSnapshot::decode_pack(&snapshot.encode_pack()).expect("Object Pack decode"), snapshot);
            let encoded = semio_framework_pack_json::to_json_string(&snapshot);
            let independent: serde_json::Value = serde_json::from_str(&encoded).expect("independent JSON decode");
            assert!(store::pack_rt::json_values_equal(&independent, input), "{independent} != {input}");
        }
    }
    let rich: SemioObjectSnapshot = semio_framework_pack_json::from_json_str(&fixture["snapshotCases"][1]["input"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("rich Object");
    for case in fixture["diffCases"].as_array().expect("diff vectors") {
        let input = &case["input"];
        let diff = semio_framework_pack_json::from_json_str::<SemioObjectDiff>(&input.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject);
        let valid = case["valid"].as_bool().expect("expected diff admission");
        assert_eq!(diff.is_ok(), valid, "diff: {input}");
        if valid {
            let diff = diff.expect("valid diff");
            assert_eq!(SemioObjectDiff::parse_diff(&diff.print_diff()).expect("diff text"), diff);
            assert_eq!(SemioObjectDiff::decode_diff(&diff.encode_diff().expect("diff Pack")).expect("diff Pack decode"), diff);
            let next = protocol::apply_diff(&diff, &rich).expect("valid diff applies");
            if diff.mesh.is_none() { assert_eq!(next.mesh, rich.mesh); }
            if diff.brep.is_none() { assert_eq!(next.brep, rich.brep); }
            if diff.properties.is_none() { assert_eq!(next.properties, rich.properties); }
        }
    }
    for case in fixture["patchCases"].as_array().expect("neutral parent edit vectors") {
        let before: SemioObjectSnapshot = semio_framework_pack_json::from_json_str(&case["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("edit before");
        let diff: SemioObjectDiff = semio_framework_pack_json::from_json_str(&case["diff"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("edit diff");
        let expected: SemioObjectSnapshot = semio_framework_pack_json::from_json_str(&case["after"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("independent expected edit");
        assert_eq!(protocol::apply_diff(&diff, &before).expect("edit applies"), expected);
    }
    let text = include_str!("../../../🖼️assets/📦️crate/🗣️.dsl.semio");
    let pack = include_bytes!("../../../🖼️assets/📦️crate/🎒️.pack.semio");
    assert_eq!(SemioObjectSnapshot::parse_dsl(text).expect("authored text asset"), SemioObjectSnapshot::decode_pack(pack).expect("authored Pack asset"));
}

#[semio_framework_async_macros::async_test]
async fn stdio_document_contract_object_rejects_invalid_typed_mutations() {
    use protocol::Mutation;
    use crate::standards::v1::subsets::object::schema::mutations::{SemioObjectMutation, create_mesh::CreateMesh};
    let mut snapshot = SemioObjectSnapshot::default();
    let before = snapshot.clone();
    let mutation = SemioObjectMutation::CreateMesh(CreateMesh {
        child_id: "wrong-owner-child".into(),
        target: semio_framework_artifact_reference::ArtifactRef { artifact_id: "mesh-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "foreign.artifact".into(), standard: "v1".into(), subset: "mesh".into() } },
    });
    let (__next, outcome) = crate::applied(&snapshot, &mutation);
    snapshot = __next;
    assert_eq!(snapshot, before);
    assert!(mutation.inverse(&before).expect("valid retained mutation inverse fixture").is_empty(), "rejected creation has no inverse effect");
    assert_eq!(outcome.messages().len(), 1);
    assert_eq!(outcome.messages()[0].code.0.as_str(), "mutation.invariant");
    let foreign = serde_json::json!({"CreateMesh": {"child_id": "mesh-1", "target": {"artifactId": "mesh-1", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "mesh"}}, "locale": "de"}});
    assert!(semio_framework_pack_json::from_json_str::<SemioObjectMutation>(&foreign.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "closed mutation payload rejects OS settings");
}

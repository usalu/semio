
use super::*;

#[semio_framework_async_macros::async_test]
async fn artifact_kind_uses_the_playbook_media_kind_as_both_id_and_schema() {
    assert_eq!(artifact_kind().id, "text.playbook");
    assert_eq!(artifact_kind().schema, PLAYBOOK_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn block_fields_roundtrip() {
    let json = r#"{
            "id":"b1",
            "label":"Panel Count",
            "kind":"number",
            "required":true,
            "min":4,
            "max":64,
            "step":1,
            "unit":"panels"
        }"#;
    let block: PlaybookBlock = protocol::json::from_json_str(json).expect("block json");
    assert_eq!(block.min, Some(4.0));
    assert_eq!(block.unit.as_deref(), Some("panels"));
    assert!(block.required.unwrap_or(false));
}

//#region 🌉️ContentBridgeLaws
fn sample_steps() -> Vec<PlaybookStep> {
    vec![
        PlaybookStep {
            id: "intro".into(),
            title: "Introduction".into(),
            description: Some("What this playbook does.".into()),
            blocks: vec![PlaybookBlock {
                id: "name".into(),
                label: "Name".into(),
                kind: "text".into(),
                description: None,
                required: Some(true),
                placeholder: None,
                default: None,
                min: None,
                max: None,
                step: None,
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                fixture_slug: None,
                params: None,
                condition: Some(PlaybookExpr::Truthy { expr: Box::new(PlaybookExpr::Var { name: "enabled".into() }) }),
            }],
        },
        PlaybookStep { id: "review".into(), title: "Review".into(), description: None, blocks: Vec::new() },
    ]
}

/// ⚖️ LAW: `flow` is the LOSSLESS source of truth — every step field (including nested
/// `condition` trees) round-trips through `flow_content_snapshot_from_steps`/
/// `steps_from_flow_content` exactly.
#[semio_framework_async_macros::async_test]
async fn flow_content_round_trips_every_step_field_losslessly() {
    let steps = sample_steps();
    let content = flow_content_snapshot_from_steps(&steps);
    assert_eq!(content.nodes.len(), steps.len());
    assert_eq!(content.edges.len(), steps.len() - 1, "sequential steps chain via one edge per adjacent pair");
    let restored = steps_from_flow_content(&content);
    assert_eq!(restored, steps);
}

/// ⚖️ LAW: `document` is an HONEST narrative projection — `steps -> document` preserves every
/// title/description, and `document -> steps` recovers exactly that title/description skeleton
/// (never `blocks`/`condition`, which prose carries none of — documented lossy by design).
#[semio_framework_async_macros::async_test]
async fn document_projection_round_trips_titles_and_descriptions_only() {
    let steps = sample_steps();
    let content = document_snapshot_from_steps(Some("My Playbook"), &steps);
    let (title, restored) = steps_from_document(&content);
    assert_eq!(title.as_deref(), Some("My Playbook"));
    assert_eq!(restored.len(), steps.len());
    for (original, projected) in steps.iter().zip(restored.iter()) {
        assert_eq!(projected.title, original.title);
        assert_eq!(projected.description, original.description);
        assert!(projected.blocks.is_empty(), "document alone cannot recover block data — flow is that data's source of truth");
    }
}

fn one_step(title: &str) -> Vec<PlaybookStep> {
    vec![PlaybookStep { id: "step".into(), title: title.into(), description: None, blocks: Vec::new() }]
}

#[semio_framework_async_macros::async_test]
async fn scene_owner_fixture_proves_identity_isolation_aba_wire_omission_and_bounded_close() {
    let fixture: protocol::os_pack::json::Value = protocol::json::parse(include_str!("../../🧪️fixtures/👑️playbook-scene-owner-law.json")).expect("language-neutral playbook scene fixture");
    let cases = fixture["cases"].as_array().expect("fixture cases");
    assert_eq!(fixture["schemaVersion"], 1);
    assert_eq!(fixture["ownedSlots"], 1);
    assert_eq!(cases.len(), fixture["maximumCases"].as_u64().expect("bounded maximum") as usize);
    assert_eq!(cases.len(), 5);

    for case in cases {
        let law = case["law"].as_str().expect("law");
        let first = case["first"].as_str().expect("first");
        let second = case["second"].as_str().expect("second");
        match law {
            "cloneIdentity" => {
                let snapshot = playbook_snapshot_with_steps(PLAYBOOK_DOCUMENT_SCHEMA, "identity", "1", None, one_step(first));
                let retained = playbook_working_scene_owner(&snapshot.flow);
                let cloned = snapshot.clone();
                let cloned_owner = playbook_working_scene_owner(&cloned.flow);
                assert!(Arc::ptr_eq(&retained, &cloned_owner));
                assert_eq!(cloned_owner.steps[0].title, first);
                assert_eq!(Arc::strong_count(&retained), 4);
            }
            "instanceIsolation" => {
                let mut left = flow_content_child_handle(&one_step(first));
                let mut right = left.clone();
                attach_playbook_steps(&mut left, one_step(first));
                attach_playbook_steps(&mut right, one_step(second));
                assert_eq!(playbook_working_scene_owner(&left).steps[0].title, first);
                assert_eq!(playbook_working_scene_owner(&right).steps[0].title, second);
            }
            "abaIsolation" => {
                let mut stale = flow_content_child_handle(&one_step("same-identity"));
                attach_playbook_steps(&mut stale, one_step(first));
                let mut reused_identity = flow_content_child_handle(&one_step("same-identity"));
                assert_eq!(stale.child_id, reused_identity.child_id);
                attach_playbook_steps(&mut reused_identity, one_step(second));
                assert_eq!(playbook_working_scene_owner(&stale).steps[0].title, first);
                assert_eq!(playbook_working_scene_owner(&reused_identity).steps[0].title, second);
            }
            "wireOmission" => {
                let snapshot = playbook_snapshot_with_steps(PLAYBOOK_DOCUMENT_SCHEMA, "wire", "1", None, one_step(first));
                let wire = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&snapshot)).expect("third-party JSON oracle reads snapshot wire");
                assert!(wire.pointer("/flow/localOwner").is_none());
                let decoded: PlaybookSnapshot = dsl::os_pack::from_json_str(&wire.to_string()).expect("first-party codec decodes snapshot wire");
                assert!(decoded.flow.local_owner::<PlaybookWorkingScene>().is_none());
                assert!(playbook_working_scene_owner(&decoded.flow).steps.is_empty());
                assert_eq!(playbook_working_scene_owner(&snapshot.flow).steps[0].title, first);
            }
            "boundedClose" => {
                let snapshot = playbook_snapshot_with_steps(PLAYBOOK_DOCUMENT_SCHEMA, "close", "1", None, one_step(first));
                let retained = playbook_working_scene_owner(&snapshot.flow);
                let weak = Arc::downgrade(&retained);
                assert_eq!(Arc::strong_count(&retained), fixture["ownedSlots"].as_u64().expect("owned slots") as usize + 1);
                drop(snapshot);
                assert_eq!(Arc::strong_count(&retained), 1);
                drop(retained);
                assert!(weak.upgrade().is_none());
            }
            other => panic!("unexpected playbook scene law {other}"),
        }
    }
}
//#endregion 🌉️ContentBridgeLaws

use super::*;

#[semio_framework_async_macros::async_test]
async fn checkpoint_of_no_instances_round_trips_through_json() {
    let runtime = plugin_runtime::PluginRuntime::<crate::app::NoPluginApp>::new();
    let bytes = checkpoint(&runtime, &[], vec![1, 2], vec![7], Vec::new()).await.expect("an empty instance list must still encode");
    let pack: CheckpointPack = serde_json::from_slice(&bytes).expect("checkpoint bytes must be valid CheckpointPack json");
    assert!(pack.instances.is_empty());
    assert_eq!(pack.timers, vec![1, 2]);
    assert_eq!(pack.pending_requests, vec![7]);
    assert!(pack.task_restarts.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn task_restarts_round_trip_through_json_and_are_exposed_by_the_accessor() {
    let runtime = plugin_runtime::PluginRuntime::<crate::app::NoPluginApp>::new();
    let restarts = vec![TaskRestart { instance: 5, command: vec![1, 2, 3] }, TaskRestart { instance: 6, command: vec![4] }];
    let bytes = checkpoint(&runtime, &[], Vec::new(), Vec::new(), restarts.clone()).await.expect("must encode");
    let pack = restore(&runtime, &bytes).await.expect("must decode back").pack;
    assert_eq!(pack.task_restarts().await.len(), 2);
    assert_eq!(pack.task_restarts().await[0].instance, 5);
    assert_eq!(pack.task_restarts().await[0].command, vec![1, 2, 3]);
    assert_eq!(pack.task_restarts().await[1].instance, 6);
}

#[semio_framework_async_macros::async_test]
async fn checkpoint_requires_its_complete_declared_authority() {
    let incomplete = r#"{"instances":[],"timers":[],"pending_requests":[]}"#;
    assert!(serde_json::from_str::<CheckpointPack>(incomplete).is_err());
    assert!(semio_framework_pack_json::from_json_str::<CheckpointPack>(incomplete, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
}

#[test]
fn checkpoint_actor_contract_matches_independent_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("neutral checkpoint fixture");
    for case in fixture["cases"].as_array().expect("checkpoint cases") {
        let source = serde_json::to_string(&case["pack"]).expect("neutral checkpoint JSON");
        let independent = serde_json::from_str::<CheckpointPack>(&source);
        let actual = semio_framework_pack_json::from_json_str::<CheckpointPack>(&source, semio_framework_pack_json::JsonMemberPolicy::Reject);
        let expected = case["expected"].as_bool().expect("checkpoint verdict");
        assert_eq!(independent.is_ok(), expected, "independent: {}", case["id"]);
        assert_eq!(actual.is_ok(), expected, "first-party: {}", case["id"]);
        if let Ok(pack) = actual {
            let encoded: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&pack)).expect("first-party checkpoint JSON");
            assert_eq!(encoded, case["pack"], "actor and document authority survive: {}", case["id"]);
        }
    }
}

#[test]
fn checkpoint_preserves_explicit_instance_actor_in_authored_json() {
    let source = include_str!("📸️actor.json");
    let pack: CheckpointPack = semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored checkpoint must decode");
    assert_eq!(pack.instances[0].actor, "checkpoint-owner");
    let expected: serde_json::Value = serde_json::from_str(source).expect("independent checkpoint oracle must decode");
    let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&pack)).expect("first-party checkpoint must encode JSON");
    assert_eq!(actual, expected);
}

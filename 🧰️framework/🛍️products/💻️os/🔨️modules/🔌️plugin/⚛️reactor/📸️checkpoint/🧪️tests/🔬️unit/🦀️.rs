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
    let pack = restore(&runtime, &bytes).await.expect("must decode back");
    assert_eq!(pack.task_restarts().await.len(), 2);
    assert_eq!(pack.task_restarts().await[0].instance, 5);
    assert_eq!(pack.task_restarts().await[0].command, vec![1, 2, 3]);
    assert_eq!(pack.task_restarts().await[1].instance, 6);
}

#[semio_framework_async_macros::async_test]
async fn a_checkpoint_pack_encoded_before_task_restarts_existed_still_decodes() {
    // 🧬️ `#[serde(default)]` on `task_restarts` — an older pack (or a hand-built JSON blob
    // missing the field entirely) must not fail to restore just because this wave added a
    // field to the envelope (greenfield repo, no migration script, but a checkpoint taken
    // moments before an actor upgrade is a real same-process scenario, not legacy support).
    let legacy_json = r#"{"instances":[],"timers":[],"pending_requests":[]}"#;
    let pack: CheckpointPack = serde_json::from_str(legacy_json).expect("a pack missing task_restarts must still decode");
    assert!(pack.task_restarts().await.is_empty());
}

use super::*;

async fn owner_entry(mutation_id: &str) -> HostMutationRosterEntry {
    HostMutationRosterEntry { mutation_id: mutation_id.to_string(), verb: "set".into(), entity: "widget".into(), kind: "set-color".into(), record: "widget.doc".into(), contributor: None, artifact_kind: None }
}

async fn contributed_entry(mutation_id: &str, contributor: &str, artifact_kind: &str) -> HostMutationRosterEntry {
    HostMutationRosterEntry {
        mutation_id: mutation_id.to_string(),
        verb: "annotate".into(),
        entity: "widget".into(),
        kind: "annotate".into(),
        record: "widget.doc".into(),
        contributor: Some(contributor.to_string()),
        artifact_kind: Some(artifact_kind.to_string()),
    }
}

// 🪪️ `io::ArtifactKindId::parse("s.owner.widget").plugin()` returns the BARE middle segment
// ("owner", not "s.owner") — plugin ids throughout these fixtures are deliberately bare to
// match the real grammar (a real loaded plugin's `manifest.plugin_id` is its Cargo component
// metadata id, e.g. `"cad"`, never `"s.cad"`; only a canonical artifact kind string carries the
// `s.` prefix).
#[semio_framework_async_macros::async_test]
async fn owner_and_contributed_rows_both_resolve_correctly() {
    let router = ArtifactMutationRouter::new();
    router.register_roster("owner", &[], vec![owner_entry("widget.doc#set-color").await]).await.unwrap();
    let dependency = semio_framework::PluginDependency::new("owner", semio_framework::VersionReq::Any);
    router.register_roster("contributor", &[dependency], vec![contributed_entry("widget.doc#contributor:annotate", "contributor", "s.owner.widget").await]).await.unwrap();

    assert_eq!(router.resolve("s.owner.widget", "widget.doc#set-color").await.unwrap(), MutationOwnership::Owner { plugin_id: "owner".into() });
    assert_eq!(router.resolve("s.owner.widget", "widget.doc#contributor:annotate").await.unwrap(), MutationOwnership::Contributed { plugin_id: "contributor".into() });
    assert_eq!(router.roster().await.unwrap().len(), 2, "both the owner and the contributed row must be visible");
}

#[semio_framework_async_macros::async_test]
async fn a_contribution_onto_a_non_dependency_is_rejected() {
    let router = ArtifactMutationRouter::new();
    router.register_roster("owner", &[], vec![owner_entry("widget.doc#set-color").await]).await.unwrap();
    let error = router.register_roster("contributor", &[], vec![contributed_entry("widget.doc#contributor:annotate", "contributor", "s.owner.widget").await]).await.unwrap_err();
    assert!(matches!(error, PluginHostError::Plugin(_)));
}

#[semio_framework_async_macros::async_test]
async fn conflicting_owner_rows_are_rejected_unless_byte_identical() {
    let router = ArtifactMutationRouter::new();
    router.register_roster("a", &[], vec![owner_entry("widget.doc#set-color").await]).await.unwrap();
    let error = router.register_roster("a", &[], vec![HostMutationRosterEntry { verb: "different".into(), ..owner_entry("widget.doc#set-color").await }]).await.unwrap_err();
    assert!(matches!(error, PluginHostError::Plugin(_)));
    router.register_roster("a", &[], vec![owner_entry("widget.doc#set-color").await]).await.expect("byte-identical re-registration is idempotent");
}

#[semio_framework_async_macros::async_test]
async fn unregister_drops_only_that_plugins_rows() {
    let router = ArtifactMutationRouter::new();
    router.register_roster("owner", &[], vec![owner_entry("widget.doc#set-color").await]).await.unwrap();
    let dependency = semio_framework::PluginDependency::new("owner", semio_framework::VersionReq::Any);
    router.register_roster("contributor", &[dependency], vec![contributed_entry("widget.doc#contributor:annotate", "contributor", "s.owner.widget").await]).await.unwrap();
    router.unregister_plugin("contributor").await.unwrap();
    assert_eq!(router.roster().await.unwrap().len(), 1);
    assert!(router.resolve("s.owner.widget", "widget.doc#set-color").await.is_ok());
}

async fn mock_handle(actor: RuntimeActorId) -> (Arc<MockGuestRuntime>, Arc<PluginInstanceHandle>) {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
    let compiled = mock.compile(&PackageRef { package: PackageId("mutplan".to_string()), hash: PackageHash([7u8; 32]) }, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &budget).await.expect("mock instantiate");
    let handle = Arc::new(PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock.clone())), instance).await);
    (mock, handle)
}

/// 🎯️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): `register_plugin` keeps the handle
/// `plan` later needs, and `plan` resolves ownership THEN drives the owner's real
/// `semio.mutation-plan` job to completion (`MockGuestRuntime`-backed, not mocked-away resolution
/// alone) — proves this router's new `runtimes` field/`plan` method actually reach a handle, not
/// just that `resolve` still works.
#[semio_framework_async_macros::async_test]
async fn plan_drives_the_registered_owners_mutation_plan_job_to_completion() {
    let router = ArtifactMutationRouter::new();
    let (mock, handle) = mock_handle(RuntimeActorId(300)).await;
    let roster_bytes = encode_wire_dsl(&vec![owner_entry("widget.doc#set-color").await]).await.expect("encode roster");
    router.register_plugin("owner", &[], handle, &roster_bytes).await.expect("register_plugin must register the roster AND keep the handle");

    let request = HostArtifactMutationPlanRequest { artifact_kind: "s.owner.widget".to_string(), mutation_id: "widget.doc#set-color".to_string(), revision: 1, generation: 1, snapshot_pack: vec![1, 2, 3], payload: vec![4, 5] };
    let request_bytes = encode_wire_dsl(&request).await.expect("encode request");
    let expected_result =
        HostArtifactMutationPlanResult { artifact_kind: request.artifact_kind.clone(), mutation_id: request.mutation_id.clone(), revision: 1, generation: 1, owner_ops: vec![vec![9]], label: "mocked".to_string(), foreign: Vec::new() };
    mock.script_job_step(RuntimeActorId(300), JobStep::Done { output: encode_wire_dsl(&expected_result).await.expect("encode expected result") }).await;

    let result_bytes = router.plan(&request_bytes).await.expect("plan must resolve ownership AND drive the job to completion");
    let result: HostArtifactMutationPlanResult = decode_wire_dsl(&result_bytes).await.expect("decode plan result");
    assert_eq!(result, expected_result);
}

#[semio_framework_async_macros::async_test]
async fn plan_fails_with_a_named_error_when_the_owner_is_not_loaded() {
    let router = ArtifactMutationRouter::new();
    router.register_roster("owner", &[], vec![owner_entry("widget.doc#set-color").await]).await.expect("register the owner row");
    let request = HostArtifactMutationPlanRequest { artifact_kind: "s.owner.widget".to_string(), mutation_id: "widget.doc#set-color".to_string(), revision: 1, generation: 1, snapshot_pack: Vec::new(), payload: Vec::new() };
    let request_bytes = encode_wire_dsl(&request).await.expect("encode request");
    let error = router.plan(&request_bytes).await.expect_err("resolve succeeds but no handle was ever registered, so plan must still fail");
    assert!(matches!(error, PluginHostError::Plugin(message) if message.contains("not loaded")), "unexpected error");
}

use super::*;

async fn manifest(plugin_id: &str, version: &str, deps: &[(&str, &str)]) -> PluginManifest {
    let mut dependencies = Vec::with_capacity(deps.len());
    for (id, req) in deps {
        dependencies.push(semio_framework::PluginDependency::new(*id, semio_framework::VersionReq::parse(req).unwrap()));
    }
    PluginManifest {
        plugin_id: plugin_id.to_string(),
        label: plugin_id.to_string(),
        version: version.to_string(),
        apps: vec![],
        examples: vec![],
        capabilities: vec![],
        topic_contributions: vec![],
        commands: vec![],
        artifact_kinds: vec![],
        dependencies,
        contributions: vec![],
    }
}

/// 🔗️ The three ways a contribution can be blocked at DISPATCH time are distinguishable — the
/// frozen taxonomy has separate codes for them, and collapsing "owner gone" into
/// "not permitted" would tell an operator to fix a declaration that is already correct.
#[semio_framework_async_macros::async_test]
async fn contribution_block_separates_missing_owner_from_version_mismatch_from_undeclared() {
    let graph = PluginGraph::new();
    graph.register(manifest("cad", "1.0.0", &[]).await).await.unwrap();
    graph.register(manifest("aec", "1.0.0", &[("cad", "^1.0.0")]).await).await.unwrap();
    assert_eq!(graph.contribution_block("aec", "cad").await.unwrap(), None, "a declared, satisfied dependency blocks nothing");

    let (code, _) = graph.contribution_block("ghost", "cad").await.unwrap().expect("an unloaded contributor is blocked");
    assert_eq!(code, "transaction.dependency-missing");

    let (code, _) = graph.contribution_block("cad", "aec").await.unwrap().expect("an undeclared dependency is blocked");
    assert_eq!(code, "transaction.contribution-not-permitted");

    // 🛡️ The version branch is defence-in-depth, and this asserts WHY it cannot fire today rather
    // than pretending to exercise it: `register` re-validates the whole graph, so swapping `cad`
    // for a build `aec`'s requirement excludes is refused outright and the registered set keeps
    // its invariant. The branch stays because `contribution_block` is called per transaction, and
    // a future load path that mutates the set without that re-validation would otherwise hand a
    // contributor an owner it was never compiled against.
    let drift = graph.register(manifest("cad", "2.0.0", &[]).await).await.unwrap_err();
    assert!(matches!(drift, PluginGraphError::Graph(semio_framework::DependencyGraphError::VersionMismatch { .. })));
    assert_eq!(graph.contribution_block("aec", "cad").await.unwrap(), None, "the refused swap must leave the satisfied dependency intact");
}

#[semio_framework_async_macros::async_test]
async fn load_order_respects_a_real_dependency_edge() {
    let graph = PluginGraph::new();
    graph.register(manifest("base", "1.0.0", &[]).await).await.unwrap();
    graph.register(manifest("dependent", "1.0.0", &[("base", "^1.0.0")]).await).await.unwrap();
    assert_eq!(graph.load_order().await.unwrap(), vec!["base".to_string(), "dependent".to_string()]);
    assert_eq!(graph.dependents("base").await.unwrap(), vec!["dependent".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn register_rejects_a_missing_dependency() {
    let graph = PluginGraph::new();
    let error = graph.register(manifest("dependent", "1.0.0", &[("missing", "*")]).await).await.unwrap_err();
    assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::MissingDependency { .. })));
    assert!(!graph.is_registered("dependent").await.unwrap(), "a rejected registration must not partially commit");
}

#[semio_framework_async_macros::async_test]
async fn register_rejects_a_version_mismatch() {
    let graph = PluginGraph::new();
    graph.register(manifest("base", "1.0.0", &[]).await).await.unwrap();
    let error = graph.register(manifest("dependent", "1.0.0", &[("base", "^2.0.0")]).await).await.unwrap_err();
    assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::VersionMismatch { .. })));
}

#[semio_framework_async_macros::async_test]
async fn a_later_registration_that_would_close_a_cycle_is_rejected() {
    let graph = PluginGraph::new();
    graph.register(manifest("a", "1.0.0", &[]).await).await.unwrap();
    graph.register(manifest("b", "1.0.0", &[("a", "*")]).await).await.unwrap();
    // Re-registering "a" (as if hot-reloading it) to depend on "b" would close a -> b -> a.
    let error = graph.register(manifest("a", "1.0.0", &[("b", "*")]).await).await.unwrap_err();
    assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::Cycle { .. })));
}

#[semio_framework_async_macros::async_test]
async fn unload_is_refused_while_a_dependent_is_registered() {
    let graph = PluginGraph::new();
    graph.register(manifest("base", "1.0.0", &[]).await).await.unwrap();
    graph.register(manifest("dependent", "1.0.0", &[("base", "^1.0.0")]).await).await.unwrap();
    let error = graph.guard_unload("base").await.unwrap_err();
    assert!(matches!(error, PluginGraphError::UnloadBlocked { .. }));
    graph.unregister("dependent").await.unwrap();
    graph.guard_unload("base").await.expect("no dependents left, unload must now be permitted");
}

#[semio_framework_async_macros::async_test]
async fn hot_reload_is_rejected_when_it_would_break_a_live_dependents_version_requirement() {
    let graph = PluginGraph::new();
    graph.register(manifest("base", "1.0.0", &[]).await).await.unwrap();
    graph.register(manifest("dependent", "1.0.0", &[("base", "^1.0.0")]).await).await.unwrap();
    let error = graph.prepare_hot_reload(&manifest("base", "2.0.0", &[]).await).await.unwrap_err();
    assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::VersionMismatch { .. })));
    graph.prepare_hot_reload(&manifest("base", "1.1.0", &[]).await).await.expect("a caret-compatible bump must still validate");
}

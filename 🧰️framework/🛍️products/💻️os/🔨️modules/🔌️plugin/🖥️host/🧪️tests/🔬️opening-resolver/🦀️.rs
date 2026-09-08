use super::app_router_tests::{dialect, fixture_app};
use super::*;

#[semio_framework_async_macros::async_test]
async fn step1_explicit_default_still_in_router_wins() {
    let router = AppRouter::new();
    let editor_dialect = dialect("*").await;
    router
        .register_manifest(
            "cad",
            &PluginManifest {
                plugin_id: "cad".into(),
                label: "cad".into(),
                version: "0.1.0".into(),
                apps: vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await],
                examples: Vec::new(),
                capabilities: Vec::new(),
                topic_contributions: Vec::new(),
                commands: Vec::new(),
                artifact_kinds: Vec::new(),
                dependencies: Vec::new(),
                contributions: Vec::new(),
            },
        )
        .await
        .expect("owner registers");
    router
        .register_manifest(
            "norm",
            &PluginManifest {
                plugin_id: "norm".into(),
                label: "norm".into(),
                version: "0.1.0".into(),
                apps: vec![fixture_app("s.cad.cad@1/*#editor-alt", editor_dialect.clone(), semio_framework::AppRole::Editor).await],
                examples: Vec::new(),
                capabilities: Vec::new(),
                topic_contributions: Vec::new(),
                commands: Vec::new(),
                artifact_kinds: Vec::new(),
                dependencies: vec![semio_framework::PluginDependency::new("cad", semio_framework::VersionReq::Any)],
                contributions: Vec::new(),
            },
        )
        .await
        .expect("norm contributes a second editor for the same dialect");
    let pinned = semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#editor-alt".into() };
    let resolved = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Editor, Some(&pinned)).await.expect("pinned default resolves");
    assert_eq!(resolved, pinned);
}

#[semio_framework_async_macros::async_test]
async fn step2_and_step3_collapse_to_the_owner_surface_when_default_is_stale() {
    let router = AppRouter::new();
    let editor_dialect = dialect("*").await;
    router
        .register_manifest(
            "cad",
            &PluginManifest {
                plugin_id: "cad".into(),
                label: "cad".into(),
                version: "0.1.0".into(),
                apps: vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await],
                examples: Vec::new(),
                capabilities: Vec::new(),
                topic_contributions: Vec::new(),
                commands: Vec::new(),
                artifact_kinds: Vec::new(),
                dependencies: Vec::new(),
                contributions: Vec::new(),
            },
        )
        .await
        .expect("owner registers");
    let stale_default = semio_framework::AppRef { plugin_id: "gone".into(), app_id: "s.cad.cad@1/*#editor".into() };
    let resolved = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Editor, Some(&stale_default)).await.expect("falls through to owner surface");
    assert_eq!(resolved, semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() });
}

#[semio_framework_async_macros::async_test]
async fn step3_first_entry_when_the_owner_has_no_surface_for_this_role() {
    let router = AppRouter::new();
    let editor_dialect = dialect("*").await;
    router
        .register_manifest(
            "cad",
            &PluginManifest {
                plugin_id: "cad".into(),
                label: "cad".into(),
                version: "0.1.0".into(),
                apps: Vec::new(),
                examples: Vec::new(),
                capabilities: Vec::new(),
                topic_contributions: Vec::new(),
                commands: Vec::new(),
                artifact_kinds: Vec::new(),
                dependencies: Vec::new(),
                contributions: Vec::new(),
            },
        )
        .await
        .expect("owner claims nothing yet, zero apps");
    router
        .register_manifest(
            "norm",
            &PluginManifest {
                plugin_id: "norm".into(),
                label: "norm".into(),
                version: "0.1.0".into(),
                apps: vec![fixture_app("s.cad.cad@1/*#viewer", editor_dialect.clone(), semio_framework::AppRole::Viewer).await],
                examples: Vec::new(),
                capabilities: Vec::new(),
                topic_contributions: Vec::new(),
                commands: Vec::new(),
                artifact_kinds: Vec::new(),
                dependencies: Vec::new(),
                contributions: Vec::new(),
            },
        )
        .await
        .expect("s.cad.cad has no owner yet, so norm becomes it by being first to declare a surface for it");
    let resolved = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Viewer, None).await.expect("first (only) entry resolves");
    assert_eq!(resolved, semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#viewer".into() });
}

#[semio_framework_async_macros::async_test]
async fn step4_unknown_dialect_when_the_router_has_nothing() {
    let router = AppRouter::new();
    let editor_dialect = dialect("*").await;
    let error = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Editor, None).await.expect_err("empty router must fault");
    assert_eq!(error.code.0, "surface.unknown-dialect");
}

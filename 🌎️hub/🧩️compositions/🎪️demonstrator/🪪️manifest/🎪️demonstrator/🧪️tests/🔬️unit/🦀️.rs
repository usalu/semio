use super::*;

fn test_bundle() -> Plugin<DemonstratorApps> {
    plugin().unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn bundle_keeps_its_plugin_identity() {
    let manifest = test_bundle().manifest;
    assert_eq!(manifest.plugin_id, PLUGIN_ID);
    assert_eq!(manifest.label, PLUGIN_LABEL);
    assert_eq!(manifest.version, PLUGIN_VERSION);
    assert_eq!(manifest.dependencies.iter().map(|dependency| dependency.plugin_id.as_str()).collect::<Vec<_>>(), vec!["process", "sourcing"]);
}

/// 🔗️ Ticket 26/09/05/S-END-TO-END lane H: the dependency list above is not free-standing prose —
/// every surface this bundle registers for an artifact kind it does NOT own must name that kind's
/// owner among its dependencies, or the host's `AppRouter` excludes the whole plugin
/// (`surface.contribution-not-permitted`) and none of its six surfaces route. Derived from the
/// built manifest's own app dialects, so adding a borrowed app without its dependency fails here.
#[test]
fn every_borrowed_surface_is_backed_by_a_declared_dependency() {
    artifact_app_laws::assert_surface_dependencies_declared(&test_bundle().manifest);
}

/// 🎯️ Playground's own two native surfaces are registered first (right after `.artifact(...)`),
/// then the `sourcing` and `process` editor+viewer pairs.
#[test]
fn bundle_registers_its_own_and_the_two_foreign_demonstrator_surfaces() {
    let ids: Vec<String> = test_bundle().manifest.apps.iter().map(|app| app.id.clone()).collect();
    assert_eq!(
        ids,
        vec![
            "s.demonstrator.playground@1/*#editor",
            "s.demonstrator.playground@1/*#viewer",
            "s.sourcing.curation@1/*#editor",
            "s.sourcing.curation@1/*#viewer",
            "s.process.process3d@1/*#editor",
            "s.process.process3d@1/*#viewer",
        ]
    );
}

/// 🏠️ LAW: demonstrator HOSTS the kinds of the two plugins whose surfaces it embeds — it owns only its playground, every
/// embedded surface's kind is a hosted row naming that surface's owner (a declared dependency), and every embedded editor
/// presents a kind it hosts — so a trusted catalog with demonstrator and the owners has one creating editor per kind .
#[test]
fn demonstrator_hosts_the_kinds_of_every_embedded_surface_and_owns_only_its_playground() {
    let manifest = test_bundle().manifest;
    let owned: std::collections::BTreeSet<&str> = manifest.artifact_kinds.iter().chain(manifest.apps.iter().flat_map(|app| app.artifact_kinds.iter())).map(|kind| kind.id.as_str()).collect();
    assert_eq!(owned, std::collections::BTreeSet::from(["playground.document"]));
    let hosted: std::collections::BTreeSet<(&str, &str, &str)> = manifest.hosted_artifact_kinds.iter().map(|kind| (kind.id.as_str(), kind.schema.as_str(), kind.owner.as_str())).collect();
    assert_eq!(
        hosted,
        std::collections::BTreeSet::from([
            ("catalogue.sourcing", "sourcing.curation/v1", "sourcing"),
            ("catalogue.kinds", "catalogue.kinds", "sourcing"),
            ("kit.catalog", "kit.catalog", "sourcing"),
            ("3d.process", "process.3d", "process"),
        ])
    );
    let dependencies: std::collections::BTreeSet<&str> = manifest.dependencies.iter().map(|dependency| dependency.plugin_id.as_str()).collect();
    assert!(hosted.iter().all(|(_, _, owner)| dependencies.contains(owner)), "every hosted row names a declared dependency as its owner");
    for app in manifest.apps.iter().filter(|app| app.role == semio_framework::AppRole::Editor && !app.dialect.artifact_kind.starts_with("s.demonstrator.")) {
        assert!(hosted.iter().any(|(id, schema, _)| *id == app.io.artifact.id && *schema == app.io.artifact_schema), "{} presents {} ({}), a kind demonstrator does not host", app.id, app.io.artifact.id, app.io.artifact_schema);
    }
}

#[test]
fn every_surface_declares_a_artifact_schema() {
    for app in test_bundle().manifest.apps {
        assert!(!app.io.artifact_schema.is_empty(), "app {} declares no document schema", app.id);
    }
}

#[test]
fn contribution_consumers_declare_the_hidden_app_command() {
    let consumers: Vec<String> = test_bundle().manifest.apps.iter().filter(|app| app.commands.iter().any(|command| command.id == "setContributions")).map(|app| app.id.clone()).collect();
    assert_eq!(consumers, vec!["s.sourcing.curation@1/*#editor", "s.process.process3d@1/*#editor"]);
    for app in test_bundle().manifest.apps {
        if let Some(command) = app.commands.iter().find(|command| command.id == "setContributions") {
            assert!(!command.in_palette, "host catalogue command leaked into {}'s palette", app.id);
            let args: Vec<&str> = command.args.iter().map(|arg| arg.id.as_str()).collect();
            assert!(args == vec!["json"] || args == vec!["json", "page", "pageCount"], "{} declares an unknown contributions argument roster: {args:?}", app.id);
            assert_eq!(command.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "host catalogue command in {} must be admitted as migrated interactive work", app.id);
        }
    }
}

fn assert_tree_reconciles(tree: ComponentTree, generation: u64, label: &str) {
    assert!(!tree.root.key.is_empty(), "{label} must contain an authored root");
    let mut producer = semio_ui_runtime::ComponentTreeProducer::try_new(tree.root, generation).expect("nonzero aggregate tree generation");
    for _ in 0..65_536 {
        match producer.step(generation, false, false) {
            semio_ui_runtime::ComponentTreeProducerStep::MoreWork => {}
            semio_ui_runtime::ComponentTreeProducerStep::Complete => {
                assert!(producer.take_complete().is_some(), "completed {label} tree transfers its exact owner");
                return;
            }
            semio_ui_runtime::ComponentTreeProducerStep::Fault(fault) => panic!("{label} tree must enter retained reconciliation: {fault:?}"),
        }
    }
    panic!("{label} tree producer did not settle within its fixed bound");
}

#[semio_framework_async_macros::async_test]
async fn aggregate_runtime_renders_every_demonstrator_window() {
    let runtime = semio_plugin::plugin_runtime::PluginRuntime::<DemonstratorApps>::new(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT).expect("explicit test mounted owner policy");
    semio_plugin::plugin_runtime::install_plugin_bundle(&runtime, test_bundle());
    let apps: &[(&str, &[&str])] = &[
        ("s.sourcing.curation@1/*#editor", &["sourcing.pool", "sourcing.curated", "sourcing.preview", "sourcing.grid"]),
        ("s.process.process3d@1/*#editor", &["process.play.main"]),
    ];
    let view_state = serde_json::to_string(&semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).expect("aggregate view state serializes");
    let mut generation = 1_u64;
    for (app_index, (app_id, body_keys)) in apps.iter().enumerate() {
        let instance_id = u32::try_from(app_index + 1).expect("aggregate app instances");
        semio_plugin::plugin_runtime::plugin_create_app_with_id(&runtime, instance_id, app_id, semio_os_kernel::ActorId(semio_os_kernel::os_spr::LOCAL_ACTOR_ID.into())).await.unwrap_or_else(|fault| panic!("aggregate app {app_id} opens: {fault:?}"));
        for body_key in *body_keys {
            let tree = semio_plugin::plugin_runtime::plugin_render(&runtime, instance_id, body_key, &view_state).await.unwrap_or_else(|fault| panic!("aggregate body {body_key} renders: {fault:?}"));
            assert_tree_reconciles(tree, generation, body_key);
            generation += 1;
        }
    }
}

/// ✏️🌱️ Asserts one bundled editor's own roster declares every dialect it derives a genesis child for,
/// pinning how many it derives so a surface that stops composing cannot make the assertion vacuous.
fn assert_editor_children_declared<E: ArtifactEditor>(app_id: &str, derived: usize) {
    let checked = artifact_app_laws::assert_editor_genesis_children_declared::<E>().unwrap_or_else(|fault| panic!("{app_id} derives a genesis child its member roster does not declare: {fault:?}"));
    assert_eq!(checked, derived, "{app_id} derives a different number of genesis children than this bundle pins");
}

/// 👁️🌱️ Viewer twin of `assert_editor_children_declared`.
fn assert_viewer_children_declared<V: ArtifactViewer>(app_id: &str, derived: usize) {
    let checked = artifact_app_laws::assert_viewer_genesis_children_declared::<V>().unwrap_or_else(|fault| panic!("{app_id} derives a genesis child its member roster does not declare: {fault:?}"));
    assert_eq!(checked, derived, "{app_id} derives a different number of genesis children than this bundle pins");
}

/// 🧩️ Ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS: bundling a foreign app into this component
/// must not change the roster its derived children are opened through. `VcsArtifactApp<_, M>` faults
/// at app creation (`seed_genesis_children` → `genesis_member_schema::<M>`) for any dialect `M` does
/// not declare — aussuchen and verfolgen crashed on `s.stdio.semio@v1/kit` exactly this way while the
/// bundle registered them over `NoMembers`. Every surface this manifest lists is proven here, and the
/// two that really compose children carry a nonzero count, so the next app bundled in cannot regress
/// this without failing natively first.
#[test]
fn every_bundled_surface_declares_its_derived_child_dialects() {
    assert_editor_children_declared::<crate::editor::playground::PlaygroundEditor>("s.demonstrator.playground@1/*#editor", 0);
    assert_viewer_children_declared::<crate::viewer::playground::PlaygroundViewer>("s.demonstrator.playground@1/*#viewer", 0);
    assert_editor_children_declared::<SourcingCurationApp>("s.sourcing.curation@1/*#editor", 1);
    assert_viewer_children_declared::<SourcingViewer>("s.sourcing.curation@1/*#viewer", 1);
    assert_editor_children_declared::<Process3dPlayApp>("s.process.process3d@1/*#editor", 5);
    assert_viewer_children_declared::<Process3dViewer>("s.process.process3d@1/*#viewer", 5);
}

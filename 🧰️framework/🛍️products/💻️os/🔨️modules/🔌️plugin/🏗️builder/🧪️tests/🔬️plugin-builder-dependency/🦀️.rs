use super::*;
use crate::app::{ArtifactContribution, FlowExtensionExecutableIdentity, FlowExtensionManifest};
use std::sync::atomic::{AtomicUsize, Ordering};

static MESH_IMPORT_EXECUTIONS: AtomicUsize = AtomicUsize::new(0);
/// 🔒️ `MESH_IMPORT_EXECUTIONS` is process-global, and BOTH tests that read it reset it to 0 first,
/// so running them concurrently makes each one observe the other's increments — a race that is
/// invisible whenever either test is run alone. Every test touching that counter takes this
/// guard, which is what actually makes the assertions about it meaningful.
static MESH_IMPORT_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

async fn host_media_kind() -> semio_framework::ArtifactKindSpec {
    semio_framework::ArtifactKindSpec {
        id: "3d.builder-test".into(),
        name: "Builder Test 3D".into(),
        source_format: "semio.builder-test.mesh/v1".into(),
        component_kind: "builder-test".into(),
        dimension: "3d".into(),
        media_capability: semio_framework::OsMediaCapability::MeshOnly,
        media_type: semio_framework::MediaType { class: semio_framework::MediaClass::ThreeD, form: semio_framework::MediaForm::Mesh },
        schema: "semio.builder-test.mesh/v1".into(),
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        export_stdio_kinds: Vec::new(),
        import_stdio_kinds: Vec::new(),
    }
}

fn counting_mesh_importer(_mesh: &semio_framework::MeshData) -> Result<dsl::os_pack::json::Value, String> {
    MESH_IMPORT_EXECUTIONS.fetch_add(1, Ordering::SeqCst);
    Ok(dsl::json!({ "bridge": "counting" }))
}

fn alternate_mesh_importer(_mesh: &semio_framework::MeshData) -> Result<dsl::os_pack::json::Value, String> {
    MESH_IMPORT_EXECUTIONS.fetch_add(100, Ordering::SeqCst);
    Ok(dsl::json!({ "bridge": "alternate" }))
}

use super::dependency_fixture::{AddValue, DependencyTestOp, DependencyTestSnapshot};

async fn contribution(target_artifact_kind: &str) -> ArtifactContribution {
    ArtifactContribution::builder(target_artifact_kind).await.mutation::<DependencyTestSnapshot, DependencyTestOp, AddValue>("dep-target.document", 1, 1).await.build()
}

#[semio_framework_async_macros::async_test]
async fn dependency_gating_rejects_a_contribution_onto_a_non_dependency() {
    let error = Plugin::<crate::app::NoPluginApp>::builder("builder-test-contributor-missing-dep")
        .label("Builder Test Contributor Missing Dep")
        .version("0.1.0")
        .contributes(contribution("s.builder-test-dep-target.thing").await)
        .try_build()
        .err()
        .expect("a contribution with no matching declared dependency must be rejected");
    assert_eq!(error.code, "plugin-assembly.contribution-gate");
    assert!(error.message.contains("not a direct dependency"), "unexpected message: {}", error.message);
}

#[semio_framework_async_macros::async_test]
async fn a_direct_dependency_permits_its_contribution_and_lands_on_the_manifest() {
    let plugin = Plugin::<crate::app::NoPluginApp>::builder("builder-test-contributor-ok")
        .label("Builder Test Contributor Ok")
        .version("0.1.0")
        .depends_on("builder-test-dep-target-ok", semio_framework::VersionReq::Any)
        .contributes(contribution("s.builder-test-dep-target-ok.thing").await)
        .try_build()
        .expect("a contribution onto a direct dependency must be accepted");
    assert_eq!(plugin.manifest.dependencies.len(), 1);
    assert_eq!(plugin.manifest.dependencies[0].plugin_id, "builder-test-dep-target-ok");
    assert_eq!(plugin.manifest.contributions.len(), 1);
    assert_eq!(plugin.manifest.contributions[0].artifact_kind, "s.builder-test-dep-target-ok.thing");
    assert_eq!(plugin.manifest.contributions[0].mutations[0].mutation_id, "dep-target.document#builder-test-contributor-ok:add-value");
}

#[semio_framework_async_macros::async_test]
async fn host_media_contributions_are_idempotent_and_execute_only_at_runtime() {
    let _guard = MESH_IMPORT_GUARD.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    MESH_IMPORT_EXECUTIONS.store(0, Ordering::SeqCst);
    let kind = host_media_kind().await;
    let bridge = HostMediaHandlerDeclaration::mesh_import("builder-test.media.mesh-import", kind.clone(), kind.schema.clone(), counting_mesh_importer).expect("typed bridge declaration");
    let plugin = Plugin::<crate::app::NoPluginApp>::builder("builder-test-media")
        .label("Builder Test Media")
        .version("0.1.0")
        .artifact_kind(kind.clone())
        .host_media_handler(bridge.clone())
        .host_media_handler(bridge)
        .try_build()
        .expect("identical frozen host-media declarations are idempotent");
    assert_eq!(MESH_IMPORT_EXECUTIONS.load(Ordering::SeqCst), 0, "assembly must never execute a media converter");
    assert_eq!(plugin.host_media_handlers().len(), 1);
    let result = plugin.import_mesh(&crate::MeshImportRequest { artifact_kind: kind.id.clone(), document_schema: kind.schema.clone(), mesh: semio_framework::MeshData::default() }).expect("runtime bridge execution");
    assert_eq!(result.document, dsl::json!({ "bridge": "counting" }));
    assert_eq!(MESH_IMPORT_EXECUTIONS.load(Ordering::SeqCst), 1);
}

#[semio_framework_async_macros::async_test]
async fn host_media_conflicts_reject_the_whole_candidate_before_execution() {
    let _guard = MESH_IMPORT_GUARD.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    MESH_IMPORT_EXECUTIONS.store(0, Ordering::SeqCst);
    let kind = host_media_kind().await;
    let first = HostMediaHandlerDeclaration::mesh_import("builder-test.media.first", kind.clone(), kind.schema.clone(), counting_mesh_importer).expect("first bridge");
    let second = HostMediaHandlerDeclaration::mesh_import("builder-test.media.second", kind.clone(), kind.schema.clone(), alternate_mesh_importer).expect("second bridge");
    let error = Plugin::<crate::app::NoPluginApp>::builder("builder-test-media-conflict")
        .label("Builder Test Media Conflict")
        .version("0.1.0")
        .artifact_kind(kind)
        .host_media_handler(first)
        .host_media_handler(second)
        .try_build()
        .err()
        .expect("two executable identities may not own one host-media target");
    assert_eq!(error.code, "plugin-assembly.host-media-target");
    assert_eq!(MESH_IMPORT_EXECUTIONS.load(Ordering::SeqCst), 0, "a rejected aggregate must have no runtime side effect");
}

#[semio_framework_async_macros::async_test]
async fn flow_extension_descriptors_are_idempotent_and_conflict_rejecting() {
    let manifest = FlowExtensionManifest::new("builder-test-flow", "Builder Test Flow", "0.1.0").expect("typed manifest");
    let executable = FlowExtensionExecutableIdentity::native("semio.builder-test.flow", "semio.builder-test.flow.module", "activate").expect("typed executable identity");
    let declaration = FlowExtensionDeclaration::new("builder-test.flow.contribution", manifest.clone(), executable.clone()).expect("flow declaration");
    let plugin = Plugin::<crate::app::NoPluginApp>::builder("builder-test-flow")
        .label("Builder Test Flow")
        .version("0.1.0")
        .flow_extension(declaration.clone())
        .flow_extension(declaration)
        .try_build()
        .expect("identical frozen flow declarations are idempotent");
    assert_eq!(plugin.flow_extensions().len(), 1);
    let conflict = FlowExtensionDeclaration::new("builder-test.flow.other", manifest, executable).expect("conflicting target descriptor");
    let error = Plugin::<crate::app::NoPluginApp>::builder("builder-test-flow-conflict")
        .label("Builder Test Flow Conflict")
        .version("0.1.0")
        .flow_extension(
            plugin
                .flow_extensions()
                .into_iter()
                .next()
                .map(|descriptor| FlowExtensionDeclaration::new(descriptor.id, descriptor.manifest, descriptor.executable_identity))
                .expect("a flow extension to rebuild from")
                .expect("the rebuilt descriptor to be valid"),
        )
        .flow_extension(conflict)
        .try_build()
        .err()
        .expect("one flow extension id may have exactly one contribution owner");
    assert_eq!(error.code, "plugin-assembly.flow-extension-target");
}

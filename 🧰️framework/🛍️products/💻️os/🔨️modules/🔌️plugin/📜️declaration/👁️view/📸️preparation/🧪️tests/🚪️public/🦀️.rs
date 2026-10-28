use semio_framework_plugin::*;
#[global_allocator]
static ORIGINAL_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_job::{StepBudget, StepContext};
use semio_framework_ui_runtime::ComponentTree;

#[path = "../🦀️.rs"]
mod receipts;

#[derive(Clone, Copy, Default)]
struct Probe { step: MountedSnapshotReadPreparation, calls: usize, reads: usize, parent_identity:usize, source_version:Option<semio_framework_value_registry::fixed_request_owners::RequestSourceVersion>, pre_heap:Option<semio_framework_trace::HeapAllocationObservation>, grant: RetainedCloneGrant }
std::thread_local! { static ORIGINAL: std::cell::Cell<Option<Probe>> = const { std::cell::Cell::new(None) }; }

#[derive(Default)]
struct PreparationApp;
impl ArtifactApp for PreparationApp {
    const DIALECT: Dialect = Dialect { artifact_kind: "test.mounted-preparation", standard: StandardId("1"), subset: SubsetId::ANY };
    const APP_ID: &'static str = "test.mounted-preparation@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "config.empty";
    type Snapshot = NoConfig;
    type Mutation = NoConfigMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = NoConfigMutation;
    fn reference_birth()->Option<fn()->Self::Snapshot>{Some(||{ NoConfig::default() })}
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view: Option<&ViewModel>, _draft: &DraftView<'_, Self::Draft>, _engines: &semio_framework_2d_compute::EngineHandles) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation> {
        Ok(Emit::default())
    }
    fn mounted_job_prepare_snapshot_read(_operation: AppRenderOperationContext, _snapshot: &Self::Snapshot, _windows: &WindowConfigOwnerRegistry, _view: Option<&ViewModel>, grant: RetainedCloneGrant) -> MountedSnapshotReadPreparation {
        ORIGINAL.with(|probe| { let Some(mut original) = probe.get() else { return Default::default() }; original.pre_heap=semio_framework_trace::heap_allocation_observation_on_this_thread(); original.calls += 1; original.grant = grant; probe.set(Some(original)); original.step })
    }
    async fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, doc: &crate::ArtifactPendingView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
        ORIGINAL.with(|probe| { if let Some(mut original) = probe.get() { original.reads += usize::from(doc.take_snapshot_read().is_ok()); original.parent_identity=doc.parent_document_id().as_ptr()as usize; original.source_version=Some(doc.source_version()); probe.set(Some(original)); } });
        Vec::new()
    }
    async fn render(_body: &str, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        Err(PluginAssemblyError::new("fixture.mounted-preparation", "this fixture declares only the mounted preparation boundary"))
    }
}

#[semio_framework_async_macros::async_test]
async fn mounted_snapshot_preparation_boundary_uses_original_context_and_readiness() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut policy = artifact_app_laws::fixture_mounted_policy();
    policy.preparation.maximum_items = 8;
    let mut identity = artifact_app_laws::fixture_identity();
    let mut app = Box::pin(VcsArtifactApp::<PreparationApp>::new(PreparationApp, replication::ActorId("mounted-preparation".into()), policy, &mut identity)).await;
    artifact_app_laws::settle_store_constructor(&mut app, policy);
    let original_parent_identity=app.fixture_original_document_identity().as_ptr()as usize;
    let original_source_version=app.fixture_original_document_source_version();
    for row in fixture["cases"].as_array().unwrap() {
        let number = |field: &str, index: usize| row[field][index].as_u64().unwrap() as usize;
        let grant = RetainedCloneGrant { maximum_items: number("grant", 0), maximum_copy_bytes: number("grant", 1), maximum_capacity_bytes: number("grant", 2), maximum_release_bytes: number("grant", 3), maximum_depth: number("grant", 4) };
        let progress = RetainedCloneProgress { copied_items: number("progress", 0), copied_bytes: number("progress", 1), retained_capacity_bytes: number("progress", 2), released_bytes: number("progress", 3) };
        let ready = row["ready"].as_bool().unwrap();
        ORIGINAL.with(|probe| probe.set(Some(Probe { step: MountedSnapshotReadPreparation { ready, progress }, ..Default::default() })));
        let mut sequence = 0; let mut receipt = RetainedCloneProgress::default();
        let mut cx = StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), StepBudget::new(8, u64::MAX, grant), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
        let admitted_grant = MountedSnapshotReadPreparation::grant(policy.preparation, &cx).unwrap();
        let (result,_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_async::poll::resolve_ready(PluginApp::pending_effects(&mut app,None,&mut cx)));
        assert_eq!(result.is_err(), row["fault"].as_bool().unwrap());
        assert_eq!(cx.retained_progress(), progress);
        let observed = ORIGINAL.with(|probe| probe.take().unwrap());
        let pre_heap=observed.pre_heap.expect("original callback samples live allocator scope");assert_eq!(pre_heap.requested_bytes,fixture["preAdmissionHeap"]["requestedBytes"].as_u64().unwrap()as usize);assert_eq!(pre_heap.released_bytes,fixture["preAdmissionHeap"]["releasedBytes"].as_u64().unwrap()as usize);assert!(!pre_heap.overflowed);assert_eq!(observed.source_version,result.is_ok().then_some(original_source_version)); assert_eq!(observed.parent_identity,if result.is_ok(){original_parent_identity}else{0},"admitted poll view must borrow original envelope identity"); assert_eq!(observed.calls, 1); assert_eq!(observed.grant, admitted_grant); assert_eq!(observed.reads, usize::from(ready && result.is_ok()));
    }
    let mut sequence = 0; let mut receipt = RetainedCloneProgress::default();
    let mut cx = StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), StepBudget::new(0, u64::MAX, policy.preparation), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
    ORIGINAL.with(|probe| probe.set(Some(Default::default())));
    assert!(PluginApp::pending_effects(&mut app, None, &mut cx).await.unwrap().is_empty());
    assert_eq!(ORIGINAL.with(|probe| probe.take().unwrap().calls), 0);
    let _ = artifact_app_laws::close_registered_fixture_app(&mut app, policy);
    assert!(PluginApp::close_terminal_is_empty(&app));
}

#[test]
fn mounted_pending_source_version_compares_complete_original_authority(){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let version=|v:&serde_json::Value|{let hex=v["canonicalRevision"].as_str().unwrap();let canonical_revision=std::array::from_fn(|i|u8::from_str_radix(&hex[i*2..i*2+2],16).unwrap());(semio_framework_value_registry::fixed_request_owners::RequestSourceVersion{base_revision:v["baseRevision"].as_u64().unwrap(),generation:v["generation"].as_u64().unwrap(),canonical_revision},v["overlay"].as_u64().unwrap())};for row in f["sourceVersions"].as_array().unwrap(){assert_eq!(version(&row["before"])!=version(&row["after"]),row["changed"].as_bool().unwrap());}}

mod typed_command_full_operation_tests {
    use semio_framework_actor::typed_operation_page::{TypedOperationResultPage,TypedOperationResultToken,TYPED_OPERATION_RESULT_PAGE_BYTES};
    use semio_framework_plugin_decl_app::tool_contract::TYPED_OPERATION_FAULT_BYTES;
    use semio_framework_plugin_instance_app::typed_operation_document_is_fresh;
    use semio_framework_plugin_operation::TOOL_CANCELLATION_SLOTS;
    use semio_framework_plugin_job::reserved_jobs::{plugin_job_yield_once,QualifiedToolProof};
    use semio_framework_os_kernel::one_item_publication;
use semio_framework_os_kernel::one_item::{ArtifactStoreOneItemCheckpoint,ArtifactStoreOneItemPreparationStep};
use semio_framework_os_kernel::one_item;
use semio_framework_plugin_decl_emission::ChildEmitGenesis;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_plugin_operation::ArtifactDocumentAuthority;
use semio_framework_plugin_operation::ToolOperationKey;
use semio_framework::ArtifactDialect;
use semio_framework_os_kernel::SchemaId;
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_os_kernel::LocalizedLabel;
use semio_framework_test_fixtures::fixture_path;

    use super::*;
    use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
    use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::change_publication_presence::ChangePublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::PublicationPresenceMutation;

    /// 📖️ One module tree of the plugin SDK, read at run time from the manifest folder of the hosting crate instead of embedded: the root file first, then every file it mounts in declaration order.
    fn module_source(relative: &str) -> String {
        semio_framework_plugin_test_support::read_module_tree_source(&fixture_path!("../../../..").join(relative)).expect("the module tree reads")
    }

    /// 🧵️ Source text of the mounted artifact instance family: the composer and use-case crates of the instance, the maintenance ladder, the cancellation owners, the retained command shell and the lifted full-operation job, each crate family in path order, read at run time from the manifest folder of the hosting crate.
    #[doc(hidden)]
pub fn instance_source() -> String {
        ["🧵️instance", "🪜️maintenance-ladder", "♻️cancellation", "🧵️retained-command", "👷️job"]
            .iter()
            .map(|folder| semio_framework_plugin_test_support::read_source_family_source(&fixture_path!("../../../..").join(folder)).expect("the instance family reads"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[doc(hidden)]
pub const FIXTURE: &str = include_str!("../../🧵️retained-command/🔄️full-operation/🧪️tests/🧫️fixtures/🔣️.json");



    fn publish_mounted_fixture_unit<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, mounted: &mut MountedTypedCommandFullOperation<A>, grant: RetainedCloneGrant, cx: &mut semio_framework_job::StepContext<'_>) {
        cx.with_retained_loan(grant, |same| {
            let before = same.retained_progress();
            let fuel = same.fuel_remaining();
            app.typed_ctx().publish_mounted_typed_operation_unit(mounted, grant, same)?;
            if let Some((returned_grant, progress)) = mounted.publication_ownership_progress.take() {
                assert!(progress.fits(returned_grant) && progress.fits(grant));
                let unposted = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_unposted(progress, before, same.retained_progress())?;
                same.consume_retained(unposted)?;
            }
            if same.fuel_remaining() == fuel { same.consume_fuel(1); }
            Ok::<_, semio_framework_job::OriginalTurnFailure>(())
        }).expect("actual mounted publication returns each original receipt once");
    }

    fn present_mounted_page<A: ArtifactApp>(mounted: &mut MountedTypedCommandFullOperation<A>, grant: RetainedCloneGrant, cx: &mut semio_framework_job::StepContext<'_>) -> Option<TypedOperationResultPage> {
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
            let mut output = None;
            let demand = mounted.result_page_presentation_demands().unwrap();
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx, grant, demand, |_, grant| {
                    let (page, progress) = mounted.take_result_page(grant)?;
                    let complete = page.is_some();
                    output = page;
                    Ok((if complete { PluginLifecycleStep::Complete(progress) } else { PluginLifecycleStep::Progress(progress) }, complete))
                }).expect("original mounted page presentation")
            });
            let progress = match step { PluginLifecycleStep::Progress(p) | PluginLifecycleStep::Complete(p) => p, _ => Default::default() };
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            if output.is_some() || progress == Default::default() { return output; }
        }
        panic!("mounted result presentation exceeded the original caller bound")
    }

    fn retire_acknowledged_mounted_page<A: ArtifactApp>(mounted: &mut MountedTypedCommandFullOperation<A>, grant: RetainedCloneGrant, cx: &mut semio_framework_job::StepContext<'_>) {
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
            if !mounted.result_page_retirement_pending() { return; }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cx.with_retained_loan(grant, |same| mounted.advance_acknowledged_result_page(same)).expect("same original producer page retirement"));
            let progress = match step { PluginLifecycleStep::Progress(p) | PluginLifecycleStep::Complete(p) => p, _ => Default::default() };
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        }
        panic!("original acknowledged mounted page did not retire")
    }

    include!("🔎️census-oracles-and-fixture-machine/🦀️.rs");
    include!("🔐️two-turn-presence-and-root-retirement/🦀️.rs");
    include!("🚫️cancellation-publication-boundaries/🦀️.rs");
    include!("📌️document-cancellation-and-latest-wins-keys/🦀️.rs");
    include!("⚖️slot-and-publication-fairness/🦀️.rs");
    include!("🌐️neutral-hostile-rows/🦀️.rs");
    include!("👪️child-genesis-wire-and-group-owner/🦀️.rs");
    include!("🧾️source-rejection-anchors-and-host-lane/🦀️.rs");
}

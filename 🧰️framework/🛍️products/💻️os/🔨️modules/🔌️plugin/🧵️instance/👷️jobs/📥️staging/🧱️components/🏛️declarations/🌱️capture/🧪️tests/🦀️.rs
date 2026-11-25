//! 🧪️ Concrete registration capture uses the actual committed source and returns every original alias on cancellation.
use semio_framework_job::{close_original_retained_effect_step, close_original_retained_step, default_now_us, Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_instance_jobs::jobs_components::{produce_declared_tool_catalog, OriginalDeclaredEmptyOperationSource, OriginalDeclaredTools};
use semio_plugin_mounted_owner::MountedOwnerPolicyV1;
use semio_plugin_store_lifecycle::original_store_genesis::OriginalStoreBirth;
const HEADER: RetainedCloneGrant = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation;
const TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096, ..HEADER };
fn caller() -> OriginalJobFixtureCaller {
    let mut caller = OriginalJobFixtureCaller::admit(
        OperationId(78222),
        Generation(9),
        RetainedCloneGrant { maximum_items: 1 << 26, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 29, maximum_depth: 4096 },
        default_now_us,
        u64::MAX,
        1 << 26,
    );
    caller.retain_dispatch().unwrap();
    caller
}
fn observed<T>(caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner, g: RetainedCloneGrant, action: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> T {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.refreshed_turn_with_decoder(g, decoder, action));
    let result = result.unwrap();
    let p = caller.business_receipt().checked_add(caller.construction_receipt()).unwrap();
    assert!(!heap.overflowed);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (p.retained_capacity_bytes, p.released_bytes));
    result
}
fn fixture() -> (semio_plugin_window_kits::RegisteredAppDefinition, OriginalDeclaredTools) {
    let mut definition = semio_framework_async::poll::resolve_ready(semio_framework_manifest_app_surface::testkit::app_with(Vec::new(), Vec::new()));
    definition.id = <crate::fixture::NativeWindowApp as ArtifactApp>::APP_ID.into();
    definition.controller_id = definition.id.clone();
    definition.dialect = <crate::fixture::NativeWindowApp as ArtifactApp>::DIALECT.into();
    definition.io.artifact_schema = <crate::fixture::NativeWindowApp as ArtifactApp>::DOCUMENT_SCHEMA.into();
    definition.actions.push(semio_framework::ActionDefinition::resumable_framework("adjust", semio_ui_locale::LocalizedLabel::native("Adjust", "Anpassen"), semio_framework::ActionKind::Mutation, "pencil"));
    let mut definition = semio_plugin_window_kits::RegisteredAppDefinition::declared(definition, true);
    let mut catalog = OriginalDeclaredTools::new(produce_declared_tool_catalog::<crate::fixture::NativeWindowApp>(definition.original_source().unwrap()).unwrap());
    definition.commit_producer();
    catalog.commit_producer();
    (definition, catalog)
}
#[test]
fn concrete_original_registration_captures_same_rows_and_closes_each_cancelled_prefix() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for cutoff in std::iter::once(usize::MAX).chain(law["cancelTurns"].as_array().unwrap().iter().map(|n| n.as_u64().unwrap() as usize)) {
        let (mut definition, mut catalog) = fixture();assert_eq!(definition.id,law["controller"].as_str().unwrap());
        let first = &definition.actions[0];
        let id = first.id.clone();
        let original = std::ptr::from_ref(first) as usize;
        let mut caller = caller();
        let mut decoder = caller.admitted_decoder();
        let mut birth = OriginalStoreBirth::default();
        let mut source = None;
        for _ in 0..64 {
            if observed(&mut caller, &mut decoder, HEADER, |cx| OriginalDeclaredEmptyOperationSource::admit(cx, &mut birth, &mut source)) {
                break;
            }
        }
        assert!(source.is_some());
        let mut output = None;
        let mut completed = false;
        for turn in 0..cutoff.min(65536) {
            let ready = observed(&mut caller, &mut decoder, TURN, |cx| source.as_mut().unwrap().advance(definition.original_source().unwrap(), catalog.source().unwrap(), cx));
            if ready {
                assert_eq!(source.as_ref().unwrap().progress(), 5);
                for _ in 0..64 {
                    if observed(&mut caller, &mut decoder, HEADER, |cx| source.as_mut().unwrap().take(cx, &mut output)) {
                        break;
                    }
                }
                assert_eq!(output.as_ref().unwrap().registry().get(&id).unwrap() as *const _ as usize, original);
                assert_eq!(output.as_ref().unwrap().bus().identity(),Some(catalog.source().unwrap().bus.identity()));
                completed = true;
                break;
            }
            assert!(turn < 65535, "original registration capture stalled phase={}", source.as_ref().unwrap().progress());
        }
        if cutoff == usize::MAX {
            assert!(completed)
        }
        caller.cancel().cancel_now();
        for _ in 0..64 {
            if birth.terminal_is_empty() { break; }
            observed(&mut caller, &mut decoder, HEADER, |cx| cx.with_original_retirement(|cx| birth.close_step(cx)));
        }
        assert!(birth.terminal_is_empty());
        if let Some(owner) = output.as_mut() {
            for turn in 0..65536 {
                if owner.terminal_is_empty() {
                    break;
                }
                observed(&mut caller, &mut decoder, HEADER, |cx| cx.with_original_retirement(|cx| owner.close_step(cx)));
                assert!(turn < 65535, "original operation registration output close stalled");
            }
        }
        drop(output.take());
        for turn in 0..65536 {
            if source.as_ref().unwrap().terminal_is_empty() {
                break;
            }
            observed(&mut caller, &mut decoder, HEADER, |cx| cx.with_original_retirement(|cx| source.as_mut().unwrap().close_step(cx)));
            assert!(turn < 65535, "original operation registration source close stalled phase={}", source.as_ref().unwrap().progress());
        }
        let d = OriginalDeclaredEmptyOperationSource::boxed_release_demands();
        observed(&mut caller, &mut decoder, HEADER, |cx| {
            cx.with_original_retirement(|cx| {
                close_original_retained_step(cx, d, |_| {
                    source.take();
                    semio_framework_value::RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, released_bytes: d.release_bytes, ..Default::default() })), work: 1 }
                })?;
                Ok(())
            })
        });
        for turn in 0..65536 {
            if catalog.terminal_is_empty() {
                break;
            }
            let d = catalog.close_demands(4096).unwrap();
            observed(&mut caller, &mut decoder, HEADER, |cx| {
                cx.with_original_retirement(|cx| {
                    close_original_retained_effect_step(cx, d, |effect| catalog.close_step(effect))?;
                    Ok(())
                })
            });
            assert!(turn < 65535)
        }
        for turn in 0..65536 {
            if definition.terminal_is_empty() {
                break;
            }
            let d = definition.close_demands(4096).unwrap();
            observed(&mut caller, &mut decoder, HEADER, |cx| {
                cx.with_original_retirement(|cx| {
                    close_original_retained_effect_step(cx, d, |effect| definition.close_step(effect))?;
                    Ok(())
                })
            });
            assert!(turn < 65535)
        }
        while !decoder.terminal_is_empty() {
            caller
                .refreshed_turn(HEADER, |cx| cx.with_original_retirement(|cx| decoder.close_on_source(cx)))
                .unwrap();
        }
        caller.retire(HEADER);
    }
}

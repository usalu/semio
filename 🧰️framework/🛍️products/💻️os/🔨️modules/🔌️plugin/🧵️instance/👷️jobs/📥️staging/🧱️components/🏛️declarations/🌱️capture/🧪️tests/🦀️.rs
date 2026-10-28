//! 🧪️ Concrete registration capture uses the actual committed source and returns every original alias on cancellation.
use super::*;
const TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096, maximum_release_bytes: 1048576, maximum_depth: 4096 };
const HEADER: RetainedCloneGrant = RetainedCloneGrant { maximum_copy_bytes: 32768, maximum_capacity_bytes: 16 * 1048576, ..TURN };
fn caller() -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(
        OperationId(78222),
        Generation(9),
        RetainedCloneGrant { maximum_items: 1 << 26, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 29, maximum_depth: 4096 },
        default_now_us,
        u64::MAX,
        1 << 26,
    )
}
fn observed<T>(caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner, g: RetainedCloneGrant, action: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> T {
    let ((result, p), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        let mut p = Default::default();
        let result = caller.turn_with_decoder(g, decoder, |cx| {
            let before = cx.retained_progress();
            let result = action(cx);
            p = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before, cx.retained_progress())?;
            result
        });
        (result, p)
    });
    let result = result.unwrap();
    assert_eq!((heap.requested_bytes, heap.released_bytes), (p.retained_capacity_bytes, p.released_bytes));
    result
}
fn fixture() -> (semio_framework_plugin_window_kits::RegisteredAppDefinition, OriginalDeclaredTools) {
    let mut definition = semio_framework_async::poll::resolve_ready(semio_framework_manifest_app_surface::testkit::app_with(Vec::new(), Vec::new()));
    definition.id = <crate::fixture::NativeWindowApp as ArtifactApp>::APP_ID.into();
    definition.controller_id = definition.id.clone();
    definition.dialect = <crate::fixture::NativeWindowApp as ArtifactApp>::DIALECT.into();
    definition.io.artifact_schema = <crate::fixture::NativeWindowApp as ArtifactApp>::DOCUMENT_SCHEMA.into();
    definition.actions.push(semio_framework::ActionDefinition::resumable_framework("adjust", semio_framework_ui_locale::LocalizedLabel::native("Adjust", "Anpassen"), semio_framework::ActionKind::Mutation, "pencil"));
    let mut definition = semio_framework_plugin_window_kits::RegisteredAppDefinition::declared(definition, true);
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
        let mut decoder = OriginalJobDecodeOwner::empty();
        let mut source = None;
        observed(&mut caller, &mut decoder, HEADER, |cx| OriginalDeclaredEmptyOperationSource::admit(cx, &mut source));
        let mut output = None;
        let mut completed = false;
        for turn in 0..cutoff.min(65536) {
            let ready = observed(&mut caller, &mut decoder, TURN, |cx| source.as_mut().unwrap().advance(definition.original_source().unwrap(), catalog.source().unwrap(), cx));
            if ready {
                assert_eq!(source.as_ref().unwrap().progress(), 5);
                observed(&mut caller, &mut decoder, HEADER, |cx| source.as_mut().unwrap().take(cx, &mut output));
                assert_eq!(output.as_ref().unwrap().registry().get(&id).unwrap() as *const _ as usize, original);
                assert_eq!(output.as_ref().unwrap().bus().identity(),Some(catalog.source().unwrap().bus.identity()));
                completed = true;
                break;
            }
            assert!(turn < 65535, "[DEBUG] original registration capture stalled phase={}", source.as_ref().unwrap().progress());
        }
        if cutoff == usize::MAX {
            assert!(completed)
        }
        caller.cancel().cancel_now();
        if let Some(owner) = output.as_mut() {
            for turn in 0..65536 {
                if owner.terminal_is_empty() {
                    break;
                }
                observed(&mut caller, &mut decoder, HEADER, |cx| cx.with_original_retirement(|cx| owner.close_step(cx)));
                assert!(turn < 65535, "[DEBUG] original operation registration output close stalled");
            }
        }
        drop(output.take());
        for turn in 0..65536 {
            if source.as_ref().unwrap().terminal_is_empty() {
                break;
            }
            observed(&mut caller, &mut decoder, HEADER, |cx| cx.with_original_retirement(|cx| source.as_mut().unwrap().close_step(cx)));
            assert!(turn < 65535, "[DEBUG] original operation registration source close stalled phase={}", source.as_ref().unwrap().progress());
        }
        let d = OriginalDeclaredEmptyOperationSource::boxed_release_demands();
        observed(&mut caller, &mut decoder, HEADER, |cx| {
            cx.with_original_retirement(|cx| {
                close_original_retained_step(cx, d, |_| {
                    source.take();
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, released_bytes: d.release_bytes, ..Default::default() }))
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
                    close_original_retained_step(cx, d, |g| catalog.close_step(g))?;
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
                    close_original_retained_step(cx, d, |g| definition.close_step(g))?;
                    Ok(())
                })
            });
            assert!(turn < 65535)
        }
        while !decoder.terminal_is_empty() {
            caller
                .turn(HEADER, |cx| {
                    cx.with_original_retirement(|cx| {
                        let d = decoder.retirement_demands(4096)?;
                        close_original_retained_step(cx, d, |g| decoder.close_step(g))?;
                        Ok(())
                    })
                })
                .unwrap();
        }
        caller.retire(HEADER);
    }
}

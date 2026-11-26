use semio_framework::kernel::UiDirtyScope;
use semio_framework_actor::typed_operation_page::TypedOperationResultToken;
use semio_framework_job::{Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller, close_original_retained_effect_step, close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneStep, RetainedWorkOutcome, ValueError};
use semio_plugin_result_page::pending_result_page::{PendingPublicationPage, PublicationPageIntent, PublicationPagePlan, PublicationPageSource};

#[test]
fn compact_ui_page_writes_the_committed_frame_as_one_paid_item() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/📃️page-admission/🧫️fixtures/🔣️.json")).unwrap();
    let vector = corpus["cases"].as_array().unwrap().iter().find(|row| {
        row["payload"]["kind"] == "ui" && row["payload"]["value"]["kind"] == "partial" && row["expected"]["state"]["frame"].is_string()
    }).unwrap();
    let json = &vector["payload"]["value"];
    let scope: UiDirtyScope = serde_json::from_value(json.clone()).unwrap();
    let source = PublicationPageSource::Ui(&scope);
    let token = TypedOperationResultToken { receiver: 7, operation: 11, generation: 3, sequence: 2, attempt: 1 };
    let plan = PublicationPagePlan { token, intent: PublicationPageIntent::Ui };
    let grant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 65_536, maximum_release_bytes: 65_536, maximum_depth: 64 };
    let stock = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 64 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(44781), Generation(8), stock, || Some(1), u64::MAX, 65_536);
    let mut decoder: OriginalJobDecodeOwner = caller.admitted_decoder();
    assert!(caller.turn_with_decoder(grant, &mut decoder, |cx| {
        let mut outcome = None;
        let ran = cx.with_native_receiving(&mut outcome, |_, _| Ok(()))?;
        cx.settle_receiving_scope()?;
        Ok(ran)
    }).unwrap());
    let mut owner = None;
    caller.turn_with_decoder(grant, &mut decoder, |cx| cx.with_original_retirement(|cx| {
        close_original_retained_step(cx, PendingPublicationPage::capture_demands(), |issued| {
            let turn = PendingPublicationPage::capture(plan, issued);
            RetainedWorkOutcome { work: turn.work, result: turn.result.map(|captured| {
                let (captured, progress) = captured.unwrap();
                owner = Some(captured);
                RetainedCloneStep::Progress(progress)
            }) }
        })?;
        Ok(())
    })).unwrap();
    let mut owner = owner.unwrap();
    for _ in 0..128 {
        if owner.backing_ready() { break; }
        caller.turn_with_decoder(grant, &mut decoder, |cx| cx.with_original_retirement(|cx| {
            let d = owner.backing_demands(grant.maximum_copy_bytes)?;
            close_original_retained_step(cx, d, |issued| owner.advance_backing(issued))?;
            Ok(())
        })).unwrap();
    }
    assert!(owner.backing_ready(), "backing demand {:?} after witness {}", owner.backing_demands(grant.maximum_copy_bytes), owner.phase_witness());
    let work = 4096;
    let write_grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_depth: 3, ..Default::default() };
    assert_eq!(owner.write_admission_demands(&source, write_grant, work).unwrap().capacity_bytes, 0);
    caller.turn_with_decoder(grant, &mut decoder, |cx| cx.with_original_retirement(|cx| {
        let mut write = None;
        cx.with_native_retirement(|decode| {
            write = Some(owner.write_retirement_step(&source, write_grant, work, decode));
            Ok::<(), ValueError>(())
        })?;
        let write = write.unwrap();
        assert!(write.result.unwrap_or(false));
        assert!(write.work <= work && write.progress.fits(write_grant));
        assert_eq!(write.progress.copied_items, 1);
        if let Some(issued) = cx.receiving_issued_grant() { cx.consume_receiving_retained(issued, write.progress)?; } else { cx.consume_retained(write.progress)?; }
        cx.consume_fuel(write.work);
        Ok(())
    })).unwrap();
    assert!(owner.ready_to_handoff());
    let mut page = None;
    caller.turn_with_decoder(grant, &mut decoder, |cx| cx.with_original_retirement(|cx| {
        close_original_retained_step(cx, owner.handoff_demands()?, |issued| {
            let turn = owner.take_page(issued);
            RetainedWorkOutcome { work: turn.work, result: turn.result.map(|result| {
                let (output, progress) = result.unwrap();
                page = Some(output);
                RetainedCloneStep::Complete(progress)
            }) }
        })?;
        Ok(())
    })).unwrap();
    let mut page = page.unwrap();
    assert_eq!(page.token, token);
    assert_eq!(page.bytes(), vector["expected"]["state"]["frame"].as_str().unwrap().as_bytes());
    assert_eq!(serde_json::from_slice::<serde_json::Value>(page.bytes()).unwrap(), *json);
    for _ in 0..1024 {
        if page.terminal_is_empty() { break; }
        caller.turn_with_decoder(grant, &mut decoder, |cx| cx.with_original_retirement(|cx| {
            close_original_retained_effect_step(cx, page.next_close_demand(grant.maximum_copy_bytes)?, |effect| page.close_step(effect))?;
            Ok(())
        })).unwrap();
    }
    assert!(page.terminal_is_empty());
    for _ in 0..1024 {
        if decoder.terminal_is_empty() { break; }
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
            decoder.close_on_source(cx)?;
            Ok(())
        })).unwrap();
    }
    assert!(decoder.terminal_is_empty());
    caller.retire(stock);
}

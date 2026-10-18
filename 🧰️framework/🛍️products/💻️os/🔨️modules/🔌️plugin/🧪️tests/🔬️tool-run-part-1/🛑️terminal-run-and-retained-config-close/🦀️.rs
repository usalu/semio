// 🛑️ Terminal run and retained config close: a terminal run that arms no further work and a retained config over one envelope page that closes after a render.

//#region 🎯️RetargetAndSettings
/// ⚖️ LAW: a TERMINAL run arms nothing. Once a run is finalized or aborted the ledger owes the driver no work
/// and hands the host no effect, however many turns the host takes.
///
/// 🪪️ Every wake a run hands the host is a REQUEST the host answers by dispatching an action back into the
/// guest, so a request a run that is OVER keeps re-making is an endless action storm on a shell nobody is
/// touching. Generation3d's finalized read-only `previewEval` re-armed its pace wake once per presentation
/// retry and filled the console with `toolRunPace` on a quiet, converged editor — seq 107…116 within seconds,
/// 2 800 dropped console lines (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, 2026-09-14 23:25). The latch is read
/// once per ANSWER, never once per REFRESH, and a terminal run answers nothing.
#[semio_framework_async_macros::async_test]
async fn a_terminal_run_arms_no_further_work_however_many_turns_the_host_takes() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["terminalQuiet"];
    let turns = number(&expected["turns"]);
    for row in expected["rows"].as_array().expect("the terminalQuiet rows") {
        let id = text(&row["id"]);
        let mut app = toy_app(number(&row["units"]), &mut caller).await;
        start(&mut app, text(&row["toolId"]), &mut caller).await;
        if text(&row["terminalBy"]) == "abort" {
            pump_until(&mut app, "the run leaves starting", |app| app.tool_runs.state().is_some_and(|state| state != ToolRunState::Starting), &mut caller).await;
            run_action(&mut app, "toolRunAbort", &mut caller).await;
        }
        pump_until(&mut app, "the run reaches a terminal state", |app| app.tool_runs.state().is_some_and(ToolRunState::is_terminal) && !app.tool_runs.has_pending_work(), &mut caller).await;
        while app.take_typed_operation_effect().is_some() {}
        let mut armed: Vec<String> = Vec::new();
        let mut pending_turns = 0u64;
        for _ in 0..turns {
            if app.tool_runs.has_pending_work() {
                pending_turns += 1;
            }
            advance_toy(&mut app, &mut caller).await.unwrap_or_else(|fault| panic!("{id}: a quiet turn faulted: {fault:?}"));
            while let Some(effect) = app.take_typed_operation_effect() {
                armed.push(format!("{effect:?}"));
            }
        }
        let state = app.tool_runs.state().expect("a terminal state");
        println!("[STATS] terminalQuiet {id}: state={} turns={turns} pendingWorkTurns={pending_turns} armed={}", state.as_str(), armed.len());
        assert!(state.is_terminal(), "{id}: the run stays terminal");
        assert_eq!(pending_turns == 0, expected_flag(&row["expected"]["hasPendingWork"]) == false, "{id}: a terminal run is no driver work over {turns} turns");
        assert_eq!(armed.len() as u64, number(&row["expected"]["effects"]), "{id}: a terminal run arms nothing, got {armed:?}");
        close(&mut app, &mut caller);
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

fn expected_flag(value: &Value) -> bool {
    value.as_bool().expect("a boolean expectation")
}

//#endregion 🎯️RetargetAndSettings

//#region 🧹️RetainedConfigCloseCliff
/// 🧹️ A retained config larger than one `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` page must still reach
/// terminal-empty AFTER a body has rendered against it.
///
/// 🏁️ Measured 2026-09-16 (ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS): with a body rendered,
/// a config over ~one page livelocked `close_step(1, 4_096)` at `Pending { 0, 0 }` for 15 M turns,
/// while the same config closed with no render. The render backfills the command log, whose LABEL
/// carries the applied config op text; `close_retained_fields_step` priced that whole log entry
/// against one turn grant and pushed it back unreleased every turn. The cliff capped EVERY app's
/// retained config at one page regardless of its declared maximum (cad declares 65 536 B), so the
/// `sourcing.module` / `cad.computer` / `process.machines` host packs could never be retained.
#[semio_framework_async_macros::async_test]
async fn a_retained_config_over_one_envelope_page_closes_after_a_render() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    for (bytes, rendered) in [(2_909usize, true), (3_706, true), (4_360, true), (16_384, true), (65_536, true), (3_820, false)] {
        let mut app = toy_app(1, &mut caller).await;
        app.stores.config_store
            .dispatch(ArtifactCommand::Apply { mutations: vec![ChangeTestConfigSelection { selected: Some("c".repeat(bytes)) }.into()], transaction: None }, &mut caller.identity)
            .await
            .expect("a retained config past one envelope page applies");
        if rendered {
            let _ = render_text(&mut app, "main", &mut caller).await;
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut turns = 0u64;
        let mut productive = 0u64;
        let mut last = String::new();
        while std::time::Instant::now() < deadline {
            if app.close_terminal_is_empty() {
                break;
            }
            turns += 1;
            let demand = app.close_retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("original fixture close demand");
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_copy_bytes: store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, maximum_release_bytes: store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, ..caller.policy.close };
            assert!(demand.copy_bytes<=grant.maximum_copy_bytes && demand.capacity_bytes<=grant.maximum_capacity_bytes && demand.release_bytes<=grant.maximum_release_bytes && demand.depth<=grant.maximum_depth,"actual demand fits original independent page and caller ceilings");
            assert!(grant.maximum_copy_bytes <= store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES && grant.maximum_release_bytes <= store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, "original fixture body and physical grant limits remain unchanged");
            match { let step = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(caller.context,grant,demand,|same,grant|Ok((app.close_step(same,grant)?,app.close_terminal_is_empty()))).expect("original close step"); let _ = app.take_constructor_receipt(); step } {
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) => {
                    assert!(progress.fits(grant));
                    let released_items = progress.copied_items;
                    let released_bytes = progress.released_bytes;
                    assert!(released_items <= 1 && released_bytes <= store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, "a close page stays inside its exact grant");
                    if released_items != 0 || released_bytes != 0 {
                        productive = turns;
                    }
                    last = format!("Pending {{ {released_items}, {released_bytes} }}");
                }
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress) => { assert!(progress.fits(grant)); assert!(app.close_terminal_is_empty()); break; },
                step => last = format!("{step:?}"),
            }
        }
        println!("[STATS] retainedConfigClose bytes={bytes} rendered={rendered} turns={turns} lastProductiveTurn={productive} last={last}");
        assert!(app.close_terminal_is_empty(), "a {bytes} B retained config (rendered={rendered}) must reach terminal-empty; last step {last} after {turns} turns");
        assert_eq!(productive, turns, "every close turn released something; the pump never spends a turn on zero progress");
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🧹️RetainedConfigCloseCliff

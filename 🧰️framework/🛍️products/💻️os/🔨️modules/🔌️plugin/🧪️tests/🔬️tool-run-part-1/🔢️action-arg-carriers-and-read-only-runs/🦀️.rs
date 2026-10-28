// 🔢️ Action arg carriers and read-only runs: the exact integer carrier identity, the wire finalize and the self-finalizing read-only run.

//#region 🔢️ActionArgCarriers
/// 🔢️ The wire value a fixture row names, in the carrier it names — the shapes a §2.5 action argument
/// really arrives in: a guest's `DslValue::uint`, the shell's `UiValue::Number` float, and the text a
/// targeted action's `runId` travels as.
fn carrier_value(row: &Value) -> DslValue {
    match text(&row["carrier"]) {
        "uint" => DslValue::uint(row["value"].as_u64().expect("an unsigned fixture carrier")),
        "float" => DslValue::float(row["value"].as_f64().expect("a float fixture carrier")),
        "string" => DslValue::String(text(&row["value"]).to_string()),
        other => panic!("the fixture named an unknown arg carrier {other}"),
    }
}

/// ⚖️ LAW: every `actionArgCarriers` row — a §2.5 action argument that is an exact non-negative integer
/// reads as that integer whichever carrier minted it, and nothing else names an identity at all.
///
/// 🪪️ `generation` crosses as `Number(Float(1.0))` from the shell's own action arguments and from a
/// guest's `DslValue::uint` alike; a reader that took only an exact `u64` carrier or a numeric string
/// answered `None`, and `(_, None)` is `ToolRunRejection::Stale`. Every `toolRunFinalize` of every app
/// was refused, so runs reached `Complete` and stayed there — and because a start against a live run is
/// `ToolRunRejection::Busy`, no tool could run a second time in a session. Measured on 6018 as a 3d
/// preview that never re-evaluated after an inspector edit
/// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
#[test]
fn every_action_arg_carrier_of_an_exact_integer_reads_the_same_identity() {
    for row in fixture()["actionArgCarriers"]["rows"].as_array().expect("carrier rows") {
        let id = text(&row["id"]);
        let args = DslValue::Object(vec![("generation".to_string(), carrier_value(row)), ("runId".to_string(), carrier_value(row))]);
        let read = semio_framework_plugin_tool_run::tool_run_arg_u64(Some(&args), "generation");
        let expected = row["reads"].as_u64();
        println!("[STATS] actionArgCarriers {id}: carrier={} read={read:?}", text(&row["carrier"]));
        assert_eq!(read, expected, "{id}: generation");
        assert_eq!(semio_framework_plugin_tool_run::tool_run_arg_u64(Some(&args), "runId"), expected, "{id}: runId — one reader, both identity arguments");
    }
    assert_eq!(semio_framework_plugin_tool_run::tool_run_arg_u64(Some(&DslValue::Object(Vec::new())), "generation"), None, "an absent argument names no identity");
    assert_eq!(semio_framework_plugin_tool_run::tool_run_arg_u64(None, "generation"), None, "an action with no arguments at all names no identity");
}

/// ⚖️ LAW: a `toolRunFinalize` whose `generation` arrives in the fixture's declared wire carrier really
/// finalizes the complete run it names — the end-to-end reading of the law above, through the same
/// `handle_action` door the shell dispatches through.
#[semio_framework_async_macros::async_test]
async fn a_finalize_in_the_wire_carrier_finalizes_the_run_it_names() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let carrier = text(&fixture["actionArgCarriers"]["finalizeCarrier"]);
    let mut app = toy_app(number(&fixture["finalize"]["units"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "the run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    let slot = app.tool_runs.slot().expect("a complete run has a slot");
    let row = serde_json::json!({ "carrier": carrier, "value": f64::from(slot.generation) });
    let arguments = vec![("runId".to_string(), DslValue::String(slot.run.to_string())), ("generation".to_string(), carrier_value(&row))];
    let output = tool_run_action(&mut app, "toolRunFinalize", arguments, &mut caller).await;
    println!("[STATS] finalize carrier={carrier} output={output:?}");
    assert_eq!(output.get("rejected").and_then(DslValue::as_str), None, "a finalize naming the run's own generation is never stale, whatever carrier the number arrived in");
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"), "it begins the finalize");
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Finalizing), "and the run leaves complete");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🔢️ActionArgCarriers

/// ⚖️ LAW: a READ-ONLY run's completion is its finalization — the driver turn that completes its job
/// leaves `complete` on its own, and the tool can be started again without any finalize action ever
/// being dispatched.
///
/// 🪪️ `complete` means "the job is done and the run awaits the finalize that publishes its provisional
/// edits". A `mutating: false` run authors none, so there is nothing to publish and nothing to review;
/// parking it in `complete` only holds the tool's single run slot, and a `toolRunStart` against a
/// non-terminal run is `toolRun.busy`. Generation3d's read-only `previewEval` therefore ran exactly ONCE
/// per session and its 3d preview stopped re-evaluating after the first evaluation
/// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
///
/// ▶️ And the slot is free: the next start is admitted, with no finalize action anywhere in this test.
#[semio_framework_async_macros::async_test]
async fn a_read_only_run_finalizes_itself_and_frees_its_slot_for_the_next_start() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["readOnlyRun"];
    let tool_id = text(&expected["toolId"]);
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    start(&mut app, tool_id, &mut caller).await;
    pump_until(&mut app, "the read-only run leaves complete on its own", |app| app.tool_runs.state().is_some_and(|state| state != ToolRunState::Running && state != ToolRunState::Starting && state != ToolRunState::Complete), &mut caller).await;
    let state = app.tool_runs.state().expect("a state");
    println!("[STATS] readOnlyRun state={} after its job completed, with no finalize action dispatched", state.as_str());
    assert_ne!(state, ToolRunState::Complete, "a read-only run never parks in complete waiting for a finalize nobody owes it");
    pump_until(&mut app, "the read-only run settles", |app| app.tool_runs.state().is_some_and(ToolRunState::is_terminal) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let output = tool_run_action(&mut app, "toolRunStart", vec![("toolId".into(), DslValue::String(tool_id.into()))], &mut caller).await;
    assert_eq!(output.get("rejected").and_then(DslValue::as_str), None, "the finished read-only run never refuses the next start as busy");
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "and the next start really spawns a job");
    assert_eq!(expected["startsAgainWithoutAnyFinalizeAction"].as_bool(), Some(true));
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn original_preview_updates_the_mounted_ledger_before_ack_and_the_next_producer_step(){
 let law:Value=serde_json::from_str(include_str!("../../🔬️tool-run/📬️native-ledger/🔣️.json")).unwrap();let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().unwrap();let mut context=original_caller.context();let cx=&mut context;
 let(policy,maximum)=toy_fixture_caller_policy();let mut observer=|_|true;let identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum,&mut observer).unwrap();let mut caller=ToyFixtureCaller{context:cx,policy,identity,receipts:Vec::new()};let mut app=toy_app(1,&mut caller).await;start(&mut app,law["toolId"].as_str().unwrap(),&mut caller).await;
 pump_until(&mut app,"original preview reaches the actual immutable mounted ledger",|app|app.tool_runs.original_view_for(None).is_some_and(|view|view.progress.completed==law["completed"].as_u64().unwrap()),&mut caller).await;
 let view=app.tool_runs.original_view_for(None).unwrap();assert_eq!(view.progress.counters.len(),law["counters"].as_array().unwrap().len());for(counter,expected)in view.progress.counters.iter().zip(law["counters"].as_array().unwrap()){assert_eq!(counter.value,expected.as_u64().unwrap());}assert_eq!(view.payload.unwrap().read_fixed::<4>(0).unwrap(),[17,23,41,67]);let job=app.tool_runs.entries.last().unwrap().job.as_ref().unwrap();let descriptor=job.outcome.as_ref().unwrap();assert_eq!(descriptor.kind(),semio_framework_job::JobOutcomeKind::PreviewReady);assert!(!descriptor.is_acknowledged());
 pump_until(&mut app,"original loan acknowledges and the actual next producer step completes",|app|app.tool_runs.state()==Some(ToolRunState::Finalized)&&!app.tool_runs.has_pending_work(),&mut caller).await;
 let view=app.tool_runs.original_view_for(None).unwrap();assert_eq!(view.progress.completed,8760);assert_eq!(view.payload.unwrap().read_fixed::<4>(0).unwrap(),[17,23,41,67]);close(&mut app,&mut caller);drop(app);let fuel=cx.fuel_remaining();drop(context);original_caller.retire(fuel).unwrap();
}

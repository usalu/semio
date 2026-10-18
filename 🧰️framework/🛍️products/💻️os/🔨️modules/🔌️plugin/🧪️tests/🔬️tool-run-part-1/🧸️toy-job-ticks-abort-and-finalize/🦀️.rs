/// 🧸️ Toy job, ticks, abort and finalize: the toy run job, the overlay ticks over an unchanged document, abort, partial and grouped finalize, and the one transaction label per locale.

#[test]
fn original_toy_run_job_closes_same_writer_counts_under_fixed_full_grant(){
    use semio_framework_job::InteractiveJob;
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
    let source=toy_fixture_caller_policy().0.preparation;for count in[0,1,8192]{let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(99110),semio_framework_job::Generation(1),source,semio_framework_job::default_now_us,u64::MAX,1);let(job,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let identity=ToolRunIdentity::new(ToolRunId{app_instance_id:7,run:11},[17;32]);let mut writer=ToolRunTickWriter::new(identity);writer.payload(vec![b'x';8192]);ToyRunJob{unit:OriginalToyUnit::new(),emission:OriginalToyEmission::new(),phase:0,batch_units:0,scan:0,retraction:None,progress_source:None,counter_source:None,trace_source:None,payload_source:[0;4],payload_source_ready:false,payload_position:0,successor:None,writer_retirement:None,progress_retirement:None,purpose:ToolRunJobPurpose::Run,original:std::mem::ManuallyDrop::new(ToyRunJobOriginal{writer,provisional_counts:(0..count).collect()}),original_live:true,original_close:None,base_count:0,target:0,done:0,resumed_from:0,checkpoint_due:false,finished:false,closing:false}});let mut job=job;let mut born=heap.requested_bytes;let mut released=heap.released_bytes;let pointer=job.original.provisional_counts.as_ptr();assert_eq!(serde_json::to_value(&job.original.provisional_counts).unwrap(),serde_json::json!((0..count).collect::<Vec<_>>()));job.begin_close();let mut turns=0;
     while !job.terminal_is_empty(){let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.original_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];if demand.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}if demand.capacity_bytes>0{denied.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant});}if demand.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}for grant in denied{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|job.close_step(cx)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if job.original_live{assert_eq!(job.original.provisional_counts.as_ptr(),pointer);}}
      let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|job.close_step(cx)).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<100000);
     }assert_eq!(born,released);let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));caller.retire(source);
    }
}

async fn render_text(app: &mut ToyApp, body_key: &str, caller:&mut ToyFixtureCaller<'_,'_,'_>) -> String {
    let view=ViewModel::new(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native);
    for _ in 0..semio_framework_plugin_testing_caller::OriginalFixtureCaller::maximum_turns(){if let Some(tree)=app.render(body_key,None,&view,caller.context).await.unwrap_or_else(|fault|panic!("render {body_key}: {fault:?}")){return artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap_or_else(|error|panic!("project {body_key}: {error}"))}}
    panic!("render {body_key} did not settle under the original fixture caller")
}

/// 🆔️ A fixture panel id template scoped to run `run` (`{run}` substituted).
fn panel_id(template: &Value, run: u64) -> String {
    text(template).replace("{run}", &run.to_string())
}

#[doc(hidden)]
pub fn find_node<'a>(node: &'a Value, key: &str) -> Option<&'a Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| find_node(child, key))
}

async fn attach_probe(app: &mut ToyApp, channel: &str) -> MemoryBackbone {
    let (backbone, probe) = MemoryBackbone::pair(channel, channel).await;
    app.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.expect("attach probe backbone");
    probe
}

/// 📮️ Drains what the app sent: `(mutations batches, genesis announcements)`; a member lane's batch is
/// a mutations batch of that document, and acks are not document traffic.
#[doc(hidden)]
pub async fn drain(probe: &mut MemoryBackbone) -> (usize, usize) {
    probe.receive().await.expect("probe receive").into_iter().fold((0, 0), |(mutations, genesis), message| match message {
        BackboneMessage::Mutations { .. } | BackboneMessage::Member { .. } => (mutations + 1, genesis),
        BackboneMessage::Genesis { .. } => (mutations, genesis + 1),
        BackboneMessage::Ack { .. } | BackboneMessage::Retract { .. } => (mutations, genesis),
    })
}

#[semio_framework_async_macros::async_test]
async fn tool_run_ticks_render_the_overlay_while_the_committed_document_stays_untouched() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["overlay"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    let generation = app.stores.store.generation();
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "overlay run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    assert_eq!(app.stores.store.generation(), generation, "ticks never touch the store generation");
    assert_eq!(app.snapshot().expect("committed snapshot").count, number(&expected["committedCount"]) as i32);
    assert_eq!(app.tool_runs.provisional().len() as u64, number(&expected["provisionalOps"]));
    let entities: Vec<u64> = expected["entities"].as_array().expect("entities").iter().map(number).collect();
    assert_eq!(app.tool_runs.provisional_entities().expect("entities").iter().copied().collect::<Vec<_>>(), entities);
    assert_eq!(app.tool_runs.trace().expect("trace").len() as u64, number(&expected["traceRecords"]));
    let body = render_text(&mut app, "main", &mut caller).await;
    assert!(body.contains(text(&expected["mainBodyText"])), "the renderer reads committed ⊕ provisional: {body}");
    let panel_expected = &fixture["panel"];
    let run = app.tool_runs.slot().expect("slot").run;
    let panel: Value = serde_json::from_str(&render_text(&mut app, text(&panel_expected["bodyKey"]), &mut caller).await).expect("panel projection parses");
    assert_eq!(panel["key"], panel_expected["rootId"]);
    let status = find_node(&panel, &panel_id(&panel_expected["status"]["id"], run)).expect("status line");
    assert_eq!(status["accessibility"]["live"], panel_expected["status"]["live"]);
    let bar = find_node(&panel, &panel_id(&panel_expected["progress"]["id"], run)).expect("progress bar");
    assert_eq!(bar["component"]["type"], panel_expected["progress"]["type"]);
    assert_eq!(bar["component"]["completed"].as_f64(), Some(number(&panel_expected["progress"]["completed"]) as f64));
    assert_eq!(bar["component"]["total"].as_f64(), Some(number(&panel_expected["progress"]["total"]) as f64));
    let finalize = find_node(&panel, &panel_id(&panel_expected["finalize"]["id"], run)).expect("finalize button");
    assert_eq!(finalize["component"]["type"], "button");
    assert_eq!(finalize["accessibility"]["shortcut"], panel_expected["finalize"]["shortcut"]);
    assert!(finalize["bindings"].as_array().is_some_and(|bindings| !bindings.is_empty()), "finalize is a real bound button");
    let presence = app.tool_run_presence().expect("presence summary");
    assert_eq!((presence.completed, presence.total), (number(&expected["units"]), Some(number(&expected["units"]))));
    assert!(app.tool_run_trace_delta(None).is_some_and(|delta| !delta.is_empty() && !delta.contains(['+', '/', '='])), "trace delta is base64url");
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted), &mut caller).await;
    assert!(render_text(&mut app, "main", &mut caller).await.contains(text(&expected["committedBodyText"])), "an aborted run renders the committed document again");
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_abort_leaves_store_generation_edits_command_log_and_outbox_untouched() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["abort"];
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    let mut probe = attach_probe(&mut app, "tool-run-abort").await;
    drain(&mut probe).await;
    app.render_ctx().refresh_cache().await.expect("backfill the command log before the invariant capture");
    let (generation, edits, commands) = (app.stores.store.generation(), app.stores.store.envelope().vcs.edits.len(), app.history.command_log.len());
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "first provisional unit", |app| app.tool_runs.provisional().len() as u64 >= number(&expected["unitsBeforeAbort"]) * number(&fixture["opsPerUnit"]), &mut caller).await;
    let output = run_action(&mut app, "toolRunAbort", &mut caller).await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("closeJob"));
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    assert_eq!(app.tool_runs.state().map(ToolRunState::as_str), Some(text(&expected["state"])));
    assert_eq!(app.stores.store.generation(), generation, "abort: store generation");
    assert_eq!(app.stores.store.envelope().vcs.edits.len(), edits, "abort: envelope.vcs.edits length");
    assert_eq!(app.history.command_log.len(), commands, "abort: command log");
    assert_eq!(drain(&mut probe).await, (0, 0), "abort: memory-backbone outbox");
    assert!(app.tool_runs.provisional().is_empty(), "abort retires every provisional op");
    assert_eq!(app.snapshot().expect("committed").count, 0);
    drop(probe);
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a run that has not completed finalizes its PARTIAL result — the run job stops at its tick boundary and the one
/// grouped edit carries exactly the provisional ops computed until the finalize, never an op the job would have computed
/// after it; the run ends `finalized` and the document holds exactly that many units.
#[semio_framework_async_macros::async_test]
async fn tool_run_finalize_while_running_publishes_exactly_the_partial_result_computed_so_far() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["partialFinalize"];
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    let ops_per_unit = number(&fixture["opsPerUnit"]);
    pump_until(&mut app, "the partial result", |app| app.tool_runs.provisional().len() as u64 >= number(&expected["unitsBeforeFinalize"]) * ops_per_unit, &mut caller).await;
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running), "the run is still computing");
    let computed = app.tool_runs.provisional().len();
    let edits = app.stores.store.envelope().vcs.edits.len();
    let output = run_action(&mut app, "toolRunFinalize", &mut caller).await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    pump_until(&mut app, "partial finalize publishes", |app| app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work(), &mut caller).await;
    assert_eq!((app.stores.store.envelope().vcs.edits.len() - edits) as u64, number(&expected["editsAdded"]));
    let edit = app.stores.store.envelope().vcs.edits.last().expect("finalized edit");
    assert_eq!(edit.forwards.len(), computed, "the edit carries exactly the ops computed until the finalize");
    assert!(edit.mutation_meta.iter().all(|meta| meta.group_id.as_deref() == Some(text(&expected["groupId"]))));
    assert_eq!(app.snapshot().expect("committed").count as u64, computed as u64 / ops_per_unit, "the document holds exactly the computed units");
    assert!(app.tool_runs.provisional().is_empty());
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_finalize_publishes_one_grouped_edit_one_mutations_batch_and_undo_removes_all() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["finalize"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    let mut probe = attach_probe(&mut app, "tool-run-finalize").await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    drain(&mut probe).await;
    let (generation, edits, commands) = (app.stores.store.generation(), app.stores.store.envelope().vcs.edits.len(), app.history.command_log.len());
    let output = run_action(&mut app, "toolRunFinalize", &mut caller).await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    pump_until(&mut app, "finalize publishes", |app| app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work(), &mut caller).await;
    assert_eq!(app.stores.store.generation() - generation, number(&expected["storeGenerationAdded"]));
    assert_eq!((app.stores.store.envelope().vcs.edits.len() - edits) as u64, number(&expected["editsAdded"]));
    assert_eq!((app.history.command_log.len() - commands) as u64, number(&expected["commandRowsAdded"]));
    let edit = app.stores.store.envelope().vcs.edits.last().expect("finalized edit");
    assert_eq!(edit.forwards.len(), number(&fixture["overlay"]["provisionalOps"]) as usize, "every provisional op lands in the one edit");
    assert!(edit.mutation_meta.iter().all(|meta| meta.group_id.as_deref() == Some(text(&expected["groupId"]))), "the edit is stamped with the run's group id");
    assert_eq!(drain(&mut probe).await, (number(&expected["mutationsMessages"]) as usize, number(&expected["genesisMessages"]) as usize), "one Mutations batch, no genesis re-announcement");
    let committed = app.snapshot().expect("committed after finalize");
    assert_eq!((committed.count, committed.label.as_str()), (number(&expected["countAfterFinalize"]) as i32, text(&expected["labelAfterFinalize"])));
    assert!(app.tool_runs.provisional().is_empty());
    app.stores.store.dispatch(ArtifactCommand::Undo, &mut caller.identity).await.expect("undo the finalized run");
    let undone = app.snapshot().expect("committed after undo");
    assert_eq!((undone.count, undone.label.as_str()), (number(&expected["countAfterUndo"]) as i32, text(&expected["labelAfterUndo"])), "one undo removes the whole run");
    drop(probe);
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a finalized run is ONE tool transaction — every op of its one edit carries the same `TransactionRef`
/// (`tx-<hex16>`, tool `<appId>#<toolId>`), the edit keeps no single-locale description, and its history row is the
/// tool's own declared label in every locale.
#[semio_framework_async_macros::async_test]
async fn tool_run_finalize_is_one_transaction_labelled_by_its_tool_in_every_locale() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let mut app = toy_app(number(&fixture["finalize"]["units"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    run_action(&mut app, "toolRunFinalize", &mut caller).await;
    pump_until(&mut app, "finalize publishes", |app| app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let edit = app.stores.store.envelope().vcs.edits.last().expect("finalized edit");
    let edit_id = edit.id.clone();
    let transaction = edit.mutation_meta.first().and_then(|meta| meta.transaction.clone()).expect("the finalized edit is a tool transaction");
    assert!(edit.mutation_meta.iter().all(|meta| meta.transaction.as_ref() == Some(&transaction)), "every op of the run carries the one ref");
    assert_eq!(transaction.tool, format!("{}#{}", ToyRunApp::APP_ID, text(&fixture["toolId"])));
    assert!(transaction.id.starts_with("tx-") && transaction.id.len() == "tx-".len() + 16, "a minted transaction id: {}", transaction.id);
    let history = app.render_ref().build_history_view(None).await;
    let row = history.commands.iter().find(|row| row.edit_id.as_deref() == Some(edit_id.as_str())).expect("the run's history row");
    assert_eq!(row.transaction.as_ref(), Some(&transaction));
    assert_eq!((row.label.resolve(Terminology::Native, Locale::En).to_string(), row.label.resolve(Terminology::Native, Locale::De).to_string()), ("Toy fill".to_string(), "Spielfüllung".to_string()));
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

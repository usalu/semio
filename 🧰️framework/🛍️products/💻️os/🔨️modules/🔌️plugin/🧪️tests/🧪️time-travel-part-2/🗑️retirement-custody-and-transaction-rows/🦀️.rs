/// 🗑️ Retirement custody and transaction rows: the original inline custody of sessions, pending commands and discards, the overwrite head, the finalize dialog, tool transaction rows and the catalogue route.

#[test]
fn time_travel_actor_session_retirement_original_inline_custody() {
    time_travel::assert_session_original::<ToyHistoryApp>();
}

#[test]
fn time_travel_actor_pending_command_original_inline_custody(){
    time_travel::assert_pending_command_original::<ToyHistoryApp>();
}

#[semio_framework_async_macros::async_test]
async fn time_travel_actor_discard_original_prepares_effects_before_atomic_publication(){
    let mut original_caller=semio_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

 let (_,maximum_identity_bytes)=original_time_travel_fixture_caller();let grant=time_travel::native_original_actor_grant();let mounted_policy=semio_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:grant,maintenance:grant,close:grant}.validate().unwrap();
 let mut observer=|_|true;
 let mut identity=semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
 let mut app=artifact_app_laws::new_registered_app::<ToyHistoryApp,_>(toy_manifest(),protocol::ActorId("original-discard".into()),mounted_policy,&mut identity).await;
 assert_discard_publication_original(&mut app,||SetLabel{value:String::with_capacity(65536)}.into());
 collect_original_history_command_receipt(&mut app);close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: an overwrite leaves exactly the document a fresh store folding the edited log reaches.
#[semio_framework_async_macros::async_test]
async fn an_overwrite_head_equals_a_fresh_fold_of_the_edited_log() {
    let mut original_caller=semio_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let scenario = fixture["scenarios"].as_array().expect("scenarios").iter().find(|scenario| text(&scenario["id"]) == "overwrite-supersedes-in-place").expect("overwrite scenario");
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in scenario["steps"].as_array().expect("steps") {
        run_step(&mut app, &fixture, step, mounted_policy, &mut identity,cx).await;
    }
    let mut fresh = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into()), mounted_policy, &mut identity).await;
    for (index, op) in fixture["seed"].as_array().expect("seed").iter().enumerate() {
        let op = if index == 1 { SetLabel { value: "b".into() }.into() } else { seed_op(op) };
        fresh.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![op], transaction: None }, &mut identity).await.expect("fresh edit");
    }
    assert_eq!(app.stores.store.snapshot().expect("edited head"), fresh.stores.store.snapshot().expect("fresh head"));
    close(&mut app,cx);
    close(&mut fresh,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: finalizing opens the framework-injected dialog seeded with the localized default name; the injected
/// definition names the commit and back verbs, and a viewer manifest declares neither the verbs nor the dialog.
#[semio_framework_async_macros::async_test]
async fn finalize_opens_the_injected_dialog_with_a_localized_default_name() {
    let mut original_caller=semio_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let result = verb(&mut app, &fixture, "historyEditFinalize", Vec::new(), mounted_policy, &mut identity).await;
    let Some(Effect::OpenDialog { dialog_id, args, .. }) = result.requested_effects.first() else { panic!("finalize opens a dialog: {:?}", result.requested_effects) };
    assert_eq!(dialog_id, semio_framework::history_edit::HISTORY_EDIT_FINALIZE_DIALOG_ID);
    assert_eq!(args.as_ref().and_then(|args| args.get("name")).and_then(DslValue::as_str), Some("Edited history"), "the default name is localized, never a literal");
    let manifest = toy_manifest().await;
    let definition = &manifest.definition;
    assert!(definition.dialogs.iter().any(|dialog| dialog.id == semio_framework::history_edit::HISTORY_EDIT_FINALIZE_DIALOG_ID), "every editor declares the finalize dialog");
    assert!(semio_framework::history_edit::HISTORY_EDIT_ACTION_IDS.iter().all(|id| definition.actions.iter().any(|action| action.id == *id)), "every editor declares the history-edit verbs");
    assert!(result.history_patch.as_ref().and_then(|patch| patch.time_travel.as_ref()).is_some_and(|status| status.stage == semio_framework::kernel::HistoryTimeTravelStage::Choosing), "the verb result carries the choosing status");
    verb(&mut app, &fixture, "historyEditBack", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Reviewing, "cancel goes back to reviewing, never discards");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: one committed tool transaction is one edit and one history row carrying its `TransactionRef`, labelled from
/// its mutations when it has no description, with one editable mutation row per op.
#[semio_framework_async_macros::async_test]
async fn a_committed_tool_transaction_is_one_row_with_its_reference_and_mutation_rows() {
    let mut original_caller=semio_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let edits = app.stores.store.envelope().vcs.edits.len();
    let transaction = protocol::TransactionRef { id: "tx-0123456789abcdef".into(), tool: format!("{}#select", ToyHistoryApp::APP_ID) };
    let emit = Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::commit_transaction(transaction.clone(), vec![SetCount { value: 2 }.into(), SetLabel { value: "t".into() }.into()]);
    app.dispatch_emit("select", emit, &meta(&fixture), &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the transaction publishes");
    assert_eq!(app.stores.store.envelope().vcs.edits.len(), edits + 1, "one transaction is one edit");
    let patch = app.render_ctx().history_patch(true).await.expect("history patch");
    let row = patch.upserts.iter().find(|entry| entry.transaction.is_some()).expect("the transaction row");
    assert_eq!(row.transaction.as_ref().map(|reference| (reference.id.as_str(), reference.tool.as_str())), Some((transaction.id.as_str(), transaction.tool.as_str())));
    assert_eq!(row.mutations.len(), 2, "one mutation row per op");
    assert!(row.mutations.iter().all(|mutation| mutation.editable), "both leaves declare an input schema");
    assert_eq!(row.label.resolve(Terminology::Native, Locale::En), "Set count to 2 (+1)", "the row reads its mutations' labels");
    assert_eq!(row.key(), format!("edit:{}", row.edit_id.as_deref().expect("edit id")));
    assert!(!row.op_lines.is_empty(), "the wire row keeps its printed operations");
    let history = render_history(&mut app, Locale::En).await;
    let mut rendered = find_node(&history, &format!("framework.history.entry.{}", row.seq)).unwrap_or_else(|| panic!("the transaction row: {history}")).clone();
    rendered.as_object_mut().expect("a node").remove("children");
    assert!(row.op_lines.iter().all(|line| !rendered.to_string().contains(line.as_str())), "labelled mutation children replace the raw op-lines description: {rendered}");
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §19.1): a tool that declares intent leaf kinds labels its transaction's row by the first operation of
/// such a kind — a support leaf before it counts only in `(+N)` — on the authoring replica and after a text and a pack
/// reload (the stamped tool id and the leaf kinds are persisted); a tool declaring none keeps the first leaf's label.
#[semio_framework_async_macros::async_test]
async fn a_transaction_row_reads_its_declared_intent_leaf_before_and_after_reload() {
    let mut original_caller=semio_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let support_then_intent = || -> Vec<TestMutation> { vec![SetCount { value: 2 }.into(), SetLabel { value: "t".into() }.into()] };
    assert_eq!(support_then_intent().iter().map(|op| protocol::SemanticMutation::<TestSnapshot>::semantics(op).kind).collect::<Vec<_>>(), ["set-count", "set-label"]);
    let gesture = protocol::TransactionRef { id: "tx-00000000000000c1".into(), tool: format!("{}#gesture", ToyHistoryApp::APP_ID) };
    let select = protocol::TransactionRef { id: "tx-00000000000000c2".into(), tool: format!("{}#select", ToyHistoryApp::APP_ID) };
    for transaction in [&gesture, &select] {
        app.dispatch_emit("select", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::commit_transaction(transaction.clone(), support_then_intent()), &meta(&fixture), &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the transaction publishes");
    }
    let mut reloaded = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into()), mounted_policy, &mut identity).await;
    artifact_app_laws::load_document_text(&mut reloaded, &app.document_text().await.expect("document text"), &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("text reload");
    reloaded.render_ctx().refresh_cache().await.expect("the reloaded log backfills");
    let mut repacked = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into()), mounted_policy, &mut identity).await;
    artifact_app_laws::load_document(&mut repacked, &app.document_pack().await.expect("document pack"), &mut identity).await.expect("pack reload");
    repacked.render_ctx().refresh_cache().await.expect("the repacked log backfills");
    for (who, app) in [("authored", &mut app), ("text reload", &mut reloaded), ("pack reload", &mut repacked)] {
        let patch = app.render_ctx().history_patch(true).await.expect("history patch");
        let label = |transaction: &protocol::TransactionRef| {
            let row = patch.upserts.iter().find(|entry| entry.transaction.as_ref().is_some_and(|reference| reference.id == transaction.id)).unwrap_or_else(|| panic!("{who}: the row of {}", transaction.id));
            (row.label.resolve(Terminology::Native, Locale::En).to_string(), row.label.resolve(Terminology::Native, Locale::De).to_string())
        };
        assert_eq!(label(&gesture), ("Set label to t (+1)".to_string(), "Beschriftung auf t setzen (+1)".to_string()), "{who}: the declared intent leaf labels the row");
        assert_eq!(label(&select), ("Set count to 2 (+1)".to_string(), "Anzahl auf 2 setzen (+1)".to_string()), "{who}: an undeclared tool keeps the first leaf");
    }
    for app in [&mut app, &mut reloaded, &mut repacked] {
        close(app,cx);
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🏷️ LAW (design §20.6): the runtime's catalogue route (`setActiveExample` of an app that declares none) carries no
/// hand-written description — its verb reads the framework's own label in every locale, any row it records reads that
/// label — and its emit still loads the example.
#[semio_framework_async_macros::async_test]
async fn the_catalogue_route_reads_the_framework_label_never_a_hand_written_one() {
    let mut original_caller=semio_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let label = app.tools.verb_label(CATALOGUE_EXAMPLE_ACTION_ID).expect("the framework labels its runtime verb");
    assert_eq!((label.resolve(Terminology::Native, Locale::En), label.resolve(Terminology::Native, Locale::De)), ("Load example", "Beispiel laden"));
    let result = app.dispatch_action(CATALOGUE_EXAMPLE_ACTION_ID, Some(&dsl(&serde_json::json!({ "exampleId": "five" }))), &meta(&fixture), &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the catalogue route loads the example");
    assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })), "the example is loaded: {:?}", result.requested_effects);
    let patch = app.render_ctx().history_patch(true).await.expect("history patch");
    assert!(patch.upserts.iter().filter(|row| row.action_id == CATALOGUE_EXAMPLE_ACTION_ID).all(|row| row.label.resolve(Terminology::Native, Locale::En) == "Load example"), "no row reads a hand-written label");
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🪢️ Runs one retirement turn as a genuine original producer invocation and observes its exact heap traffic.
fn discard_run<R>(grant:semio_framework_value::RetainedCloneGrant,close:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->R)->(R,semio_framework_trace::HeapAllocationObservation){
 use semio_framework_value_testing_admission::{driven,BankWallet,EffectBank};
 let mut bank=EffectBank::new(BankWallet::new(semio_framework_value::RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<10},1<<20));
 let(mut close,mut output)=(Some(close),None);
 let(poll,_)=driven::<semio_framework_value::ValueError>(&mut bank,grant,1<<20,|effect,reply,_|{let close=close.take().ok_or_else(||semio_framework_value::ValueError::invariant("fixture operation runs once"))?;output=Some(semio_framework_trace::observe_heap_allocations_on_this_thread(||close(effect)));Ok(reply.complete(Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default()))))});
 assert!(matches!(poll,semio_framework_value::RetainedWorkPoll::Complete(ref turn) if turn.result.is_ok()),"fixture effect turn did not complete");
 assert!(bank.into_wallet().is_ok(),"fixture bank closed with a live original action");
 output.expect("fixture operation ran")
}

/// 🗑️ A discard publishes atomically after its store effects retire under independent grants.
fn assert_discard_publication_original<A:ArtifactApp>(app:&mut artifact_instance::VcsArtifactApp<A>,make_mutation:fn()->A::Mutation){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let policy=&law["discardPublicationGrant"];let grant=semio_framework_value::RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
 let ((session,editor,mutation,preview,input),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut id=String::with_capacity(65536);id.push_str("original/α\0😀");let target=protocol::MutationId(id);let input=protocol::InputReplacement::Input{schema:String::with_capacity(8192),payload:Vec::with_capacity(65536)};let session=semio_framework_time_travel::TimeTravelSession{id:37,generation:5,base:TimeTravelBase{content_revision:app.stores.store.content_revision_now()},stage:TimeTravelStage::Editing,pending:Some(semio_framework_time_travel::TimeTravelPending{target:TimeTravelTarget{mutation:target.clone(),position:0},original:protocol::InputReplacement::Withdrawn,replacement:protocol::InputReplacement::Withdrawn,return_stage:TimeTravelStage::Inactive}),..Default::default()};let editor=time_travel::TimeTravelEditor{target,position:0,op_index:0,label:LocalizedLabel::native("Original","Original"),schema:Some("native.v1"),inputs:vec![semio_framework::ActionArgDef { schema:ArgSchema::Array{items:Box::new(semio_framework::ArgSchema::Object{fields:vec![semio_framework::ActionArgDef::text("leaf",semio_ui_locale::LocalizedLabel::native("Leaf","Blatt"))]}),min_items:None,max_items:None},..semio_framework::ActionArgDef::text("original",semio_ui_locale::LocalizedLabel::native("Original","Ursprünglich")) }],inputs_refused:None,validator:Ok(semio_framework_schema::OwnedJsonSchemaValidator::compile("{\"type\":\"string\",\"pattern\":\"^(a|[b-d]){1,4}$\"}").unwrap()),value:DslValue::String(String::with_capacity(65536)),outcome:Vec::new(),refused:None};(session,editor,make_mutation(),app.stores.store.derived_snapshot_head(),input)});let held=heap.requested_bytes-heap.released_bytes;let editor_pointer=app.time_travel.editor.as_ref().map(|editor|editor.target.0.as_ptr());assert!(editor_pointer.is_none());app.time_travel.session=session;app.time_travel.editor=Some(editor);app.time_travel.document.adopt_draft_preview(mutation,preview,input);let pointer=app.time_travel.editor.as_ref().unwrap().target.0.as_ptr();let mut original=Some(semio_framework_time_travel::TimeTravelEvent::Discard{generation:5});let (receipt,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.time_travel.stage_time_travel_command(&mut original,grant).unwrap().unwrap());assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut births=0;let mut releases=0;let mut committed=false;let mut turns=0;
 for _ in 0..100000{
  let pending=app.time_travel.pending_command.is_some()||app.time_travel.discard_cursor.is_some()||!app.time_travel.command_retirements.terminal_is_empty();if !pending&&!app.time_travel.document.discard_effects_held(){break}
  if pending{let (pending,heap)=discard_run(semio_framework_value::RetainedCloneGrant::default(),|effect|app.time_travel_ctx().drive_original_discard_command(semio_framework_value::RetainedCloneGrant::default(),effect).unwrap());assert!(pending);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let (original,receipt)=app.time_travel.take_command_receipt().unwrap();assert_eq!(original,semio_framework_value::RetainedCloneGrant::default());assert_eq!(receipt,Default::default());if !committed{assert_eq!(app.time_travel.editor.as_ref().unwrap().target.0.as_ptr(),pointer);assert_eq!(app.time_travel.session.stage,semio_framework_time_travel::TimeTravelStage::Editing)}}
  else{
   let (quote,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.time_travel_ref().time_travel_retirement_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   for denied in [Some(semio_framework_value::RetainedCloneGrant{maximum_items:0,..grant}),Some(semio_framework_value::RetainedCloneGrant{maximum_depth:0,..grant}),(quote.copy_bytes>0).then_some(semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:0,..grant}),(quote.capacity_bytes>0).then_some(semio_framework_value::RetainedCloneGrant{maximum_capacity_bytes:0,..grant}),(quote.release_bytes>0).then_some(semio_framework_value::RetainedCloneGrant{maximum_release_bytes:0,..grant})].into_iter().flatten(){let (step,heap)=discard_run(denied,|effect|app.time_travel_ctx().time_travel_retire_step(effect).unwrap().unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(app.time_travel.document.discard_effects_held())}
  }
  let (receipt,heap)=discard_run(grant,|effect|{if app.time_travel_ctx().drive_original_discard_command(grant,effect).unwrap(){let (original,receipt)=app.time_travel.take_command_receipt().unwrap();assert_eq!(original,grant);receipt}else{app.time_travel_ctx().time_travel_retire_step(effect).unwrap().expect("original Discard residue frontier").progress()}});assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1;
  if app.time_travel.session.stage==semio_framework_time_travel::TimeTravelStage::Inactive{committed=true;assert!(app.time_travel.editor.is_none());assert!(app.time_travel.document.draft_preview_retired())}else{assert_eq!(app.time_travel.editor.as_ref().unwrap().target.0.as_ptr(),pointer)}
 }
 assert!(committed);assert!(app.time_travel.pending_command.is_none());assert!(app.time_travel.discard_cursor.is_none());assert!(app.time_travel.command_retirements.terminal_is_empty());assert!(!app.time_travel.document.discard_effects_held());assert_eq!(releases,held+births);assert!(time_travel::TimeTravelOwners::terminal_is_empty(&app.time_travel.document));
}


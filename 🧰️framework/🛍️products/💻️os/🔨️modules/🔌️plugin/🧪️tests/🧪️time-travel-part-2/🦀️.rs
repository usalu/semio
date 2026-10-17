//! ⏪️ Part 2 of 2 of the history-edit session laws of the plugin runtime (cut from `🧪️time-travel`; the fixtures several parts use are the support module).

use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_ui_contract::BuiltNode;
use semio_framework_plugin_command::lifted_command::HISTORY_ROW_MUTATION_ROWS;
#[cfg(test)]
use semio_framework_ui_locale::Locale;
#[cfg(test)]
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin_command::lifted_command::TimeTravelEditorPanel;
use semio_framework_plugin_command::lifted_command::TimeTravelPanel;
use semio_framework_plugin_window_kits_rich::tree_kit::TreeWindows;
use semio_framework_plugin_decl_app::ArtifactApp;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobFactory, ArtifactReservedJob};
use semio_framework_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, Emit, ToolCommandCatalog};
use semio_framework_ui_contract::{Buildable, Component, HasBase, HasChildren};
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
#[cfg(test)]
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1;
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_plugin_decl_views::NoDraftMutation;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_plugin_program::PluginProgram;
use semio_framework_plugin_time_travel::TimeTravelActionRefusal;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_tests_06::time_travel_support_1::{ToyApp, ToyHistoryApp, apply_long_edit, close, collect_original_history_command_receipt, commit_scenario, dsl, edit_authors, find_node, fixture, head, meta, original_time_travel_fixture_caller, pump_until, rejected, render_body, render_history, row_actions, run_step, seed_op, seeded_app, seeded_mutation, text, toy_manifest, verb};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_instance_app as artifact_instance;
use semio_framework_os_kernel::io;
use semio_framework::*;
use semio_framework::kernel::{ActorId, Effect};
use store::os_store::store_schema::ArtifactCommand;
use pack::snapshot::ArtifactPack;
use std::collections::BTreeSet;
use semio_framework_plugin_command::CATALOGUE_EXAMPLE_ACTION_ID;
use semio_framework_value::DslValue;
use semio_framework::{FaultCode, NOTE_SHELL_COMMAND_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
use std::future::Future;
#[cfg(test)]
use semio_framework_plugin_command::{HISTORY_ROWS_BUILT, history_panel_mutation_row};
#[cfg(test)]
use semio_framework_plugin_decl_views::history_view::HistoryCommandFilter;
use store::{MemberFactory, Mutation, SpaceMember};
#[cfg(test)]
use semio_framework_plugin_decl_surface::replay_envelopes_fault;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_framework_plugin_time_travel as time_travel;
#[cfg(test)]
use semio_framework_plugin_decl_app::viewer_rejects_action;
use semio_framework_ui_contract::*;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_time_travel::TimeTravelStage;
use store::os_store::backbone::{Backbone, MemoryBackbone};


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
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

 let (_,maximum_identity_bytes)=original_time_travel_fixture_caller();let grant=time_travel::native_original_actor_grant();let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:grant,maintenance:grant,close:grant}.validate().unwrap();
 let mut observer=|_|true;
 let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
 let mut app=artifact_app_laws::new_registered_app::<ToyHistoryApp,_>(toy_manifest(),protocol::ActorId("original-discard".into()),mounted_policy,&mut identity).await;
 time_travel::assert_discard_publication_original(&mut app,||SetLabel{value:String::with_capacity(65536)}.into());
 collect_original_history_command_receipt(&mut app);close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: an overwrite leaves exactly the document a fresh store folding the edited log reaches.
#[semio_framework_async_macros::async_test]
async fn an_overwrite_head_equals_a_fresh_fold_of_the_edited_log() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
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
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
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
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let edits = app.stores.store.envelope().vcs.edits.len();
    let transaction = protocol::TransactionRef { id: "tx-0123456789abcdef".into(), tool: format!("{}#select", ToyHistoryApp::APP_ID) };
    let emit = Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::commit_transaction(transaction.clone(), vec![SetCount { value: 2 }.into(), SetLabel { value: "t".into() }.into()]);
    app.dispatch_emit("select", emit, &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the transaction publishes");
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
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let support_then_intent = || -> Vec<TestMutation> { vec![SetCount { value: 2 }.into(), SetLabel { value: "t".into() }.into()] };
    assert_eq!(support_then_intent().iter().map(|op| protocol::SemanticMutation::<TestSnapshot>::semantics(op).kind).collect::<Vec<_>>(), ["set-count", "set-label"]);
    let gesture = protocol::TransactionRef { id: "tx-00000000000000c1".into(), tool: format!("{}#gesture", ToyHistoryApp::APP_ID) };
    let select = protocol::TransactionRef { id: "tx-00000000000000c2".into(), tool: format!("{}#select", ToyHistoryApp::APP_ID) };
    for transaction in [&gesture, &select] {
        app.dispatch_emit("select", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::commit_transaction(transaction.clone(), support_then_intent()), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the transaction publishes");
    }
    let mut reloaded = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into()), mounted_policy, &mut identity).await;
    artifact_app_laws::load_document_text(&mut reloaded, &app.document_text().await.expect("document text"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("text reload");
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
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let label = app.tools.verb_label(CATALOGUE_EXAMPLE_ACTION_ID).expect("the framework labels its runtime verb");
    assert_eq!((label.resolve(Terminology::Native, Locale::En), label.resolve(Terminology::Native, Locale::De)), ("Load example", "Beispiel laden"));
    let result = app.dispatch_action(CATALOGUE_EXAMPLE_ACTION_ID, Some(&dsl(&serde_json::json!({ "exampleId": "five" }))), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the catalogue route loads the example");
    assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })), "the example is loaded: {:?}", result.requested_effects);
    let patch = app.render_ctx().history_patch(true).await.expect("history patch");
    assert!(patch.upserts.iter().filter(|row| row.action_id == CATALOGUE_EXAMPLE_ACTION_ID).all(|row| row.label.resolve(Terminology::Native, Locale::En) == "Load example"), "no row reads a hand-written label");
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🧭️ The pointer and coercion helpers behind `historyEditInput`: object members and array slots are addressed by
/// RFC 6901 pointers, host values take the input's declared shape, and a pointer into a scalar is refused.
#[test]
fn input_pointers_and_host_values_take_the_declared_shape() {
    let mut value = DslValue::object([("pivot".to_string(), DslValue::object([("x".to_string(), DslValue::float(1.0))])), ("targets".to_string(), DslValue::Array(vec![DslValue::String("a".into())]))]);
    assert!(time_travel::time_travel_pointer_set(&mut value, "/pivot/x", DslValue::float(2.0)));
    assert!(time_travel::time_travel_pointer_set(&mut value, "/targets/1", DslValue::String("b".into())));
    assert!(!time_travel::time_travel_pointer_set(&mut value, "/pivot/x/deeper", DslValue::Null));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/pivot/x").and_then(DslValue::as_f64), Some(2.0));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/targets/1").and_then(DslValue::as_str), Some("b"));
    let count = semio_framework::ActionArgDef::index("/value", LocalizedLabel::native("Value", "Wert"));
    assert_eq!(time_travel::time_travel_coerce(&count, false, &DslValue::String("4".into())), Some(DslValue::uint(4)));
    assert_eq!(time_travel::time_travel_coerce(&count, false, &DslValue::String("four".into())), None);
    let flag = semio_framework::ActionArgDef::toggle("/on", LocalizedLabel::native("On", "An"));
    assert_eq!(time_travel::time_travel_coerce(&flag, false, &DslValue::String("true".into())), Some(DslValue::Bool(true)));
    let angle = semio_framework::ActionArgDef::number("/angle", LocalizedLabel::native("Angle", "Winkel"));
    let handles = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::Array { items: Box::new(semio_framework::ArgSchema::Object { fields: vec![angle] }), min_items: None, max_items: None },
        ..semio_framework::ActionArgDef::text("/handles", LocalizedLabel::native("Handles", "Griffe"))
    };
    let inputs = vec![
        semio_framework::ActionArgDef::object("/pivot", LocalizedLabel::native("Pivot", "Drehpunkt"), vec![semio_framework::ActionArgDef::number("/x", LocalizedLabel::native("X", "X"))]),
        semio_framework::ActionArgDef::vector("/offset", LocalizedLabel::native("Offset", "Versatz"), 2),
        handles,
    ];
    let payload = dsl(&serde_json::json!({ "pivot": { "x": 1.0 }, "offset": [0.0, 0.0], "handles": [{ "angle": 0.0 }, { "angle": 90.0 }] }));
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/pivot/x").is_some_and(|(input, element)| input.id == "/x" && !element));
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/offset/1").is_some_and(|(input, element)| input.id == "/offset" && element));
    assert!(
        time_travel::time_travel_input_at(&inputs, &payload, "/handles/1/angle").is_some_and(|(input, element)| input.id == "/angle" && !element && matches!(input.schema, semio_framework::ArgSchema::Number { .. })),
        "a field inside an array of objects is addressable"
    );
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/handles/1").is_some_and(|(input, element)| !element && matches!(input.schema, semio_framework::ArgSchema::Object { .. })), "an item takes the array's item schema");
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/handles/x/angle").is_none() && time_travel::time_travel_input_at(&inputs, &payload, "/missing").is_none());
    let rows = time_travel::time_travel_input_rows(&inputs, &payload);
    assert_eq!(
        rows.iter().map(|row| row.pointer.as_str()).collect::<Vec<_>>(),
        ["/pivot/x", "/offset", "/handles", "/handles/0", "/handles/0/angle", "/handles/1", "/handles/1/angle"],
        "objects flatten into field rows; a list is its own row, then every item's row and fields"
    );
    assert_eq!(
        (rows[2].list, rows[2].item, rows[3].list, rows[3].item, rows[4].item),
        (Some(time_travel::TimeTravelList { len: 2, addable: true, max: None }), None, None, Some(time_travel::TimeTravelListItem { index: 0, removable: true, min: None }), None)
    );
    assert_eq!((rows[6].label.resolve(Terminology::Native, Locale::En), rows[6].label.resolve(Terminology::Native, Locale::De), rows[6].value.as_f64()), ("Handles 2 \u{b7} Angle", "Griffe 2 \u{b7} Winkel", Some(90.0)));
}

/// ✂️ LAW (design §6, gap N2): a list input of any item kind is reachable item by item — its own row adds an item until
/// `maxItems`, every item's row removes it while more than `minItems` remain — and the list edits of `historyEditInput`
/// insert before an index or after the last item (`-`) and remove by index; a new item starts from its schema default.
#[test]
fn list_inputs_add_and_remove_items_within_their_bounds() {
    let point = semio_framework::ArgSchema::Object { fields: vec![semio_framework::ActionArgDef::number("/x", LocalizedLabel::native("X", "X")).required(), semio_framework::ActionArgDef::text("/tag", LocalizedLabel::native("Tag", "Markierung"))] };
    let points = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::Array { items: Box::new(point.clone()), min_items: Some(1), max_items: Some(2) },
        ..semio_framework::ActionArgDef::text("/points", LocalizedLabel::native("Points", "Punkte"))
    };
    let weights = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::Array { items: Box::new(semio_framework::ArgSchema::number(Some(0.0), Some(1.0), None, false)), min_items: None, max_items: None },
        ..semio_framework::ActionArgDef::text("/weights", LocalizedLabel::native("Weights", "Gewichte"))
    };
    let inputs = vec![points, weights];
    let mut value = dsl(&serde_json::json!({ "points": [{ "x": 1.0 }, { "x": 2.0 }], "weights": [0.5] }));
    let rows = time_travel::time_travel_input_rows(&inputs, &value);
    assert_eq!(rows.iter().map(|row| row.pointer.as_str()).collect::<Vec<_>>(), ["/points", "/points/0", "/points/0/x", "/points/0/tag", "/points/1", "/points/1/x", "/points/1/tag", "/weights", "/weights/0"]);
    assert_eq!(rows[0].list, Some(time_travel::TimeTravelList { len: 2, addable: false, max: Some(2) }), "two points fill maxItems");
    assert_eq!((rows[1].item, rows[4].item), (Some(time_travel::TimeTravelListItem { index: 0, removable: true, min: Some(1) }), Some(time_travel::TimeTravelListItem { index: 1, removable: true, min: Some(1) })));
    assert!(matches!(rows[8].input.schema, semio_framework::ArgSchema::Number { .. }) && rows[8].item == Some(time_travel::TimeTravelListItem { index: 0, removable: true, min: None }), "a number item is its own row: {:?}", rows[8]);
    assert!(time_travel::time_travel_pointer_insert(&mut value, "/weights/-", DslValue::float(0.25)));
    assert!(time_travel::time_travel_pointer_insert(&mut value, "/weights/0", DslValue::float(0.75)));
    assert!(time_travel::time_travel_pointer_remove(&mut value, "/points/0"));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/weights").and_then(DslValue::as_array).map(|items| items.iter().filter_map(DslValue::as_f64).collect::<Vec<_>>()), Some(vec![0.75, 0.5, 0.25]));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/points/0/x").and_then(DslValue::as_f64), Some(2.0));
    assert!(
        !time_travel::time_travel_pointer_insert(&mut value, "/weights/9", DslValue::Null)
            && !time_travel::time_travel_pointer_remove(&mut value, "/weights/3")
            && !time_travel::time_travel_pointer_remove(&mut value, "/missing/0")
            && !time_travel::time_travel_pointer_insert(&mut value, "/points/0/x/-", DslValue::Null)
    );
    let rows = time_travel::time_travel_input_rows(&inputs, &value);
    assert_eq!(
        (rows[0].list, rows[1].item),
        (Some(time_travel::TimeTravelList { len: 1, addable: true, max: Some(2) }), Some(time_travel::TimeTravelListItem { index: 0, removable: false, min: Some(1) })),
        "one point left: addable, not removable below minItems"
    );
    let item = time_travel::time_travel_default_value(&point);
    assert_eq!((time_travel::time_travel_pointer_get(&item, "/x").and_then(DslValue::as_f64), time_travel::time_travel_pointer_get(&item, "/tag")), (Some(0.0), None), "a new point holds its required fields only");
    let mut floor = semio_framework::ArgSchema::number(Some(0.0), Some(4.0), Some(0.5), false);
    if let semio_framework::ArgSchema::Number { min_exclusive, .. } = &mut floor {
        *min_exclusive = true;
    }
    assert_eq!(time_travel::time_travel_default_value(&floor).as_f64(), Some(0.5), "an excluded floor is stepped over");
    let limits = [(8, true, Locale::En), (8, true, Locale::De), (1, false, Locale::En), (1, false, Locale::De), (1, true, Locale::En), (3, false, Locale::De)].map(|(limit, max, locale)| time_travel::time_travel_item_limit_text(limit, max, locale));
    assert_eq!(limits, ["Maximum 8 items", "Höchstens 8 Einträge", "Minimum 1 item", "Mindestens 1 Eintrag", "Maximum 1 item", "Mindestens 3 Einträge"].map(str::to_string), "a disabled list edit names its bound");
}

/// 🔀️ A union payload's inputs resolve against the variant the draft is in: the reader groups every variant's fields by
/// the variant's value (a glTF `create-node` declares `/value` once per variant), the editor shows the variant selector
/// and the active variant's fields only (a hidden restore diff never), switching the selector drops the members only
/// other variants declare, and layout `group`s of a payload that is no union hide nothing.
#[test]
fn union_inputs_resolve_against_the_active_variant() {
    let option = |value: &str, en: &str, de: &str| semio_framework::ActionArgOption { value: value.into(), label: LocalizedLabel::native(en, de) };
    let phase = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::String { options: vec![option("apply", "Apply", "Anwenden"), option("restore", "Restore", "Wiederherstellen")], option_source: None, min_len: None, max_len: None, pattern: None, format: None },
        required: true,
        ..semio_framework::ActionArgDef::text("/phase", LocalizedLabel::native("Mutation phase", "Mutationsphase"))
    };
    let parameters = semio_framework::ActionArgDef {
        group: Some("apply".into()),
        ..semio_framework::ActionArgDef::object("/value", LocalizedLabel::native("Parameters", "Parameter"), vec![semio_framework::ActionArgDef::index("/position", LocalizedLabel::native("Insert position", "Einfügeposition"))])
    };
    let weight = semio_framework::ActionArgDef { group: Some("apply".into()), ..semio_framework::ActionArgDef::number("/weight", LocalizedLabel::native("Weight", "Gewicht")) };
    let diff = semio_framework::ActionArgDef {
        group: Some("restore".into()),
        presentation: Some(semio_framework::ArgPresentation::Hidden),
        schema: semio_framework::ArgSchema::Any,
        ..semio_framework::ActionArgDef::text("/value", LocalizedLabel::native("Restore diff", "Wiederherstellungs-Diff"))
    };
    let union = vec![phase, parameters, weight, diff];
    assert_eq!(time_travel::time_travel_variant_selector(&union).map(|selector| selector.id.as_str()), Some("/phase"));
    let applying = dsl(&serde_json::json!({ "phase": "apply", "value": { "position": 2 }, "weight": 1.0 }));
    let restoring = dsl(&serde_json::json!({ "phase": "restore", "value": { "nodes": [] } }));
    assert!(time_travel::time_travel_input_at(&union, &applying, "/value/position").is_some_and(|(input, _)| input.id == "/position"), "the apply variant's parameters are addressable");
    assert!(time_travel::time_travel_input_at(&union, &restoring, "/value").is_some_and(|(input, _)| input.group.as_deref() == Some("restore")), "the restore variant's /value wins while restoring");
    assert!(time_travel::time_travel_input_at(&union, &restoring, "/value/position").is_none() && time_travel::time_travel_input_at(&union, &restoring, "/weight").is_none(), "another variant's fields are not addressable");
    let pointers = |value: &DslValue| time_travel::time_travel_input_rows(&union, value).into_iter().map(|row| row.pointer).collect::<Vec<_>>();
    assert_eq!(pointers(&applying), ["/phase", "/value/position", "/weight"]);
    assert_eq!(pointers(&restoring), ["/phase"], "a hidden restore diff is never a row");
    let switched = time_travel::time_travel_switch_variant(&union, &union[0], &applying, "restore");
    assert_eq!(switched, dsl(&serde_json::json!({ "phase": "restore", "value": { "position": 2 } })), "members only other variants declare are dropped");
    let laid_out = vec![
        semio_framework::ActionArgDef { group: Some("position".into()), ..semio_framework::ActionArgDef::number("/newX", LocalizedLabel::native("X", "X")) },
        semio_framework::ActionArgDef { group: Some("target".into()), ..semio_framework::ActionArgDef::text("/id", LocalizedLabel::native("Target region", "Zielregion")) },
    ];
    assert!(time_travel::time_travel_variant_selector(&laid_out).is_none());
    assert_eq!(time_travel::time_travel_input_rows(&laid_out, &dsl(&serde_json::json!({ "newX": 1.0, "id": "r" }))).len(), 2, "layout groups hide nothing");
}

/// 🗝️ LAW (design §20.11): an `optionSource` choice offers the keys of the object its template leads to in the previewed
/// document, each `{field}` segment taking the edited payload's member — an object's key, or the record of an array whose own
/// `field` equals it; a key names itself by its record's `label`/`name`, else the glossary, else the key. The row's own value stays an option when the document lacks it — the editor adds no enum, so the
/// fold refuses an unknown key — and a template the payload cannot fill offers nothing (a plain text field).
#[test]
fn sourced_options_are_the_keys_of_the_previewed_document() {
    let schema = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "additionalProperties": false,
        "required": ["mutation", "id", "channel"],
        "properties": {
            "mutation": { "const": "changeWidgetInput" },
            "id": { "type": "string", "minLength": 1, "x-semio-ui": { "label": { "en": "Widget", "de": "Widget" } } },
            "channel": { "type": "string", "minLength": 1, "x-semio-ui": { "widget": "select", "label": { "en": "Channel", "de": "Kanal" }, "optionSource": { "snapshot": "/hostSnapshot/widgets/{id}/params" } } }
        }
    }"#;
    let inputs = semio_framework::mutation_input_defs(schema, &semio_framework::registered_input_schema_document).expect("the sourced choice reads");
    let document = dsl(&serde_json::json!({ "hostSnapshot": { "widgets": [{ "kind": "source", "id": "w0" }, { "kind": "operator", "id": "w/1", "params": { "gain": { "label": { "en": "Gain", "de": "Verstärkung" } }, "zorp": 1.0 } }] } }));
    let resolved = |payload: serde_json::Value| {
        let payload = dsl(&payload);
        let mut rows = time_travel::time_travel_input_rows(&inputs, &payload);
        time_travel::time_travel_resolve_options(&mut rows, &payload, &document);
        rows.into_iter().find(|row| row.pointer == "/channel").expect("the channel row")
    };
    let keyed = dsl(&serde_json::json!({ "groups": { "a/b": { "x": 1 } }, "list": [{ "n": 7, "v": { "y": 2 } }] }));
    assert_eq!(time_travel::time_travel_option_target("/groups/{g}", &dsl(&serde_json::json!({ "g": "a/b" })), &keyed), Some(&dsl(&serde_json::json!({ "x": 1 }))), "a filled segment is an object key, never re-parsed as a pointer");
    assert_eq!(time_travel::time_travel_option_target("/list/{n}/v", &dsl(&serde_json::json!({ "n": 7 })), &keyed), Some(&dsl(&serde_json::json!({ "y": 2 }))), "on an array a filled segment selects the record whose own member matches");
    assert_eq!(time_travel::time_travel_option_target("/list/0/v", &DslValue::Null, &keyed), Some(&dsl(&serde_json::json!({ "y": 2 }))), "a literal segment indexes");
    assert_eq!(time_travel::time_travel_option_target("/list/{n}/v", &dsl(&serde_json::json!({ "n": 8 })), &keyed), None, "no record matches");
    let channel = resolved(serde_json::json!({ "mutation": "changeWidgetInput", "id": "w/1", "channel": "gain" }));
    let semio_framework::ActionArgControl::Select { options } = channel.input.control() else { panic!("a sourced choice is a select: {:?}", channel.input.control()) };
    let named = options.iter().map(|option| (option.value.as_str(), option.label.resolve(Terminology::Native, Locale::En).to_string(), option.label.resolve(Terminology::Native, Locale::De).to_string())).collect::<Vec<_>>();
    assert_eq!(named, [("gain", "Gain".to_string(), "Verstärkung".to_string()), ("zorp", "zorp".to_string(), "zorp".to_string())], "keys in document order, named by their record");
    let stale = resolved(serde_json::json!({ "mutation": "changeWidgetInput", "id": "w/1", "channel": "gone" }));
    assert!(
        matches!(&stale.input.schema, semio_framework::ArgSchema::String { options, option_source: Some(_), .. } if options.iter().map(|option| option.value.as_str()).collect::<Vec<_>>() == ["gain", "zorp", "gone"]),
        "the recorded key stays selectable for the fold to judge: {:?}",
        stale.input.schema
    );
    assert!(!format!("{:?}", stale.input.json_schema(Terminology::Native, Locale::En)).contains("enum"), "the editor never narrows the payload schema");
    let unfilled = resolved(serde_json::json!({ "mutation": "changeWidgetInput", "channel": "gain" }));
    assert!(
        matches!(unfilled.input.control(), semio_framework::ActionArgControl::Select { ref options } if options.iter().map(|option| option.value.as_str()).collect::<Vec<_>>() == ["gain"]),
        "an unfillable template offers only the recorded key: {:?}",
        unfilled.input.control()
    );
}

/// 🧲️ LAW: every `snapSource` is numbers before a host sees it. Puzzle 2d's position inputs (`newX`, `snapSource:
/// {config: "gridFactor"}`, here read by the real input reader) step by the configured grid factor, a bounded input
/// snaps to the factor's multiples, and a `Snapshot` pointer reads the document the open session previews.
#[semio_framework_async_macros::async_test]
async fn snap_sources_resolve_from_the_grid_factor_and_the_previewed_document() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let schema = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": "https://json.schemas.assets.semio-tech.com/framework/os/plugin/test/snap-source/schema.json",
        "type": "object",
        "additionalProperties": false,
        "required": ["mutation", "newX", "radius"],
        "properties": {
            "mutation": { "const": "moveTargetRegion" },
            "newX": { "type": "number", "x-semio-ui": { "widget": "stepper", "label": { "en": "X", "de": "X" }, "step": 1, "precision": 2, "snapSource": { "config": "gridFactor" } } },
            "radius": { "type": "number", "minimum": 0, "maximum": 10, "x-semio-ui": { "widget": "slider", "label": { "en": "Radius", "de": "Radius" }, "snapSource": { "config": "gridFactor" } } }
        }
    }"#;
    let inputs = semio_framework::mutation_input_defs(schema, &semio_framework::registered_input_schema_document).expect("the puzzle-shaped schema reads");
    let mut rows = time_travel::time_travel_input_rows(&inputs, &dsl(&serde_json::json!({ "newX": 3.0, "radius": 1.0 })));
    let config = dsl(&serde_json::json!({ "gridFactor": 0.5 }));
    time_travel::time_travel_resolve_snaps(&mut rows, |source| match source {
        semio_framework::SnapSource::Config { key } => time_travel::time_travel_pointer_get(&config, &format!("/{key}")).and_then(DslValue::as_f64),
        _ => None,
    });
    assert!(matches!(rows[0].input.control(), semio_framework::ActionArgControl::Stepper { step: Some(step), .. } if step == 0.5), "newX steps by the grid factor: {:?}", rows[0].input.control());
    assert!(matches!(&rows[1].input.schema, semio_framework::ArgSchema::Number { snaps, snap_source: None, .. } if snaps.len() == 21 && snaps[1] == 0.5 && snaps[20] == 10.0), "a bounded radius snaps to the grid: {:?}", rows[1].input.schema);
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 3 }), mounted_policy, &mut identity,cx).await;
    let mut panel = app.time_travel.panel().expect("an editing session");
    let editor = panel.editor.as_mut().expect("the draft editor");
    let mut count = semio_framework::ArgSchema::number(Some(0.0), Some(20.0), None, false);
    if let semio_framework::ArgSchema::Number { snap_source, .. } = &mut count {
        *snap_source = Some(semio_framework::SnapSource::Snapshot { pointer: "/count".into() });
    }
    editor.rows[0].input.schema = count;
    app.time_travel_ref().resolve_time_travel_snaps(&mut panel, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let resolved = &panel.editor.as_ref().expect("the draft editor").rows[0].input.schema;
    assert!(matches!(resolved, semio_framework::ArgSchema::Number { snaps, snap_source: None, .. } if *snaps == vec![0.0, 5.0, 10.0, 15.0, 20.0]), "the previewed document's count is the grid: {resolved:?}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🔗️ LAW (design §16.4): the open draft's `Reference` inputs are its draft references per selection domain — what every
/// render seam hands the app (`InteractionView::draft_references`) so it highlights what the edited mutation acts on: a
/// list and a single reference of one domain merge in draft order with each id once, a reference without a domain and a
/// number contribute nothing, and nothing is referenced before the session opens or once it exits.
#[semio_framework_async_macros::async_test]
async fn the_open_draft_references_its_reference_inputs_per_domain() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    assert!(app.time_travel.draft_references().is_empty(), "no session references nothing");
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let reference = |pointer: &str, domain: Option<&str>, many: bool| semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::Reference { kinds: Vec::new(), domain: domain.map(str::to_string), granularity: None, many, min_items: None, max_items: None, id_type: semio_framework::ReferenceIdType::String },
        ..semio_framework::ActionArgDef::number(pointer, LocalizedLabel::native(pointer, pointer))
    };
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = vec![
        reference("/targets", Some("items"), true),
        reference("/anchor", Some("items"), false),
        reference("/cells", Some("cells"), true),
        reference("/loose", None, true),
        semio_framework::ActionArgDef::number("/dx", LocalizedLabel::native("dx", "dx")),
    ];
    editor.value = dsl(&serde_json::json!({ "targets": ["a", "b", "a"], "anchor": "c", "cells": ["k"], "loose": ["z"], "dx": 4.0 }));
    let references = app.time_travel.draft_references();
    assert_eq!(references.get("items").map(Vec::as_slice), Some(["a".to_string(), "b".to_string(), "c".to_string()].as_slice()), "{references:?}");
    assert_eq!(references.get("cells").map(Vec::as_slice), Some(["k".to_string()].as_slice()), "{references:?}");
    assert_eq!(references.len(), 2, "only declared domains are referenced: {references:?}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    assert!(app.time_travel.draft_references().is_empty(), "an exited session references nothing");
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🎨️ LAW: the draft editor renders a colour input with the UI contract's `color_input` recipe (swatch, hex text and,
/// for four components, the opacity slider bound to the fourth component) and passes a vector input's snaps to every
/// axis; a hex commit keeps the stored opacity and the slider sets it alone.
#[semio_framework_async_macros::async_test]
async fn colour_inputs_render_the_contract_recipe_and_vector_axes_keep_their_snaps() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let mut offset = semio_framework::ActionArgDef::vector("/offset", LocalizedLabel::native("Offset", "Versatz"), 2);
    if let semio_framework::ArgSchema::Vector { snaps, .. } = &mut offset.schema {
        *snaps = vec![0.0, 0.5];
    }
    let tint = semio_framework::ActionArgDef { presentation: Some(semio_framework::ArgPresentation::Color), ..semio_framework::ActionArgDef::vector("/tint", LocalizedLabel::native("Tint", "Tönung"), 4) };
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = vec![tint, offset];
    editor.value = dsl(&serde_json::json!({ "tint": [1.0, 0.5, 0.0, 0.25], "offset": [0.0, 0.0] }));
    let history = render_history(&mut app, Locale::De).await;
    let colour = find_node(&history, "framework.history.editor.input.tint").unwrap_or_else(|| panic!("the colour recipe: {history}"));
    let text = colour.to_string();
    assert!(text.contains("#ff8000") && text.contains("#ff800040") && text.contains("Deckkraft") && text.contains("0.25"), "swatch, hex with alpha and the opacity slider: {colour}");
    let vector = find_node(&history, "framework.history.editor.input.offset").unwrap_or_else(|| panic!("the vector input: {history}"));
    assert!(vector.to_string().contains("0.5"), "every axis carries the vector's snaps: {vector}");
    let inputs = app.time_travel.editor().expect("editor").inputs.clone();
    let value = app.time_travel.editor().expect("editor").value.clone();
    let (hex_input, hex_element) = time_travel::time_travel_input_at(&inputs, &value, "/tint").expect("the colour input");
    assert_eq!(time_travel::time_travel_coerce(&hex_input, hex_element, &DslValue::String("#00ff00".into())).and_then(|value| value.as_array().map(<[DslValue]>::len)), Some(3), "a hex without alpha leaves the opacity to the stored value");
    let (alpha_input, alpha_element) = time_travel::time_travel_input_at(&inputs, &value, "/tint/3").expect("the opacity component");
    assert_eq!(time_travel::time_travel_coerce(&alpha_input, alpha_element, &DslValue::float(0.5)).and_then(|value| value.as_f64()), Some(0.5), "the slider sets the fourth component alone");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🧽️ LAW: a nullable input carries an accessible Clear button ("Clear <label>", en/de) that drafts `null` at the input's
/// pointer; while the input is null the row reads as cleared; a non-nullable input has none.
#[semio_framework_async_macros::async_test]
async fn a_nullable_input_offers_a_clear_control_that_drafts_null() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let scale = semio_framework::ActionArgDef { nullable: true, ..semio_framework::ActionArgDef::number("/scale", LocalizedLabel::native("Scale", "Maßstab")) };
    let width = semio_framework::ActionArgDef::number("/width", LocalizedLabel::native("Width", "Breite"));
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = vec![scale, width];
    editor.value = dsl(&serde_json::json!({ "scale": 2.0, "width": 1.0 }));
    let history = render_history(&mut app, Locale::De).await;
    let clear = find_node(&history, "framework.history.editor.input.scale.clear").unwrap_or_else(|| panic!("the clear control: {history}"));
    assert!(clear.to_string().contains("Maßstab leeren"), "a Clear named for its input: {clear}");
    assert!(clear.to_string().contains(semio_framework::history_edit::HISTORY_EDIT_INPUT_ACTION_ID) && clear.to_string().contains("/scale"), "Clear drafts the input's pointer: {clear}");
    assert!(find_node(&history, "framework.history.editor.input.width.clear").is_none(), "a non-nullable input has no Clear");
    let inputs = app.time_travel.editor().expect("editor").inputs.clone();
    let value = app.time_travel.editor().expect("editor").value.clone();
    let (input, element) = time_travel::time_travel_input_at(&inputs, &value, "/scale").expect("the nullable input");
    assert_eq!(time_travel::time_travel_coerce(&input, element, &DslValue::Null), Some(DslValue::Null), "Clear drafts null");
    app.time_travel.editor_mut().expect("the draft editor").value = dsl(&serde_json::json!({ "scale": null, "width": 1.0 }));
    let history = render_history(&mut app, Locale::En).await;
    let cleared = find_node(&history, "framework.history.editor.input.scale.clear").expect("the clear control");
    assert!(cleared.to_string().contains("Clear Scale"), "Clear stays named in every locale: {cleared}");
    assert!(find_node(&history, "framework.history.editor.input.scale.row").is_some_and(|row| row.to_string().contains("Cleared (no value)")), "the row reads as cleared");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §7, gap N2): while a history edit is open the body leads with the session band and the draft editor, then
/// the alternatives, the actions and the commands; without a session the actions lead. The editor's inputs are a tree
/// window over every input row, so a payload of any size stays reachable; the first rows are materialised the moment the
/// section shows.
#[semio_framework_async_macros::async_test]
async fn the_session_leads_the_history_body_and_the_editor_inputs_are_a_window_over_every_row() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    fn sections(body: &Value) -> Vec<&str> {
        body["children"].as_array().expect("sections").iter().filter_map(|section| section["key"].as_str()).collect()
    }
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    assert_eq!(sections(&render_history(&mut app, Locale::En).await), ["framework.history.actions", "framework.history.commands"]);
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = ["/dx", "/dy", "/dz"].map(|pointer| semio_framework::ActionArgDef::number(pointer, LocalizedLabel::native(pointer, pointer))).to_vec();
    editor.value = dsl(&serde_json::json!({ "dx": 1.0, "dy": 2.0, "dz": 3.0 }));
    let body = render_history(&mut app, Locale::En).await;
    assert_eq!(sections(&body), ["framework.history.timeTravel", "framework.history.editor", "framework.history.editor.inputs", "framework.history.actions", "framework.history.commands"]);
    let inputs = find_node(&body, "framework.history.editor.inputs").expect("the inputs section");
    assert_eq!(inputs["component"]["window"]["total"].as_u64(), Some(3), "the inputs are a window over every row: {inputs}");
    for pointer in ["dx", "dy", "dz"] {
        assert!(find_node(inputs, &format!("framework.history.editor.input.{pointer}.row")).is_some(), "the {pointer} row is materialised: {inputs}");
    }
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🧯️ LAW: the input path fails closed — when the edited leaf's payload schema compiles no validator, a draft is refused
/// with the typed `timeTravel.schema-unavailable` naming the reason on the editor, and the draft and session stay
/// untouched; never an unvalidated payload.
#[semio_framework_async_macros::async_test]
async fn a_schema_without_a_validator_refuses_every_draft() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let refusals = &fixture["refusals"];
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let generation = app.time_travel.session().generation;
    let before = app.time_travel.editor().expect("the draft editor").value.clone();
    app.time_travel.editor_mut().expect("the draft editor").refuse_validator("unresolved $ref");
    let refused = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::uint(9))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&refused), Some(text(&refusals["schemaUnavailable"])), "no validator, no draft");
    let editor = app.time_travel.editor().expect("the draft editor");
    assert!(editor.refused.as_ref().is_some_and(|(path, reason)| path == "/value" && reason.contains("unresolved $ref")), "the editor names why: {:?}", editor.refused);
    assert_eq!((&editor.value, app.time_travel.session().generation, app.time_travel.session().pending.as_ref().map(|pending| app.time_travel.session().unchanged(pending))), (&before, generation, Some(true)), "the draft and session are untouched");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: replay progress reaches every host without a dispatch — a driver turn that moves the replay ships a fresh
/// `HistoryPatch` whose `timeTravel` carries `done ≤ total` on the unsolicited UI progress frame, and the completion
/// ships the reviewing status the same way.
#[semio_framework_async_macros::async_test]
async fn replay_progress_rides_the_unsolicited_ui_frame_with_the_session_status() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    apply_long_edit(&mut app, 3_000, &mut identity).await;
    app.render_ctx().refresh_cache().await.expect("backfill");
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    while app.take_typed_operation_ui_progress().is_some() {}
    let mut stages = Vec::new();
    let mut cursor = app.history.log_generation;
    for _ in 0..10_000 {
        app.advance_typed_operation_publication(&mut identity,cx).await.expect("driver turn");
        collect_original_history_command_receipt(&mut app);
        while let Some(progress) = app.take_typed_operation_ui_progress() {
            let Some(patch) = progress.history_patch else { continue };
            assert!(patch.cursor > cursor, "every shipped patch advances the cursor");
            cursor = patch.cursor;
            let status = patch.time_travel.expect("the patch carries the session status");
            if status.stage == semio_framework::kernel::HistoryTimeTravelStage::Replaying {
                assert!(status.done.zip(status.total).is_some_and(|(done, total)| done <= total && total > 0), "progress is bounded: {status:?}");
            }
            stages.push(status.stage);
        }
        if app.time_travel.session().stage == TimeTravelStage::Reviewing && !app.time_travel.has_pending_work() {
            break;
        }
    }
    assert!(stages.contains(&semio_framework::kernel::HistoryTimeTravelStage::Replaying), "at least one progress status shipped: {stages:?}");
    assert_eq!(stages.last(), Some(&semio_framework::kernel::HistoryTimeTravelStage::Reviewing), "the completion ships the reviewing status");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a noted shell command keeps its label in every locale (an `{en, de}` pair), plain text stays data, a
/// detail is appended in every locale; undo is enabled wherever its chord is, even with nothing local to undo.
#[semio_framework_async_macros::async_test]
async fn noted_shell_commands_keep_every_locale_and_undo_matches_its_chord() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into()), mounted_policy, &mut identity).await;
    let history = render_history(&mut app, Locale::En).await;
    let undo = find_node(&history, "framework.history.undo").expect("undo row");
    assert_ne!(undo["disabled"], Value::Bool(true), "undo is enabled like its chord: the host routes a remote undo this guest cannot see");
    let args = DslValue::object([
        ("commandId".to_string(), DslValue::String("shell.export".into())),
        ("detail".to_string(), DslValue::String("a.png".into())),
        ("label".to_string(), DslValue::object([("de".to_string(), DslValue::String("Exportieren".into())), ("en".to_string(), DslValue::String("Export".into()))])),
    ]);
    let admitted = app.handle_action(NOTE_SHELL_COMMAND_ACTION_ID, Some(&args), &meta(&fixture), &mut identity).await.expect("noteShellCommand admits");
    semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut app, admitted).await.expect("noteShellCommand settles");
    let patch = app.render_ctx().history_patch(true).await.expect("history patch");
    let row = patch.upserts.iter().find(|entry| entry.action_id == "shell.export").expect("the noted row");
    assert_eq!(row.label.resolve(Terminology::Native, Locale::En), "Export - a.png");
    assert_eq!(row.label.resolve(Terminology::Native, Locale::De), "Exportieren - a.png");
    assert!(viewer_rejects_action(semio_framework::history_edit::HISTORY_EDIT_BEGIN_ACTION_ID) && viewer_rejects_action("undo"), "a viewer rejects every history-edit verb");
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a ledger whose check-in replay blocks under `MergePolicy::Normal` crosses the guest codec boundary as the
/// typed `history.ledger-not-replayable` naming only its blocking messages; any other refusal keeps its own fault.
#[test]
fn a_blocking_ledger_replay_crosses_the_guest_boundary_as_ledger_not_replayable() {
    let fault = replay_envelopes_fault(store::VcsError::Rejected {
        policy: protocol::MergePolicy::Normal,
        messages: vec![protocol::MutationMessage::error("mutation.target-missing", "gone"), protocol::MutationMessage::warning("mutation.clamped", "clamped")],
    });
    assert_eq!(fault.code, FaultCode::new("history.ledger-not-replayable"));
    assert!(fault.message.contains("mutation.target-missing") && !fault.message.contains("mutation.clamped"), "{}", fault.message);
    assert_ne!(replay_envelopes_fault(store::VcsError::Deserialize("malformed".into())).code, FaultCode::new("history.ledger-not-replayable"));
}

/// 🧲️ A `Config`/`Snapshot` snap source becomes numbers on the wire: the grid multiples inside the travel range when at
/// most one fixed list of them fits, else the grid spacing as the step; a colour input takes `#rrggbb` text.
#[test]
fn snap_sources_resolve_to_numbers_and_colours_take_hex_text() {
    let grid = |min: f64, max: f64| semio_framework::ArgSchema::Number {
        min: Some(min),
        min_exclusive: false,
        max: Some(max),
        max_exclusive: false,
        step: None,
        integer: false,
        unit: None,
        snaps: Vec::new(),
        snap_source: Some(semio_framework::SnapSource::Config { key: "gridFactor".into() }),
        soft_min: None,
        soft_max: None,
        precision: None,
        display_unit: None,
        display_factor: None,
        scale: None,
    };
    let mut fine = grid(-1.0, 1.0);
    time_travel::time_travel_apply_snap_spacing(&mut fine, Some(0.5));
    assert!(matches!(&fine, semio_framework::ArgSchema::Number { snaps, snap_source: None, step: None, .. } if *snaps == vec![-1.0, -0.5, 0.0, 0.5, 1.0]), "{fine:?}");
    let mut wide = grid(0.0, 1_000.0);
    time_travel::time_travel_apply_snap_spacing(&mut wide, Some(0.5));
    assert!(matches!(&wide, semio_framework::ArgSchema::Number { snaps, snap_source: None, step: Some(step), .. } if snaps.is_empty() && *step == 0.5), "{wide:?}");
    let mut unresolved = grid(0.0, 1.0);
    time_travel::time_travel_apply_snap_spacing(&mut unresolved, None);
    assert!(matches!(&unresolved, semio_framework::ArgSchema::Number { snap_source: None, step: None, .. }), "an unresolvable source never reaches a host");
    assert_eq!(time_travel::time_travel_color_components("#ff8000").map(|components| components.len()), Some(3));
    assert_eq!(time_travel::time_travel_color_components("#ff800040").map(|components| components.len()), Some(4), "a text with alpha sets the opacity too");
    assert_eq!(time_travel::time_travel_color_components("#f80").map(|components| components.len()), Some(3), "the contract's short form parses");
    let colour = semio_framework::ActionArgDef { presentation: Some(semio_framework::ArgPresentation::Color), ..semio_framework::ActionArgDef::vector("/tint", LocalizedLabel::native("Tint", "Tönung"), 4) };
    assert!(time_travel::time_travel_coerce(&colour, false, &DslValue::String("#00ff00".into())).is_some_and(|value| value.as_array().is_some_and(|components| components.len() == 3)), "an rgb pick keeps the alpha apart");
    assert_eq!(time_travel::time_travel_coerce(&colour, false, &DslValue::String("green".into())), None);
}

/// 🪧️ LAW (gap N3): a reference chip no app hook names reads the framework's generic default — the entity's own name where
/// the document gives one (a label, name, title or text; data or an `{en, de}` pair; a blank one names nothing), else its
/// kind word from the framework input-label glossary (or the kind spelled out) and a short id, in every locale; in a German
/// shell the toy's anonymous child reads "Kindelement anon-1".
#[semio_framework_async_macros::async_test]
async fn reference_chips_fall_back_to_the_document_name_then_the_kind_and_a_short_id() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let both = |label: &LocalizedLabel| (label.resolve(Terminology::Native, Locale::En).to_string(), label.resolve(Terminology::Native, Locale::De).to_string());
    let pair = |en: &str, de: &str| (en.to_string(), de.to_string());
    let document = dsl(&serde_json::json!({ "nodes": [{ "id": "a", "label": "Alpha" }, { "id": "b", "name": { "en": "Bee", "de": "Biene" } }, { "id": "c", "text": "  " }], "edges": [{ "id": "c", "title": "Gamma" }] }));
    let names = time_travel::time_travel_entity_names(&document, &["a", "b", "c", "z"].into_iter().collect());
    assert_eq!(names.iter().map(|(id, label)| (id.as_str(), both(label))).collect::<Vec<_>>(), vec![("a", pair("Alpha", "Alpha")), ("b", pair("Bee", "Biene")), ("c", pair("Gamma", "Gamma"))], "z is not in the document");
    let fallback = |kinds: &[&str], id: &str| both(&time_travel::time_travel_reference_fallback_label(&kinds.iter().map(|kind| kind.to_string()).collect::<Vec<_>>(), id));
    assert_eq!(fallback(&["s.test.child"], "anon-1!s.test.child@native/*"), pair("Child anon-1", "Kindelement anon-1"));
    assert_eq!(fallback(&["node", "edge"], "n1"), pair("Node n1", "Knoten n1"), "the first declared kind names it");
    assert_eq!(fallback(&["vortex.targetRegion"], "0123456789abcdefXYZ"), pair("Target region 0123456789ab\u{2026}", "Target region 0123456789ab\u{2026}"), "an unknown kind is spelled out, a long id cut");
    assert_eq!(fallback(&[], "x"), pair("x", "x"));
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 2 }), serde_json::json!({ "input": { "path": "/children", "value": ["anon-1!s.test.child@native/*", "child-2!s.test.child@native/*"] } })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let history = render_history(&mut app, Locale::De).await;
    let chip = |index: usize| find_node(&history, &format!("framework.history.editor.input.children.chip.{index}.row")).and_then(|chip| chip["component"]["label"].as_str()).unwrap_or_else(|| panic!("chip {index}: {history}")).to_string();
    assert_eq!((chip(0), chip(1)), ("Kindelement anon-1".to_string(), "Kind child-2".to_string()), "the generic default where the app names nothing, the app's own label elsewhere");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🎯️ "Use selection" reads the reference input's declared domain at its granularity: every selected id up to
/// `maxItems` for a many-reference, the first for a single one, typed by the input's id type (an id the type cannot
/// spell is skipped); an input without a domain is no selection target, and a missing, empty or other-granularity
/// selection selects nothing.
#[test]
fn selection_values_follow_the_reference_domain_granularity_count_and_id_type() {
    use semio_framework::{ArgSchema, ReferenceIdType};
    use time_travel::{time_travel_selection_value, TimeTravelActionRefusal};
    let reference = |domain: Option<&str>, many: bool, max_items: Option<u32>, id_type: ReferenceIdType| semio_framework::ActionArgDef {
        schema: ArgSchema::Reference { kinds: vec!["node".into()], domain: domain.map(str::to_string), granularity: Some("node".into()), many, min_items: None, max_items, id_type },
        ..semio_framework::ActionArgDef::number("/targets", LocalizedLabel::native("Targets", "Ziele"))
    };
    let selection = |granularity: &str, ids: &[&str]| protocol::DomainSelection { granularity: granularity.into(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: None };
    let text = |id: &str| DslValue::String(id.into());
    let many = reference(Some("board"), true, Some(2), ReferenceIdType::String);
    assert_eq!(time_travel_selection_value(&many, Some(&selection("node", &["a", "b", "c"]))), Ok(DslValue::Array(vec![text("a"), text("b")])), "a many-reference takes the selection up to maxItems");
    assert_eq!(time_travel_selection_value(&reference(Some("board"), true, None, ReferenceIdType::String), Some(&selection("node", &["a", "b", "c"]))), Ok(DslValue::Array(vec![text("a"), text("b"), text("c")])), "without maxItems every selected id");
    assert_eq!(time_travel_selection_value(&reference(Some("board"), false, None, ReferenceIdType::String), Some(&selection("node", &["a", "b"]))), Ok(text("a")), "a single reference takes the first");
    assert_eq!(time_travel_selection_value(&reference(None, true, None, ReferenceIdType::String), Some(&selection("node", &["a"]))), Err(TimeTravelActionRefusal::UnknownInput), "an input without a domain is no selection target");
    assert_eq!(time_travel_selection_value(&semio_framework::ActionArgDef::number("/dx", LocalizedLabel::native("dx", "dx")), Some(&selection("node", &["a"]))), Err(TimeTravelActionRefusal::UnknownInput), "a number is no reference");
    assert_eq!(time_travel_selection_value(&many, None), Err(TimeTravelActionRefusal::NoSelection), "the domain selects nothing");
    assert_eq!(time_travel_selection_value(&many, Some(&selection("region", &["a"]))), Err(TimeTravelActionRefusal::NoSelection), "a selection at another granularity");
    assert_eq!(time_travel_selection_value(&many, Some(&selection("node", &[]))), Err(TimeTravelActionRefusal::NoSelection), "an empty selection");
    let integers = reference(Some("board"), true, None, ReferenceIdType::Integer);
    assert_eq!(time_travel_selection_value(&integers, Some(&selection("node", &["7", "x", "-3"]))), Ok(DslValue::Array(vec![DslValue::uint(7), DslValue::int(-3)])), "integer ids stage the integers they spell");
    assert_eq!(time_travel_selection_value(&integers, Some(&selection("node", &["x"]))), Err(TimeTravelActionRefusal::NoSelection), "no id the type can spell");
    assert_eq!(time_travel_selection_value(&reference(Some("board"), false, None, ReferenceIdType::Integer), Some(&selection("node", &["x", "12"]))), Ok(DslValue::uint(12)), "a single integer reference skips text it cannot spell");
}

/// 🌿️ The history transitions `app` gained since it held `before` of them, decoded.
fn added_transitions(app: &ToyApp, before: usize) -> Vec<store::os_spr::HistoryTransition> {
    app.stores.store.envelope().transitions[before..].iter().map(|envelope| store::os_spr::history_transition_from_envelope(envelope).expect("a transition decodes").expect("a history transition")).collect()
}

/// ⚖️ LAW (R15): from a clean review the user begins another mutation and accepts it, so the session holds both drafts;
/// one finalize commits exactly ONE unscoped `Supersede` naming both, and the head equals a fresh fold of the edited log.
#[semio_framework_async_macros::async_test]
async fn several_drafts_from_a_review_finalize_as_one_overwrite_supersede() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let before = app.stores.store.envelope().transitions.len();
    commit_scenario(&mut app, &fixture, "several-drafts-accumulated-from-a-review-finalize-once", mounted_policy, &mut identity).await;
    let added = added_transitions(&app, before);
    let [store::os_spr::HistoryTransition::Supersede(supersede)] = added.as_slice() else { panic!("an overwrite is one Supersede: {added:?}") };
    let targets: BTreeSet<String> = supersede.inputs.iter().map(|input| input.target.0.clone()).collect();
    assert_eq!((supersede.scope.as_deref(), targets), (None, [0, 1].map(|index| seeded_mutation(&app, index)).into_iter().collect()), "one unscoped supersede names both drafts");
    let mut fresh = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), app.stores.store.local_actor_id().clone(), mounted_policy, &mut identity).await;
    for (index, op) in fixture["seed"].as_array().expect("seed").iter().enumerate() {
        let op = match index {
            0 => SetCount { value: 7 }.into(),
            1 => SetLabel { value: "b".into() }.into(),
            _ => seed_op(op),
        };
        fresh.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![op], transaction: None }, &mut identity).await.expect("fresh edit");
    }
    assert_eq!(app.stores.store.snapshot().expect("edited head"), fresh.stores.store.snapshot().expect("fresh head"));
    close(&mut app,cx);
    close(&mut fresh,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §2, §4): a new alternative is one `Branch` followed by one `Supersede` scoped to that alternative
/// (after the checkpoint of any pending edit), and the new alternative is current.
#[semio_framework_async_macros::async_test]
async fn a_new_alternative_is_one_branch_then_one_scoped_supersede() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let before = app.stores.store.envelope().transitions.len();
    commit_scenario(&mut app, &fixture, "a-new-alternative-keeps-the-original", mounted_policy, &mut identity).await;
    let added = added_transitions(&app, before);
    let alternative = app.stores.store.envelope().active_alternative_id.clone().expect("the new alternative is current");
    let (checkpoints, tail) = added.split_at(added.len().saturating_sub(2));
    assert!(checkpoints.iter().all(|transition| matches!(transition, store::os_spr::HistoryTransition::Commit(..))), "only a checkpoint of pending edits precedes the branch: {added:?}");
    assert!(
        matches!(tail, [store::os_spr::HistoryTransition::Branch { .. }, store::os_spr::HistoryTransition::Supersede(supersede)] if supersede.scope.as_deref() == Some(alternative.as_str())),
        "Branch, then a Supersede scoped to the new alternative: {added:?}"
    );
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}


//#region 🗝️EditorKeys
/// 🗝️ The shared band corpus whose `editorKeys` every shell's focus resolution reads (React `timeTravelFocusElementV1`).
const TIME_TRAVEL_BAND_CORPUS_JSON: &str = include_str!("../../../📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🧫️time-travel-band/🔣️.json");

/// ⚖️ LAW (design §16, G13): the draft editor's node keys are exactly the shared corpus's — the History panel tab the body
/// mounts in, Accept inside its row, and per input pointer (object fields flattened) one row holding its control — so a shell's
/// focus lands on the first input, else on Accept, whichever shell renders the body.
#[semio_framework_async_macros::async_test]
async fn the_draft_editor_keys_are_the_shared_corpus_keys_every_shell_focuses() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let corpus: Value = serde_json::from_str(TIME_TRAVEL_BAND_CORPUS_JSON).expect("band corpus parses");
    let keys = &corpus["editorKeys"];
    assert_eq!(text(&keys["panel"]), semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_HISTORY_ID, "the body mounts in the History panel tab");
    let rows = keys["inputs"].as_array().expect("editor input keys");
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = vec![
        semio_framework::ActionArgDef::number("/dx", LocalizedLabel::native("dx", "dx")),
        semio_framework::ActionArgDef::object("/pivot", LocalizedLabel::native("Pivot", "Drehpunkt"), vec![semio_framework::ActionArgDef::number("/x", LocalizedLabel::native("x", "x"))]),
    ];
    editor.value = dsl(&serde_json::json!({ "dx": 1.0, "pivot": { "x": 2.0 } }));
    let history = render_history(&mut app, Locale::En).await;
    let accept_row = find_node(&history, text(&keys["accept"]["row"])).unwrap_or_else(|| panic!("the Accept row: {history}"));
    assert!(find_node(accept_row, text(&keys["accept"]["control"])).is_some(), "Accept sits in its row: {accept_row}");
    let inputs = find_node(&history, "framework.history.editor.inputs").expect("the inputs section");
    let produced: Vec<&str> = inputs["children"].as_array().expect("input rows").iter().filter_map(|row| row["key"].as_str()).collect();
    assert_eq!(produced, rows.iter().map(|row| text(&row["row"])).collect::<Vec<_>>(), "one row per corpus pointer, in order");
    for row in rows {
        let holder = find_node(inputs, text(&row["row"])).expect("the input row");
        assert!(find_node(holder, text(&row["control"])).is_some(), "{} edits through {}: {holder}", text(&row["pointer"]), text(&row["control"]));
    }
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🗝️EditorKeys

//#region 🩹️HistoryViewPatch
/// ✏️ Applies one toy edit straight on the document store — a row the command log backfills, like a seeded or ingested edit.
async fn apply_toy_edit(app: &mut ToyApp, value: i32, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    app.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value }.into()], transaction: None }, identity).await.expect("toy edit applies");
}

/// 🧮️ Refreshes the history and answers how many rows it built and whether the cached view equals a full rebuild.
async fn refreshed_rows(app: &mut ToyApp) -> usize {
    HISTORY_ROWS_BUILT.with(|built| built.set(0));
    app.render_ctx().refresh_cache().await.expect("history refreshes");
    let built = HISTORY_ROWS_BUILT.with(std::cell::Cell::get);
    let cached = std::sync::Arc::clone(&app.history.cache.as_ref().expect("history cache").3);
    assert_eq!(cached.as_ref(), &app.render_ref().build_history_view(None).await, "the patched history equals a full rebuild");
    built
}

/// ⚖️ LAW (design §20.14, audit W2A-1): over a long history, appending an edit, undoing it, redoing it and re-rendering rebuild
/// at most the touched rows (the new row, the previous newest row and the previous top's row) — never the history — and the
/// patched view always equals a full rebuild; a history edit (a supersede transition) rebuilds the whole view.
#[semio_framework_async_macros::async_test]
async fn the_history_view_patches_only_the_rows_a_change_touched() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for value in 0..200 {
        apply_toy_edit(&mut app, value, mounted_policy, &mut identity).await;
    }
    let seeded = app.history.command_log.len();
    let built = refreshed_rows(&mut app).await;
    let rows = app.history.command_log.len();
    assert_eq!(rows, seeded + 200, "the 200 edits are backfilled rows");
    assert!(built <= 200 + 2, "the first refresh after 200 backfilled edits builds the new rows once ({built})");
    assert_eq!(refreshed_rows(&mut app).await, 0, "a refresh without a change builds no row");
    apply_toy_edit(&mut app, 1_000, mounted_policy, &mut identity).await;
    assert!(refreshed_rows(&mut app).await <= 3, "an appended edit rebuilds its own row and its neighbours only");
    app.stores.store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo");
    assert!(refreshed_rows(&mut app).await <= 3, "an undo rebuilds the undone row and the uncovered top only");
    app.stores.store.dispatch(ArtifactCommand::Redo, &mut identity).await.expect("redo");
    assert!(refreshed_rows(&mut app).await <= 3, "a redo rebuilds the redone row and the covered top only");
    let _ = render_history(&mut app, Locale::En).await;
    assert_eq!(refreshed_rows(&mut app).await, 0, "rendering the history builds no row");
    let target = app.stores.store.mutation_ops().expect("mutation ops")[0].mutation_id.clone();
    app.stores.store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![store::os_store::store_schema::SupersedeInput { target, replacement: Some(SetCount { value: 7 }.into()) }] }, &mut identity).await.expect("a history edit lands");
    assert!(refreshed_rows(&mut app).await >= rows, "a history edit rebuilds the whole view");
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (audit W2A-11): a row whose operations past the cap carry outcomes projects its most severe one, so the row's worst
/// severity is the worst over every operation of its edit, however many are flagged; the projection stays bounded and in op
/// order.
#[test]
fn the_projected_rows_carry_the_worst_severity_of_the_whole_edit() {
    use semio_framework_diagnostic::Severity;
    for (len, error_at) in [(200, 199), (200, 33), (65, 64), (40, 39), (10, 5)] {
        let flagged = (0..len).filter(|index| *index >= HISTORY_ROW_MUTATION_ROWS || *index == error_at).map(|index| (index, Some(if index == error_at { Severity::Error } else { Severity::Warning })));
        let shown = time_travel::history_projected_operations(len, flagged);
        assert!(shown.contains(&error_at), "{len} operations: the Error at {error_at} is projected");
        assert!(shown.len() <= 2 * HISTORY_ROW_MUTATION_ROWS, "{len} operations: the projection stays bounded");
        assert!(shown.windows(2).all(|pair| pair[0] < pair[1]), "{len} operations: the projection is in op order");
        assert!(shown.iter().take(len.min(HISTORY_ROW_MUTATION_ROWS)).copied().eq(0..len.min(HISTORY_ROW_MUTATION_ROWS)), "{len} operations: the first ones always lead");
    }
}

/// ⚖️ LAW (design §22.34, the store half — red until a plain `Apply` authors as the store's actor): every route authors
/// as its acting actor. A route without an actor of its own (the text ingest) authors as the instance's admitted actor; a
/// route with one (the plain emit route) authors as its separately admitted user instance.
#[semio_framework_async_macros::async_test]
async fn every_route_authors_as_its_acting_actor() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let acting = |actor: &str| ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(actor) };
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("ada".into()), mounted_policy, &mut identity).await;
    PluginApp::bind_actor(&mut app, "ada").await;
    let first: TestMutation = SetCount { value: 1 }.into();
    PluginApp::ingest_operations_text(&mut app, &::protocol::OpText::print_op(&first), &mut identity).await.expect("the text ingest applies");
    let ingested = edit_authors(&app);
    let pack = app.document_pack().await.expect("Ada's authoritative document");
    let mut grace = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("grace".into()), mounted_policy, &mut identity).await;
    artifact_app_laws::load_document(&mut grace, &pack, &mut identity).await.expect("Grace opens Ada's shared history");
    grace.dispatch_emit("setCount", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::mutations(vec![SetCount { value: 2 }.into()]), &acting("grace"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the emit publishes");
    let emitted = edit_authors(&grace);
    close(&mut app,cx);
    close(&mut grace,cx);
    assert_eq!(ingested, [Some("ada".to_string())], "a route without an actor of its own authors as the instance's");
    assert_eq!(emitted, [Some("ada".to_string()), Some("grace".to_string())], "the plain emit route authors as its acting actor");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🪪️ActorIdentity

//#region 🪣️SharedSchemaDocuments
/// 🪣️ A payload schema whose one input references the framework value schema (its numeric transport).
const SHARED_REFERENCE_SCHEMA: &str = r#"{"type":"object","additionalProperties":false,"required":["opacity"],"properties":{"opacity":{"$ref":"https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64Transport","x-semio-ui":{"label":{"en":"Opacity","de":"Deckkraft"}}}}}"#;

/// ⚖️ LAW (design §23): the framework's own shared schema documents are in every instance's input-schema resolver — a
/// registryless instance, for which no plugin assembly published anything, holds the framework value schema, the io
/// vocabulary and the store's document model (child, owner, link, blob); an input that references the value schema reads
/// as one input and the draft editor renders its control.
#[semio_framework_async_macros::async_test]
async fn an_input_that_references_a_shared_framework_schema_resolves_on_any_instance() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for id in ["framework/value/schema.json", "framework/io/schema.json", "os/store/child/schema.json", "os/store/child/owner/schema.json", "os/store/link/schema.json", "os/store/blob/schema.json"] {
        let id = format!("https://json.schemas.assets.semio-tech.com/{id}");
        assert!(semio_framework::registered_input_schema_document(&id).is_some(), "{id} is always in the runtime resolver");
    }
    let inputs = semio_framework::mutation_input_defs(SHARED_REFERENCE_SCHEMA, &semio_framework::registered_input_schema_document).unwrap_or_else(|error| panic!("the reference resolves: {error:?}"));
    assert_eq!(inputs.len(), 1, "the referenced value is one input: {inputs:?}");
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = inputs;
    editor.value = dsl(&serde_json::json!({ "opacity": 0.5 }));
    let history = render_history(&mut app, Locale::De).await;
    let row = find_node(&history, "framework.history.editor.input.opacity").unwrap_or_else(|| panic!("the input's control: {history}"));
    assert!(history.to_string().contains("Deckkraft"), "the control carries the input's own label: {row}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🪣️SharedSchemaDocuments

//#region 🎚️HistoryFilter
/// ⚖️ LAW (schema-first, one vocabulary): every option `setHistoryCommandFilter` declares is a value its dispatch accepts and
/// the history wire echoes as `commandFilter`, the history body's select offers exactly those values, and an undeclared value
/// is refused instead of silently showing every row.
#[semio_framework_async_macros::async_test]
async fn every_declared_history_filter_option_is_dispatched_and_echoed() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let definition = semio_framework::set_history_command_filter_action_definition();
    let declared: Vec<String> = match &definition.args[0].schema {
        semio_framework::ArgSchema::String { options, .. } => options.iter().map(|option| option.value.clone()).collect(),
        other => panic!("the filter is a choice: {other:?}"),
    };
    assert_eq!(declared, HistoryCommandFilter::ALL.map(|filter| filter.value().to_string()), "the declared options are the runtime's filters, in order");
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(&actor) };
    for value in &declared {
        let args = DslValue::Object(vec![("value".into(), DslValue::String(value.clone()))]);
        let admitted = app.handle_action(SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&args), &meta, &mut identity).await.unwrap_or_else(|fault| panic!("{value}: {fault:?}"));
        semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut app, admitted).await.unwrap_or_else(|fault| panic!("{value}: {fault:?}"));
        assert_eq!(&app.render_ctx().history_patch(true).await.expect("history patch").command_filter, value, "the wire echoes the declared option");
        let body = render_history(&mut app, Locale::En).await;
        let select = find_node(&body, "framework.history.filter.control").expect("the filter select");
        for option in &declared {
            assert!(select.to_string().contains(&format!("\"{option}\"")), "the select offers {option}: {select}");
        }
    }
    let undeclared = DslValue::Object(vec![("value".into(), DslValue::String("withoutOperations".into()))]);
    let refused = match app.handle_action(SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&undeclared), &meta, &mut identity).await {
        Ok(admitted) => semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut app, admitted).await.err(),
        Err(fault) => Some(fault),
    };
    assert!(refused.is_some(), "an undeclared filter value is refused");
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🎚️HistoryFilter


/// ☑️ LAW (audit W1E-2, conformance `💬️row-semantics` `selectedRows`): a choice with more options than one fixed list holds is
/// windowed option rows, each a tree row holding its choose button and stating its choice as `selected` — the chosen option
/// `Some(true)`, every other `Some(false)` — so both renderers announce (`aria-selected`, the mirror's `selected`) and paint it,
/// never by its icon alone.
#[test]
fn long_option_rows_state_their_choice_as_selected() {
    let option = |index: usize| semio_framework::ActionArgOption { value: format!("o{index}"), label: LocalizedLabel::native(&format!("Option {index}"), &format!("Option {index}")) };
    let mode = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::String { options: (0..40).map(option).collect(), option_source: None, min_len: None, max_len: None, pattern: None, format: None },
        required: true,
        ..semio_framework::ActionArgDef::text("/mode", LocalizedLabel::native("Mode", "Modus"))
    };
    let value = dsl(&serde_json::json!({ "mode": "o3" }));
    let editor = TimeTravelEditorPanel { target: "m-1".into(), label: LocalizedLabel::native("Set mode", "Modus setzen"), rows: time_travel::time_travel_input_rows(&[mode], &value), editable: true, inputs_refused: None, withdrawn: false, outcome: Vec::new(), refused: None, changed: false, reference_labels: Default::default() };
    let panel = TimeTravelPanel { status: Default::default(), store: None, stage: TimeTravelStage::Editing, pending_after: None, finalize_refusal: None, review: None, rerun_refusal: None, begin_refusal: None, next_problem: None, editor: Some(editor.clone()), outcomes: Default::default(), edited: Default::default(), accepted: Default::default() };
    let sections = time_travel::time_travel_editor_sections(&panel, &editor, &TreeWindows::unhosted(), "toy", Locale::En).expect("the editor builds");
    fn option_rows(node: &BuiltNode, rows: &mut Vec<(String, Option<bool>)>) {
        if let semio_framework_ui_contract::Component::TreeItem(props) = &node.component {
            if node.key.as_str().contains(".option.") && node.key.as_str().ends_with(".row") {
                rows.push((node.key.as_str().to_string(), props.selected));
            }
        }
        node.children.iter().for_each(|child| option_rows(child, rows));
    }
    let mut rows = Vec::new();
    sections.iter().for_each(|section| option_rows(section, &mut rows));
    assert!(rows.len() >= 4 && rows.iter().any(|(_, selected)| *selected == Some(true)), "the first paint holds the chosen option among several: {rows:?}");
    for (key, selected) in &rows {
        assert_eq!(*selected, Some(key == "framework.history.editor.input.mode.option.3.row"), "{key}");
    }
}

/// ⚖️ LAW (design §22.24, live fault F4): the base a session watches is the content revision of its store, never the
/// store's local generation. A backbone attaching and detaching while a draft is edited moves that generation and no
/// event: driver turns deliver no base move — the stage, the session generation, the base and the draft are what they
/// were — and an input stamped with the generation from before is accepted, not stale.
#[semio_framework_async_macros::async_test]
async fn a_backbone_attach_and_detach_while_editing_is_no_base_move() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let before = app.time_travel.session().clone();
    let (store_generation, revision) = (app.stores.store.generation(), app.stores.store.content_revision());
    let (backbone, probe) = MemoryBackbone::pair("attach-while-editing", "attach-while-editing").await;
    app.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.expect("the backbone attaches");
    app.detach_backbone().await.expect("the backbone detaches");
    assert!(app.stores.store.generation() > store_generation && app.stores.store.content_revision() == revision, "the port moved the store's local generation and no event");
    for _ in 0..4 {
        app.advance_typed_operation_publication(&mut identity,cx).await.expect("a driver turn");
        collect_original_history_command_receipt(&mut app);
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(app.time_travel.session(), &before, "no base move: the stage, the generation, the base and the draft are what they were");
    let typed = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("c".into())), ("generation".into(), DslValue::uint(u64::from(before.generation)))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&typed), None, "an input stamped before the port change is accepted");
    assert!(render_body(&mut app).await.contains("count=1 label=c"), "the preview follows the draft");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    drop(probe);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §22.24, store wave RB): a replay finished before a port change still commits. A backbone attaching and
/// detaching while the session reviews its finished replay moves the store's local generation and no event: driver turns
/// deliver no base move and start no second replay, and Finalize → Overwrite commits the head reviewed — no
/// `timeTravel.stale`, the session closes with its commit.
#[semio_framework_async_macros::async_test]
async fn a_replay_finished_before_a_backbone_attach_and_detach_still_commits() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        if let Some(result) = run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await {
            assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
        }
    }
    let before = app.time_travel.session().clone();
    assert_eq!(before.stage, TimeTravelStage::Reviewing, "the replay finished");
    let (store_generation, revision) = (app.stores.store.generation(), app.stores.store.content_revision());
    let (backbone, probe) = MemoryBackbone::pair("attach-while-reviewing", "attach-while-reviewing").await;
    app.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.expect("the backbone attaches");
    app.detach_backbone().await.expect("the backbone detaches");
    assert!(app.stores.store.generation() > store_generation && app.stores.store.content_revision() == revision, "the port moved the store's local generation and no event");
    for _ in 0..4 {
        app.advance_typed_operation_publication(&mut identity,cx).await.expect("a driver turn");
        collect_original_history_command_receipt(&mut app);
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(app.time_travel.session(), &before, "no base move: the review stands and no second replay started");
    for step in [serde_json::json!({ "finalize": null }), serde_json::json!({ "commit": { "choice": "overwrite" } })] {
        let result = run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: the replay finished before the port change still commits: {:?}", result.output);
    }
    pump_until(&mut app, "the finalize retires", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    assert!(app.time_travel.status().is_none(), "the session closed with its commit: {:?}", app.time_travel.session().stage);
    assert_eq!(head(&app), (5, "b".to_string()), "the committed document is the head reviewed");
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "every window shows the committed edit");
    drop(probe);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §22.1, §22.20): a mutation row always names Edit and, unless it is withdrawn, Withdraw — or Restore on a
/// mutation holding an accepted draft. A mutation without editable inputs keeps Withdraw and reads why Edit is refused, a
/// mutation the store's supersede law lets nobody withdraw reads that, a viewer reads why both are refused, in both
/// locales; only an offered Edit is the row's activation.
#[test]
fn a_mutation_row_names_why_edit_or_withdraw_is_refused() {
    let entry = |editable: bool, withdrawable: bool, withdrawn: bool| semio_framework::kernel::HistoryMutationEntry {
        mutation_id: "e-1#0".into(),
        position: 0,
        op_index: 0,
        label: LocalizedLabel::native("Set label", "Beschriftung setzen"),
        worst: None,
        messages: Vec::new(),
        superseded: withdrawn,
        withdrawn,
        editable,
        withdrawable,
        pending: false,
        edited: false,
        introduced: false,
        store: None,
    };
    let row = |entry: &semio_framework::kernel::HistoryMutationEntry, locale: Locale, read_only: bool, refusals: time_travel::MutationRowRefusals| {
        let row = history_panel_mutation_row(entry, "toy", locale, read_only, refusals).expect("the row builds");
        let semio_framework_ui_contract::Component::TreeItem(props) = &row.component else { panic!("a mutation row is a tree row") };
        let actions: Vec<(String, Option<String>)> = props.row_actions.iter().map(|action| (action.verb.as_str().to_string(), action.reason.as_ref().map(|reason| reason.0.as_str().to_string()))).collect();
        assert!(props.row_actions.iter().all(|action| action.disabled == action.reason.is_some()), "a disabled action names why: {actions:?}");
        (actions, props.target.as_ref().and_then(|target| target.activation.as_ref().map(|verb| verb.as_str().to_string())))
    };
    let offered = |verb: &str| (verb.to_string(), None);
    let refused = |verb: &str, reason: &str| (verb.to_string(), Some(reason.to_string()));
    let idle = time_travel::MutationRowRefusals::default();
    assert_eq!(row(&entry(true, true, false), Locale::En, false, idle), (vec![offered("historyEditBegin"), offered("historyEditWithdraw")], Some("historyEditBegin".to_string())), "an editable mutation");
    assert_eq!(
        row(&entry(false, true, false), Locale::En, false, idle),
        (vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), offered("historyEditWithdraw")], None),
        "a mutation without editable inputs keeps Withdraw and activates nothing"
    );
    assert_eq!(
        row(&entry(false, true, false), Locale::De, false, idle).0,
        vec![refused("historyEditBegin", "Die Eingaben dieser Mutation können nicht bearbeitet werden"), offered("historyEditWithdraw")],
        "the reason is the reader's language"
    );
    assert_eq!(
        row(&entry(false, false, false), Locale::En, false, idle).0,
        vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), refused("historyEditWithdraw", "This mutation cannot be withdrawn here")],
        "the store's supersede law refuses the withdrawal"
    );
    assert_eq!(
        row(&entry(false, false, false), Locale::En, true, idle),
        (vec![refused("historyEditBegin", "History cannot be edited in a read-only view"), refused("historyEditWithdraw", "History cannot be edited in a read-only view")], None),
        "a viewer reads why"
    );
    assert_eq!(row(&entry(true, false, true), Locale::En, false, idle).0, vec![offered("historyEditBegin")], "a withdrawn mutation offers no second withdrawal");
    let accepted = time_travel::MutationRowRefusals { accepted: true, ..idle };
    assert_eq!(row(&entry(false, false, true), Locale::En, false, accepted).0, vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), offered("historyEditRestore")], "an accepted draft is restored from its row");
    let replaying = time_travel::MutationRowRefusals { accepted: true, restore: Some(semio_framework_time_travel::TimeTravelLabel::RefusalIllegal), ..idle };
    assert_eq!(row(&entry(false, false, true), Locale::En, false, replaying).0[1], refused("historyEditRestore", "Not possible right now"), "outside the review Restore names why");
}
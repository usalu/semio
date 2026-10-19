/// 🎨️ References, colours, nullable and replay: draft reference inputs, colour and vector inputs, nullable clear controls, the session lead, schemas without validators, replay progress and noted shell commands.

/// 🔗️ LAW (design §16.4): the open draft's `Reference` inputs are its draft references per selection domain — what every
/// render seam hands the app (`InteractionView::draft_references`) so it highlights what the edited mutation acts on: a
/// list and a single reference of one domain merge in draft order with each id once, a reference without a domain and a
/// number contribute nothing, and nothing is referenced before the session opens or once it exits.
use serde_json::Value;
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

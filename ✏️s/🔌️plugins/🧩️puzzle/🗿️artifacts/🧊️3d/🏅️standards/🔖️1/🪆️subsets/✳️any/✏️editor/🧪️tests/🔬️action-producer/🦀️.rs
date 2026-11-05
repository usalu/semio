//! 🏭️ A normal puzzle3d action reaches its typed command through the framework-declared producer: the plugin declares none, yet an action and its undo run end to end on the mounted app.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

use semio_plugin::artifact_app_laws::{close_registered_fixture_app, dispatch_fixture_action, fixture_identity, fixture_mounted_policy, meta, new_app_with_registry, settle_history_verb, settle_registered_typed_operation, OriginalFixtureScope};
use semio_plugin::{App, ArtifactApp, ArtifactEditor, EditorApp, PluginApp, VcsArtifactApp};
use semio_s_artifact_puzzle_3d_editor::{create_puzzle3d_app, Puzzle3dPlayApp};

type Puzzle3dMounted = VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>;

/// 🔬️ Heap-observing fixture laws (`take_fixture_result_page`) need the counting allocator installed in the test binary.
#[global_allocator]
static HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

/// 🪟️ The puzzle3d window commands require the host's per-window view state, exactly as the browser host supplies it.
fn window_view() -> semio_plugin::ViewModel {
    semio_plugin::ViewModel { window_id: Some("puzzle3d-main".into()), window_instances: vec![semio_plugin::ViewWindowInstance { id: "puzzle3d-main".into(), window_kind_id: "puzzle3d-main".into() }], ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) }
}

macro_rules! window_meta {
    ($actor:expr) => {{
        let mut window_meta = meta($actor);
        window_meta.view_state = Some(window_view());
        window_meta
    }};
}

/// 🏗️ The registered, instance-bound app, boxed because the raw app is tens of KiB.
async fn mounted(scope: &mut OriginalFixtureScope) -> Box<Puzzle3dMounted> {
    let mut app = Box::new(new_app_with_registry::<EditorApp<Puzzle3dPlayApp>>(|| App { definition: create_puzzle3d_app(), examples: Vec::new() }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), fixture_mounted_policy(), &mut fixture_identity()).await);
    app.bind_instance_id(1).await;
    let _ = scope;
    app
}

fn target_volumes(app: &Puzzle3dMounted) -> usize {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(app.snapshot().expect("snapshot").typed())).get("targetVolumes").and_then(|volumes| volumes.as_array()).map_or(0, Vec::len)
}

#[semio_framework_async_macros::async_test]
async fn target_volume_action_and_undo_reach_the_command_through_the_framework_declared_producer() {
    assert!(<Puzzle3dPlayApp as ArtifactEditor>::original_action_command_producer().is_none(), "the plugin hand-writes no producer");
    assert!(<EditorApp<Puzzle3dPlayApp> as ArtifactApp>::original_action_command_producer().is_some(), "the framework supplies the generic producer");
    let mut scope = OriginalFixtureScope::admit();
    let mut app = mounted(&mut scope).await;
    let before = target_volumes(&app);
    let action_meta = window_meta!("local");
    let args: semio_framework_value::DslValue = serde_json::json!({ "origin": [1.0, 2.0, 3.0] }).into();
    dispatch_fixture_action(&mut *app, "addTargetVolume", Some(args), &action_meta, &mut fixture_identity(), scope.cx(), fixture_mounted_policy().close).await.expect("addTargetVolume is admitted through the producer");
    if let Err(error) = settle_registered_typed_operation(&mut *app, action_meta.instance_id, fixture_mounted_policy(), &mut fixture_identity(), scope.cx()).await {
        panic!("the edit publishes: {error:?}");
    }
    assert_eq!(target_volumes(&app), before + 1, "the typed command the producer born applied the edit");
    settle_history_verb(&mut *app, "undo", action_meta.instance_id, fixture_mounted_policy(), &mut fixture_identity(), scope.cx()).await;
    assert_eq!(target_volumes(&app), before, "undo restores the document");
    close_registered_fixture_app(&mut *app, fixture_mounted_policy(), scope.cx()).expect("original caller paid app close");
}

/// 🔬️ Diagnostic ledger of the settlement loop: how many turns, how much fuel and retained work one addTargetVolume settlement really costs, and whether any progress is made per turn.
#[semio_framework_async_macros::async_test]
async fn settlement_work_ledger_is_finite_and_progresses() {
    use semio_plugin::__job::testkit::renew_payload_page_grant;
    let mut scope = OriginalFixtureScope::admit();
    let mut app = mounted(&mut scope).await;
    let action_meta = window_meta!("local");
    let args: semio_framework_value::DslValue = serde_json::json!({ "origin": [1.0, 2.0, 3.0] }).into();
    let policy = fixture_mounted_policy();
    dispatch_fixture_action(&mut *app, "addTargetVolume", Some(args), &action_meta, &mut fixture_identity(), scope.cx(), policy.close).await.expect("admitted");
    let start_fuel = scope.cx().fuel_remaining();
    let start_progress = scope.cx().retained_progress();
    let mut last = (start_fuel, start_progress);
    let mut trace = Vec::new();
    let mut receipts = Vec::new();
    let mut rungs: Vec<String> = Vec::new();
    let mut turns = 0usize;
    while (app.has_pending_typed_operations() || !app.maintenance_terminal_is_empty()) && turns < 200 {
        turns += 1;
        let cx = scope.cx();
        renew_payload_page_grant(cx);
        let mut note = String::new();
        if !app.has_runnable_typed_operations() {
            note = format!("maint:{:?}", app.maintenance_step(cx, policy.maintenance).map(|_| ()).map_err(|error| format!("{error:?}")));
        }
        if let Err(error) = app.advance_typed_operation_publication(&mut fixture_identity(), cx).await { note.push_str(&format!(" pub:{error:?}")); }
        semio_plugin::artifact_app_laws::retain_original_ownership_receipts(&mut *app, policy, &mut receipts).expect("receipts");
        let mut page_log = Vec::new();
        for _ in 0..16 {
            let (answer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.take_typed_operation_result_page(action_meta.instance_id, policy.preparation));
            let Ok((page, progress)) = answer else { page_log.push("refused".to_string()); break };
            page_log.push(format!("heap(req {}, rel {}) progress(cap {}, rel {}, copy {}) page {}", heap.requested_bytes, heap.released_bytes, progress.retained_capacity_bytes, progress.released_bytes, progress.copied_bytes, page.is_some()));
            if let Some(page) = page {
                let lane = page.lane;
                let text = String::from_utf8_lossy(page.bytes()).into_owned();
                app.acknowledge_typed_operation_result(page.token).expect("ack");
                if format!("{lane:?}") == "Fault" { panic!("the settlement faulted with: {text}; presentation log: {page_log:#?}"); }
                break;
            }
            if progress == Default::default() { break }
        }
        while app.take_typed_operation_effect().is_some() {}
        while app.take_typed_operation_event().is_some() {}
        while app.take_typed_operation_ui_scope().is_some() {}
        while app.take_typed_operation_ui_progress().is_some() {}
        while app.take_task_resume_outcome().is_some() {}
        while app.take_typed_operation_composed_result().is_some() {}
        while app.take_local_interaction_query_reply().is_some() {}
        while app.take_typed_operation_completion().await.expect("completion").is_some() {}
        let now = (cx.fuel_remaining(), cx.retained_progress());
        if now != last || !note.is_empty() && turns % 256 == 0 { trace.push(format!("turn {turns}: fuel {} items {} runnable {} pending {} maint_empty {} {note}", now.0, now.1.copied_items, app.has_runnable_typed_operations(), app.has_pending_typed_operations(), app.maintenance_terminal_is_empty())); }
        if turns < 40 || (3000..3012).contains(&turns) { rungs.push(format!("turn {turns} delta items {} copy {} cap {} rel {} fuel {} note '{note}'", now.1.copied_items - last.1.copied_items, now.1.copied_bytes - last.1.copied_bytes, now.1.retained_capacity_bytes - last.1.retained_capacity_bytes, now.1.released_bytes - last.1.released_bytes, last.0 - now.0)); }
        if turns % 50 == 0 || turns < 4 { rungs.push(format!("turn {turns}: constructor_pending {} close_diagnostic '{}' op_diag '{}' maint_phase {:?}", app.store_constructor_is_pending(), app.close_diagnostic(), app.typed_operation_diagnostic(), app.mounted_owner_maintenance_phase().map(|phase| format!("{phase:?}")))); }
        last = now;
    }
    let spent = start_fuel - scope.cx().fuel_remaining();
    let tail: Vec<_> = trace.iter().rev().take(3).rev().cloned().collect();
    let (grant, progress, deadline, now) = { let cx = scope.cx(); (cx.retained_grant(), cx.retained_progress(), (cx.deadline_us(), cx.is_cancelled(), cx.should_yield()), cx.now_us()) };
    assert!(turns < 200, "settlement did not finish within 200 turns (fuel spent {spent}, distinct-progress turns {}); grant {grant:?} progress {progress:?} (deadline, cancelled, should_yield) {deadline:?} now {now:?}; diagnostic {}; rungs: {rungs:#?}; tail: {tail:#?}", trace.len(), app.typed_operation_diagnostic());
    assert!(spent < 32000, "settlement fuel {spent}");
    assert_eq!(target_volumes(&app), 1);
    close_registered_fixture_app(&mut *app, policy, scope.cx()).expect("close");
}

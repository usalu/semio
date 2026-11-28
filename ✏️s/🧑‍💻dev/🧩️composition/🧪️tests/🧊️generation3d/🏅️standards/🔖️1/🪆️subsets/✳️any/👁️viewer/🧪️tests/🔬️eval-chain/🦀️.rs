//! ⏱️ The read-only surface's half of the ONE addressed evaluation law.
//!
//! 🐛️ Regression guard for `📓️audit-window-inventory-2026-09-12.md` §4 P0 item 1: the viewer used
//! to build a fresh `FlowEvalSession::new()` per command and `tick` it synchronously to completion
//! with ZERO `ExtensionInvocation` sites anywhere in the surface. The brep/math operators are
//! contributed by the host at runtime and are never linked into the guest, so that loop could only
//! ever fault and no brep-bearing document could tessellate in the viewer at all.

use super::*;
use crate::viewer_domain::viewer_laws::context::{self, app};
use semio_plugin::app::TypedOperationResultLane;
use semio_plugin::{ActionMeta, PluginApp, ViewModel, ViewWindowInstance};
use semio_framework_test_fixtures::repo_text;

/// ⚖️ The ONE language-agnostic addressing fixture both surfaces answer, with a third-party twin at
/// `✏️editor/🧪️tests/🔬️tick-addressing/contract.ts`.
fn tick_addressing_fixture_json() -> &'static str { repo_text!("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🪟️tick-addressing.json") }

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TickAddressingFixture {
    format: String,
    version: u8,
    window_kinds: std::collections::BTreeMap<String, String>,
    arming: Vec<ArmingCase>,
    dispatch: Vec<DispatchCase>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AttachedWindow {
    id: String,
    kind: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArmingCase {
    id: String,
    surface: String,
    attached: Vec<AttachedWindow>,
    armed_window_ids: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct DispatchCase {
    id: String,
    surface: String,
    attached: Vec<AttachedWindow>,
    current_window_id: String,
    payload_window_id: String,
    payload_window_kind: String,
    admitted: bool,
}

fn fixture() -> TickAddressingFixture {
    let fixture: TickAddressingFixture = serde_json::from_str(tick_addressing_fixture_json()).expect("tick addressing fixture");
    assert_eq!(fixture.format, "semio.generation3d.tick-addressing");
    assert_eq!(fixture.version, 1);
    fixture
}

fn roster(fixture: &TickAddressingFixture, attached: &[AttachedWindow]) -> ViewModel {
    let window_instances = attached
        .iter()
        .map(|window| ViewWindowInstance { id: window.id.clone(), window_kind_id: fixture.window_kinds.get(&window.kind).unwrap_or_else(|| panic!("fixture window kind {}", window.kind)).clone() })
        .collect();
    ViewModel { window_instances, ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) }
}

/// ⚖️ LAW: which windows the READ-ONLY surface's `previewEval` run evaluates for a given attached roster.
/// Every hop names a concrete preview window, and a roster with none starts NO run — the same law the
/// sibling surface answers, read off the same fixture rows.
#[semio_framework_async_macros::async_test]
async fn every_run_hop_names_a_viewer_preview_window_that_is_actually_attached() {
    let _serial = context::lock();
    let fixture = fixture();
    for case in fixture.arming.iter().filter(|case| case.surface == "viewer") {
        let mut app = app().await;
        let view = roster(&fixture, &case.attached);
        let owed = semio_plugin::artifact_app_laws::fixture_pending_effects(&mut app.0, Some(&view)).await.expect("the host pending-effects poll").unwrap_or_default();
        let started = context::run_actions(&owed);
        assert_eq!(started.is_empty(), case.armed_window_ids.is_empty(), "arming case {}: a run starts exactly when a preview window is attached, got {started:?}", case.id);
        let run = context::drive_preview_run(&mut app, &view, &owed).await;
        let hops: Vec<String> = run.hop_windows.iter().cloned().collect::<std::collections::BTreeSet<_>>().into_iter().collect();
        eprintln!("viewer run hops {}: attached={:?} hops={:?}", case.id, view.window_instances.iter().map(|window| window.id.as_str()).collect::<Vec<_>>(), run.hop_windows);
        assert_eq!(hops, case.armed_window_ids, "arming case {}", case.id);
        assert!(context::owed_run_actions(&mut app, &semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).await.is_empty(), "no roster at all starts nothing");
        semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
    }
}

/// ⚖️ LAW: destroying a CONVERGED instance retires every ordered root it holds.
///
/// 🐛️ Regression guard for the teardown panic `ordered-map root must be explicitly retired before
/// drop` (`📡️replication/🌱️value/🗂️ordered/🦀️.rs:81`) seen once inside `flow-extension-brep` at the
/// end of a browser run, followed by `shard 0 worker fault [handler/turn] … unreachable`
/// (`📓️react-remaining-reds-2026-09-15.md` §11.1). Every `OrderedMap` in this process panics on a
/// bare drop, so this law needs no assertion of its own: it converges a real preview chain with the
/// geometry kernel served, performs the teardown a role switch performs — the whole-registry cancel
/// the shell sends, then the instance close — and a single unretired root anywhere in that path
/// aborts the test.
#[semio_framework_async_macros::async_test]
async fn destroying_a_converged_instance_retires_every_ordered_root() {
    let _serial = context::lock();
    let mut app = app().await;
    let view = context::view_shell_view("view-preview");
    let settled = context::drive_preview_run(&mut app, &view, &[]).await;
    assert!(settled.hops > 0, "the chain must actually run, or the teardown proves nothing: {settled:?}");
    let body = context::render_with_view(&mut app, preview::BODY_KEY, &view).await;
    let meshes = context::preview_mesh_count(&body);
    assert!(meshes >= 1, "the chain must actually converge onto painted geometry, got {meshes}");
    let (retired, _) = crate::brep_extension::cancel_brep_evaluations(None);
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
    drop(app);
}

/// ⚖️ LAW: which of those addresses the REAL retained route admits, driven through
/// `PluginApp::handle_command` → wire decode → `ArtifactRetainedCommandPhase::Preflight` → work.
#[semio_framework_async_macros::async_test]
async fn only_a_viewer_preview_addressed_tick_passes_the_retained_preflight() {
    let _serial = context::lock();
    let fixture = fixture();
    for case in fixture.dispatch.iter().filter(|case| case.surface == "viewer") {
        let mut app = app().await;
        let view = roster(&fixture, &case.attached).for_window_instance(&case.current_window_id).expect("the current window is attached");
        let kind = fixture.window_kinds.get(&case.payload_window_kind).cloned().unwrap_or_default();
        let args = semio_s_artifact_procedural_3d_preview_eval::window_args(&case.payload_window_id, &kind);
        let action_meta = ActionMeta { view_state: Some(std::sync::Arc::new(view)), ..semio_plugin::artifact_app_laws::meta("local") };
        let outcome = match context::dispatch_effect_command(&mut app, "flowEvalTick", Some(&args), &action_meta).await {
            Ok(()) => match semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, action_meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await {
                Ok(receipt) => receipt.lanes.contains(&TypedOperationResultLane::Fault).then(|| format!("retained publication faulted: {:?}", receipt.lanes)),
                Err(fault) => Some(format!("{fault:?}")),
            },
            Err(fault) => Some(format!("{fault:?}")),
        };
        assert_eq!(outcome.is_none(), case.admitted, "dispatch case {}: {outcome:?}", case.id);
        semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
    }
}

/// ⚖️ LAW: an admitted viewer tick either declares extension work or leaves its window owing the run
/// another hop. It must NEVER complete as a host-local synchronous tick with no continuation — that shape
/// is exactly what made the viewer unable to paint a brep-bearing document.
#[semio_framework_async_macros::async_test]
async fn a_viewer_tick_emits_extension_work_or_re_arms_but_never_settles_silently() {
    let _serial = context::lock();
    let mut app = app().await;
    let view = context::view_shell_view("view-preview");
    let action_meta = ActionMeta { view_state: Some(std::sync::Arc::new(view.clone())), ..semio_plugin::artifact_app_laws::meta("local") };
    let args = semio_s_artifact_procedural_3d_preview_eval::window_args("view-preview", preview::WINDOW_KIND_ID);
    context::dispatch_effect_command(&mut app, "flowEvalTick", Some(&args), &action_meta).await.expect("viewer preview tick");
    let tick = semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, action_meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("retained publication");
    assert!(!tick.lanes.contains(&TypedOperationResultLane::Fault), "viewer preview tick faulted: {:?}", tick.lanes);
    let answered = crate::brep_extension::settle(&mut app.0, action_meta.instance_id, &action_meta, app.1.cx()).await.answered;
    let owed = context::owed_run_actions(&mut app, &view).await;
    assert!(answered > 0 || !owed.is_empty(), "the viewer evaluation must emit ExtensionInvocation or owe the run another hop, not a dead sync tick");
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

/// ⚖️ LAW: the SERVED ORDER on the read-only surface. The shell opens the document and only THEN
/// pushes the contributions closure, so the first evaluation always runs against a registry that
/// cannot address the geometry kernel. Installing operators into a process-wide registry publishes
/// nothing, so the recovery has to be armed by the install itself — and it has to reach a PAINTED
/// preview, because a viewer whose only window shows an empty world is the defect this lane closed.
///
/// 🧪️ A `--lib` binary LINKS both extension packs, and a linked pack shadows the contributed stub —
/// so an empty contribution table alone still evaluates and would prove nothing.
/// [`crate::flow_operators::UnlinkedFlowExtensions`] therefore retires the linked installers for the
/// length of this law, which is the served guest's actual shape: nothing linked, everything
/// contributed. The evaluate hop then crosses the real extension wire, answered in-process by
/// `🔬️brep-extension` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn a_late_contributions_install_re_arms_the_viewer_evaluation_the_empty_registry_faulted() {
    let _serial = context::lock();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::take();
    semio_os_flow::sync_host_flow_extension_contributions("[]".to_string()).expect("an empty closure is a legal registry state — it is the state every served boot starts in");
    assert!(
        semio_os_flow::flow_extension_invocation_address(crate::flow_operators::BREP_EXTENSION_FLOW_ID).is_err(),
        "the geometry kernel must be unaddressable before the host pushes anything, or this law proves nothing"
    );

    let mut app = app().await;
    let view = context::view_shell_view("view-preview");
    let faulted = context::render_with_view(&mut app, preview::BODY_KEY, &view).await;
    assert_eq!(context::preview_mesh_count(&faulted), 0, "an unaddressable geometry kernel paints nothing");

    let contributions = crate::flow_operators::staged_flow_extension_contributions_json(&[]);
    let pages = semio_framework::public_invocation_string_pages(&contributions);
    let mut install_effects = Vec::new();
    let settled = context::drive_preview_run(&mut app, &view, &[]).await;
    assert_eq!((settled.state.as_deref(), settled.hops), (None, 0), "a graph the empty registry cannot serve starts no run: {settled:?}");
    for (index, page) in pages.iter().enumerate() {
        let receipt = context::dispatch_with_view(
            &mut app,
            Generation3dViewCommand::SetContributions(set_contributions::SetContributions { json: page.clone(), page: index as u64, page_count: pages.len() as u64 }),
            view.clone(),
        )
        .await
        .expect("contributions page");
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "contributions page {index} faulted in the retained job ladder");
        let carried = context::run_actions(&receipt.effects);
        if index + 1 < pages.len() {
            assert!(carried.is_empty(), "page {index} installs nothing yet, so it carries nothing");
        } else {
            assert_eq!(carried, vec![semio_framework_tool_run::TOOL_RUN_START_ACTION_ID.to_string()], "the last page carries the attached preview exactly one run start");
            install_effects = receipt.effects.clone();
        }
    }
    assert!(
        semio_os_flow::flow_extension_invocation_address(crate::flow_operators::BREP_EXTENSION_FLOW_ID).is_ok(),
        "the pushed closure must make the geometry kernel addressable"
    );

    let ticks = context::drive_preview_run(&mut app, &view, &install_effects).await.hops;
    assert!(ticks > 0, "the install's own effects must be the thing that restarts the read-only chain");
    let preview_body = context::render_with_view(&mut app, preview::BODY_KEY, &view).await;
    let meshes = context::preview_mesh_count(&preview_body);
    assert!(meshes >= 1, "the re-armed viewer chain must reach a painted preview, got {meshes}");
    println!("[STATS] viewer late install re-armed {ticks} ticks and painted meshes={meshes}");
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

/// 🛑️ LAW: the abort a running preview OFFERS, dispatched through the production action path, reaches the run and ends it as `aborted` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, 2026-11-25: `cancellable` was `false` for 190 s of a live run, so no abort could be sent at all).
///
/// The run is started by the very effect the host's refresh owes (`toolRunStart`), the abort's `runId`/`generation` are read back out of the status the RENDERED preview window publishes (not built by the test), and the framework's `toolRunAbort` is dispatched with exactly those `cancelArgs` through `dispatch_fixture_action`, the entry the shell's action channel uses.
#[semio_framework_async_macros::async_test]
async fn the_abort_a_running_preview_offers_reaches_its_run_and_ends_it_as_aborted() {
    use semio_plugin::artifact_app_laws::{dispatch_fixture_action, fixture_identity, meta};
    let _serial = context::lock();
    let mut app = app().await;
    let view = context::view_shell_view("view-preview");
    let action_meta = ActionMeta { view_state: Some(std::sync::Arc::new(view.clone())), ..meta("local") };
    let owed = semio_plugin::artifact_app_laws::fixture_pending_effects(&mut app.0, Some(&view)).await.expect("the host pending-effects poll").unwrap_or_default();
    assert_eq!(context::run_actions(&owed), vec![semio_framework_tool_run::TOOL_RUN_START_ACTION_ID.to_string()], "an attached preview owes exactly one run start");
    for effect in owed {
        if let semio_plugin::Effect::DispatchAction { action, args, .. } = effect {
            dispatch_fixture_action(&mut app.0, &action, args, &action_meta, &mut fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"));
        }
    }
    let started = app.0.tool_run_presence().map(|presence| presence.state.wire_name());
    assert!(matches!(started, Some("starting" | "running")), "the host's run start must leave a live run, got {started:?}");
    let body = context::render_with_view(&mut app, preview::BODY_KEY, &view).await.replace('\\', "");
    assert!(body.contains("\"cancellable\":true"), "a live preview run must offer its abort through the production render binding");
    let offered = &body[body.find("\"cancelArgs\"").expect("the offered abort carries its run address")..];
    let digits = |key: &str| -> String { let at = offered.find(key).unwrap_or_else(|| panic!("cancelArgs misses {key}")) + key.len(); offered[at..].trim_start_matches('"').chars().take_while(char::is_ascii_digit).collect() };
    let (run, generation) = (digits("\"runId\":"), digits("\"generation\":"));
    let args = semio_framework_value::DslValue::object([(semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID.to_string(), semio_framework_value::DslValue::String(run)), (semio_framework_tool_run::TOOL_RUN_ARG_GENERATION.to_string(), semio_framework_value::DslValue::String(generation))]);
    dispatch_fixture_action(&mut app.0, semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID, Some(args), &action_meta, &mut fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("the offered abort is accepted by the run's owner");
    let settled = context::drive_preview_run(&mut app, &view, &[]).await;
    assert_eq!(settled.state.as_deref(), Some("aborted"), "the abort must reach the run and end it as aborted, not complete or leave it running: {settled:?}");
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

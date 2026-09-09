use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{ActionMeta, App, EditorApp, Fault, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

pub type NoteApp = VcsArtifactApp<EditorApp<NotePlayApp>>;

/// 🧪️ SDK gap (contract §2.4/§7.4 handoff): `testkit::new_app_with_registry`'s signature is still
/// `fn(manifest: fn() -> App)`, not yet updated for the `AppDefinition`-returning `create_note_app`
/// convention — this tiny local wrapper adapts it back into the `App { definition, examples }`
/// shape that fn still expects (mirrors trinity/jack's `trinity_jack_manifest_for_testkit`, the
/// first real W2 packet to hit this exact gap).
fn note_manifest_for_testkit() -> App {
    App { definition: create_note_app(), examples: Vec::new() }
}

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
pub async fn note_app() -> NoteApp {
    let mut app = new_app::<EditorApp<NotePlayApp>>().await;
    app.bind_instance_id(1).await;
    app
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn note_app_with_registry() -> NoteApp {
    let mut app = new_app_with_registry::<EditorApp<NotePlayApp>>(note_manifest_for_testkit).await;
    app.bind_instance_id(1).await;
    app
}

pub fn composite_view(id: &str) -> ViewModel {
    ViewModel {
        window_instances: vec![
            ViewWindowInstance { id: id.into(), window_kind_id: NOTE_PLAY_WINDOW_COMPOSITE.into() },
            ViewWindowInstance { id: NOTE_PLAY_WINDOW_NAVIGATOR.into(), window_kind_id: NOTE_PLAY_WINDOW_NAVIGATOR.into() },
        ],
        ..Default::default()
    }
    .for_window_instance(id)
    .expect("Note composite test window roster")
}

fn action_meta(actor: &str, view_state: ViewModel) -> ActionMeta {
    ActionMeta { view_state: Some(view_state), ..meta(actor) }
}

async fn settle(app: &mut NoteApp, result: Result<InvocationResult, Fault>) -> Result<InvocationResult, Fault> {
    let mut result = result?;
    for _ in 0..1_048_576 {
        if !app.has_pending_typed_operations() {
            return Ok(result);
        }
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)?;
        app.advance_typed_operation_publication().await?;
        if let Some(page) = app.take_typed_operation_result_page(1) {
            if page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
                return Err(Fault::from(String::from_utf8_lossy(page.bytes()).into_owned()));
            }
            app.acknowledge_typed_operation_result(page.token)?;
        }
        result.requested_effects.extend(app.take_typed_operation_effect());
        result.events.extend(app.take_typed_operation_event());
        if let Some(scope) = app.take_typed_operation_ui_scope() {
            result.ui_scope = scope;
        }
    }
    Err(Fault::from("Note test operation did not settle"))
}

pub async fn dispatch(app: &mut NoteApp, command: NoteCommand) -> InvocationResult {
    dispatch_with_view(app, command, composite_view(NOTE_PLAY_WINDOW_COMPOSITE)).await
}

pub async fn dispatch_with_view(app: &mut NoteApp, command: NoteCommand, view_state: ViewModel) -> InvocationResult {
    let result = app.dispatch_typed(command, &action_meta("local", view_state)).await;
    settle(app, result).await.expect("dispatch")
}

pub async fn render(app: &mut NoteApp, body_key: &str) -> String {
    render_with_view(app, body_key, &composite_view(NOTE_PLAY_WINDOW_COMPOSITE)).await
}

pub async fn render_with_view(app: &mut NoteApp, body_key: &str, view_state: &ViewModel) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
/// injected `interactionSelect` verb, dispatched against the "blocks" domain declared on this app —
/// requires `note_app_with_registry()` (a bare `note_app()` has no declared interaction domains to
/// select against). `ids` are raw block ids, converted to the row-id-prefixed `InteractionTarget`
/// id the document panel tree/`interaction_topology` both use (see `note_blocks_topology`'s doc
/// comment).
pub async fn select_blocks(app: &mut NoteApp, ids: &[&str]) {
    let target_list: Vec<serde_json::Value> = ids.iter().map(|id| serde_json::json!({ "granularity": "block", "id": format!("note-play-block:{id}") })).collect();
    let targets = serde_json::to_string(&target_list).expect("targets json");
    let args = semio_framework_plugin::optional_json_to_dsl(Some(serde_json::json!({ "domainId": NOTE_INTERACTION_BLOCKS, "targets": targets, "merge": "replace" })));
    let result = app.handle_action("interactionSelect", args.as_ref(), &action_meta("test", composite_view(NOTE_PLAY_WINDOW_COMPOSITE))).await;
    settle(app, result).await.expect("interactionSelect");
}

pub async fn close_app(app: &mut NoteApp) {
    for _ in 0..1_048_576 {
        if app.close_terminal_is_empty() {
            return;
        }
        if PluginApp::close_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).await.expect("Note registered app close") == semio_framework_plugin::PluginCloseStep::Complete {
            break;
        }
    }
    assert!(app.close_terminal_is_empty(), "Note registered app close did not reach terminal-empty ownership");
}

use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

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
    new_app::<EditorApp<NotePlayApp>>().await
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn note_app_with_registry() -> NoteApp {
    new_app_with_registry::<EditorApp<NotePlayApp>>(note_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut NoteApp, command: NoteCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut NoteApp, body_key: &str) -> String {
    render_with_view(app, body_key, &ViewModel::default()).await
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
    app.handle_action("interactionSelect", args.as_ref(), &meta("test")).await.expect("interactionSelect");
}

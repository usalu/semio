use crate::editor::note::testkit::{note_app, render as render_body};
use crate::editor::note::NOTE_PLAY_BODY_DOCUMENT as BODY_DOCUMENT;
use semio_framework_plugin::PluginApp;

/// 🩹️ Pre-existing bug fixed here (confirmed via `git log --date=iso`: `SetActiveExample`'s
/// `reset_document_effect`/`Effect::LoadDocument` conversion — and this very test — both
/// predate this ticket's dispatch to note, unrelated to composition). Dispatching a command only
/// ever RETURNS a `Effect::LoadDocument` as data for a real host to re-apply; `dispatch_typed`
/// never loops it back into the same app instance, so `app.snapshot()`/subsequent `render()` never
/// reflected it — this assertion could never have passed as originally written, on ANY content.
/// Fixed the same way writer's own `app_with_jack()` and cad's `two_instances_converge_…` tests
/// already do: call `PluginApp::load_document_pack` directly, the same technique a real host uses
/// when it receives the effect.
#[semio_framework_async_macros::async_test]
async fn renders_document_tree() {
    let mut app = note_app().await;
    let document = crate::schema::semio_example_snapshot();
    let envelope = store::create_document_envelope::<crate::NoteSnapshot, crate::NoteMutation>(&document.schema.clone(), &document.id.clone(), document, None);
    let files = store::print_document_pack(&envelope).await.expect("print semio example document pack");
    app.load_document_pack(&files).await.expect("load semio example");
    let json = render_body(&mut app, BODY_DOCUMENT).await;
    assert!(json.contains("\"type\":\"tree\""));
    assert!(json.contains("Welcome"));
}

#[semio_framework_async_macros::async_test]
async fn note_labels_resolve_native_by_default() {
    let mut app = note_app().await;
    let document_json = render_body(&mut app, BODY_DOCUMENT).await;
    assert!(document_json.contains("Add Text"));
    assert!(document_json.contains("Add Table"));
    assert!(document_json.contains("Add Math"));
    assert!(document_json.contains("Add Image"));
    assert!(document_json.contains("Add Group"));
    assert!(document_json.contains("Drop blocks here"));
}

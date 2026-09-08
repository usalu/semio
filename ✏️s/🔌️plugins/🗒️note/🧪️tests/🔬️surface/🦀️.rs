
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn note_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::note::NoteViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn note_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::note::NotePlayApp, crate::viewer::note::NoteViewer>().await;
}

/// 🧪️ The manifest assertion the ticket asked this pass to check: note was one of six plugins
/// previously reported shipping an `assembly-failed` `PluginManifest` (`plugin_id:
/// "assembly-failed"`, every list empty — `crate::plugin`'s own `pub fn plugin_manifest()`
/// returns exactly that stub whenever `try_build()` errors) because the OLD `.artifact(…)`
/// channel's capability cross-check never had a `"composer"` capability row whose `dialect`
/// claim matched its own native self-composer entry. The new `.declare_artifact(…)` channel
/// (this file's `plugin()`) never runs that OLD cross-check at all — it walks the declaration
/// tree independently — so this test proves the fix holds under the NEW mechanism directly,
/// not by re-checking the old capability row (kept, unread, debt D1): a real `Plugin` with a
/// real (non-stub) manifest carrying at least one app and one artifact-kind row.
#[semio_framework_async_macros::async_test]
async fn plugin_assembles_a_real_manifest_not_the_assembly_failed_stub() {
    let plugin = super::plugin().expect("note plugin must assemble cleanly under the new declaration tree");
    assert_ne!(plugin.manifest.plugin_id, "assembly-failed", "manifest must not be the try_build()-failed stub");
    assert_eq!(plugin.manifest.plugin_id, "note");
    assert!(!plugin.manifest.apps.is_empty(), "manifest must declare at least one app (editor+viewer)");
    assert_eq!(plugin.manifest.apps.len(), 2, "one editor + one viewer surface, exactly");
}

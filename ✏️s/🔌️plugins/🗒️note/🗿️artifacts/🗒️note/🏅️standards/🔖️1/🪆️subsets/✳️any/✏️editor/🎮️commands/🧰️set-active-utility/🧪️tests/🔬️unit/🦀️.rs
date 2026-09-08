
use super::*;
use crate::editor::note::testkit::{dispatch, note_app, note_app_with_registry, render};
use crate::editor::note::{NOTE_PLAY_BODY_PROPERTIES, NoteCommand};

/// 🧰️ The active utility now lives in `cfg.active_utility_id` — switching utilities is still
/// document-op-free, but it must actually persist.
#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_artifact_mutations_but_persists_in_config() {
    let mut app = note_app().await;
    let before = app.snapshot().expect("snapshot");
    let result = dispatch(&mut app, NoteCommand::SetActiveUtility(SetActiveUtility { utility_id: "pencil".into() })).await;
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    assert_eq!(app.snapshot().expect("snapshot"), before, "utility switching does not mutate the document");
    assert!(render(&mut app, NOTE_PLAY_BODY_PROPERTIES).await.contains("Utility: pencil"), "cfg.active_utility_id reflects the switch");
}

#[semio_framework_async_macros::async_test]
async fn world_pick_style_registry_enforcement_allows_the_active_utility_switch() {
    // 🧬️ Mirrors `shooting_ui`'s registry-backed coverage: dispatching through
    // `new_app_with_registry` exercises `AppActionRegistry` kind discipline for a View command.
    let mut app = note_app_with_registry().await;
    let result = dispatch(&mut app, NoteCommand::SetActiveUtility(SetActiveUtility { utility_id: "pencil".into() })).await;
    assert!(result.mutations.is_empty(), "SetActiveUtility (View) emits no operations even under registry enforcement");
}

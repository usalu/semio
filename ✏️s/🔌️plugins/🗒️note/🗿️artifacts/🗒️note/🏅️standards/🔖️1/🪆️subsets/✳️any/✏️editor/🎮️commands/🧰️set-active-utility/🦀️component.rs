//! 🧰️ 🧰️ Note play app command command — `set-active-utility`.

use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-utility")]
pub struct SetActiveUtility {
    pub utility_id: String,
}

pub async fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::config(vec![NoteConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::note::testkit::{dispatch, note_app, note_app_with_registry, render};
    use crate::editor::note::{NoteCommand, NOTE_PLAY_BODY_PROPERTIES};

    /// 🧰️ The active utility now lives in `cfg.active_utility_id` — switching utilities is still
    /// document-op-free, but it must actually persist.
    #[test]
    async fn set_active_utility_emits_no_artifact_mutations_but_persists_in_config() {
        let mut app = note_app();
        let before = app.snapshot().expect("snapshot");
        let result = dispatch(&mut app, NoteCommand::SetActiveUtility(SetActiveUtility { utility_id: "pencil".into() }));
        assert!(result.mutations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.snapshot().expect("snapshot"), before, "utility switching does not mutate the document");
        assert!(render(&mut app, NOTE_PLAY_BODY_PROPERTIES).contains("Utility: pencil"), "cfg.active_utility_id reflects the switch");
    }

    #[test]
    async fn world_pick_style_registry_enforcement_allows_the_active_utility_switch() {
        // 🧬️ Mirrors `shooting_ui`'s registry-backed coverage: dispatching through
        // `new_app_with_registry` exercises `AppActionRegistry` kind discipline for a View command.
        let mut app = note_app_with_registry();
        let result = dispatch(&mut app, NoteCommand::SetActiveUtility(SetActiveUtility { utility_id: "pencil".into() }));
        assert!(result.mutations.is_empty(), "SetActiveUtility (View) emits no operations even under registry enforcement");
    }
}
//#endregion 🧪️Tests

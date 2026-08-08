//! 🧰️ Note play app command — the active canvas utility switch (select/pencil/eraser/…). Host-owned,
//! config-only.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetActiveUtility
pub mod set_active_utility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-utility")]
    pub struct SetActiveUtility {
        pub utility_id: String,
    }

    pub fn handle(payload: &SetActiveUtility, _doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(Emit::config(vec![NoteConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
    }
}
//#endregion 🔖️SetActiveUtility

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{dispatch, note_app, note_app_with_registry, render};
    use crate::apps::note::{NoteCommand, NOTE_PLAY_BODY_PROPERTIES};

    /// 🧰️ The active utility now lives in `cfg.active_utility_id` — switching utilities is still
    /// document-op-free, but it must actually persist.
    #[test]
    fn set_active_utility_emits_no_document_mutations_but_persists_in_config() {
        let mut app = note_app();
        let before = app.projection().expect("projection");
        let result = dispatch(&mut app, NoteCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "pencil".into() }));
        assert!(result.document_mutations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.projection().expect("projection"), before, "utility switching does not mutate the document");
        assert!(render(&mut app, NOTE_PLAY_BODY_PROPERTIES).contains("Utility: pencil"), "cfg.active_utility_id reflects the switch");
    }

    #[test]
    fn world_pick_style_registry_enforcement_allows_the_active_utility_switch() {
        // 🧬️ Mirrors `shooting_ui`'s registry-backed coverage: dispatching through
        // `new_app_with_registry` exercises `AppActionRegistry` kind discipline for a View command.
        let mut app = note_app_with_registry();
        let result = dispatch(&mut app, NoteCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "pencil".into() }));
        assert!(result.document_mutations.is_empty(), "SetActiveUtility (View) emits no operations even under registry enforcement");
    }
}
//#endregion 🧪️Tests

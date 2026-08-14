//! 👁️ 👁️ Remodel play app commands command — `set-selection`.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation, RemodelWorldCamera};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "selection")]
pub struct SetSelection {
    pub mode: String,
    pub ids: Vec<String>,
}

pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelConfigMutation::SetSelection { mode: payload.mode.clone(), ids: payload.ids.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::commands::{set_active_utility, set_camera, set_frame_cursor, set_layer_visibility, set_locale, set_report_table};
    use crate::apps::remodel::testkit::{app, dispatch};
    use crate::apps::remodel::RemodelCommand;

    #[test]
    fn view_actions_emit_config_mutations_not_artifact_mutations() {
        let mut app = app();
        let result = dispatch(&mut app, RemodelCommand::SetCamera(set_camera::SetCamera { camera: RemodelWorldCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 } }));
        assert!(result.mutations.is_empty());
        let result = dispatch(&mut app, RemodelCommand::SetLayerVisibility(set_layer_visibility::SetLayerVisibility { layer: "dense".into(), visible: false }));
        assert!(result.mutations.is_empty());
        let result = dispatch(&mut app, RemodelCommand::SetSelection(SetSelection { mode: "rectangle".into(), ids: vec!["a".into()] }));
        assert!(result.mutations.is_empty());
        let result = dispatch(&mut app, RemodelCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 }));
        assert!(result.mutations.is_empty());
        let result = dispatch(&mut app, RemodelCommand::SetReportTable(set_report_table::SetReportTable { table: "gcps".into() }));
        assert!(result.mutations.is_empty());
    }

    #[test]
    fn set_active_utility_switches_host_view_state_without_ops_or_history() {
        let mut app = app();
        let result = dispatch(&mut app, RemodelCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "measure".into() }));
        assert!(result.mutations.is_empty(), "utility switch is host-owned config state, never a document operation");
    }

    /// 🗣️ `setLocale` rewrites the config's locale tag, which is what every panel's label resolution
    /// reads — asserted through rendered output, since `VcsArtifactApp` exposes no config accessor.
    #[test]
    fn set_locale_switches_the_rendered_label_language() {
        use crate::apps::remodel::testkit::render;
        let mut app = app();
        assert!(render(&mut app, crate::apps::remodel::panels::quality::REMODEL_PLAY_BODY_QC).contains("No quality report yet"));
        dispatch(&mut app, RemodelCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        assert!(render(&mut app, crate::apps::remodel::panels::quality::REMODEL_PLAY_BODY_QC).contains("Noch kein"));
    }
}
//#endregion 🧪️Tests

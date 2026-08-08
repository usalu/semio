//! 👁️ Remodel play app commands — config-only view state (was the ephemeral `RemodelPlayRuntime`).
//! Every row here emits `config_mutations`, never document operations.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation, RemodelWorldCamera};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "selection")]
    pub struct SetSelection {
        pub mode: String,
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::config(vec![RemodelConfigMutation::SetSelection { mode: payload.mode.clone(), ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(block)]
        pub camera: RemodelWorldCamera,
    }

    pub fn handle(payload: &SetCamera, _doc: &DocumentView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::config(vec![RemodelConfigMutation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️SetCamera

//#region 🔖️SetLayerVisibility
pub mod set_layer_visibility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "layer-visibility")]
    pub struct SetLayerVisibility {
        pub layer: String,
        pub visible: bool,
    }

    pub fn handle(payload: &SetLayerVisibility, _doc: &DocumentView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::config(vec![RemodelConfigMutation::SetLayerVisibility { layer: payload.layer.clone(), visible: payload.visible }]))
    }
}
//#endregion 🔖️SetLayerVisibility

//#region 🔖️SetFrameCursor
pub mod set_frame_cursor {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "frame-cursor")]
    pub struct SetFrameCursor {
        #[serde(default)]
        pub stream_id: Option<String>,
        pub frame_index: u32,
    }

    pub fn handle(payload: &SetFrameCursor, _doc: &DocumentView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::config(vec![RemodelConfigMutation::SetFrameCursor { stream_id: payload.stream_id.clone(), frame_index: payload.frame_index }]))
    }
}
//#endregion 🔖️SetFrameCursor

//#region 🔖️SetReportTable
pub mod set_report_table {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "report-table")]
    pub struct SetReportTable {
        pub table: String,
    }

    pub fn handle(payload: &SetReportTable, _doc: &DocumentView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::config(vec![RemodelConfigMutation::SetReportTable { table: payload.table.clone() }]))
    }
}
//#endregion 🔖️SetReportTable

//#region 🔖️SetActiveUtility
pub mod set_active_utility {
    use super::*;

    /// 🧰️ The host-injected `setActiveUtility` action (framework-owned id — see
    /// `semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID`), routed into remodel's own config.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-utility")]
    pub struct SetActiveUtility {
        pub utility_id: String,
    }

    pub fn handle(payload: &SetActiveUtility, _doc: &DocumentView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::config(vec![RemodelConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
    }
}
//#endregion 🔖️SetActiveUtility

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::config(vec![RemodelConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::testkit::{app, dispatch};
    use crate::apps::remodel::RemodelCommand;

    #[test]
    fn view_actions_emit_config_mutations_not_document_mutations() {
        let mut app = app();
        let result = dispatch(&mut app, RemodelCommand::SetCamera(set_camera::SetCamera { camera: RemodelWorldCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 } }));
        assert!(result.mutations.is_empty());
        let result = dispatch(&mut app, RemodelCommand::SetLayerVisibility(set_layer_visibility::SetLayerVisibility { layer: "dense".into(), visible: false }));
        assert!(result.mutations.is_empty());
        let result = dispatch(&mut app, RemodelCommand::SetSelection(set_selection::SetSelection { mode: "rectangle".into(), ids: vec!["a".into()] }));
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
    /// reads — asserted through rendered output, since `VcsDocumentApp` exposes no config accessor.
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

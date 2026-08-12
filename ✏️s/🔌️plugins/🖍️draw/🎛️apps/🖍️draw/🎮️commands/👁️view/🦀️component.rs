//! 👁️ Draw play app commands — host-owned/ephemeral view state vocabulary (constitutional: was
//! `ui`'s `ConfigOnly` region, plus `engagementSubmit`, a content-mutating rename command).

use crate::apps::draw::commands::canvas::DrawSession;
use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::engine::{flatten_draw_layers, layer_id};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️EngagementSubmit
pub mod engagement_submit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-submit")]
    pub struct EngagementSubmit {
        pub value: Option<String>,
    }

    /// ✏️ Renames the single selected layer to the submitted engagement-input text (or the config's
    /// own in-progress `engagement_input` if the caller doesn't pass one) — the one `Config`-only
    /// row that actually mutates the document, mirroring the pre-migration behaviour exactly.
    pub fn handle(payload: &EngagementSubmit, _doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        let config = cfg.snapshot;
        let value = payload.value.clone().unwrap_or_else(|| config.engagement_input.clone());
        let value = value.trim();
        if value.is_empty() || config.selected_ids.len() != 1 {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(vec![crate::artifacts::draw::mutations::rename_layer(config.selected_ids[0].clone(), value.into())]))
    }
}
//#endregion 🔖️EngagementSubmit

//#region 🔖️SetActiveUtility
pub mod set_active_utility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-utility")]
    pub struct SetActiveUtility {
        pub utility_id: String,
    }

    /// 🧰️ Host-owned utility switch: clear any in-progress gesture scratch (discarding any
    /// document-op the FSM would produce — `UtilityChanged` never carries one).
    pub fn handle(payload: &SetActiveUtility, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        let document = doc.snapshot;
        let mut config = cfg.snapshot.clone();
        session.step_gesture(crate::apps::draw::commands::canvas::draw_gesture::Event::UtilityChanged, document, &mut config);
        Ok(Emit::config(vec![DrawConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
    }
}
//#endregion 🔖️SetActiveUtility

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(block)]
        pub camera: DrawCamera,
    }

    /// 📷️ Camera — session-only runtime pose, never a document operation.
    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        Ok(Emit::config(vec![DrawConfigMutation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️SetCamera

//#region 🔖️SetCameraZoom
pub mod set_camera_zoom {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera-zoom")]
    pub struct SetCameraZoom {
        pub value: f64,
    }

    pub fn handle(payload: &SetCameraZoom, _doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        let config = cfg.snapshot;
        let camera = DrawCamera { zoom: payload.value, ..config.camera.clone() };
        Ok(Emit::config(vec![DrawConfigMutation::SetCamera { camera }]))
    }
}
//#endregion 🔖️SetCameraZoom

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        Ok(Emit::config(vec![DrawConfigMutation::SetSelection { ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️SetHover
pub mod set_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-hover")]
    pub struct SetHover {
        pub id: Option<String>,
    }

    pub fn handle(payload: &SetHover, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        Ok(Emit::config(vec![DrawConfigMutation::SetHovered { id: payload.id.clone() }]))
    }
}
//#endregion 🔖️SetHover

//#region 🔖️SelectAll
pub mod select_all {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-all")]
    pub struct SelectAll {}

    pub fn handle(_payload: &SelectAll, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        let document = doc.snapshot;
        let ids = flatten_draw_layers(&document.layers).into_iter().map(|layer| layer_id(layer).to_string()).collect();
        Ok(Emit::config(vec![DrawConfigMutation::SetSelection { ids }]))
    }
}
//#endregion 🔖️SelectAll

//#region 🔖️ClearSelection
pub mod clear_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "clear-selection")]
    pub struct ClearSelection {}

    pub fn handle(_payload: &ClearSelection, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        Ok(Emit::config(vec![DrawConfigMutation::SetSelection { ids: Vec::new() }]))
    }
}
//#endregion 🔖️ClearSelection

//#region 🔖️EngagementInput
pub mod engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-input")]
    pub struct EngagementInput {
        pub value: String,
    }

    pub fn handle(payload: &EngagementInput, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        Ok(Emit::config(vec![DrawConfigMutation::SetEngagementInput { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️EngagementInput

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        Ok(Emit::config(vec![DrawConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

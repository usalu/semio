//! 🧮️ Draw play app — view state (constitutional: was `engine`'s `Config` struct + `op`'s
//! `ConfigOperation`, split out per the taxonomy recipe: view state lives at app level, not artifact).

use crate::artifacts::draw::DrawCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ B1: draw's real `DocumentApp::Config` — absorbs every former `DrawInteractionState`
/// (`ui`-crate `RefCell`) field (selection, hover, in-progress engagement-input text, the
/// session-only free viewport camera) plus the two former `ViewModel`-driven fields the draw UI
/// actually reads (`active_utility_id`/`locale` — mirrors `shooting_engine::ShootingConfig`'s
/// identical B1 migration) — session view state now round-trips through the config `DocumentStore`
/// exactly like document content, with a real `backwards` per `DrawConfigOperation` instead of
/// never being VCS'd at all.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "drawcfg")]
#[dsl(layout = "lines")]
pub struct DrawConfig {
    /// 👁️ Selected layer ids — was `DrawInteractionState::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 👁️ Hovered layer id — was `DrawInteractionState::hovered_id`.
    pub hovered_id: Option<String>,
    /// 👁️ In-progress rename/engagement input text — was `DrawInteractionState::engagement_input`.
    pub engagement_input: String,
    /// 🎥️ The free/live canvas camera — session-only, never a document field. Was
    /// `DrawInteractionState::camera`.
    #[dsl(block)]
    pub camera: DrawCamera,
    /// 🧰️ The active canvas utility — was read off `view_state.active_utility_id` (host-pushed
    /// `ViewModel`, deleted by B1). Default mirrors the pre-migration `DRAW_DEFAULT_UTILITY`
    /// (`"selectDirect"`).
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for DrawConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), hovered_id: None, engagement_input: String::new(), camera: DrawCamera::default(), active_utility_id: "selectDirect".into(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(DrawConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `DrawConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `DrawInteractionState` field writes), plus a generic `Snapshot` every variant's
/// `backwards()` returns: since a config-only "View" dispatch is a plain `Apply` (not an
/// `AmendLast`), each tick is its own distinct, real config edit, and "undo this tick" is exactly
/// "restore the whole-config snapshot from just before it" — mirrors
/// `shooting_op::ShootingConfigOperation`'s identical shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum DrawConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: DrawConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "hover")]
    SetHovered { id: Option<String> },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: DrawCamera,
    },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<DrawConfig> for DrawConfigOperation {
    type Diff = DrawConfig;

    fn diff(&self, base: &DrawConfig) -> DrawConfig {
        let mut next = base.clone();
        match self {
            DrawConfigOperation::Snapshot { config } => return config.clone(),
            DrawConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            DrawConfigOperation::SetHovered { id } => next.hovered_id = id.clone(),
            DrawConfigOperation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            DrawConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            DrawConfigOperation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            DrawConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &DrawConfig) -> Vec<Self> {
        vec![DrawConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_config_default_matches_ui_selectdirect_utility() {
        let config = DrawConfig::default();
        assert_eq!(config.active_utility_id, "selectDirect");
        assert_eq!(config.locale, "en-US");
        assert!(config.selected_ids.is_empty());
    }

    #[test]
    fn draw_config_dsl_round_trips() {
        let config = DrawConfig {
            selected_ids: vec!["layer-1".into(), "layer-2".into()],
            hovered_id: Some("layer-3".into()),
            engagement_input: "Renaming \"layer\"".into(),
            camera: DrawCamera { x: 12.0, y: -4.0, zoom: 1.5 },
            active_utility_id: "pen".into(),
            locale: "de-DE".into(),
        };
        store::test_support::assert_dsl_round_trip(&config);
    }

    #[test]
    fn draw_config_operation_round_trips_and_backwards_restores_snapshot() {
        let base = DrawConfig { selected_ids: vec!["a".into()], active_utility_id: "selectDirect".into(), ..Default::default() };
        let operation = DrawConfigOperation::SetSelection { ids: vec!["a".into(), "b".into()] };
        let forward = operation.diff(&base);
        assert_eq!(forward.selected_ids, vec!["a".to_string(), "b".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![DrawConfigOperation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
    }

    #[test]
    fn draw_config_operation_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&DrawConfigOperation::Snapshot { config: DrawConfig::default() });
        store::test_support::assert_op_line_round_trip(&DrawConfigOperation::SetSelection { ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_line_round_trip(&DrawConfigOperation::SetHovered { id: Some("a".into()) });
        store::test_support::assert_op_line_round_trip(&DrawConfigOperation::SetHovered { id: None });
        store::test_support::assert_op_line_round_trip(&DrawConfigOperation::SetEngagementInput { value: "New \"Name\"".into() });
        store::test_support::assert_op_line_round_trip(&DrawConfigOperation::SetCamera { camera: DrawCamera { x: 1.0, y: -2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&DrawConfigOperation::SetActiveUtility { utility_id: "pen".into() });
        store::test_support::assert_op_line_round_trip(&DrawConfigOperation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests

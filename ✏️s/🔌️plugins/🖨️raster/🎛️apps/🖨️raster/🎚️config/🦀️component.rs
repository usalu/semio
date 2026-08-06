//! 🧮️ Raster app — view-state configuration (constitutional: general/config). B1: this absorbs every
//! former `RasterPlayRuntime` (`ui`-crate `RefCell`) field (selection, hover, brush size/opacity,
//! navigator composite-viewport size, the session-only free camera) plus the two former `ViewModel`-
//! driven fields the raster UI actually reads (`active_utility_id`/`locale` — mirrors
//! `shooting_engine::ShootingConfig`'s identical B1 migration). `RasterConfigOperation` lives here too,
//! next to the `RasterConfig` it patches (TEMPLATE.md §4).

use crate::artifacts::raster::RasterCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "rastercfg")]
#[dsl(layout = "lines")]
pub struct RasterConfig {
    /// 👁️ Selected layer ids — was `RasterPlayRuntime::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 👁️ Hovered layer id — was `RasterPlayRuntime::hovered_id`.
    pub hovered_id: Option<String>,
    /// 🖌️ Brush diameter (px) — was `RasterPlayRuntime::brush_size`.
    pub brush_size: f64,
    /// 🖌️ Brush opacity (0..1) — was `RasterPlayRuntime::brush_opacity`.
    pub brush_opacity: f64,
    /// 🔭️ Navigator's last-known composite-window viewport size — was
    /// `RasterPlayRuntime::composite_viewport`.
    #[dsl(block)]
    pub composite_viewport: Option<RasterConfigViewportSize>,
    /// 🎥️ The free/live composite camera — session-only, never a document field. Was
    /// `RasterPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: RasterCamera,
    /// 🧰️ The active composite-window utility — was read off `view_state.active_utility_id`
    /// (host-pushed `ViewModel`, deleted by B1). Default mirrors the app's `RASTER_DEFAULT_UTILITY`.
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterConfigViewportSize {
    pub width: f64,
    pub height: f64,
}

impl Default for RasterConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), hovered_id: None, brush_size: 24.0, brush_opacity: 1.0, composite_viewport: None, camera: RasterCamera::default(), active_utility_id: "selectMarquee".into(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(RasterConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `RasterConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `RasterPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — mirrors `shooting_op::ShootingConfigOperation`'s identical shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum RasterConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: RasterConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "hover")]
    SetHovered { id: Option<String> },
    #[dsl(key = "brush-size")]
    SetBrushSize { value: f64 },
    #[dsl(key = "brush-opacity")]
    SetBrushOpacity { value: f64 },
    #[dsl(key = "composite-viewport")]
    SetCompositeViewport {
        #[dsl(block)]
        viewport: Option<RasterConfigViewportSize>,
    },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: RasterCamera,
    },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<RasterConfig> for RasterConfigOperation {
    type Diff = RasterConfig;

    fn diff(&self, base: &RasterConfig) -> RasterConfig {
        let mut next = base.clone();
        match self {
            RasterConfigOperation::Snapshot { config } => return config.clone(),
            RasterConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            RasterConfigOperation::SetHovered { id } => next.hovered_id = id.clone(),
            RasterConfigOperation::SetBrushSize { value } => next.brush_size = *value,
            RasterConfigOperation::SetBrushOpacity { value } => next.brush_opacity = value.clamp(0.0, 1.0),
            RasterConfigOperation::SetCompositeViewport { viewport } => next.composite_viewport = viewport.clone(),
            RasterConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            RasterConfigOperation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            RasterConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &RasterConfig) -> Vec<Self> {
        vec![RasterConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raster_config_operation_round_trips_and_backwards_restores_snapshot() {
        let base = RasterConfig { selected_ids: vec!["a".into()], ..Default::default() };
        let operation = RasterConfigOperation::SetSelection { ids: vec!["a".into(), "b".into()] };
        let forward = operation.diff(&base);
        assert_eq!(forward.selected_ids, vec!["a".to_string(), "b".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![RasterConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&forward), base);
    }

    #[test]
    fn raster_config_operation_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::Snapshot { config: RasterConfig::default() });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetSelection { ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetHovered { id: Some("a".into()) });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetHovered { id: None });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetBrushSize { value: 40.0 });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetBrushOpacity { value: 0.5 });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetCompositeViewport { viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }) });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetCompositeViewport { viewport: None });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetCamera { camera: RasterCamera { x: 1.0, y: -2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetActiveUtility { utility_id: "paintBrush".into() });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetLocale { value: "de-DE".into() });
    }

    #[test]
    fn raster_config_default_matches_ui_selectmarquee_utility() {
        let config = RasterConfig::default();
        assert_eq!(config.active_utility_id, "selectMarquee");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.brush_size, 24.0);
        assert_eq!(config.brush_opacity, 1.0);
    }

    #[test]
    fn raster_config_dsl_round_trips() {
        let config = RasterConfig {
            selected_ids: vec!["l1".into(), "l2".into()],
            hovered_id: Some("l3".into()),
            brush_size: 40.0,
            brush_opacity: 0.5,
            composite_viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }),
            camera: RasterCamera { x: 5.0, y: -3.0, zoom: 2.0 },
            active_utility_id: "paintBrush".into(),
            locale: "de-DE".into(),
        };
        store::test_support::assert_dsl_round_trip(&config);
    }
}
//#endregion 🧪️Tests

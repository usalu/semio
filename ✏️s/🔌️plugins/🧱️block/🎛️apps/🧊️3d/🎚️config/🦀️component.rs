//! 🧮️ Block 3D play app — the view-state config artifact and its operation enum, plus the per-window
//! view record (`Block3dWindowView`) and transient brush-preview pose (`Block3dBrushPreview`) nested
//! inside it. Session-only but real, undoable config: it round-trips through the config `DocumentStore`
//! exactly like document content, with a true `backwards` per operation. Nothing here is document
//! state — the object kind's identity/representations/vortices live in `crate::artifacts::block3d`.

use crate::core::BlockCamera3d;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️WindowView
/// 🪟️ Per-window-instance view state (representation subset, layout, active utility).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block3dWindowView {
    pub window_id: String,
    #[serde(default)]
    pub representation_ids: Vec<String>,
    #[serde(default = "default_arrangement")]
    pub arrangement: String,
    #[serde(default = "default_spacing")]
    pub spacing: f64,
    #[serde(default = "default_active_utility")]
    pub active_utility: String,
}

fn default_arrangement() -> String {
    "overlap".into()
}

fn default_spacing() -> f64 {
    8.0
}

fn default_active_utility() -> String {
    crate::apps::block3d::BLOCK3D_UTILITY_SELECT.into()
}

impl Block3dWindowView {
    pub fn for_window(window_id: impl Into<String>) -> Self {
        Self { window_id: window_id.into(), representation_ids: Vec::new(), arrangement: default_arrangement(), spacing: default_spacing(), active_utility: default_active_utility() }
    }
}

/// 🖌️ Transient brush hover pose in world space (config-only).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block3dBrushPreview {
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(dir)]
    pub direction: [f64; 3],
}
//#endregion 🔖️WindowView

//#region 🔖️Config
/// 🧮️ `Block3dPlayApp`'s real `DocumentApp::Config` — B1 pure-trait conversion. Absorbs every former
/// `Block3dPlayApp` `RefCell` runtime field (`selected_ids`/`active_representation_id`) plus the
/// locale this app resolves itself. `wanted_tags` is ready for whenever a later wave threads `cfg`
/// into `export_media` (see that fn's doc for why it's currently unused there).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "block3dcfg")]
#[dsl(layout = "lines")]
pub struct Block3dConfig {
    /// 👁️ Multi-selected row ids in the document tree — was `Block3dPlayApp::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 👁️ The representation shown in the inspector's representation select — was
    /// `Block3dPlayApp::active_representation_id`.
    pub active_representation_id: Option<String>,
    /// 🏷️ Tag filter for `puzzle3d_catalog_fragment`'s active-representation resolution. Empty means
    /// "all tags".
    pub wanted_tags: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewModel.locale`.
    pub locale: String,
    #[serde(default)]
    #[dsl(table)]
    pub windows: Vec<Block3dWindowView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brush_vortex_kind_id: Option<String>,
    #[serde(default = "default_brush_radius")]
    pub brush_radius: f64,
    #[serde(default)]
    pub brush_flip: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brush_preview: Option<Block3dBrushPreview>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<BlockCamera3d>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovered_vortex_full_id: Option<String>,
}

fn default_brush_radius() -> f64 {
    0.3
}

impl Default for Block3dConfig {
    fn default() -> Self {
        Self {
            selected_ids: Vec::new(),
            active_representation_id: None,
            wanted_tags: Vec::new(),
            locale: "en-US".into(),
            windows: Vec::new(),
            brush_vortex_kind_id: None,
            brush_radius: default_brush_radius(),
            brush_flip: false,
            brush_preview: None,
            camera: None,
            hovered_vortex_full_id: None,
        }
    }
}

store::impl_whole_record_config!(Block3dConfig);

//#region 🔖️Accessors
pub fn block3d_window_view(config: &Block3dConfig, window_id: &str) -> Block3dWindowView {
    config.windows.iter().find(|row| row.window_id == window_id).cloned().unwrap_or_else(|| Block3dWindowView::for_window(window_id))
}

pub fn block3d_active_utility(config: &Block3dConfig, window_id: &str) -> String {
    block3d_window_view(config, window_id).active_utility
}

pub fn upsert_window_view_index(windows: &mut Vec<Block3dWindowView>, window_id: &str) -> usize {
    if let Some(index) = windows.iter().position(|row| row.window_id == window_id) {
        return index;
    }
    windows.push(Block3dWindowView::for_window(window_id));
    windows.len() - 1
}
//#endregion 🔖️Accessors
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `Block3dConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `Block3dPlayApp` `RefCell` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns.
// 🧯️ `large_enum_variant`: `Snapshot` deliberately carries the WHOLE `Block3dConfig` while every other
// row carries one or two scalars — that whole-config snapshot IS the inverse mechanism every variant's
// `backwards()` returns. Boxing it would change the derived `dsl::DslOps` wire encoding, which this
// migration must preserve byte-for-byte, so the size skew is accepted by design (same tradeoff as gis's
// `Gis2dConfigOperation`).
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Block3dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Block3dConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "active-representation")]
    SetActiveRepresentation { representation_id: Option<String> },
    #[dsl(key = "wanted-tags")]
    SetWantedTags { tags: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "window-representations")]
    SetWindowRepresentations { window_id: String, representation_ids: Vec<String> },
    #[dsl(key = "toggle-window-representation")]
    ToggleWindowRepresentation { window_id: String, representation_id: String, visible: bool },
    #[dsl(key = "window-arrangement")]
    SetWindowArrangement { window_id: String, arrangement: String },
    #[dsl(key = "window-spacing")]
    SetWindowSpacing { window_id: String, spacing: f64 },
    #[dsl(key = "active-utility")]
    SetActiveUtility { window_id: String, utility_id: String },
    #[dsl(key = "brush-vortex-kind")]
    SetBrushVortexKind { vortex_kind_id: Option<String> },
    #[dsl(key = "brush-radius")]
    SetBrushRadius { radius: f64 },
    #[dsl(key = "brush-flip")]
    SetBrushFlip { flip: bool },
    #[dsl(key = "brush-preview")]
    SetBrushPreview { preview: Option<Block3dBrushPreview> },
    #[dsl(key = "camera")]
    SetCamera { camera: BlockCamera3d },
    #[dsl(key = "hovered-vortex")]
    SetHoveredVortexFullId { full_id: Option<String> },
}

impl Operation<Block3dConfig> for Block3dConfigOperation {
    type Diff = Block3dConfig;

    fn diff(&self, base: &Block3dConfig) -> Block3dConfig {
        let mut next = base.clone();
        match self {
            Block3dConfigOperation::Snapshot { config } => return config.clone(),
            Block3dConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            Block3dConfigOperation::SetActiveRepresentation { representation_id } => next.active_representation_id = representation_id.clone(),
            Block3dConfigOperation::SetWantedTags { tags } => next.wanted_tags = tags.clone(),
            Block3dConfigOperation::SetLocale { value } => next.locale = value.clone(),
            Block3dConfigOperation::SetWindowRepresentations { window_id, representation_ids } => {
                let index = upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].representation_ids = representation_ids.clone();
            }
            Block3dConfigOperation::ToggleWindowRepresentation { window_id, representation_id, visible } => {
                let index = upsert_window_view_index(&mut next.windows, window_id);
                let row = &mut next.windows[index];
                if *visible {
                    if !row.representation_ids.contains(representation_id) {
                        row.representation_ids.push(representation_id.clone());
                    }
                } else {
                    row.representation_ids.retain(|id| id != representation_id);
                }
            }
            Block3dConfigOperation::SetWindowArrangement { window_id, arrangement } => {
                let index = upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].arrangement = arrangement.clone();
            }
            Block3dConfigOperation::SetWindowSpacing { window_id, spacing } => {
                let index = upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].spacing = *spacing;
            }
            Block3dConfigOperation::SetActiveUtility { window_id, utility_id } => {
                let index = upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].active_utility = utility_id.clone();
            }
            Block3dConfigOperation::SetBrushVortexKind { vortex_kind_id } => next.brush_vortex_kind_id = vortex_kind_id.clone(),
            Block3dConfigOperation::SetBrushRadius { radius } => next.brush_radius = *radius,
            Block3dConfigOperation::SetBrushFlip { flip } => next.brush_flip = *flip,
            Block3dConfigOperation::SetBrushPreview { preview } => next.brush_preview = preview.clone(),
            Block3dConfigOperation::SetCamera { camera } => next.camera = Some(camera.clone()),
            Block3dConfigOperation::SetHoveredVortexFullId { full_id } => next.hovered_vortex_full_id = full_id.clone(),
        }
        next
    }

    fn backwards(&self, base: &Block3dConfig) -> Vec<Self> {
        vec![Block3dConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block3d_config_default_has_no_selection_and_all_tags() {
        let config = Block3dConfig::default();
        assert!(config.selected_ids.is_empty());
        assert!(config.active_representation_id.is_none());
        assert!(config.wanted_tags.is_empty());
        assert_eq!(config.locale, "en-US");
        assert!(config.windows.is_empty());
        assert_eq!(config.brush_radius, 0.3);
    }

    #[test]
    fn config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = Block3dConfig::default();
        let operation = Block3dConfigOperation::SetSelection { ids: vec!["r0".into()] };
        let next = operation.diff(&base);
        assert_eq!(next.selected_ids, vec!["r0".to_string()]);
        let inverse = operation.backwards(&base);
        assert_eq!(inverse, vec![Block3dConfigOperation::Snapshot { config: base.clone() }]);
        let restored = inverse[0].diff(&next);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests

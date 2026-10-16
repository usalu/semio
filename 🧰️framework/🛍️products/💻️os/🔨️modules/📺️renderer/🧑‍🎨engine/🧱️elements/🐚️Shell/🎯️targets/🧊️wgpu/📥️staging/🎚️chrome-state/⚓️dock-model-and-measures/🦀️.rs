//! 🎚️ `shell-chrome-state` staging of `dock-model-and-measures`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧭️PanelAnchorModel
/// 🗄️ One persisted tab id plus, for a branch, its persisted child order, or, for a leaf, its tree-unit
/// order — byte-shape identical to React's `DockTabSkeleton` (`🔨️modules/🖥️platform/🟦️.ts`).
///
/// 🌳️ `trees` is React's `PanelTreeUnit` id list (`🧭️PanelTabBar/🟦️.tsx`'s `panelTabNodeToSkeleton`). A
/// wgpu leaf hosts exactly ONE retained document, so its unit list is always the single
/// `"<leafId>.tree"` id React's `singleTreeLeaf` mints — written so a skeleton this renderer saves
/// round-trips through React's `applyDockSkeleton` unit resolution instead of losing every leaf's
/// units, and read back only to be validated, since there is no second unit to reorder.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DockTabSkeleton {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DockTabSkeleton>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trees: Option<Vec<String>>,
}

/// 🗄️ The full persisted dock arrangement, one tab tree per anchor — React's `DockSkeleton`, stored
/// under the same `semio.os.config` document's `dockLayouts` so a layout survives a renderer switch.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DockSkeleton {
    pub version: u8,
    pub anchors: BTreeMap<String, Vec<DockTabSkeleton>>,
}

impl Default for DockSkeleton {
    fn default() -> Self {
        Self { version: DOCK_SKELETON_VERSION, anchors: BTreeMap::new() }
    }
}

/// 🗄️ One anchor's persisted chrome state — only the fields that differ from the shell's computed
/// defaults are ever stored, byte-shape identical to React's `DockUiPanelState`.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DockUiPanelState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
}

/// 🗄️ The persisted per-anchor panel chrome — React's `DockUiState`, same `semio.os.config` document,
/// same `dockUi` layer split (a per-app entry wins over the shared `os` one).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DockUiState {
    pub version: u8,
    #[serde(default)]
    pub anchors: BTreeMap<String, DockUiPanelState>,
}

impl Default for DockUiState {
    fn default() -> Self {
        Self { version: DOCK_SKELETON_VERSION, anchors: BTreeMap::new() }
    }
}

//#endregion 🧭️PanelAnchorModel
//#endregion ShellLifecycle


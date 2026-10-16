//! 🎛️ `shell-prefs` staging of `dock-model-and-measures`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧭️PanelAnchorModel
/// 🧭️ One node of an anchor's tab tree — a leaf when `children` is empty, otherwise a collapsible
/// branch, the Rust twin of React's `PanelTabNode` (`🧭️PanelTabBar/🟦️.tsx`). A leaf's `id` is the
/// panel surface id whose retained document the open panel paints.
#[derive(Clone, Debug, PartialEq)]
pub struct DockTabNode {
    pub id: String,
    pub label: String,
    pub icon_id: String,
    pub order: i32,
    pub children: Vec<DockTabNode>,
}

impl DockTabNode {
    pub fn leaf(id: impl Into<String>, label: impl Into<String>, icon_id: impl Into<String>, order: i32) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), order, children: Vec::new() }
    }

    pub fn branch(id: impl Into<String>, label: impl Into<String>, icon_id: impl Into<String>, order: i32, children: Vec<DockTabNode>) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), order, children }
    }

    pub fn is_branch(&self) -> bool {
        !self.children.is_empty()
    }

    pub(crate) fn contains(&self, id: &str) -> bool {
        self.id == id || self.children.iter().any(|child| child.contains(id))
    }
}

//#endregion 🧭️PanelAnchorModel
//#endregion ShellLifecycle


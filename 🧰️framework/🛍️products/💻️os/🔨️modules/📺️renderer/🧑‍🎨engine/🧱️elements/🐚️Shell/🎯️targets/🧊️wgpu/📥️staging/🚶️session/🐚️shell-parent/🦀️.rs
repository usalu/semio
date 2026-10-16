//! 🚶️ `shell-session` staging of `shell-parent`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
/// 🧭️ Projects the structural fields observed by React Mode without presentation-only titles.
pub fn dock_mode_layout_identity(layout: &WindowLayout) -> WindowLayout {
    use ui_wgpu::wgpu::{WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode};
    fn stack(node: &mut WindowLayoutStackNode) {
        node.active_window_kind_id = None;
        for leaf in &mut node.children {
            leaf.title = None;
            leaf.window_kind_id = leaf.instance_id.take().unwrap_or_else(|| leaf.window_kind_id.clone());
            leaf.template_id = None;
        }
    }
    fn axis(node: &mut WindowLayoutAxisNode) {
        for child in &mut node.children {
            match child {
                WindowLayoutChild::Axis(child) => axis(child),
                WindowLayoutChild::Stack(child) => stack(child),
            }
        }
    }
    let mut projected = layout.clone();
    match &mut projected.root {
        WindowLayoutRoot::Axis(node) => axis(node),
        WindowLayoutRoot::Stack(node) => stack(node),
    }
    projected
}

//#endregion ShellTypes


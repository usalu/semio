//! 🚶️ `shell-session` staging of `shell-parent`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
/// 🧭️ Projects the structural fields observed by React Mode without presentation-only titles.
pub fn dock_mode_layout_identity(layout: &WindowLayout) -> WindowLayout {
    use semio_ui_declaration::layout::WindowLayoutNode;
    fn project(node: &mut WindowLayoutNode) {
        match node {
            WindowLayoutNode::Window { window_kind_id, title, instance_id, template_id, .. } => {
                *title = None;
                *window_kind_id = instance_id.take().unwrap_or_else(|| window_kind_id.clone());
                *template_id = None;
            }
            WindowLayoutNode::Stack { active_window_id, children, .. } => {
                *active_window_id = None;
                children.iter_mut().for_each(project);
            }
            WindowLayoutNode::Split { children, .. } => children.iter_mut().for_each(project),
        }
    }
    let mut projected = layout.clone();
    project(&mut projected.root);
    projected
}

//#endregion ShellTypes


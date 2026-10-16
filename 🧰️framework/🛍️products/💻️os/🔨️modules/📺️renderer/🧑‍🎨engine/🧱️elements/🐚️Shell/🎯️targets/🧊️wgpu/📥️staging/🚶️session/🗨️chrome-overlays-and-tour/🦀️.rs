//! 🚶️ `shell-session` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
/// 🧰️ Whether `target_id` (a bare Button/Toggle utility id) exists anywhere in `nodes`, recursing through
/// nested `Collection`s.
fn utility_node_contains(nodes: &[UtilityNode], target_id: &str) -> bool {
    nodes.iter().any(|node| match node {
        UtilityNode::Button { id, .. } | UtilityNode::Toggle { id, .. } => id == target_id,
        UtilityNode::Collection { id, children, .. } => id == target_id || utility_node_contains(children, target_id),
        UtilityNode::Separator { .. } => false,
    })
}

/// 🧰️ Ancestor `Collection` ids (root → immediate parent) that must be expanded for `target_id` to be
/// visible in the footer utility bar — empty when `target_id` isn't nested in any collection (already
/// visible) or doesn't exist. Pure so it's independently testable from the frame-reveal wiring.
pub(crate) fn utility_collection_path_to_id(nodes: &[UtilityNode], target_id: &str) -> Vec<String> {
    for node in nodes {
        if let UtilityNode::Collection { id, children, .. } = node {
            if utility_node_contains(children, target_id) {
                let mut path = vec![id.clone()];
                path.extend(utility_collection_path_to_id(children, target_id));
                return path;
            }
        }
    }
    Vec::new()
}

//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome


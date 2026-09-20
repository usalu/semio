//! ♿️ The accessibility projection this target publishes for an assistive technology to read.
//!
//! A DOM renderer gets accessibility for free: React's `Interpreter` turns each record's
//! [`ui_contract::AccessibilitySpec`] into real `aria-label`/`aria-describedby`/`aria-live`
//! attributes on an element whose tag already carries the role. A GPU canvas has no elements at
//! all, so it must SAY the same tree out loud — that is this module: one flat, ordered projection
//! built from the same retained document the paint walk consumes, published to the host, and
//! mirrored there into an offscreen ARIA subtree beside the canvas.
//!
//! The per-node semantics (the role a `Component` implies, whether it takes focus, the wire
//! spelling of a liveness level) deliberately live ONE level up, in `ui_contract`'s own
//! `♿️accessibility` region, so this target and every other answer the same shared fixture rather
//! than each inventing a role vocabulary. What lives here is only what needs the arena: tree order,
//! depth, live focus and the laid-out rect.
//!
//! Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap #3 of `📓️audit-wgpu-parity-2026-09-13.md`.

use crate::wgpu::tree::{NodeFlags, UiTree};
use ui_contract::{accessibility_projection_node, AccessibilityProjectionNode, UiNodeId, UI_DOCUMENT_NODES};

//#region ♿️Projection

/// 🪜️ The deepest document the projection walks in one pass — the same ceiling the document
/// reconcile itself carries (`UI_DOCUMENT_RECONCILE_DEPTH`), because a tree the reconcile cannot
/// mount has nothing laid out to announce.
pub const UI_ACCESSIBILITY_PROJECTION_DEPTH: usize = 64;

/// 🧭️ One pending step of the pre-order walk: the record to project, its depth, and the absolute
/// origin its own laid-out rect is relative to.
struct PendingProjection {
    id: UiNodeId,
    depth: usize,
    origin: (f32, f32),
}

const SELECT_LISTBOX_KEY_SUFFIX: &str = "::listbox";
const SELECT_OPTION_KEY_INFIX: &str = "::option::";

pub(crate) fn select_accessibility_option_value(record: &ui_contract::UiNodeRecord, key: &str) -> Option<String> {
    let ui_contract::Component::Select(select) = &record.component else { return None };
    let prefix = format!("{}{SELECT_OPTION_KEY_INFIX}", record.key.as_str());
    let value = key.strip_prefix(&prefix)?;
    select.items.iter().find(|item| item.value.as_str() == value).map(|item| item.value.as_str().to_string())
}

fn select_accessibility_nodes(record: &ui_contract::UiNodeRecord, depth: usize, owner: &AccessibilityProjectionNode) -> Vec<AccessibilityProjectionNode> {
    let ui_contract::Component::Select(select) = &record.component else { return Vec::new() };
    let mut nodes = Vec::with_capacity(select.items.len().saturating_add(1));
    let mut listbox = owner.clone();
    listbox.key = format!("{}{SELECT_LISTBOX_KEY_SUFFIX}", record.key.as_str());
    listbox.role = "listbox".to_string();
    listbox.depth = depth;
    listbox.description = None;
    listbox.shortcut = None;
    listbox.focusable = false;
    listbox.actionable = false;
    listbox.focused = false;
    listbox.checked = None;
    listbox.selected = None;
    listbox.expanded = None;
    listbox.level = None;
    listbox.rect = None;
    listbox.value_min = None;
    listbox.value_max = None;
    listbox.value_now = None;
    listbox.value_text = None;
    listbox.busy = false;
    nodes.push(listbox.clone());
    for item in select.items.iter() {
        let mut option = listbox.clone();
        option.key = format!("{}{SELECT_OPTION_KEY_INFIX}{}", record.key.as_str(), item.value.as_str());
        option.role = "option".to_string();
        option.depth = depth.saturating_add(1);
        option.label = Some(item.label.0.as_str().to_string());
        option.actionable = !record.disabled;
        option.selected = Some(item.value == select.value);
        option.value_text = Some(item.value.as_str().to_string());
        nodes.push(option);
    }
    nodes
}

/// ♿️ The accessibility tree ONE window's retained document publishes, in pre-order — the reading
/// order an assistive technology walks.
///
/// Answers an empty projection rather than a fault for a window that has published no document yet:
/// "nothing to announce" and "not laid out" are the same thing to a reader, and a probe can tell
/// them apart from the window list instead.
///
/// `focused` and `rect` are stamped from the ARENA (the retained node the record mounted to), since
/// they are live interaction state the published document itself does not carry: `rect` is absolute,
/// accumulated down the walk exactly the way `paint_node` accumulates its own origin, and comes from
/// the double-buffered `mounted_layout` the paint pass consumes — never the immediate-mode
/// `Node::layout` bucket, which stays zero on the retained path.
pub fn accessibility_projection(tree: &UiTree) -> Vec<AccessibilityProjectionNode> {
    let Some(document) = tree.document() else { return Vec::new() };
    let mut projection = Vec::new();
    let mut stack = vec![PendingProjection { id: document.root_id(), depth: 0, origin: (0.0, 0.0) }];
    while let Some(pending) = stack.pop() {
        if projection.len() >= UI_DOCUMENT_NODES || pending.depth >= UI_ACCESSIBILITY_PROJECTION_DEPTH {
            continue;
        }
        let Some(record) = document.record(pending.id) else { continue };
        let mut node = accessibility_projection_node(record, pending.depth);
        let mut origin = pending.origin;
        let mut children_visible = true;
        if let Some(mounted) = tree.document_node(pending.id) {
            if let Some(arena_node) = tree.node(mounted) {
                node.focused = arena_node.flags.contains(NodeFlags::FOCUSED);
                if node.role == "treeitem" {
                    node.selected = Some(arena_node.spec.0.presence().selected);
                }
                match &arena_node.spec.0 {
                    crate::wgpu::UiNode::Input(input) => node.value_text = Some(arena_node.state.edit.as_ref().map(|edit| edit.text.clone()).unwrap_or_else(|| input.value.clone())),
                    crate::wgpu::UiNode::Select(_) => node.expanded = Some(arena_node.state.open),
                    crate::wgpu::UiNode::Toggle(toggle) => node.checked = Some(toggle.presence.selected),
                    _ => {}
                }
                if let Some(open) = tree.disclosure_open(mounted) {
                    node.expanded = Some(open);
                    children_visible = open;
                }
            }
            if let Some((x, y, width, height)) = tree.mounted_layout(mounted) {
                origin = (pending.origin.0 + x, pending.origin.1 + y);
                node.rect = Some([origin.0, origin.1, width, height]);
            }
        }
        let select_open = tree
            .document_node(pending.id)
            .and_then(|mounted| tree.node(mounted))
            .is_some_and(|arena_node| matches!(&arena_node.spec.0, crate::wgpu::UiNode::Select(_)) && arena_node.state.open);
        projection.push(node.clone());
        if select_open {
            for virtual_node in select_accessibility_nodes(record, pending.depth, &node) {
                if projection.len() >= UI_DOCUMENT_NODES {
                    break;
                }
                projection.push(virtual_node);
            }
        }
        if children_visible {
            for index in (0..record.children.len()).rev() {
                if let Some(child) = record.children.get(index) {
                    stack.push(PendingProjection { id: *child, depth: pending.depth + 1, origin });
                }
            }
        }
    }
    projection
}

/// 🏷️ Every node a reader can actually reach and name — the subset an ARIA mirror must expose, and
/// the exact subset this target's law measures: focusable or actionable, not hidden, carrying a
/// label of its own.
pub fn accessibility_announced(projection: &[AccessibilityProjectionNode]) -> Vec<&AccessibilityProjectionNode> {
    projection.iter().filter(|node| !node.hidden && (node.focusable || node.actionable) && node.label.is_some()).collect()
}

//#endregion ♿️Projection

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-accessibility-projection/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "../../../🧪️tests/📂️retained-section-collapse/♿️.rs"]
mod retained_section_collapse_tests;

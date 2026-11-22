//! 🪟️ Reference operations over the schema-owned [`WindowLayoutNode`] tree. Paths are child-index lists; every drop path addresses the layout AFTER the dragged window or stack was removed. Verified by the shared corpus `../🧪️tests/🧫️fixtures/🔣️.json`, which the TS twin runs too.

use super::*;

pub type LayoutPath = Vec<usize>;

/// 🧭️ Dock side of a split drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LayoutSide {
    Left,
    Right,
    Top,
    Bottom,
}

/// 🎯️ Where a drag lands.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum LayoutDropZone {
    Tab { stack_path: LayoutPath, corner: WindowStackCorner, #[serde(default)] index: Option<usize> },
    Split { stack_path: LayoutPath, side: LayoutSide },
    RootSplit { side: LayoutSide },
}

/// 🧰️ One pure layout edit.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum WindowLayoutOp {
    SetActive { window_id: String },
    CloseWindow { window_id: String },
    ReorderTab { stack_path: LayoutPath, from: usize, to: usize },
    SetCorner { window_id: String, corner: WindowStackCorner },
    ApplySizes { split_path: LayoutPath, sizes: Vec<f64> },
    Reconcile { window_ids: Vec<String> },
    DropWindow { window: WindowLayoutNode, zone: LayoutDropZone },
    DropStack { source_path: LayoutPath, zone: LayoutDropZone },
}

type N = WindowLayoutNode;

pub fn empty_stack() -> N {
    N::Stack { size: None, active_window_id: None, children: vec![] }
}

pub fn window_id(node: &N) -> &str {
    match node {
        N::Window { window_kind_id, instance_id, .. } => instance_id.as_deref().unwrap_or(window_kind_id),
        _ => "",
    }
}

fn corner_of(node: &N) -> WindowStackCorner {
    match node {
        N::Window { corner, .. } => corner.unwrap_or_default(),
        _ => WindowStackCorner::TopLeft,
    }
}

fn size_of(node: &N) -> Option<f64> {
    match node {
        N::Window { .. } => None,
        N::Stack { size, .. } | N::Split { size, .. } => *size,
    }
}

fn with_size(node: N, new_size: Option<f64>) -> N {
    match node {
        N::Stack { active_window_id, children, .. } => N::Stack { size: new_size, active_window_id, children },
        N::Split { axis, children, .. } => N::Split { axis, size: new_size, children },
        window => window,
    }
}

fn with_corner(node: &N, new_corner: Option<WindowStackCorner>) -> N {
    match node {
        N::Window { window_kind_id, title, instance_id, template_id, .. } => N::Window { window_kind_id: window_kind_id.clone(), title: title.clone(), instance_id: instance_id.clone(), template_id: template_id.clone(), corner: new_corner },
        other => other.clone(),
    }
}

fn single_stack(window: &N) -> N {
    N::Stack { size: None, active_window_id: Some(window_id(window).to_string()), children: vec![with_corner(window, None)] }
}

fn split_pair(side: LayoutSide, incoming: N, existing: N, size: Option<f64>) -> N {
    let axis = if matches!(side, LayoutSide::Left | LayoutSide::Right) { Axis::Horizontal } else { Axis::Vertical };
    let children = if matches!(side, LayoutSide::Left | LayoutSide::Top) { vec![incoming, existing] } else { vec![existing, incoming] };
    N::Split { axis, size, children }
}

pub fn collect_window_ids(node: &N) -> Vec<String> {
    match node {
        N::Window { .. } => vec![window_id(node).to_string()],
        N::Stack { children, .. } => children.iter().map(|child| window_id(child).to_string()).collect(),
        N::Split { children, .. } => children.iter().flat_map(collect_window_ids).collect(),
    }
}

pub fn read_at<'a>(layout: &'a N, path: &[usize]) -> Option<&'a N> {
    let Some((head, rest)) = path.split_first() else { return Some(layout) };
    match layout {
        N::Window { .. } => None,
        N::Stack { .. } => Some(layout),
        N::Split { children, .. } => read_at(children.get(*head)?, rest),
    }
}

pub fn update_at(layout: &N, path: &[usize], updater: &dyn Fn(&N) -> N) -> N {
    let Some((head, rest)) = path.split_first() else { return updater(layout) };
    match layout {
        N::Window { .. } => layout.clone(),
        N::Stack { .. } => updater(layout),
        N::Split { axis, size, children } => {
            let Some(child) = children.get(*head) else { return layout.clone() };
            let mut next = children.clone();
            next[*head] = update_at(child, rest, updater);
            N::Split { axis: *axis, size: *size, children: next }
        }
    }
}

fn map_stacks(layout: &N, mapper: &mut dyn FnMut(&N, &[usize]) -> N, path: &mut Vec<usize>) -> N {
    match layout {
        N::Window { .. } => layout.clone(),
        N::Stack { .. } => mapper(layout, path),
        N::Split { axis, size, children } => {
            let mut next = Vec::with_capacity(children.len());
            for (index, child) in children.iter().enumerate() {
                path.push(index);
                next.push(map_stacks(child, mapper, path));
                path.pop();
            }
            N::Split { axis: *axis, size: *size, children: next }
        }
    }
}

pub fn stack_path_of(layout: &N, id: &str) -> Option<LayoutPath> {
    let mut found = None;
    map_stacks(layout, &mut |stack, path| {
        if let N::Stack { children, .. } = stack {
            if children.iter().any(|child| window_id(child) == id) {
                found = Some(path.to_vec());
            }
        }
        stack.clone()
    }, &mut vec![]);
    found
}

pub fn collapse(node: Option<N>) -> Option<N> {
    match node? {
        window @ N::Window { .. } => Some(window),
        stack @ N::Stack { .. } => match &stack {
            N::Stack { children, .. } if children.is_empty() => None,
            _ => Some(stack),
        },
        N::Split { axis, size, children } => {
            let mut kept: Vec<N> = children.into_iter().filter_map(|child| collapse(Some(child))).collect();
            match kept.len() {
                0 => None,
                1 => {
                    let only = kept.remove(0);
                    let merged = size_of(&only).or(size);
                    Some(with_size(only, merged))
                }
                _ => Some(N::Split { axis, size, children: kept }),
            }
        }
    }
}

pub fn remove_window(layout: &N, id: &str) -> Option<N> {
    match layout {
        N::Window { .. } => (window_id(layout) != id).then(|| layout.clone()),
        N::Stack { size, active_window_id, children } => {
            let kept: Vec<N> = children.iter().filter(|child| window_id(child) != id).cloned().collect();
            if kept.is_empty() {
                return None;
            }
            let active = if active_window_id.as_deref() == Some(id) { kept.first().map(|child| window_id(child).to_string()) } else { active_window_id.clone() };
            Some(N::Stack { size: *size, active_window_id: active, children: kept })
        }
        N::Split { axis, size, children } => {
            let kept: Vec<N> = children.iter().filter_map(|child| remove_window(child, id)).collect();
            if kept.is_empty() {
                return None;
            }
            collapse(Some(N::Split { axis: *axis, size: *size, children: kept }))
        }
    }
}

pub fn set_active(layout: &N, id: &str) -> N {
    map_stacks(layout, &mut |stack, _| match stack {
        N::Stack { size, active_window_id, children } if children.iter().any(|child| window_id(child) == id) && active_window_id.as_deref() != Some(id) => {
            N::Stack { size: *size, active_window_id: Some(id.to_string()), children: children.clone() }
        }
        other => other.clone(),
    }, &mut vec![])
}

fn normalize_active(node: &N) -> N {
    match node {
        N::Window { .. } => N::Stack { size: None, active_window_id: Some(window_id(node).to_string()), children: vec![node.clone()] },
        N::Stack { size, active_window_id, children } => N::Stack { size: *size, active_window_id: active_window_id.clone().or_else(|| children.first().map(|child| window_id(child).to_string())), children: children.clone() },
        N::Split { axis, size, children } => N::Split { axis: *axis, size: *size, children: children.iter().map(normalize_active).collect() },
    }
}

fn remove_absent(node: &N, allowed: &std::collections::HashSet<&str>) -> N {
    match node {
        N::Window { .. } => if allowed.contains(window_id(node)) { node.clone() } else { empty_stack() },
        N::Stack { size, active_window_id, children } => {
            let kept: Vec<N> = children.iter().filter(|child| allowed.contains(window_id(child))).cloned().collect();
            let active = active_window_id.clone().filter(|id| allowed.contains(id.as_str())).or_else(|| kept.first().map(|child| window_id(child).to_string()));
            N::Stack { size: *size, active_window_id: active, children: kept }
        }
        N::Split { axis, size, children } => {
            let kept = children.iter().map(|child| remove_absent(child, allowed)).filter(|child| !matches!(child, N::Stack { children, .. } if children.is_empty())).collect();
            N::Split { axis: *axis, size: *size, children: kept }
        }
    }
}

pub fn reconcile(layout: &N, window_ids: &[String]) -> N {
    let normalized = normalize_active(layout);
    let allowed: std::collections::HashSet<&str> = window_ids.iter().map(String::as_str).collect();
    let result = if window_ids.is_empty() { normalized } else { remove_absent(&normalized, &allowed) };
    collapse(Some(result)).unwrap_or_else(empty_stack)
}

pub fn set_corner(layout: &N, id: &str, corner: WindowStackCorner) -> N {
    match layout {
        N::Window { .. } => if window_id(layout) == id { with_corner(layout, Some(corner)) } else { layout.clone() },
        N::Stack { size, active_window_id, children } => N::Stack { size: *size, active_window_id: active_window_id.clone(), children: children.iter().map(|child| if window_id(child) == id { with_corner(child, Some(corner)) } else { child.clone() }).collect() },
        N::Split { axis, size, children } => N::Split { axis: *axis, size: *size, children: children.iter().map(|child| set_corner(child, id, corner)).collect() },
    }
}

pub fn apply_sizes(layout: &N, split_path: &[usize], sizes: &[f64]) -> N {
    update_at(layout, split_path, &|node| match node {
        N::Split { axis, size, children } => {
            let even = 1.0 / children.len() as f64;
            let next = children.iter().enumerate().map(|(index, child)| with_size(child.clone(), Some(if sizes.len() == children.len() { sizes[index] } else { even }))).collect();
            N::Split { axis: *axis, size: *size, children: next }
        }
        other => other.clone(),
    })
}

pub fn reorder_tab(layout: &N, stack_path: &[usize], from: usize, to: usize) -> N {
    update_at(layout, stack_path, &|node| match node {
        N::Stack { size, active_window_id, children } if from < children.len() => {
            let mut next = children.clone();
            let item = next.remove(from);
            next.insert(to.min(next.len()), item);
            N::Stack { size: *size, active_window_id: active_window_id.clone(), children: next }
        }
        other => other.clone(),
    })
}

fn flat_index_for_corner(children: &[N], corner: WindowStackCorner, corner_index: Option<usize>) -> usize {
    let indices: Vec<usize> = children.iter().enumerate().filter(|(_, child)| corner_of(child) == corner).map(|(index, _)| index).collect();
    match (indices.last(), corner_index) {
        (None, _) => children.len(),
        (Some(last), None) => last + 1,
        (Some(last), Some(at)) => indices.get(at).copied().unwrap_or(last + 1),
    }
}

pub fn insert_tab(layout: &N, stack_path: &[usize], window: &N, corner: WindowStackCorner, index: Option<usize>) -> N {
    update_at(layout, stack_path, &|node| match node {
        N::Stack { size, children, .. } => {
            let mut next = children.clone();
            next.insert(flat_index_for_corner(&next, corner, index), with_corner(window, Some(corner)));
            N::Stack { size: *size, active_window_id: Some(window_id(window).to_string()), children: next }
        }
        other => other.clone(),
    })
}

fn split_with(layout: &N, stack_path: &[usize], incoming: N, side: LayoutSide) -> N {
    update_at(layout, stack_path, &|node| match node {
        N::Stack { .. } => split_pair(side, incoming.clone(), node.clone(), size_of(node)),
        other => other.clone(),
    })
}

fn extract_stack(layout: &N, path: &[usize]) -> (Option<N>, Option<N>) {
    let Some(stack @ N::Stack { .. }) = read_at(layout, path) else { return (Some(layout.clone()), None) };
    let Some((index, parent_path)) = path.split_last() else { return (None, Some(stack.clone())) };
    let Some(N::Split { axis, size, children }) = read_at(layout, parent_path) else { return (Some(layout.clone()), None) };
    let rest: Vec<N> = children.iter().enumerate().filter(|(at, _)| at != index).map(|(_, child)| child.clone()).collect();
    let next_parent = collapse(Some(N::Split { axis: *axis, size: *size, children: rest }));
    if parent_path.is_empty() {
        return (next_parent, Some(stack.clone()));
    }
    let without = update_at(layout, parent_path, &|_| next_parent.clone().unwrap_or_else(empty_stack));
    (collapse(Some(without)), Some(stack.clone()))
}

fn is_stack_at(layout: &N, path: &[usize]) -> bool {
    matches!(read_at(layout, path), Some(N::Stack { .. }))
}

fn drop_window(layout: &N, window: &N, zone: &LayoutDropZone) -> N {
    let id = window_id(window);
    let without = remove_window(layout, id);
    if let LayoutDropZone::RootSplit { side } = zone {
        return split_pair(*side, single_stack(window), without.unwrap_or_else(empty_stack), None);
    }
    let Some(without) = without else { return layout.clone() };
    match zone {
        LayoutDropZone::Split { stack_path, side } if is_stack_at(&without, stack_path) => split_with(&without, stack_path, single_stack(window), *side),
        LayoutDropZone::Tab { stack_path, corner, index } if is_stack_at(&without, stack_path) => insert_tab(&without, stack_path, window, *corner, *index),
        _ => layout.clone(),
    }
}

fn drop_stack(layout: &N, source_path: &[usize], zone: &LayoutDropZone) -> N {
    let (without, stack) = extract_stack(layout, source_path);
    let Some(stack) = stack else { return layout.clone() };
    let base = without.unwrap_or_else(empty_stack);
    match zone {
        LayoutDropZone::RootSplit { side } => split_pair(*side, stack, base, None),
        LayoutDropZone::Split { stack_path, side } if is_stack_at(&base, stack_path) => split_with(&base, stack_path, stack, *side),
        LayoutDropZone::Tab { stack_path, corner, index } if is_stack_at(&base, stack_path) => {
            let N::Stack { active_window_id, children, .. } = &stack else { return layout.clone() };
            let mut result = base.clone();
            for (offset, child) in children.iter().enumerate() {
                result = insert_tab(&result, stack_path, child, *corner, index.map(|at| at + offset));
            }
            match active_window_id.clone().or_else(|| children.first().map(|child| window_id(child).to_string())) {
                Some(active) => set_active(&result, &active),
                None => result,
            }
        }
        _ => layout.clone(),
    }
}

/// 🧰️ Applies one edit to a layout and returns the new layout.
pub fn apply_window_layout_op(layout: &N, op: &WindowLayoutOp) -> N {
    match op {
        WindowLayoutOp::SetActive { window_id } => set_active(layout, window_id),
        WindowLayoutOp::CloseWindow { window_id } => remove_window(layout, window_id).unwrap_or_else(empty_stack),
        WindowLayoutOp::ReorderTab { stack_path, from, to } => reorder_tab(layout, stack_path, *from, *to),
        WindowLayoutOp::SetCorner { window_id, corner } => set_corner(layout, window_id, *corner),
        WindowLayoutOp::ApplySizes { split_path, sizes } => apply_sizes(layout, split_path, sizes),
        WindowLayoutOp::Reconcile { window_ids } => reconcile(layout, window_ids),
        WindowLayoutOp::DropWindow { window, zone } => drop_window(layout, window, zone),
        WindowLayoutOp::DropStack { source_path, zone } => drop_stack(layout, source_path, zone),
    }
}

/// ⚖️ Even row of single-window stacks (one stack for one window), each sized `1/n` and active on its window.
pub fn even_layout(window_ids: &[String]) -> N {
    let window = |id: &String| N::Window { window_kind_id: id.clone(), title: None, instance_id: None, template_id: None, corner: None };
    let stack = |id: &String, size: Option<f64>| N::Stack { size, active_window_id: Some(id.clone()), children: vec![window(id)] };
    match window_ids {
        [] => empty_stack(),
        [only] => stack(only, None),
        many => N::Split { axis: Axis::Horizontal, size: None, children: many.iter().map(|id| stack(id, Some(1.0 / many.len() as f64))).collect() },
    }
}

/// 🧱️ One window leaf, addressed by its kind and optionally by a distinct instance id.
pub fn create_window_layout(window_kind_id: impl Into<String>, title: Option<String>, instance_id: Option<String>, template_id: Option<String>) -> N {
    N::Window { window_kind_id: window_kind_id.into(), title, instance_id, template_id, corner: None }
}

/// 🗂️ One stack holding every window as a tab; no tab is active until the shell picks one.
pub fn create_stack_layout(window_kind_ids: &[String], titles: Option<&[String]>) -> WindowLayout {
    let children = window_kind_ids.iter().enumerate().map(|(index, id)| create_window_layout(id.clone(), titles.and_then(|rows| rows.get(index).cloned()), None, None)).collect();
    WindowLayout { root: N::Stack { size: None, active_window_id: None, children } }
}

/// 🆕️ One single-window stack per window along `axis`, each sized by the matching fraction of `sizes` and titled by `titles` (default: its id). A lone window is the stack itself: a split has at least two children.
pub fn create_default_layout(window_ids: &[String], axis: Axis, sizes: Option<&[f64]>, titles: Option<&[String]>) -> WindowLayout {
    let mut stacks: Vec<N> = window_ids
        .iter()
        .enumerate()
        .map(|(index, id)| N::Stack { size: sizes.and_then(|rows| rows.get(index).copied()), active_window_id: None, children: vec![create_window_layout(id.clone(), titles.and_then(|rows| rows.get(index).cloned()).or_else(|| Some(id.clone())), None, None)] })
        .collect();
    match stacks.len() {
        0 => WindowLayout { root: empty_stack() },
        1 => WindowLayout { root: with_size(stacks.remove(0), None) },
        _ => WindowLayout { root: N::Split { axis, size: None, children: stacks } },
    }
}

/// 🔎️ Every window KIND id a layout references, in tree order (a window's id may be a distinct instance id: see [`collect_window_ids`]).
pub fn collect_window_kind_ids(node: &N) -> Vec<String> {
    match node {
        N::Window { window_kind_id, .. } => vec![window_kind_id.clone()],
        N::Stack { children, .. } | N::Split { children, .. } => children.iter().flat_map(collect_window_kind_ids).collect(),
    }
}

/// 🏷️ Every extra instance's authored title keyed by instance id: only a window whose instance id differs from its kind id and carries a non-empty title.
pub fn collect_instance_titles(node: &N) -> Vec<(String, String)> {
    match node {
        N::Window { window_kind_id, title, instance_id, .. } => match (instance_id, title) {
            (Some(instance), Some(title)) if instance != window_kind_id && !title.is_empty() => vec![(instance.clone(), title.clone())],
            _ => vec![],
        },
        N::Stack { children, .. } | N::Split { children, .. } => children.iter().flat_map(collect_instance_titles).collect(),
    }
}

/// 🛡️ Checks a layout against `../🧬️schema/🔣️.json`: the root and split children are stacks or splits, stack children are windows, a split has at least two children, sizes lie in `(0, 1]`, ids are non-empty and a window carries no size. Every authored layout (plugin builders, hub compositions, named layouts) must pass.
pub fn validate_window_layout(node: &N) -> Result<(), String> {
    fn size_ok(size: &Option<f64>) -> bool {
        size.map_or(true, |value| value > 0.0 && value <= 1.0)
    }
    fn walk(node: &N, path: &mut Vec<usize>) -> Result<(), String> {
        match node {
            N::Window { .. } => Err(format!("a window may only be a stack child (at {path:?})")),
            N::Stack { size, active_window_id, children } => {
                if !size_ok(size) {
                    return Err(format!("stack size {size:?} must lie in (0, 1] (at {path:?})"));
                }
                if active_window_id.as_deref() == Some("") {
                    return Err(format!("stack active window id is empty (at {path:?})"));
                }
                for child in children {
                    match child {
                        N::Window { window_kind_id, instance_id, template_id, .. } if !window_kind_id.is_empty() && instance_id.as_deref() != Some("") && template_id.as_deref() != Some("") => {}
                        other => return Err(format!("stack children must be windows with non-empty ids, found {other:?} (at {path:?})")),
                    }
                }
                Ok(())
            }
            N::Split { size, children, .. } => {
                if !size_ok(size) {
                    return Err(format!("split size {size:?} must lie in (0, 1] (at {path:?})"));
                }
                if children.len() < 2 {
                    return Err(format!("a split needs at least two children, found {} (at {path:?})", children.len()));
                }
                for (index, child) in children.iter().enumerate() {
                    path.push(index);
                    walk(child, path)?;
                    path.pop();
                }
                Ok(())
            }
        }
    }
    walk(node, &mut vec![])
}

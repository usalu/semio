//! 🌲️ Tree metrics and window measures: the layout credits, the fault and intrinsic size, the tree row metrics and heights, the document table helpers and the tree window measures. See [`super`].

use super::*;

//#region 🧵️MountedLayoutText
pub const LAYOUT_NODE_CREDITS: usize = semio_framework_ui_wgpu_layout::flex::FLEX_NODE_CREDITS;

pub const LAYOUT_GLYPH_CREDITS: usize = 16_384;

pub const LAYOUT_DEPTH_CREDITS: usize = 64;

pub const LAYOUT_ATLAS_PAGE_CREDITS: usize = 4;

pub const LAYOUT_ATLAS_PAGE_BYTES: usize = 16 * 1024;

pub(super) const DEFAULT_TEXT_SIZE_PX: f32 = 14.0;

pub const TREE_DETAIL_HEIGHT: f32 = 160.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MountedLayoutFault {
    NodeCredits,
    GlyphCredits,
    DepthCredits,
    Stale,
    /// 📐️ The flex solver refused the tree (an insert, a link, or the solve itself) — reported as a
    /// fault rather than publishing a half-solved surface.
    Solver,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct IntrinsicSize {
    pub(super) width: f32,
    pub(super) height: f32,
}

/// 🧩️ The band a host-provided content leaf reserves, read off the slot's own `params_json`
/// (`{"hostContentHeight": <logical px>}`) and clamped to something a panel can actually hold. A slot
/// whose params carry no height (or no JSON at all) falls back to [`HOST_CONTENT_DEFAULT_ROWS`] control
/// rows, so a host that forgets to declare one still gets a visible box rather than a collapsed line.
pub fn host_content_height(params_json: &str, theme: &Theme) -> f32 {
    let declared = serde_json::from_str::<serde_json::Value>(params_json).ok().and_then(|params| params.get("hostContentHeight").and_then(serde_json::Value::as_f64)).map(|height| height as f32);
    declared.unwrap_or(theme.control_height * HOST_CONTENT_DEFAULT_ROWS).clamp(0.0, HOST_CONTENT_MAX_HEIGHT_PX)
}

/// 🖥️ Control rows a host-provided leaf reserves when its host declares no height.
const HOST_CONTENT_DEFAULT_ROWS: f32 = 6.0;

/// 🏔️ The ceiling one host-provided leaf may reserve, so a malformed number cannot ask for a band no
/// viewport could hold (the host scrolls inside its own rect instead).
const HOST_CONTENT_MAX_HEIGHT_PX: f32 = 4096.0;

/// 🌳️ The `UiTreeNode` spec that owns the row `id` mounts as — the nearest `UiNode::Tree`
/// ancestor, the same walk `events::find_tree_item_spec` does to re-derive a row's authored item.
pub fn owning_tree_spec(tree: &UiTree, id: NodeId) -> Option<&UiTreeNode> {
    let mut ancestor = tree.node(id)?.parent;
    let mut hops = 0usize;
    while let Some(candidate) = ancestor {
        if hops >= TREE_ROW_MAX_DEPTH {
            return None;
        }
        hops += 1;
        let node = tree.node(candidate)?;
        if let UiNode::Tree(tree_node) = &node.spec.0 {
            return Some(tree_node);
        }
        ancestor = node.parent;
    }
    None
}

pub fn find_tree_item<'a>(items: &'a [UiTreeItemNode], id: &str, depth: usize) -> Option<&'a UiTreeItemNode> {
    if depth >= TREE_ROW_MAX_DEPTH {
        return None;
    }
    for item in items {
        if item.id == id {
            return Some(item);
        }
        if let Some(found) = item.items.as_deref().and_then(|nested| find_tree_item(nested, id, depth + 1)) {
            return Some(found);
        }
    }
    None
}

/// 🌴️ Resolves the owning Tree's presentation and nearest authored item for one mounted node.
pub fn retained_tree_row_metrics(tree: &UiTree, id: NodeId, metrics: &TreeRowMetrics) -> TreeRowMetrics {
    let owner = match tree.node(id).map(|node| &node.spec.0) {
        Some(UiNode::Tree(owner)) => Some(owner),
        _ => owning_tree_spec(tree, id),
    };
    let Some(owner) = owner else { return *metrics };
    let scoped = metrics.with_presentation(owner.presentation);
    let mut candidate = Some(id);
    let mut depth = 0usize;
    while let Some(current) = candidate {
        if depth >= TREE_ROW_MAX_DEPTH {
            break;
        }
        depth += 1;
        let Some(node) = tree.node(current) else { break };
        if matches!(&node.spec.0, UiNode::Tree(_)) {
            break;
        }
        if let NodeKey::Explicit(key) = &node.key {
            if let Some(item) = owner.sections.iter().find_map(|section| find_tree_item(&section.items, key, 0)) {
                return scoped.for_item(item);
            }
        }
        candidate = node.parent;
    }
    scoped
}

pub fn retained_section_title_height(tree: &UiTree, id: NodeId) -> f32 {
    let Some(node) = tree.node(id) else { return 0.0 };
    let UiNode::Section(section) = &node.spec.0 else { return 0.0 };
    if section.label.is_none() {
        return 0.0;
    }
    node.first_child
        .and_then(|child| tree.accepted_layout(child))
        .map(|child| (child.y - semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_BODY_GAP).max(semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_LINE_HEIGHT))
        .or_else(|| tree.accepted_layout(id).map(|owner| (owner.height - semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_BODY_GAP - semio_framework_ui_wgpu_layout::flex::SECTION_TRAILING_MARGIN).max(semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_LINE_HEIGHT)))
        .unwrap_or(semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_LINE_HEIGHT)
}

/// 🔖️ Classifies the synthesized row `id` against the `Tree` that owns it — a section row when its
/// parent is the tree itself and its key names one of that tree's sections, an item row when its
/// parent is already a row and its key names one of that tree's items. Anything else is an ordinary
/// container that merely happens to live inside a tree, and keeps its own kind.
pub(super) fn tree_row_kind(tree: &UiTree, id: NodeId, parent_kind: Option<LayoutNodeKind>, metrics: &TreeRowMetrics) -> Option<LayoutNodeKind> {
    let parent_kind = parent_kind?;
    if !matches!(parent_kind, LayoutNodeKind::Tree { .. } | LayoutNodeKind::TreeSection { .. } | LayoutNodeKind::TreeRow { .. }) {
        return None;
    }
    let NodeKey::Explicit(key) = &tree.node(id)?.key else { return None };
    let owner = owning_tree_spec(tree, id)?;
    let metrics = retained_tree_row_metrics(tree, id, metrics);
    match parent_kind {
        LayoutNodeKind::Tree { reversed, .. } => {
            if document_table(tree, tree.node(id)?.parent?).is_some() {
                let item = owner.sections.iter().find_map(|section| find_tree_item(&section.items, key, 0))?;
                let lead = owner.sections.first().map_or(0.0, |section| window_lead(section.window.as_ref(), section.items.first(), key));
                return Some(LayoutNodeKind::TableRow { height: live_tree_item_height(tree, id, item, &metrics, 0), actions: table_actions_width_of(owner, &metrics), lead });
            }
            let section = owner.sections.iter().find(|section| &section.id == key)?;
            let expanded = tree.disclosure_open(id).unwrap_or_else(|| semio_framework_ui_wgpu_layout::layout::tree_section_default_open(section));
            let header = tree_section_header_height(section, &metrics);
            let height = live_tree_section_height(tree, id, section, &metrics);
            Some(LayoutNodeKind::TreeSection { header, height, expanded, reversed })
        }
        parent_kind => {
            if let LayoutNodeKind::TreeSection { header, reversed, .. } = parent_kind {
                let parent = tree.node(id)?.parent?;
                if is_tree_section_header_toolbar(tree, id, parent) {
                    return Some(LayoutNodeKind::TreeHeaderToolbar { header, reversed });
                }
            }
            let item = owner.sections.iter().find_map(|section| find_tree_item(&section.items, key, 0))?;
            let reversed = match parent_kind {
                LayoutNodeKind::TreeSection { reversed, .. } | LayoutNodeKind::TreeRow { reversed, .. } => reversed,
                _ => false,
            };
            if matches!(parent_kind, LayoutNodeKind::TreeSection { expanded: false, .. }) {
                return Some(LayoutNodeKind::TreeRow { row: 0.0, height: 0.0, expanded: false, reversed, lead: 0.0 });
            }
            if matches!(parent_kind, LayoutNodeKind::TreeRow { expanded: false, .. }) {
                return Some(LayoutNodeKind::TreeRow { row: 0.0, height: 0.0, expanded: false, reversed, lead: 0.0 });
            }
            let height = live_tree_item_height(tree, id, item, &metrics, 0);
            let expanded = height > 0.0 && tree.disclosure_open(id).unwrap_or(item.default_open.unwrap_or(false)) && item.has_rows();
            let lead = row_window_lead(tree, id, key, owner);
            Some(LayoutNodeKind::TreeRow { row: if expanded { metrics.row_height } else { 0.0 }, height, expanded, reversed, lead })
        }
    }
}

pub fn live_tree_item_height(tree: &UiTree, id: NodeId, item: &UiTreeItemNode, metrics: &TreeRowMetrics, depth: usize) -> f32 {
    if !item.presence.visible() {
        return 0.0;
    }
    let mut height = metrics.for_item(item).row_height;
    if tree_item_detail_node(tree, id).is_some() {
        height += TREE_DETAIL_HEIGHT;
    }
    if document_table_row(tree, id).is_some() && tree.children(id).any(|child| matches!(tree.node(child).map(|node| &node.spec.0), Some(UiNode::ComponentScene(_)))) {
        height = height.max(TREE_DETAIL_HEIGHT);
    }
    if depth >= TREE_ROW_MAX_DEPTH || !tree.disclosure_open(id).unwrap_or(item.default_open.unwrap_or(false)) {
        return height;
    }
    let (lead, trail) = tree_window_spacer_px(item.window.as_ref(), item.items.as_ref().map_or(0, Vec::len));
    height += lead + trail;
    for child in item.items.iter().flatten() {
        let Some(child_id) = tree.explicit_child(id, &child.id) else { continue };
        height += live_tree_item_height(tree, child_id, child, metrics, depth + 1);
    }
    height
}

fn tree_item_detail_node(tree: &UiTree, item: NodeId) -> Option<NodeId> {
    let document = tree.document()?;
    let record = document.record(tree.document_id(item)?)?;
    let ui_contract::Component::TreeItem(props) = &record.component else { return None };
    tree.document_node(props.detail?)
}

fn tree_section_header_toolbar_node(tree: &UiTree, section: NodeId) -> Option<NodeId> {
    let document = tree.document()?;
    let record = document.record(tree.document_id(section)?)?;
    let ui_contract::Component::TreeSection(props) = &record.component else { return None };
    tree.document_node(props.header_toolbar?)
}

fn is_tree_section_header_toolbar(tree: &UiTree, child: NodeId, parent: NodeId) -> bool {
    tree_section_header_toolbar_node(tree, parent) == Some(child)
}

pub(super) fn is_tree_item_detail(tree: &UiTree, child: NodeId, parent: NodeId) -> bool {
    tree_item_detail_node(tree, parent) == Some(child)
}

/// 📊️ The `TableProps` the mounted `Tree` node `id` renders — a table is a tree whose record is a `Component::Table`
/// (`reconcile::table_section`), and its column grid stays on that record.
pub fn document_table(tree: &UiTree, id: NodeId) -> Option<&ui_contract::TableProps> {
    let ui_contract::Component::Table(props) = &tree.document()?.record(tree.document_id(id)?)?.component else { return None };
    Some(props)
}

/// 🗄️ The `TableRowProps` of the mounted table row `id` — its cells, positional to the table's columns.
pub fn document_table_row(tree: &UiTree, id: NodeId) -> Option<&ui_contract::TableRowProps> {
    let ui_contract::Component::TableRow(props) = &tree.document()?.record(tree.document_id(id)?)?.component else { return None };
    Some(props)
}

/// 🎬️ The versioned action id the `index`-th `RowAction` of the mounted tree or table row `id` fires — its verb on the row's
/// ONE target — exactly as a click on its icon or its accessibility button dispatches it (the retained item carries only the
/// icon and the legacy descriptor). A disabled action resolves to its typed refusal, so no gesture dispatches it.
pub fn document_row_action_id(tree: &UiTree, id: NodeId, index: usize) -> Option<Result<ui_contract::ActionId, ui_contract::RowActionRefusal>> {
    let (row_actions, target) = match &tree.document()?.record(tree.document_id(id)?)?.component {
        ui_contract::Component::TreeItem(props) => (&props.row_actions, props.target.as_ref()?),
        ui_contract::Component::TableRow(props) => (&props.row_actions, props.target.as_ref()?),
        _ => return None,
    };
    Some(target.action_id(row_actions.get(index)?))
}

/// 📏️ A table's trailing actions column: one `layout::tree_row_action_rect` slot per action of its widest materialised row, as
/// React sizes its actions track by the widest action strip.
pub fn table_actions_width_of(node: &UiTreeNode, metrics: &TreeRowMetrics) -> f32 {
    semio_framework_ui_wgpu_layout::layout::table_actions_width(node.sections.iter().flat_map(|section| section.items.iter()).map(|item| item.actions.as_ref().map_or(0, Vec::len)).max().unwrap_or(0), metrics)
}

pub fn live_tree_section_height(tree: &UiTree, id: NodeId, section: &semio_framework_ui_declaration::ui::UiTreeSectionNode, metrics: &TreeRowMetrics) -> f32 {
    if !section.presence.visible() {
        return 0.0;
    }
    let header = tree_section_header_height(section, metrics);
    if !tree.disclosure_open(id).unwrap_or_else(|| semio_framework_ui_wgpu_layout::layout::tree_section_default_open(section)) {
        return header;
    }
    let (lead, trail) = tree_window_spacer_px(section.window.as_ref(), section.items.len());
    header + lead + trail + section.items.iter().filter_map(|item| tree.explicit_child(id, &item.id).map(|item_id| live_tree_item_height(tree, item_id, item, metrics, 0))).sum::<f32>()
}

/// 🪟️ The leading spacer a windowed container places before `key`, when `key` is its FIRST materialised child.
fn window_lead(window: Option<&semio_framework_ui_declaration::ui::UiTreeWindow>, first: Option<&UiTreeItemNode>, key: &str) -> f32 {
    match (window, first) {
        (Some(window), Some(first)) if first.id == key => tree_window_spacer_px(Some(window), 0).0,
        _ => 0.0,
    }
}

/// 🛤️ The leading spacer before the retained row `id` (authored key `key`): its parent is a windowed section or item.
fn row_window_lead(tree: &UiTree, id: NodeId, key: &str, owner: &UiTreeNode) -> f32 {
    let Some(NodeKey::Explicit(parent_key)) = tree.node(id).and_then(|node| node.parent).and_then(|parent| tree.node(parent)).map(|parent| &parent.key) else { return 0.0 };
    if let Some(section) = owner.sections.iter().find(|section| &section.id == parent_key) {
        return window_lead(section.window.as_ref(), section.items.first(), key);
    }
    owner.sections.iter().find_map(|section| find_tree_item(&section.items, parent_key, 0)).map_or(0.0, |parent| window_lead(parent.window.as_ref(), parent.items.as_deref().and_then(<[UiTreeItemNode]>::first), key))
}

pub fn retained_tree_height(tree: &UiTree, id: NodeId, node: &UiTreeNode, metrics: &TreeRowMetrics) -> f32 {
    let metrics = metrics.with_presentation(node.presentation);
    if document_table(tree, id).is_some() {
        let (lead, trail) = node.sections.first().map_or((0.0, 0.0), |section| tree_window_spacer_px(section.window.as_ref(), section.items.len()));
        return metrics.header_height + lead + trail + node.sections.iter().flat_map(|section| section.items.iter()).filter_map(|item| tree.explicit_child(id, &item.id).map(|row| live_tree_item_height(tree, row, item, &metrics, 0))).sum::<f32>();
    }
    node.sections
        .iter()
        .filter(|section| section.presence.visible())
        .map(|section| {
            let Some(section_id) = tree.explicit_child(id, &section.id) else { return tree_section_header_height(section, &metrics) };
            live_tree_section_height(tree, section_id, section, &metrics)
        })
        .sum()
}

//#region 🪟️TreeWindowMeasure
/// ⏱️ Node visits one measurement pass may spend walking to a surface's trees — the surface's own document ceiling several
/// times over; a tree's rows are walked through its spec, not the arena.
const TREE_WINDOW_MEASURE_NODES: usize = 4 * ui_contract::UI_DOCUMENT_NODES;

/// 🔭️ Every windowed container of `tree`, measured against the scroll viewport its first `Tree` paints in (content origin 0)
/// — the wgpu twin of `treeWindowContainersUnder` (`🗣️Interpreter/🟦️.tsx`): the key is the container's WINDOW PATH (enclosing
/// windowed containers' keys, outermost first, joined by `TREE_WINDOW_PATH_SEPARATOR`), the extent spans its spacers, rows and
/// nested content, and the rows carry their real tops. An up-flow (`reversed`) tree is measured in MIRRORED viewport space, so
/// its leading spacer and its row order read exactly like a down-flow list's. `None` when nothing is windowed.
pub fn tree_window_measures(tree: &UiTree, theme: &Theme, viewport: (f32, f32), reversed: bool) -> Option<(f64, Vec<TreeWindowContainerMeasure>)> {
    let root = tree.root?;
    let metrics = TreeRowMetrics::from_theme(theme);
    let mut frame = None;
    let mut containers = Vec::new();
    let mut pending = vec![root];
    let mut visits = 0usize;
    while let Some(id) = pending.pop() {
        visits += 1;
        if visits > TREE_WINDOW_MEASURE_NODES {
            break;
        }
        let Some(node) = tree.node(id) else { continue };
        if let UiNode::Tree(owner) = &node.spec.0 {
            let frame = *frame.get_or_insert_with(|| scroll_frame_of(tree, id).unwrap_or(semio_framework_ui_theme::geometry::Rect::new(0.0, 0.0, viewport.0, viewport.1)));
            measure_tree_windows(tree, id, owner, &metrics, frame, reversed, &mut containers);
            continue;
        }
        let children: Vec<NodeId> = tree.children(id).collect();
        pending.extend(children.into_iter().rev());
    }
    let frame = frame?;
    (!containers.is_empty()).then(|| (f64::from(frame.h), containers))
}

/// 📜️ The painted box of the nearest scrolling ancestor of `id` — the viewport its rows scroll through.
fn scroll_frame_of(tree: &UiTree, id: NodeId) -> Option<semio_framework_ui_theme::geometry::Rect> {
    let mut cursor = tree.node(id)?.parent;
    while let Some(ancestor) = cursor {
        let node = tree.node(ancestor)?;
        if node.flags.contains(NodeFlags::SCROLLABLE) {
            return tree.absolute_rect(ancestor);
        }
        cursor = node.parent;
    }
    None
}

fn measure_tree_windows(tree: &UiTree, id: NodeId, owner: &UiTreeNode, metrics: &TreeRowMetrics, frame: semio_framework_ui_theme::geometry::Rect, reversed: bool, out: &mut Vec<TreeWindowContainerMeasure>) {
    let metrics = metrics.with_presentation(owner.presentation);
    if document_table(tree, id).is_some() {
        let Some(section) = owner.sections.first() else { return };
        if let (Some(window), Some(rect)) = (section.window.as_ref().filter(|window| window.total > 0), tree.absolute_rect(id)) {
            let rows: Vec<NodeId> = section.items.iter().filter_map(|item| tree.explicit_child(id, &item.id)).collect();
            push_window_measure(tree, &section.id, window, section.items.len(), rect, metrics.header_height, &rows, frame, reversed, out);
        }
        return;
    }
    for section in owner.sections.iter().filter(|section| section.presence.visible()) {
        let Some(section_id) = tree.explicit_child(id, &section.id) else { continue };
        if !tree.disclosure_open(section_id).unwrap_or_else(|| semio_framework_ui_wgpu_layout::layout::tree_section_default_open(section)) {
            continue;
        }
        if let (Some(window), Some(rect)) = (section.window.as_ref().filter(|window| window.total > 0), tree.absolute_rect(section_id)) {
            let rows: Vec<NodeId> = section.items.iter().filter_map(|item| tree.explicit_child(section_id, &item.id)).collect();
            push_window_measure(tree, &section.id, window, section.items.len(), rect, tree_section_header_height(section, &metrics), &rows, frame, reversed, out);
        }
        let path = section.window.is_some().then_some(section.id.as_str());
        for item in &section.items {
            if let Some(item_id) = tree.explicit_child(section_id, &item.id) {
                measure_item_windows(tree, item_id, item, path, frame, reversed, 0, out);
            }
        }
    }
}

#[allow(clippy::too_many_arguments, reason = "one recursion frame of the measurement walk: the node, its spec, the path so far and the shared viewport")]
fn measure_item_windows(tree: &UiTree, id: NodeId, item: &UiTreeItemNode, parent_path: Option<&str>, frame: semio_framework_ui_theme::geometry::Rect, reversed: bool, depth: usize, out: &mut Vec<TreeWindowContainerMeasure>) {
    if depth >= TREE_ROW_MAX_DEPTH || !item.presence.visible() || !tree.disclosure_open(id).unwrap_or(item.default_open.unwrap_or(false)) {
        return;
    }
    let children = item.items.as_deref().unwrap_or_default();
    let path = item.window.is_some().then(|| match parent_path {
        Some(parent) => format!("{parent}{}{}", ui_contract::TREE_WINDOW_PATH_SEPARATOR, item.id),
        None => item.id.clone(),
    });
    if let (Some(window), Some(path), Some(rect)) = (item.window.as_ref().filter(|window| window.total > 0), path.as_deref(), tree.absolute_rect(id)) {
        let rows: Vec<NodeId> = children.iter().filter_map(|child| tree.explicit_child(id, &child.id)).collect();
        let (lead, trail) = tree_window_spacer_px(Some(window), children.len());
        let nested = lead + trail + rows.iter().filter_map(|row| tree.absolute_rect(*row)).map(|row| row.h).sum::<f32>();
        push_window_measure(tree, path, window, children.len(), rect, (rect.h - nested).max(0.0), &rows, frame, reversed, out);
    }
    let next = path.as_deref().or(parent_path);
    for child in children {
        if let Some(child_id) = tree.explicit_child(id, &child.id) {
            measure_item_windows(tree, child_id, child, next, frame, reversed, depth + 1, out);
        }
    }
}

/// 🧮️ One container's measure: its content band below its own chrome (`band`: a section's header, a table's column header, an
/// item's own row chrome), in viewport space — mirrored for an up-flow tree, where the chrome sits BELOW the rows.
#[allow(clippy::too_many_arguments, reason = "one container's whole geometry, gathered by the two walks above")]
fn push_window_measure(
    tree: &UiTree,
    key: &str,
    window: &semio_framework_ui_declaration::ui::UiTreeWindow,
    length: usize,
    rect: semio_framework_ui_theme::geometry::Rect,
    band: f32,
    rows: &[NodeId],
    frame: semio_framework_ui_theme::geometry::Rect,
    reversed: bool,
    out: &mut Vec<TreeWindowContainerMeasure>,
) {
    let height = f64::from((rect.h - band).max(0.0));
    let place = |y: f32, h: f32| if reversed { f64::from(frame.h) - (f64::from(y - frame.y) + f64::from(h)) } else { f64::from(y - frame.y) };
    let top = if reversed { place(rect.y, rect.h - band) } else { place(rect.y + band, rect.h - band) };
    let mut measured: Vec<TreeWindowRowMeasure> =
        rows.iter().enumerate().filter_map(|(position, row)| tree.absolute_rect(*row).map(|at| TreeWindowRowMeasure { index: window.offset.saturating_add(position as u32), top: place(at.y, at.h) })).collect();
    measured.sort_by(|left, right| left.top.total_cmp(&right.top).then(left.index.cmp(&right.index)));
    out.push(TreeWindowContainerMeasure {
        key: key.to_owned(),
        total: window.total,
        offset: window.offset,
        length: u32::try_from(length).unwrap_or(u32::MAX),
        top,
        height,
        row_px: f64::from(tree_window_row_extent_px(window.row_extent)),
        rows: measured,
    });
}
//#endregion 🪟️TreeWindowMeasure
//#endregion 🧵️MountedLayoutText

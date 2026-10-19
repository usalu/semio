//! 🔄️ Interactive state sync: the retained sync keys, records, frames, phases and cursor and the interactive state node step. See [`super`].

use super::*;

// #region paint
//#region 🔖️InteractiveStateSync
// 🔗️ W2 wiring: a paint-owned pre-pass, mutable (unlike `paint_node`'s own read-only walk below),
// run once per `paint_tree` call before painting anything — writes derived state `flex`/`reconcile`
// have no way to produce for composite widgets they don't fully own the interactive geometry of:
//  - an open `Select`'s synthesized item-row `Button`s (`reconcile::children_of`'s `Select` arm)
//    get real per-row `LayoutBucket` rects here (`flex::style_for`'s fallback leaf style gives every
//    one of them a zero-size rect — neither `Select` nor its rows are a flex container `flex` grants
//    space to), computed with the exact geometry `paint_select` itself paints the popup at (see
//    `select_popup_row_rect`), so `events::hit_test` can actually find and click them.
//  - a `Stack`'s `NodeFlags::DROP_TARGET` bit is kept in sync with its own `drop_action` field
//    (`events::nearest_accepting_drop_target` walks the bubble chain for this flag).
//  - a `Tree`'s synthesized per-row `Stack`s (`reconcile::children_of`'s `Tree` arm) get real
//    per-row rects too (same zero-size root cause as `Select`'s rows), computed with the exact
//    row-height/indent math `paint_tree_item` paints rows at, and their `NodeFlags::DRAG_SOURCE` bit
//    is kept in sync with the *original* `UiTreeItemNode`'s `draggable` field (`reconcile` never
//    drops fields, only clones them into `WidgetSpec` — see that module's own doc comment).

pub const RETAINED_SYNC_COLLECTION_ITEMS: usize = 256;

const RETAINED_SYNC_OUTPUTS: usize = RETAINED_SYNC_COLLECTION_ITEMS * 2;

const RETAINED_SYNC_DEPTH: usize = 64;

const RETAINED_SYNC_KEY_BYTES: usize = 256;

#[derive(Clone, Copy)]
struct RetainedSyncKey {
    pub(super) bytes: [u8; RETAINED_SYNC_KEY_BYTES],
    pub(super) len: u16,
}

impl RetainedSyncKey {
    fn try_from_str(value: &str) -> Option<Self> {
        if value.len() > RETAINED_SYNC_KEY_BYTES {
            return None;
        }
        let mut bytes = [0; RETAINED_SYNC_KEY_BYTES];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Some(Self { bytes, len: value.len() as u16 })
    }

    pub(super) fn as_str(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok()
    }
}

#[derive(Clone, Copy)]
struct RetainedSyncTreeRecord {
    pub(super) key: RetainedSyncKey,
    pub(super) parent: Option<usize>,
    pub(super) retained: Option<NodeId>,
    draggable: bool,
    pub(super) section: bool,
}

#[derive(Clone, Copy)]
struct RetainedSyncTreeFrame {
    record: usize,
    next_item: usize,
    items_pointer: usize,
    items_len: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RetainedInteractiveSyncPhase {
    Bind,
    SelectItem,
    SelectScan,
    /// 🔽️ The open popup's row for `cursor.item` has no retained child yet — synthesize it. See
    /// `reconcile::select_item_row` for why production never had one.
    SelectMint,
    /// 🔽️ A popup that CLOSED (or shrank) still owns synthesized rows — drop them one per step, so
    /// a closed `Select` leaves no arena node behind exactly as React unmounts `SelectContent`.
    SelectRetire,
    SelectWrite,
    StackWrite,
    TreeSection,
    TreeItem,
    TreeApplyPrepare,
    TreeApplyScan,
    TreeApplyWrite,
    TreeClose,
}

/// 🧭️ Retains one mounted interaction-layout synchronization authority.
pub struct RetainedInteractiveSyncCursor {
    pub(super) node: Option<NodeId>,
    pub(super) phase: RetainedInteractiveSyncPhase,
    pub(super) item: usize,
    child_scan: Option<NodeId>,
    matched: Option<NodeId>,
    select_width: f32,
    select_height: f32,
    /// 🔽️ The open popup's collision-resolved top offset, captured once per `Select` bind so every
    /// row this sync pass writes lands on the same side of the trigger paint chose.
    select_menu_top: f32,
    select_first_row: usize,
    select_last_row: usize,
    tree_section: usize,
    tree_depth: usize,
    pub tree_frames: [Option<RetainedSyncTreeFrame>; RETAINED_SYNC_DEPTH],
    tree_records: [Option<RetainedSyncTreeRecord>; RETAINED_SYNC_OUTPUTS],
    pub tree_record_len: usize,
    tree_item_count: usize,
    tree_apply: usize,
    /// 🩺️ The source line of the `let … else` arm that last answered
    /// [`RetainedInteractiveSyncStep::Fault`], `0` while the walk is healthy. See
    /// [`retained_sync_fault`].
    pub fault_line: u32,
}

impl Default for RetainedInteractiveSyncCursor {
    fn default() -> Self {
        Self {
            node: None,
            phase: RetainedInteractiveSyncPhase::Bind,
            item: 0,
            child_scan: None,
            matched: None,
            select_width: 0.0,
            select_height: 0.0,
            select_menu_top: 0.0,
            select_first_row: 0,
            select_last_row: 0,
            tree_section: 0,
            tree_depth: 0,
            tree_frames: [None; RETAINED_SYNC_DEPTH],
            tree_records: [None; RETAINED_SYNC_OUTPUTS],
            tree_record_len: 0,
            tree_item_count: 0,
            tree_apply: 0,
            fault_line: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedInteractiveSyncStep {
    Pending,
    Complete,
    Fault,
}

impl RetainedInteractiveSyncCursor {
    pub(super) fn finish(&mut self) -> RetainedInteractiveSyncStep {
        self.node = None;
        self.phase = RetainedInteractiveSyncPhase::Bind;
        self.item = 0;
        self.child_scan = None;
        self.matched = None;
        self.select_width = 0.0;
        self.select_height = 0.0;
        self.select_menu_top = 0.0;
        self.select_first_row = 0;
        self.select_last_row = 0;
        self.tree_section = 0;
        self.tree_depth = 0;
        self.tree_item_count = 0;
        self.tree_apply = 0;
        RetainedInteractiveSyncStep::Complete
    }

    pub fn close_step(&mut self) -> bool {
        if self.tree_record_len > 0 {
            self.tree_record_len -= 1;
            self.tree_records[self.tree_record_len] = None;
            return false;
        }
        if self.tree_frames[self.tree_depth].take().is_some() {
            if self.tree_depth > 0 {
                self.tree_depth -= 1;
            }
            return false;
        }
        if self.node.take().is_some() {
            self.phase = RetainedInteractiveSyncPhase::Bind;
            self.child_scan = None;
            self.matched = None;
            return false;
        }
        true
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn terminal_is_empty(&self) -> bool {
        self.node.is_none() && self.tree_record_len == 0 && self.tree_frames[0].is_none()
    }
}

/// 🩺️ The ONE fault exit of the retained interactive synchronize walk, stamping the source line that
/// took it onto the cursor.
///
/// ⚖️ Thirty-four `let … else` arms answer `RetainedInteractiveSyncStep::Fault`, the paint frame turns
/// every one of them into the single phase name `synchronize-node`, and the shell turns THAT into
/// `retained document ingress reached its terminal fault` on a body that then paints nothing. Three
/// names for one event, none of which says which arm fired — measured on 6118 as a permanently
/// faulted `framework.panel.toolRun` with no further detail available at all
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-regressions-sweep-2026-09-15.md`).
fn retained_sync_fault(cursor: &mut RetainedInteractiveSyncCursor, line: u32) -> RetainedInteractiveSyncStep {
    cursor.fault_line = line;
    RetainedInteractiveSyncStep::Fault
}

/// 🌳️ A declared `Tree` row whose retained child is absent is SKIPPED, never a fault. `reconcile`
/// mounts a row only when the published document carries a child record keyed by that row's id, so a
/// producer that authors `UiNode::Tree` sections/items without a matching record subtree is ordinary
/// — React's host renders such a tree from the component props alone, and this module's own
/// `sync_tree_row_drag_sources` twin walks past the row with `continue`. Faulting instead killed the
/// WHOLE surface: the live `framework.panel.toolRun` panel on 6118 painted `retained document
/// ingress reached its terminal fault` in place of every row.
fn retained_sync_tree_skip(cursor: &mut RetainedInteractiveSyncCursor) -> RetainedInteractiveSyncStep {
    cursor.matched = None;
    cursor.child_scan = None;
    cursor.tree_apply = cursor.tree_apply.saturating_add(1);
    cursor.phase = RetainedInteractiveSyncPhase::TreeApplyPrepare;
    RetainedInteractiveSyncStep::Pending
}

/// 🔽️ A popup row that could not be synthesized (the composite-row ledger is full) is skipped, never
/// a fault — the same rule `retained_sync_tree_skip` applies to a declared tree row with no record.
fn retained_sync_select_skip(cursor: &mut RetainedInteractiveSyncCursor) -> RetainedInteractiveSyncStep {
    cursor.matched = None;
    cursor.child_scan = None;
    cursor.item = cursor.item.saturating_add(1);
    cursor.phase = RetainedInteractiveSyncPhase::SelectItem;
    RetainedInteractiveSyncStep::Pending
}

fn retained_sync_tree_item_step(cursor: &mut RetainedInteractiveSyncCursor) -> RetainedInteractiveSyncStep {
    let Some(frame) = cursor.tree_frames[cursor.tree_depth] else { return retained_sync_fault(cursor, line!()) };
    if frame.next_item >= frame.items_len {
        cursor.tree_frames[cursor.tree_depth] = None;
        if cursor.tree_depth == 0 {
            cursor.tree_section += 1;
            cursor.phase = RetainedInteractiveSyncPhase::TreeSection;
        } else {
            cursor.tree_depth -= 1;
        }
        return RetainedInteractiveSyncStep::Pending;
    }
    let item_index = frame.next_item;
    let Some(next_item) = item_index.checked_add(1) else { return retained_sync_fault(cursor, line!()) };
    let Some(active) = cursor.tree_frames[cursor.tree_depth].as_mut() else { return retained_sync_fault(cursor, line!()) };
    active.next_item = next_item;
    let Some(item) = (unsafe { (frame.items_pointer as *const UiTreeItemNode).add(item_index).as_ref() }) else { return retained_sync_fault(cursor, line!()) };
    if !item.presence.visible() {
        return RetainedInteractiveSyncStep::Pending;
    }
    let Some(item_count) = cursor.tree_item_count.checked_add(1).filter(|count| *count <= RETAINED_SYNC_COLLECTION_ITEMS) else { return retained_sync_fault(cursor, line!()) };
    let Some(record_index) = cursor.tree_record_len.checked_add(1).filter(|count| *count <= RETAINED_SYNC_OUTPUTS).map(|_| cursor.tree_record_len) else { return retained_sync_fault(cursor, line!()) };
    let Some(key) = RetainedSyncKey::try_from_str(&item.id) else { return retained_sync_fault(cursor, line!()) };
    cursor.tree_records[record_index] = Some(RetainedSyncTreeRecord { key, parent: Some(frame.record), retained: None, draggable: item.draggable.unwrap_or(false), section: false });
    cursor.tree_record_len += 1;
    cursor.tree_item_count = item_count;
    let children = item.items.as_deref().filter(|items| !items.is_empty());
    if let Some(children) = children {
        let Some(depth) = cursor.tree_depth.checked_add(1).filter(|depth| *depth < RETAINED_SYNC_DEPTH) else { return retained_sync_fault(cursor, line!()) };
        cursor.tree_depth = depth;
        cursor.tree_frames[depth] = Some(RetainedSyncTreeFrame { record: record_index, next_item: 0, items_pointer: children.as_ptr() as usize, items_len: children.len() });
    }
    RetainedInteractiveSyncStep::Pending
}

/// 🧩️ Advances one retained synchronization item, child lookup, layout output, or close owner.
pub fn sync_interactive_state_node_step(tree: &mut UiTree, id: NodeId, theme: &Theme, cursor: &mut RetainedInteractiveSyncCursor) -> RetainedInteractiveSyncStep {
    if cursor.node.is_none() {
        cursor.node = Some(id);
        cursor.phase = RetainedInteractiveSyncPhase::Bind;
        return RetainedInteractiveSyncStep::Pending;
    }
    if cursor.node != Some(id) {
        return retained_sync_fault(cursor, line!());
    }
    match cursor.phase {
        RetainedInteractiveSyncPhase::Bind => {
            let Some(node) = tree.node(id) else { return retained_sync_fault(cursor, line!()) };
            match &node.spec.0 {
                UiNode::Select(select) if node.state.open => {
                    if select.items.len() > RETAINED_SYNC_COLLECTION_ITEMS {
                        return retained_sync_fault(cursor, line!());
                    }
                    let Some(layout) = tree.accepted_layout(id) else { return retained_sync_fault(cursor, line!()) };
                    cursor.select_width = layout.width;
                    cursor.select_height = layout.height;
                    let Some(trigger) = tree.absolute_rect(id) else { return retained_sync_fault(cursor, line!()) };
                    let viewport_h = tree.root.and_then(|root| tree.accepted_layout(root)).map_or(0.0, |layout| layout.height);
                    let opening_index = node.state.select_popup.is_none().then(|| node.state.highlighted.or_else(|| select.items.iter().position(|item| item.value == select.value))).flatten();
                    let (offset, direction) = (node.state.scroll_offset.1, node.state.scroll_offset.0);
                    let mut popup = semio_framework_ui_select_behavior::select_popup_geometry(trigger, select.items.len(), theme, viewport_h, offset, direction);
                    if let Some(index) = opening_index {
                        let revealed = semio_framework_ui_select_behavior::select_revealed_scroll(popup, index);
                        popup = semio_framework_ui_select_behavior::select_popup_geometry(trigger, select.items.len(), theme, viewport_h, revealed, 0.0);
                    }
                    cursor.select_menu_top = popup.menu.y - trigger.y;
                    cursor.select_first_row = popup.first_row;
                    cursor.select_last_row = popup.last_row;
                    let Some(node) = tree.node_mut(id) else { return retained_sync_fault(cursor, line!()) };
                    node.state.scroll_offset = (0.0, popup.scroll);
                    node.state.select_popup = Some(popup);
                    cursor.phase = RetainedInteractiveSyncPhase::SelectItem;
                    RetainedInteractiveSyncStep::Pending
                }
                // 🔽️ A closed popup that still owns rows unmounts them before anything else runs.
                UiNode::Select(_) if tree.composite_rows_of(id) > 0 => {
                    if let Some(node) = tree.node_mut(id) {
                        node.state.scroll_offset = (0.0, 0.0);
                        node.state.select_popup = None;
                    }
                    cursor.phase = RetainedInteractiveSyncPhase::SelectRetire;
                    RetainedInteractiveSyncStep::Pending
                }
                UiNode::Select(_) => {
                    if let Some(node) = tree.node_mut(id) {
                        node.state.scroll_offset = (0.0, 0.0);
                        node.state.select_popup = None;
                    }
                    cursor.finish()
                }
                UiNode::Stack(_) => {
                    cursor.phase = RetainedInteractiveSyncPhase::StackWrite;
                    RetainedInteractiveSyncStep::Pending
                }
                UiNode::Tree(tree_node) => {
                    if tree_node.sections.len() > RETAINED_SYNC_COLLECTION_ITEMS || tree.accepted_layout(id).is_none() {
                        return retained_sync_fault(cursor, line!());
                    }
                    cursor.phase = RetainedInteractiveSyncPhase::TreeSection;
                    RetainedInteractiveSyncStep::Pending
                }
                _ => cursor.finish(),
            }
        }
        RetainedInteractiveSyncPhase::SelectItem => {
            let Some(select) = tree.node(id).and_then(|node| match &node.spec.0 {
                UiNode::Select(select) => Some(select),
                _ => None,
            }) else {
                return retained_sync_fault(cursor, line!());
            };
            if cursor.item >= select.items.len() {
                // 🔽️ A popup whose item list SHRANK still owns rows for items that are gone. Drop
                // the excess one per step; `SelectScan` re-mints anything it dropped that is still
                // declared, so this converges instead of oscillating and never leaks an arena slot.
                if tree.composite_rows_of(id) > select.items.len() {
                    let _ = tree.retire_composite_row_of_step(id);
                    return RetainedInteractiveSyncStep::Pending;
                }
                return cursor.finish();
            }
            cursor.child_scan = tree.node(id).and_then(|node| node.first_child);
            cursor.matched = None;
            cursor.phase = RetainedInteractiveSyncPhase::SelectScan;
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::SelectRetire => {
            if tree.retire_composite_row_of_step(id) {
                return cursor.finish();
            }
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::SelectMint => {
            // 🔽️ React mounts `SelectContent`'s children when the popup opens; this is that mount.
            // One row per step, keyed by the item's own `value` so `SelectScan` above finds it again
            // next frame instead of minting a second copy, and ledgered on the tree so the document
            // reconcile can retire it before it relinks the arena (`UiTree::mint_composite_row`).
            let row = tree.node(id).and_then(|node| match &node.spec.0 {
                UiNode::Select(select) => select.items.get(cursor.item).map(|item| (semio_framework_ui_retained_tree::reconcile::select_item_row(select, item), item.value.clone())),
                _ => None,
            });
            let Some((spec, key)) = row else { return retained_sync_fault(cursor, line!()) };
            let minted = tree.mint_composite_row(id, Node::new(NodeKey::Explicit(key), WidgetSpec(spec)));
            let Some(minted) = minted else { return retained_sync_select_skip(cursor) };
            cursor.matched = Some(minted);
            cursor.phase = RetainedInteractiveSyncPhase::SelectWrite;
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::SelectScan => {
            let Some(child) = cursor.child_scan else {
                cursor.matched = None;
                cursor.phase = RetainedInteractiveSyncPhase::SelectMint;
                return RetainedInteractiveSyncStep::Pending;
            };
            let matches = tree
                .node(id)
                .and_then(|node| match &node.spec.0 {
                    UiNode::Select(select) => select.items.get(cursor.item),
                    _ => None,
                })
                .is_some_and(|item| matches!(tree.node(child).map(|node| &node.key), Some(NodeKey::Explicit(key)) if key == &item.value));
            if matches {
                cursor.matched = Some(child);
                cursor.phase = RetainedInteractiveSyncPhase::SelectWrite;
            } else {
                cursor.child_scan = tree.node(child).and_then(|node| node.next_sibling);
            }
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::SelectWrite => {
            let Some(child) = cursor.matched.take() else { return retained_sync_fault(cursor, line!()) };
            let spec = tree.node(id).and_then(|node| match &node.spec.0 {
                UiNode::Select(select) => select.items.get(cursor.item).map(|item| semio_framework_ui_retained_tree::reconcile::select_item_row(select, item)),
                _ => None,
            });
            let Some(spec) = spec else { return retained_sync_fault(cursor, line!()) };
            let rect = if cursor.item >= cursor.select_first_row && cursor.item < cursor.select_last_row {
                let Some(trigger) = tree.absolute_rect(id) else { return retained_sync_fault(cursor, line!()) };
                let Some(popup) = tree.node(id).and_then(|node| node.state.select_popup) else { return retained_sync_fault(cursor, line!()) };
                let absolute = semio_framework_ui_select_behavior::select_popup_row_hit_rect(trigger, cursor.item, popup, theme);
                Rect::new(absolute.x - trigger.x, absolute.y - trigger.y, absolute.w, absolute.h)
            } else {
                Rect::new(0.0, 0.0, 0.0, 0.0)
            };
            let Some(node) = tree.node_mut(child) else { return retained_sync_fault(cursor, line!()) };
            if node.spec.0 != spec {
                node.spec = WidgetSpec(spec);
            }
            node.layout.x = rect.x;
            node.layout.y = rect.y;
            node.layout.width = rect.w;
            node.layout.height = rect.h;
            let Some(item) = cursor.item.checked_add(1) else { return retained_sync_fault(cursor, line!()) };
            cursor.item = item;
            cursor.phase = RetainedInteractiveSyncPhase::SelectItem;
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::StackWrite => {
            let accepts_drop = tree.node(id).and_then(|node| match &node.spec.0 {
                UiNode::Stack(stack) => Some(stack.drop_action.is_some()),
                _ => None,
            });
            let Some(accepts_drop) = accepts_drop else { return retained_sync_fault(cursor, line!()) };
            let Some(node) = tree.node_mut(id) else { return retained_sync_fault(cursor, line!()) };
            node.flags.set(NodeFlags::DROP_TARGET, accepts_drop);
            cursor.finish()
        }
        RetainedInteractiveSyncPhase::TreeSection => {
            let Some(sections_len) = tree.node(id).and_then(|node| match &node.spec.0 {
                UiNode::Tree(tree_node) => tree.accepted_layout(id).map(|_| tree_node.sections.len()),
                _ => None,
            }) else {
                return retained_sync_fault(cursor, line!());
            };
            if cursor.tree_section >= sections_len {
                cursor.tree_apply = 0;
                cursor.phase = RetainedInteractiveSyncPhase::TreeApplyPrepare;
                return RetainedInteractiveSyncStep::Pending;
            }
            let Some(section) = tree.node(id).and_then(|node| match &node.spec.0 {
                UiNode::Tree(tree_node) => tree_node.sections.get(cursor.tree_section),
                _ => None,
            }) else {
                return retained_sync_fault(cursor, line!());
            };
            let Some(record_index) = cursor.tree_record_len.checked_add(1).filter(|count| *count <= RETAINED_SYNC_OUTPUTS).map(|_| cursor.tree_record_len) else { return retained_sync_fault(cursor, line!()) };
            let Some(key) = RetainedSyncKey::try_from_str(&section.id) else { return retained_sync_fault(cursor, line!()) };
            cursor.tree_records[record_index] = Some(RetainedSyncTreeRecord { key, parent: None, retained: None, draggable: false, section: true });
            cursor.tree_record_len += 1;
            cursor.tree_depth = 0;
            cursor.tree_frames[0] = Some(RetainedSyncTreeFrame { record: record_index, next_item: 0, items_pointer: section.items.as_ptr() as usize, items_len: section.items.len() });
            cursor.phase = RetainedInteractiveSyncPhase::TreeItem;
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::TreeItem => retained_sync_tree_item_step(cursor),
        RetainedInteractiveSyncPhase::TreeApplyPrepare => {
            if cursor.tree_apply >= cursor.tree_record_len {
                cursor.phase = RetainedInteractiveSyncPhase::TreeClose;
                return RetainedInteractiveSyncStep::Pending;
            }
            let Some(record) = cursor.tree_records[cursor.tree_apply] else { return retained_sync_fault(cursor, line!()) };
            let parent = match record.parent {
                Some(parent) => cursor.tree_records.get(parent).and_then(|record| record.as_ref()).and_then(|record| record.retained),
                None => Some(id),
            };
            let Some(parent) = parent else { return retained_sync_tree_skip(cursor) };
            cursor.child_scan = tree.node(parent).and_then(|node| node.first_child);
            cursor.matched = None;
            cursor.phase = RetainedInteractiveSyncPhase::TreeApplyScan;
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::TreeApplyScan => {
            let Some(child) = cursor.child_scan else { return retained_sync_tree_skip(cursor) };
            let Some(record) = cursor.tree_records[cursor.tree_apply] else { return retained_sync_fault(cursor, line!()) };
            let Some(target) = record.key.as_str() else { return retained_sync_fault(cursor, line!()) };
            if matches!(tree.node(child).map(|node| &node.key), Some(NodeKey::Explicit(key)) if key == target) {
                cursor.matched = Some(child);
                cursor.phase = RetainedInteractiveSyncPhase::TreeApplyWrite;
            } else {
                cursor.child_scan = tree.node(child).and_then(|node| node.next_sibling);
            }
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::TreeApplyWrite => {
            let Some(child) = cursor.matched.take() else { return retained_sync_fault(cursor, line!()) };
            let Some(record) = cursor.tree_records[cursor.tree_apply] else { return retained_sync_fault(cursor, line!()) };
            let Some(node) = tree.node_mut(child) else { return retained_sync_fault(cursor, line!()) };
            if !record.section {
                node.flags.set(NodeFlags::DRAG_SOURCE, record.draggable);
            }
            let Some(output) = cursor.tree_records[cursor.tree_apply].as_mut() else { return retained_sync_fault(cursor, line!()) };
            output.retained = Some(child);
            let Some(next) = cursor.tree_apply.checked_add(1) else { return retained_sync_fault(cursor, line!()) };
            cursor.tree_apply = next;
            cursor.phase = RetainedInteractiveSyncPhase::TreeApplyPrepare;
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::TreeClose => {
            if cursor.tree_record_len > 0 {
                cursor.tree_record_len -= 1;
                cursor.tree_records[cursor.tree_record_len] = None;
                RetainedInteractiveSyncStep::Pending
            } else {
                cursor.finish()
            }
        }
    }
}

/// 📐️ One popup row's `(x, y, w, h)` **relative to the `Select`'s own top-left** — shared by
/// `sync_select_popup_rows` (writes it into the row's retained `LayoutBucket`), `paint_select` and
/// the retained paint steps (paint it), so none of them can drift apart. The geometry itself lives
/// with the element (`crate::wgpu::select::🔖️Geometry`), which is also where the immediate-mode kit
/// and React's own `resolveSelectPlacement`-derived rules are reconciled; `menu_top` is that
/// module's collision-aware answer (see `select_menu_top_for`).
pub fn select_popup_row_rect(select_w: f32, index: usize, menu_top: f32, theme: &Theme) -> Rect {
    semio_framework_ui_select_behavior::select_row_rect(select_w, index, menu_top, theme)
}

/// 📐️ `select_menu_top` for a retained `Select`, resolving its two viewport inputs from the tree:
/// the node's own absolute top and the root's laid-out height. A popup with no room below its
/// trigger flips above it, exactly as React's `SelectContent` does.
pub fn select_menu_top_for(tree: &UiTree, id: NodeId, select_h: f32, items: usize, theme: &Theme) -> f32 {
    let menu_h = semio_framework_ui_select_behavior::select_menu_height(items, theme);
    let absolute_y = semio_framework_ui_wgpu_events::node_abs_rect(tree, id).map_or(0.0, |rect| rect.y);
    let viewport_h = tree.root.and_then(|root| tree.accepted_layout(root)).map_or(0.0, |layout| layout.height);
    semio_framework_ui_select_behavior::select_menu_top(absolute_y, select_h, menu_h, viewport_h)
}
//#endregion 🔖️InteractiveStateSync
// #endregion paint

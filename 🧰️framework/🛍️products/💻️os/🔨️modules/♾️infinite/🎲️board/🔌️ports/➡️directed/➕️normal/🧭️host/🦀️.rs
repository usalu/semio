//! 🧭️ Board host retirement, close phases, wheel and pointer plan types with delete and publication plans.

use super::*;

// #region board_host
pub struct BoardHostRetirement {
    host: std::mem::ManuallyDrop<BoardHost>,
    released: bool,
}

impl BoardHostRetirement {
    pub fn new(host: BoardHost) -> Self {
        Self { host: std::mem::ManuallyDrop::new(host), released: false }
    }

    pub fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if self.released {
            return true;
        }
        if !self.host.close_nonopaque_step(context) {
            return false;
        }
        assert!(self.host.nonopaque_terminal_is_empty(), "BoardHost nonopaque terminal witness precedes shallow release");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.host) };
        self.released = true;
        true
    }

    pub fn terminal_nonopaque_is_empty(&self) -> bool {
        self.released
    }
}

impl Drop for BoardHostRetirement {
    fn drop(&mut self) {
        debug_assert!(self.released, "BoardHostRetirement must reach terminal-empty before release");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BoardHostClosePhase {
    Events,
    Pointer,
    WorldScene,
    Icons,
    Nodes,
    Handles,
    Edges,
    Wires,
    Regions,
    Selection,
    Preselect,
    PreselectRemoved,
    SelectionExit,
    Highlighted,
    Interaction,
    HandleKinds,
    WireKinds,
    NodeKinds,
    EdgeKinds,
    EdgeTips,
    LinkRules,
    Previews,
    Strings,
    Weights,
    Done,
}

#[derive(Clone, Debug)]
pub struct BoardWheelPlan {
    pub(super) revision: u64,
    pub(super) expected: Camera,
    pub(super) next: Camera,
}

/// 🧮️ Entities (nodes + handles + edges + wires) one board descriptor, selection or gesture payload
/// may carry. Sized for the largest shipped puzzle 2d example: Nakagin Capsule Tower is 180 nodes
/// + 358 handles + 179 edges = 717 entities — the old 256 refused it silently (`load_board_snapshot_json`
/// → `false`, three empty panes, 2026-09-16).
pub const BOARD_POINTER_ITEM_CAPACITY: usize = 1_024;
/// 🧮️ Entity-id bytes the same payloads may carry — Nakagin's UUID ids total ~29 KiB; kept under the
/// guest's 64 KiB contiguous-request ceiling since every owner here is one boxed array.
pub const BOARD_POINTER_BYTE_CAPACITY: usize = 48 * 1024;
/// 🧮️ Entities one board DESCRIPTOR may carry — the render/hit-test census, decoupled from the
/// gesture payload credits above: a 100-placement puzzle 2d fill of an 11-handle kind is 1 200
/// entities, and a board must keep painting past what one pointer payload may name (a whole-board
/// selection above the pointer credits is refused by `select_set`'s own `ItemCredits` law instead).
pub const BOARD_DESCRIPTOR_ITEM_CAPACITY: usize = 8_192;
/// 🧮️ Entity-id bytes one board descriptor may carry (UUID ids: 8 192 × ~45 B).
pub const BOARD_DESCRIPTOR_BYTE_CAPACITY: usize = 384 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardPointerPhase {
    Move,
    Up,
    Leave,
}

#[derive(Clone, Copy, Debug)]
pub struct BoardPointerIntent {
    pub phase: BoardPointerPhase,
    pub x: f64,
    pub y: f64,
    pub shift: bool,
    pub ctrl_or_meta: bool,
    pub alt: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardPointerPlanFault {
    ItemCredits,
    ByteCredits,
    Unsupported,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct BoardPointerSpan {
    pub(super) start: u16,
    pub(super) len: u16,
}

#[derive(Clone, Copy, Debug)]
struct BoardPointerDelta {
    pub(super) id: BoardPointerSpan,
    pub(super) x: f64,
    pub(super) y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BoardDeleteKind {
    Edge,
    Node,
    Handle,
    Wire,
}

#[derive(Clone, Copy, Debug)]
struct BoardDeleteEntry {
    pub(super) kind: BoardDeleteKind,
    pub(super) id: BoardPointerSpan,
}

pub(super) struct BoardDeletePlan {
    pub(super) revision: u64,
    pub(super) bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub(super) byte_len: u16,
    pub(super) entries: Box<[Option<BoardDeleteEntry>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub(super) len: u16,
    property_nodes: u16,
    property_bytes: u16,
}

pub(super) struct BoardDeleteOperation {
    pub(super) plan: BoardDeletePlan,
    pub(super) select_event: Option<BoardOwnedEvent>,
    pub(super) mutation_cursor: u16,
    pub(super) publication_cursor: u16,
    pub(super) remaining_claimed_items: u16,
    pub(super) remaining_claimed_bytes: usize,
    pub(super) claimed: bool,
    pub(super) retiring_entity: Option<BoardEntityRetirement>,
    pub(super) cancelling: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BoardDeletePlanningPhase {
    SelectedEdges,
    Nodes,
    DiscoverNode,
    Handles,
    Wires,
    Edges,
    Handle,
    Node,
    SelectionEvent,
    Finish,
}

pub(super) struct BoardDeletePlanningOperation {
    pub(super) plan: BoardDeletePlan,
    pub(super) phase: BoardDeletePlanningPhase,
    pub(super) scan_after: Option<String>,
    pub(super) node_after: Option<String>,
    pub(super) handle_after: Option<String>,
    pub(super) relation_after: Option<String>,
    pub(super) node_id: Option<String>,
    pub(super) handle_id: Option<String>,
    pub(super) node_relevant: bool,
    pub(super) property_audit: Option<BoardPropertyAudit>,
    pub(super) select_builder: BoardPayloadBuilder,
    pub(super) select_first: bool,
    pub(super) fault: Option<BoardEventFault>,
    pub(super) cancelling: bool,
}

enum BoardPropertyAuditFrame {
    Array { values: Vec<semio_framework_graph_manifest::PropertyValue>, index: usize, pending: Option<usize> },
    Object { values: semio_framework_graph_manifest::PropertyBag, after: Option<String>, pending: Option<String> },
}

pub(super) struct BoardPropertyAudit {
    pub(super) kind: BoardDeleteKind,
    pub(super) id: String,
    current: Option<semio_framework_graph_manifest::PropertyValue>,
    completed: Option<semio_framework_graph_manifest::PropertyValue>,
    stack: Box<[Option<BoardPropertyAuditFrame>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub(super) depth: u16,
    pub(super) nodes: usize,
    pub(super) bytes: usize,
    pub(super) fault: Option<BoardEventFault>,
}

pub(super) enum BoardRemovedEntity {
    Node(NodeData),
    Edge(EdgeData),
    Handle(HandleData),
    Wire(WireData),
}

enum BoardPropertyContainer {
    Array(Vec<semio_framework_graph_manifest::PropertyValue>),
    Object(semio_framework_graph_manifest::PropertyBag),
}

struct BoardPropertyRetirement {
    current: Option<semio_framework_graph_manifest::PropertyValue>,
    stack: Box<[Option<BoardPropertyContainer>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub(super) depth: u16,
    pub(super) faulted: bool,
}

pub(super) struct BoardEntityRetirement {
    entity: Option<BoardRemovedEntity>,
    strings: Box<[Option<String>; 16]>,
    string_len: u8,
    pub(super) properties: Option<BoardPropertyRetirement>,
}

pub struct BoardPointerPublication {
    pub(super) bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub(super) len: u16,
}

pub(super) struct BoardPointerCommitOperation {
    pub(super) plan: BoardPointerPlan,
    pub(super) phase: u8,
    pub(super) cursor: u16,
    pub(super) changed: bool,
    pub(super) cancelling: bool,
    pub(super) faulted: bool,
    pub(super) scan_after: Option<String>,
    pub(super) points: Vec<Point>,
    pub(super) screen_points: Vec<Point>,
    pub(super) overlay_points: Vec<Point>,
    pub(super) brush_candidates: BrushCandidatePage,
    pub(super) retiring_ids: BTreeSet<String>,
    pub(super) retiring_points: Vec<Point>,
    pub(super) retiring_screen_points: Vec<Point>,
    pub(super) retiring_overlay_points: Vec<Point>,
    pub(super) retiring_signature_a: Vec<String>,
    pub(super) retiring_signature_b: Vec<String>,
    pub(super) retiring_signature_c: Vec<String>,
    pub(super) retiring_gestures: [Option<String>; 2],
}

impl BoardPointerCommitOperation {
    pub(super) fn new(plan: BoardPointerPlan) -> Self {
        Self {
            plan,
            phase: 0,
            cursor: 0,
            changed: false,
            cancelling: false,
            faulted: false,
            scan_after: None,
            points: Vec::with_capacity(BOARD_POINTER_ITEM_CAPACITY),
            screen_points: Vec::with_capacity(BOARD_POINTER_ITEM_CAPACITY),
            overlay_points: Vec::with_capacity(BOARD_POINTER_ITEM_CAPACITY),
            brush_candidates: BrushCandidatePage::default(),
            retiring_ids: BTreeSet::new(),
            retiring_points: Vec::new(),
            retiring_screen_points: Vec::new(),
            retiring_overlay_points: Vec::new(),
            retiring_signature_a: Vec::new(),
            retiring_signature_b: Vec::new(),
            retiring_signature_c: Vec::new(),
            retiring_gestures: [None, None],
        }
    }

    pub(super) fn retire_one(&mut self) -> bool {
        if let Some(id) = self.retiring_ids.pop_first() {
            drop(id);
            return true;
        }
        if self.retiring_points.pop().is_some() || self.retiring_screen_points.pop().is_some() || self.retiring_overlay_points.pop().is_some() {
            return true;
        }
        for values in [&mut self.retiring_signature_a, &mut self.retiring_signature_b, &mut self.retiring_signature_c] {
            if let Some(value) = values.pop() {
                drop(value);
                return true;
            }
        }
        for gesture in &mut self.retiring_gestures {
            if let Some(value) = gesture.take() {
                drop(value);
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardAuthorityStep {
    Pending,
    Complete,
    Cancelled,
    Fault,
}

impl BoardDeletePlan {
    pub(super) fn new(revision: u64) -> Self {
        Self { revision, bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]), byte_len: 0, entries: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]), len: 0, property_nodes: 0, property_bytes: 0 }
    }

    pub(super) fn id(&self, span: BoardPointerSpan) -> &str {
        let start = usize::from(span.start);
        let end = start + usize::from(span.len);
        std::str::from_utf8(&self.bytes[start..end]).expect("board deletion ids originate from UTF-8 strings")
    }

    pub(super) fn contains(&self, kind: BoardDeleteKind, id: &str) -> bool {
        self.entries[..usize::from(self.len)].iter().flatten().any(|entry| entry.kind == kind && self.id(entry.id) == id)
    }

    pub(super) fn removes_selection_id(&self, id: &str) -> bool {
        self.entries[..usize::from(self.len)].iter().flatten().any(|entry| self.id(entry.id) == id)
    }

    pub(super) fn push(&mut self, kind: BoardDeleteKind, id: &str) -> Result<(), BoardEventFault> {
        if self.contains(kind, id) {
            return Ok(());
        }
        if matches!(kind, BoardDeleteKind::Edge | BoardDeleteKind::Node) && 7usize.checked_add(board_json_string_bytes(id)).is_none_or(|bytes| bytes > BOARD_EVENT_PAYLOAD_BYTE_CAPACITY) {
            return Err(BoardEventFault::ByteCredits);
        }
        let index = usize::from(self.len);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        if id.len() > u16::MAX as usize {
            return Err(BoardEventFault::ByteCredits);
        }
        let start = usize::from(self.byte_len);
        let end = start.checked_add(id.len()).ok_or(BoardEventFault::ByteCredits)?;
        if end > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.bytes[start..end].copy_from_slice(id.as_bytes());
        self.byte_len = end as u16;
        self.entries[index] = Some(BoardDeleteEntry { kind, id: BoardPointerSpan { start: start as u16, len: id.len() as u16 } });
        self.len += 1;
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn push_entity(&mut self, kind: BoardDeleteKind, id: &str, properties: &semio_framework_graph_manifest::PropertyBag) -> Result<(), BoardEventFault> {
        if self.contains(kind, id) {
            return Ok(());
        }
        let (nodes, bytes) = board_property_retirement_credits(properties)?;
        let next_nodes = usize::from(self.property_nodes).checked_add(nodes).ok_or(BoardEventFault::ItemCredits)?;
        let next_bytes = usize::from(self.property_bytes).checked_add(bytes).ok_or(BoardEventFault::ByteCredits)?;
        if next_nodes > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        if next_bytes > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.push(kind, id)?;
        self.property_nodes = next_nodes as u16;
        self.property_bytes = next_bytes as u16;
        Ok(())
    }

    pub(super) fn push_admitted_entity(&mut self, kind: BoardDeleteKind, id: &str, property_nodes: usize, property_bytes: usize) -> Result<(), BoardEventFault> {
        if self.contains(kind, id) {
            return Ok(());
        }
        let next_nodes = usize::from(self.property_nodes).checked_add(property_nodes).ok_or(BoardEventFault::ItemCredits)?;
        let next_bytes = usize::from(self.property_bytes).checked_add(property_bytes).ok_or(BoardEventFault::ByteCredits)?;
        if next_nodes > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        if next_bytes > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.push(kind, id)?;
        self.property_nodes = next_nodes as u16;
        self.property_bytes = next_bytes as u16;
        Ok(())
    }

    pub(super) fn emitted_event_count(&self) -> usize {
        self.entries[..usize::from(self.len)].iter().flatten().filter(|entry| matches!(entry.kind, BoardDeleteKind::Edge | BoardDeleteKind::Node)).count()
    }

    pub(super) fn emitted_event_bytes(&self) -> Option<usize> {
        self.entries[..usize::from(self.len)]
            .iter()
            .flatten()
            .filter(|entry| matches!(entry.kind, BoardDeleteKind::Edge | BoardDeleteKind::Node))
            .try_fold(0usize, |bytes, entry| bytes.checked_add(7)?.checked_add(board_json_string_bytes(self.id(entry.id))))
    }
}

#[cfg(test)]
include!("../🧪️tests/🔬️board-host-standalone/🦀️.rs");

impl BoardPropertyAudit {
    pub(super) fn new(kind: BoardDeleteKind, id: String, properties: semio_framework_graph_manifest::PropertyBag) -> Self {
        Self { kind, id, current: Some(semio_framework_graph_manifest::PropertyValue::Object(properties)), completed: None, stack: Box::new(std::array::from_fn(|_| None)), depth: 0, nodes: 0, bytes: 0, fault: None }
    }

    fn add_bytes(&mut self, bytes: usize) {
        self.bytes = self.bytes.saturating_add(bytes);
        if self.bytes > BOARD_POINTER_BYTE_CAPACITY {
            self.fault.get_or_insert(BoardEventFault::ByteCredits);
        }
    }

    fn push_frame(&mut self, frame: BoardPropertyAuditFrame) -> Result<(), BoardPropertyAuditFrame> {
        let index = usize::from(self.depth);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(frame);
        }
        self.stack[index] = Some(frame);
        self.depth += 1;
        Ok(())
    }

    pub(super) fn step(&mut self) -> bool {
        if let Some(value) = self.completed.take() {
            if self.depth == 0 {
                self.completed = Some(value);
                return true;
            }
            let frame = self.stack[usize::from(self.depth - 1)].as_mut().expect("property audit frame");
            match frame {
                BoardPropertyAuditFrame::Array { values, pending, .. } => {
                    let index = pending.take().expect("array audit child index");
                    values[index] = value;
                }
                BoardPropertyAuditFrame::Object { values, pending, .. } => {
                    let key = pending.take().expect("object audit child key");
                    *values.get_mut(&key).expect("object audit key remains reserved") = value;
                }
            }
            return false;
        }
        if let Some(value) = self.current.take() {
            self.nodes = self.nodes.saturating_add(1);
            if self.nodes > BOARD_POINTER_ITEM_CAPACITY {
                self.fault.get_or_insert(BoardEventFault::ItemCredits);
            }
            if self.fault.is_some() {
                self.completed = Some(value);
                return false;
            }
            match value {
                semio_framework_graph_manifest::PropertyValue::Array(values) => {
                    let frame = BoardPropertyAuditFrame::Array { values, index: 0, pending: None };
                    if let Err(frame) = self.push_frame(frame) {
                        self.fault = Some(BoardEventFault::ItemCredits);
                        self.completed = Some(match frame {
                            BoardPropertyAuditFrame::Array { values, .. } => semio_framework_graph_manifest::PropertyValue::Array(values),
                            BoardPropertyAuditFrame::Object { .. } => unreachable!(),
                        });
                    }
                }
                semio_framework_graph_manifest::PropertyValue::Object(values) => {
                    let frame = BoardPropertyAuditFrame::Object { values, after: None, pending: None };
                    if let Err(frame) = self.push_frame(frame) {
                        self.fault = Some(BoardEventFault::ItemCredits);
                        self.completed = Some(match frame {
                            BoardPropertyAuditFrame::Object { values, .. } => semio_framework_graph_manifest::PropertyValue::Object(values),
                            BoardPropertyAuditFrame::Array { .. } => unreachable!(),
                        });
                    }
                }
                semio_framework_graph_manifest::PropertyValue::String(value) => {
                    self.add_bytes(value.len());
                    self.completed = Some(semio_framework_graph_manifest::PropertyValue::String(value));
                }
                scalar => self.completed = Some(scalar),
            }
            return false;
        }
        if self.depth == 0 {
            return false;
        }
        let index = usize::from(self.depth - 1);
        if self.fault.is_some() {
            let frame = self.stack[index].take().expect("faulted property audit frame");
            self.depth -= 1;
            self.completed = Some(match frame {
                BoardPropertyAuditFrame::Array { values, .. } => semio_framework_graph_manifest::PropertyValue::Array(values),
                BoardPropertyAuditFrame::Object { values, .. } => semio_framework_graph_manifest::PropertyValue::Object(values),
            });
            return false;
        }
        let frame = self.stack[index].take().expect("property audit frame");
        match frame {
            BoardPropertyAuditFrame::Array { mut values, mut index, pending: _ } => {
                if index == values.len() {
                    self.depth -= 1;
                    self.completed = Some(semio_framework_graph_manifest::PropertyValue::Array(values));
                } else {
                    let child_index = index;
                    index += 1;
                    let pending = Some(child_index);
                    self.current = Some(std::mem::replace(&mut values[child_index], semio_framework_graph_manifest::PropertyValue::Null));
                    self.stack[usize::from(self.depth - 1)] = Some(BoardPropertyAuditFrame::Array { values, index, pending });
                }
            }
            BoardPropertyAuditFrame::Object { mut values, mut after, mut pending } => {
                let next = match after.as_ref() {
                    Some(after) => values.successor(after),
                    None => values.first_key_value(),
                }
                .map(|(key, _)| admitted_board_pointer_id(key));
                match next {
                    Some(Ok(key)) => {
                        self.bytes = self.bytes.saturating_add(key.len());
                        if self.bytes > BOARD_POINTER_BYTE_CAPACITY {
                            self.fault.get_or_insert(BoardEventFault::ByteCredits);
                        }
                        after = Some(key.clone());
                        pending = Some(key.clone());
                        self.current = Some(std::mem::replace(values.get_mut(&key).expect("property audit key"), semio_framework_graph_manifest::PropertyValue::Null));
                        self.stack[usize::from(self.depth - 1)] = Some(BoardPropertyAuditFrame::Object { values, after, pending });
                    }
                    Some(Err(fault)) => {
                        self.fault = Some(fault);
                        self.stack[usize::from(self.depth - 1)] = Some(BoardPropertyAuditFrame::Object { values, after, pending });
                    }
                    None => {
                        self.depth -= 1;
                        self.completed = Some(semio_framework_graph_manifest::PropertyValue::Object(values));
                    }
                }
            }
        }
        false
    }

    pub(super) fn take_result(&mut self) -> Option<(BoardDeleteKind, String, semio_framework_graph_manifest::PropertyBag, usize, usize, Option<BoardEventFault>)> {
        if self.depth != 0 || self.current.is_some() {
            return None;
        }
        let semio_framework_graph_manifest::PropertyValue::Object(properties) = self.completed.take()? else {
            self.fault = Some(BoardEventFault::Schema);
            return None;
        };
        Some((self.kind, std::mem::take(&mut self.id), properties, self.nodes, self.bytes, self.fault.take()))
    }
}

impl BoardPropertyRetirement {
    pub(super) fn new(properties: semio_framework_graph_manifest::PropertyBag) -> Self {
        Self { current: Some(semio_framework_graph_manifest::PropertyValue::Object(properties)), stack: Box::new(std::array::from_fn(|_| None)), depth: 0, faulted: false }
    }

    fn push_container(&mut self, container: BoardPropertyContainer) -> Result<(), BoardPropertyContainer> {
        let index = usize::from(self.depth);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(container);
        }
        self.stack[index] = Some(container);
        self.depth += 1;
        Ok(())
    }

    fn pop_container(&mut self) -> Option<BoardPropertyContainer> {
        if self.depth == 0 {
            return None;
        }
        self.depth -= 1;
        self.stack[usize::from(self.depth)].take()
    }

    pub(super) fn step(&mut self) -> Result<bool, ()> {
        if self.faulted {
            return Err(());
        }
        if self.current.is_none() {
            self.current = self.pop_container().map(|container| match container {
                BoardPropertyContainer::Array(values) => semio_framework_graph_manifest::PropertyValue::Array(values),
                BoardPropertyContainer::Object(values) => semio_framework_graph_manifest::PropertyValue::Object(values),
            });
            return Ok(self.current.is_none());
        }
        let current = self.current.take().expect("property retirement current owner");
        match current {
            semio_framework_graph_manifest::PropertyValue::Array(mut values) => {
                let child = values.pop();
                if !values.is_empty() {
                    if let Err(values) = self.push_container(BoardPropertyContainer::Array(values)) {
                        self.current = Some(match values {
                            BoardPropertyContainer::Array(values) => semio_framework_graph_manifest::PropertyValue::Array(values),
                            BoardPropertyContainer::Object(_) => unreachable!(),
                        });
                        self.faulted = true;
                        return Err(());
                    }
                }
                self.current = child;
            }
            semio_framework_graph_manifest::PropertyValue::Object(mut values) => {
                let child = values.pop_last().map(|(key, value)| {
                    drop(key);
                    value
                });
                if !values.is_empty() {
                    if let Err(values) = self.push_container(BoardPropertyContainer::Object(values)) {
                        self.current = Some(match values {
                            BoardPropertyContainer::Object(values) => semio_framework_graph_manifest::PropertyValue::Object(values),
                            BoardPropertyContainer::Array(_) => unreachable!(),
                        });
                        self.faulted = true;
                        return Err(());
                    }
                }
                self.current = child;
            }
            semio_framework_graph_manifest::PropertyValue::String(value) => drop(value),
            semio_framework_graph_manifest::PropertyValue::Null | semio_framework_graph_manifest::PropertyValue::Bool(_) | semio_framework_graph_manifest::PropertyValue::Number(_) => {}
        }
        Ok(false)
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.current.is_none() && self.depth == 0 && !self.faulted
    }
}

impl BoardEntityRetirement {
    pub(super) fn new(entity: BoardRemovedEntity) -> Self {
        Self { entity: Some(entity), strings: Box::new(std::array::from_fn(|_| None)), string_len: 0, properties: None }
    }

    fn push_string(&mut self, value: String) {
        let index = usize::from(self.string_len);
        self.strings[index] = Some(value);
        self.string_len += 1;
    }

    fn push_optional_string(&mut self, value: Option<String>) {
        if let Some(value) = value {
            self.push_string(value);
        }
    }

    fn split_entity(&mut self, entity: BoardRemovedEntity) {
        let properties = match entity {
            BoardRemovedEntity::Node(NodeData { id, style, text, icon_kind, node_kind, properties, .. }) => {
                self.push_string(id);
                self.push_optional_string(style);
                self.push_optional_string(text);
                self.push_optional_string(icon_kind);
                self.push_string(node_kind);
                properties
            }
            BoardRemovedEntity::Edge(EdgeData { id, source, target, style, edge_kind, source_tip, target_tip, properties, .. }) => {
                self.push_string(id);
                self.push_string(source);
                self.push_string(target);
                self.push_optional_string(style);
                self.push_string(edge_kind);
                self.push_optional_string(source_tip);
                self.push_optional_string(target_tip);
                properties
            }
            BoardRemovedEntity::Handle(HandleData { id, node_id, style, handle_kind, icon_kind, properties, .. }) => {
                self.push_string(id);
                self.push_string(node_id);
                self.push_optional_string(style);
                self.push_string(handle_kind);
                self.push_optional_string(icon_kind);
                properties
            }
            BoardRemovedEntity::Wire(WireData { id, source, target, style, wire_kind, properties, .. }) => {
                self.push_string(id);
                self.push_string(source);
                self.push_optional_string(target);
                self.push_optional_string(style);
                self.push_string(wire_kind);
                properties
            }
        };
        self.properties = Some(BoardPropertyRetirement::new(properties));
    }

    pub(super) fn step(&mut self) -> Result<bool, ()> {
        if let Some(entity) = self.entity.take() {
            self.split_entity(entity);
            return Ok(false);
        }
        if let Some(properties) = self.properties.as_mut() {
            if !properties.step()? {
                return Ok(false);
            }
            debug_assert!(properties.terminal_is_empty());
            self.properties = None;
            return Ok(false);
        }
        if self.string_len > 0 {
            self.string_len -= 1;
            drop(self.strings[usize::from(self.string_len)].take());
            return Ok(false);
        }
        Ok(true)
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.entity.is_none() && self.string_len == 0 && self.properties.is_none()
    }
}

impl BoardPointerPublication {
    pub fn events_json(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("pointer publication is encoded from UTF-8 schema tokens")
    }

    pub fn close_step(&mut self) -> bool {
        self.len = 0;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BoardPointerPlanKind {
    Idle,
    Pan {
        camera: [f64; 3],
    },
    DragMove,
    FinishPan {
        camera: [f64; 3],
    },
    FinishDrag,
    SelectionPreview {
        start: Point,
        start_screen: Point,
    },
    SelectionCommit,
    LinkMove {
        source: BoardPointerSpan,
        target: Option<BoardPointerSpan>,
        hover: Option<BoardPointerSpan>,
        compat_key: BoardPointerSpan,
        ring_key: BoardPointerSpan,
        end_world: Point,
        activated: bool,
        start_screen: Point,
    },
    LinkFinish {
        source: BoardPointerSpan,
        target: Option<BoardPointerSpan>,
        edge: Option<BoardPointerSpan>,
        target_node: Option<BoardPointerSpan>,
        hover: Option<BoardPointerSpan>,
        compat_key: Option<BoardPointerSpan>,
        ring_key: Option<BoardPointerSpan>,
    },
    LinkRetain {
        hover: Option<BoardPointerSpan>,
    },
    Hover {
        hover: Option<BoardPointerSpan>,
    },
    Brush {
        source: Option<BoardPointerSpan>,
        hover: Option<BoardPointerSpan>,
        alt: bool,
        commit_old: bool,
    },
}

#[derive(Debug)]
pub struct BoardPointerPlan {
    pub(super) revision: u64,
    pub(super) kind: BoardPointerPlanKind,
    pub(super) bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub(super) byte_len: u16,
    pub(super) deltas: Box<[Option<BoardPointerDelta>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub(super) delta_len: u16,
    pub(super) points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]>,
    pub(super) screen_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]>,
    pub(super) point_len: u16,
    pub(super) output: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub(super) output_len: u16,
}

pub struct BoardPointerPlanRetirement {
    pub(super) plan: Option<BoardPointerPlan>,
    pub(super) cursor: u16,
}

impl BoardPointerPlanRetirement {
    pub fn new(plan: BoardPointerPlan) -> Self {
        Self { plan: Some(plan), cursor: 0 }
    }

    pub fn close_step(&mut self) -> bool {
        let Some(plan) = self.plan.as_mut() else {
            return true;
        };
        if self.cursor < plan.delta_len {
            plan.deltas[usize::from(self.cursor)] = None;
            self.cursor += 1;
            return false;
        }
        plan.byte_len = 0;
        plan.output_len = 0;
        self.plan = None;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.plan.is_none()
    }
}

impl BoardPointerPlan {
    pub(super) fn empty(revision: u64, kind: BoardPointerPlanKind) -> Self {
        Self {
            revision,
            kind,
            bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]),
            byte_len: 0,
            deltas: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]),
            delta_len: 0,
            points: Box::new(std::array::from_fn(|_| Point::new(0.0, 0.0))),
            screen_points: Box::new(std::array::from_fn(|_| Point::new(0.0, 0.0))),
            point_len: 0,
            output: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]),
            output_len: 0,
        }
    }

    pub(super) fn push_id(&mut self, id: &str) -> Result<BoardPointerSpan, BoardPointerPlanFault> {
        if id.len() > u16::MAX as usize {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        let start = usize::from(self.byte_len);
        let end = start.checked_add(id.len()).ok_or(BoardPointerPlanFault::ByteCredits)?;
        if end > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        self.bytes[start..end].copy_from_slice(id.as_bytes());
        self.byte_len = end as u16;
        Ok(BoardPointerSpan { start: start as u16, len: id.len() as u16 })
    }

    pub(super) fn push_delta(&mut self, id: &str, x: f64, y: f64) -> Result<(), BoardPointerPlanFault> {
        let index = usize::from(self.delta_len);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        let id = self.push_id(id)?;
        self.deltas[index] = Some(BoardPointerDelta { id, x, y });
        self.delta_len += 1;
        Ok(())
    }

    pub(super) fn id(&self, span: BoardPointerSpan) -> &str {
        let start = usize::from(span.start);
        let end = start + usize::from(span.len);
        std::str::from_utf8(&self.bytes[start..end]).expect("board pointer ids originate from UTF-8 strings")
    }

    pub(super) fn push_point(&mut self, world: Point, screen: Point) -> Result<(), BoardPointerPlanFault> {
        let index = usize::from(self.point_len);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        self.points[index] = world;
        self.screen_points[index] = screen;
        self.point_len += 1;
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn selection_ids(&self) -> impl Iterator<Item = &str> {
        self.deltas[..usize::from(self.delta_len)].iter().flatten().map(|delta| self.id(delta.id))
    }

    /// 🎥️ The camera this plan settles on the view lane: a finished pan's. A camera move is view state, so the host
    /// publishes it as its view verb (`setCamera`) and never as a board event.
    pub fn view_camera(&self) -> Option<[f64; 3]> {
        match self.kind {
            BoardPointerPlanKind::FinishPan { camera } => Some(camera),
            _ => None,
        }
    }

    /// 📮️ Whether committing this plan puts anything on the host's action queue: board events or a view camera.
    pub fn publishes(&self) -> bool {
        self.event_count() > 0 || self.view_camera().is_some()
    }

    pub fn event_count(&self) -> usize {
        match self.kind {
            BoardPointerPlanKind::FinishDrag => usize::from(self.output_len > 2),
            BoardPointerPlanKind::SelectionPreview { .. } | BoardPointerPlanKind::SelectionCommit => 1,
            BoardPointerPlanKind::LinkMove { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::LinkFinish { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::LinkRetain { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::Hover { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::Brush { .. } => usize::from(self.output_len > 2),
            _ => 0,
        }
    }

    pub fn requires_retained_commit(&self) -> bool {
        !matches!(self.kind, BoardPointerPlanKind::Idle)
    }

    /// 🈳️ Seals the empty event page of a hand-built plan that publishes no board event.
    #[cfg(test)]
    pub(super) fn seal_events(&mut self) -> Result<(), BoardPointerPlanFault> {
        self.output_len = 0;
        self.output_raw("[]")
    }

    fn output_raw(&mut self, value: &str) -> Result<(), BoardPointerPlanFault> {
        let start = usize::from(self.output_len);
        let end = start.checked_add(value.len()).ok_or(BoardPointerPlanFault::ByteCredits)?;
        if end > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        self.output[start..end].copy_from_slice(value.as_bytes());
        self.output_len = end as u16;
        Ok(())
    }

    pub(super) fn seal_owned_event(&mut self, event: &BoardOwnedEvent) -> Result<(), BoardPointerPlanFault> {
        self.output_len = 0;
        self.output_raw("[{\"name\":\"")?;
        self.output_raw(event.kind().name())?;
        self.output_raw("\",\"payload\":")?;
        self.output_raw(event.payload_json())?;
        self.output_raw("}]")
    }

    pub(super) fn seal_optional_events<const N: usize>(&mut self, events: &[Option<BoardOwnedEvent>; N]) -> Result<(), BoardPointerPlanFault> {
        self.output_len = 0;
        self.output_raw("[")?;
        for (emitted, event) in events.iter().flatten().enumerate() {
            if emitted > 0 {
                self.output_raw(",")?;
            }
            self.output_raw("{\"name\":\"")?;
            self.output_raw(event.kind().name())?;
            self.output_raw("\",\"payload\":")?;
            self.output_raw(event.payload_json())?;
            self.output_raw("}")?;
        }
        self.output_raw("]")
    }

    pub fn events_json(&self) -> &str {
        std::str::from_utf8(&self.output[..usize::from(self.output_len)]).expect("board event page is encoded from UTF-8 schema tokens")
    }

}

fn board_json_string_bytes(value: &str) -> usize {
    value
        .chars()
        .map(|character| match character {
            '"' | '\\' | '\n' | '\r' | '\t' => 2,
            character if character <= '\u{1f}' => 6,
            character => character.len_utf8(),
        })
        .try_fold(2usize, usize::checked_add)
        .unwrap_or(usize::MAX)
}

pub(super) fn admitted_board_pointer_id(value: &str) -> Result<String, BoardEventFault> {
    if value.len() > BOARD_POINTER_BYTE_CAPACITY {
        return Err(BoardEventFault::ByteCredits);
    }
    Ok(value.to_owned())
}

pub(super) fn board_node_move_owned_bytes(id: &str, x: f64, y: f64) -> Option<usize> {
    if id.len() > BOARD_EVENT_KEY_BYTE_CAPACITY || !x.is_finite() || !y.is_finite() {
        return None;
    }
    17usize.checked_add(board_json_string_bytes(id))?.checked_add(x.to_string().len())?.checked_add(y.to_string().len())?.checked_add(id.len())
}

pub(super) fn board_edge_event_owned_bytes(id: &str, source: &str, target: &str) -> Option<usize> {
    let bytes = 27usize.checked_add(board_json_string_bytes(id))?.checked_add(board_json_string_bytes(source))?.checked_add(board_json_string_bytes(target))?;
    (bytes <= BOARD_EVENT_PAYLOAD_BYTE_CAPACITY).then_some(bytes)
}

impl BoardWheelPlan {
    pub fn camera(&self) -> [f64; 3] {
        [self.next.x, self.next.y, self.next.zoom]
    }
}
// #endregion board_host

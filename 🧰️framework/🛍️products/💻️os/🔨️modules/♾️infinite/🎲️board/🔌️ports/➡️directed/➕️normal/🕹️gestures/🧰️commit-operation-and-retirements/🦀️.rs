//! 🧰️ Commit operation and retirements of the board gestures: the pointer commit operation impl, the authority step and the property, entity, publication and plan retirement impls and the commit state. See [`super`].

use super::*;

impl BoardPointerCommitOperation {
    pub fn new(plan: BoardPointerPlan) -> Self {
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

    pub fn retire_one(&mut self) -> bool {
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

impl BoardPropertyRetirement {
    pub fn new(properties: semio_framework_graph_manifest::PropertyBag) -> Self {
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

    pub fn step(&mut self) -> Result<bool, ()> {
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

    pub fn terminal_is_empty(&self) -> bool {
        self.current.is_none() && self.depth == 0 && !self.faulted
    }
}

impl BoardEntityRetirement {
    pub fn new(entity: BoardRemovedEntity) -> Self {
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

    pub fn step(&mut self) -> Result<bool, ()> {
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

    pub fn terminal_is_empty(&self) -> bool {
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

pub struct BoardPointerPlanRetirement {
    pub plan: Option<BoardPointerPlan>,
    pub cursor: u16,
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

impl BoardCommitState {
    pub fn take_pointer_publication(&mut self) -> Option<BoardPointerPublication> {
        self.pointer_publication.take()
    }

    pub fn pointer_publication(&self) -> Option<&BoardPointerPublication> {
        self.pointer_publication.as_ref()
    }

    pub fn pointer_cancel_pending(&self) -> bool {
        self.cancelled_pointer_plan_retirement.is_some() || self.pending_pointer_commit.as_ref().is_some_and(|operation| operation.cancelling)
    }

    pub fn pointer_authority_terminal_is_empty(&self) -> bool {
        self.pending_pointer_commit.is_none() && self.queued_pointer_commit.is_none() && self.cancelled_pointer_plan_retirement.is_none() && self.pointer_publication.is_none()
    }
    // #endregion board_host
}

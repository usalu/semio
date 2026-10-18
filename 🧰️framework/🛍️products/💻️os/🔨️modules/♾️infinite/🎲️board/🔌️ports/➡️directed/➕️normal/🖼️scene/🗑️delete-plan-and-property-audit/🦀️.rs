//! 🗑️ Delete plan and property audit of the board scene: the delete plan methods and the property audit methods. See [`super`].

use super::*;

impl BoardDeletePlan {
    pub fn new(revision: u64) -> Self {
        Self { revision, bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]), byte_len: 0, entries: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]), len: 0, property_nodes: 0, property_bytes: 0 }
    }

    pub fn id(&self, span: BoardPointerSpan) -> &str {
        let start = usize::from(span.start);
        let end = start + usize::from(span.len);
        std::str::from_utf8(&self.bytes[start..end]).expect("board deletion ids originate from UTF-8 strings")
    }

    pub fn contains(&self, kind: BoardDeleteKind, id: &str) -> bool {
        self.entries[..usize::from(self.len)].iter().flatten().any(|entry| entry.kind == kind && self.id(entry.id) == id)
    }

    pub fn removes_selection_id(&self, id: &str) -> bool {
        self.entries[..usize::from(self.len)].iter().flatten().any(|entry| self.id(entry.id) == id)
    }

    pub fn push(&mut self, kind: BoardDeleteKind, id: &str) -> Result<(), BoardEventFault> {
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

    #[cfg(any(test, feature = "testkit"))]
    pub fn push_entity(&mut self, kind: BoardDeleteKind, id: &str, properties: &semio_framework_graph_manifest::PropertyBag) -> Result<(), BoardEventFault> {
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

    pub fn push_admitted_entity(&mut self, kind: BoardDeleteKind, id: &str, property_nodes: usize, property_bytes: usize) -> Result<(), BoardEventFault> {
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

    pub fn emitted_event_count(&self) -> usize {
        self.entries[..usize::from(self.len)].iter().flatten().filter(|entry| matches!(entry.kind, BoardDeleteKind::Edge | BoardDeleteKind::Node)).count()
    }

    pub fn emitted_event_bytes(&self) -> Option<usize> {
        self.entries[..usize::from(self.len)]
            .iter()
            .flatten()
            .filter(|entry| matches!(entry.kind, BoardDeleteKind::Edge | BoardDeleteKind::Node))
            .try_fold(0usize, |bytes, entry| bytes.checked_add(7)?.checked_add(board_json_string_bytes(self.id(entry.id))))
    }
}

impl BoardPropertyAudit {
    pub fn new(kind: BoardDeleteKind, id: String, properties: semio_framework_graph_manifest::PropertyBag) -> Self {
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

    pub fn step(&mut self) -> bool {
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

    pub fn take_result(&mut self) -> Option<(BoardDeleteKind, String, semio_framework_graph_manifest::PropertyBag, usize, usize, Option<BoardEventFault>)> {
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

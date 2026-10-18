//! 🏗️ Bounded action builder: the `BoundedActionBuilder` that records the flat action values within the byte and node capacities. See [`super`].

use super::*;

impl BoundedActionBuilder {
    pub(super) fn new(controller_id: &str, action: &str, reserved_bytes: usize) -> Result<Self, BoundedActionFault> {
        if reserved_bytes > ACTION_ITEM_BYTE_CAPACITY {
            return Err(BoundedActionFault::ByteCredits);
        }
        if controller_id.len() > ACTION_STRING_BYTE_CAPACITY || action.len() > ACTION_STRING_BYTE_CAPACITY {
            return Err(BoundedActionFault::StringCredits);
        }
        let mut builder = Self {
            action: BoundedAction {
                controller_id: TextSpan { start: 0, len: 0 },
                action: TextSpan { start: 0, len: 0 },
                nodes: Box::new([None; ACTION_NODE_CAPACITY]),
                bytes: Box::new([0; ACTION_ITEM_BYTE_CAPACITY]),
                node_len: 0,
                byte_len: 0,
                root: None,
                batch_remaining: 1,
                receipt: None,
            },
            parents: [None; ACTION_DEPTH_CAPACITY],
            depth: 0,
            reserved_bytes,
            fault: None,
        };
        builder.action.controller_id = builder.copy_text(controller_id)?;
        builder.action.action = builder.copy_text(action)?;
        Ok(builder)
    }

    pub fn set_receipt(&mut self, receipt: ActionQueueReceipt) -> Result<(), BoundedActionFault> {
        self.live()?;
        if self.action.receipt.is_some() {
            return self.poison(BoundedActionFault::Structure);
        }
        self.action.receipt = Some(receipt);
        Ok(())
    }

    pub fn begin_object(&mut self, key: Option<&str>) -> Result<(), BoundedActionFault> {
        self.begin_container(key, FlatValue::Object)
    }

    pub fn begin_array(&mut self, key: Option<&str>) -> Result<(), BoundedActionFault> {
        self.begin_container(key, FlatValue::Array)
    }

    pub fn end_container(&mut self) -> Result<(), BoundedActionFault> {
        self.live()?;
        if self.depth == 0 {
            return self.poison(BoundedActionFault::Structure);
        }
        self.depth -= 1;
        self.parents[self.depth] = None;
        Ok(())
    }

    pub fn null(&mut self, key: Option<&str>) -> Result<(), BoundedActionFault> {
        self.push_leaf(key, FlatValue::Null)
    }

    pub fn boolean(&mut self, key: Option<&str>, value: bool) -> Result<(), BoundedActionFault> {
        self.push_leaf(key, FlatValue::Bool(value))
    }

    pub fn number(&mut self, key: Option<&str>, value: f64) -> Result<(), BoundedActionFault> {
        self.push_leaf(key, FlatValue::Number(Number::Float(value)))
    }

    /// 🔢️ Whole-number leaf — an id, count or index the wire must carry WITHOUT the `.0` a float
    /// carrier keeps, which is what React's `readonly number[]` selection payloads serialize to
    /// (`🌐️World3dHost/🟦️.tsx:3236`, `:6984`).
    pub fn integer(&mut self, key: Option<&str>, value: i64) -> Result<(), BoundedActionFault> {
        self.push_leaf(key, FlatValue::Number(Number::Int(value)))
    }

    pub fn string(&mut self, key: Option<&str>, value: &str) -> Result<(), BoundedActionFault> {
        self.live()?;
        if value.len() > ACTION_STRING_BYTE_CAPACITY {
            return self.poison(BoundedActionFault::StringCredits);
        }
        let span = match self.copy_text(value) {
            Ok(span) => span,
            Err(fault) => return self.poison(fault),
        };
        self.push_node(key, FlatValue::String(span)).map(|_| ())
    }

    /// 🧬️ Retains intrinsic octets using the action's reserved byte credits and one value node.
    pub fn bytes(&mut self, key: Option<&str>, value: &[u8]) -> Result<(), BoundedActionFault> {
        self.live()?;
        let span = match self.copy_octets(value) {
            Ok(span) => span,
            Err(fault) => return self.poison(fault),
        };
        self.push_node(key, FlatValue::Bytes(span)).map(|_| ())
    }

    pub fn string_joined(&mut self, key: Option<&str>, parts: &[&str]) -> Result<(), BoundedActionFault> {
        self.live()?;
        let mut len = 0usize;
        for part in parts {
            len = len.checked_add(part.len()).ok_or(BoundedActionFault::ByteCredits)?;
            if len > ACTION_STRING_BYTE_CAPACITY {
                return self.poison(BoundedActionFault::StringCredits);
            }
        }
        let start = self.action.byte_len;
        let end = start.checked_add(len).ok_or(BoundedActionFault::ByteCredits)?;
        if end > self.reserved_bytes || end > ACTION_ITEM_BYTE_CAPACITY {
            return self.poison(BoundedActionFault::ByteCredits);
        }
        let mut cursor = start;
        for part in parts {
            let next = cursor + part.len();
            self.action.bytes[cursor..next].copy_from_slice(part.as_bytes());
            cursor = next;
        }
        self.action.byte_len = end;
        let span = TextSpan { start: start as u16, len: len as u16 };
        self.push_node(key, FlatValue::String(span)).map(|_| ())
    }

    pub fn value(&mut self, key: Option<&str>, value: &DslValue) -> Result<(), BoundedActionFault> {
        self.copy_value(key, value, 0)
    }

    pub fn finish(self) -> Result<BoundedAction, BoundedActionFault> {
        if let Some(fault) = self.fault {
            return Err(fault);
        }
        if self.depth != 0 || self.action.byte_len > self.reserved_bytes {
            return Err(BoundedActionFault::Structure);
        }
        Ok(self.action)
    }

    fn begin_container(&mut self, key: Option<&str>, value: FlatValue) -> Result<(), BoundedActionFault> {
        self.live()?;
        if self.depth == ACTION_DEPTH_CAPACITY {
            return self.poison(BoundedActionFault::DepthCredits);
        }
        let index = self.push_node(key, value)?;
        self.parents[self.depth] = Some(index);
        self.depth += 1;
        Ok(())
    }

    fn copy_value(&mut self, key: Option<&str>, value: &DslValue, depth: usize) -> Result<(), BoundedActionFault> {
        if depth >= ACTION_DEPTH_CAPACITY {
            return self.poison(BoundedActionFault::DepthCredits);
        }
        match value {
            DslValue::Null => self.null(key),
            DslValue::Bool(value) => self.boolean(key, *value),
            DslValue::Number(value) => self.push_leaf(key, FlatValue::Number(*value)),
            DslValue::String(value) => self.string(key, value),
            DslValue::Bytes(value) => self.bytes(key, value),
            DslValue::Array(values) => {
                self.begin_array(key)?;
                for value in values {
                    self.copy_value(None, value, depth + 1)?;
                }
                self.end_container()
            }
            DslValue::Object(entries) => {
                self.begin_object(key)?;
                for (key, value) in entries {
                    self.copy_value(Some(key), value, depth + 1)?;
                }
                self.end_container()
            }
        }
    }

    fn push_leaf(&mut self, key: Option<&str>, value: FlatValue) -> Result<(), BoundedActionFault> {
        self.live()?;
        self.push_node(key, value).map(|_| ())
    }

    fn push_node(&mut self, key: Option<&str>, value: FlatValue) -> Result<u16, BoundedActionFault> {
        self.live()?;
        if self.action.node_len == ACTION_NODE_CAPACITY {
            return self.poison(BoundedActionFault::NodeCredits);
        }
        if self.depth == 0 && self.action.root.is_some() {
            return self.poison(BoundedActionFault::Structure);
        }
        let key = match key {
            Some(key) if key.len() > ACTION_STRING_BYTE_CAPACITY => return self.poison(BoundedActionFault::StringCredits),
            Some(key) => match self.copy_text(key) {
                Ok(span) => Some(span),
                Err(fault) => return self.poison(fault),
            },
            None => None,
        };
        let index = self.action.node_len as u16;
        self.action.nodes[index as usize] = Some(FlatNode::new(key, value));
        self.action.node_len += 1;
        if let Some(parent) = self.depth.checked_sub(1).and_then(|depth| self.parents[depth]) {
            let last_child = self.action.nodes[parent as usize].as_ref().and_then(|node| node.last_child);
            if let Some(last_child) = last_child {
                self.action.nodes[last_child as usize].as_mut().expect("bounded action sibling").next_sibling = Some(index);
            } else {
                self.action.nodes[parent as usize].as_mut().expect("bounded action parent").first_child = Some(index);
            }
            self.action.nodes[parent as usize].as_mut().expect("bounded action parent").last_child = Some(index);
        } else {
            self.action.root = Some(index);
        }
        Ok(index)
    }

    fn copy_text(&mut self, value: &str) -> Result<TextSpan, BoundedActionFault> {
        self.copy_octets(value.as_bytes())
    }

    fn copy_octets(&mut self, value: &[u8]) -> Result<TextSpan, BoundedActionFault> {
        let end = self.action.byte_len.checked_add(value.len()).ok_or(BoundedActionFault::ByteCredits)?;
        if end > self.reserved_bytes || end > ACTION_ITEM_BYTE_CAPACITY {
            return Err(BoundedActionFault::ByteCredits);
        }
        let start = self.action.byte_len;
        self.action.bytes[start..end].copy_from_slice(value);
        self.action.byte_len = end;
        Ok(TextSpan { start: start as u16, len: value.len() as u16 })
    }

    fn live(&self) -> Result<(), BoundedActionFault> {
        self.fault.map_or(Ok(()), Err)
    }

    fn poison<T>(&mut self, fault: BoundedActionFault) -> Result<T, BoundedActionFault> {
        self.fault = Some(fault);
        Err(fault)
    }
}

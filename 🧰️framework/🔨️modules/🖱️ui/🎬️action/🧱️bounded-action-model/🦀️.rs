//! 🧱️ Bounded action model: the bounded faults, the flat action value model, the queue receipts, the retained string action and the `BoundedAction` with its builder state. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundedActionFault {
    ItemCredits,
    NodeCredits,
    DepthCredits,
    StringCredits,
    ByteCredits,
    Structure,
}

pub fn checked_action_string_bytes(parts: &[&str]) -> Result<usize, BoundedActionFault> {
    let mut bytes = 0usize;
    for part in parts {
        if part.len() > ACTION_STRING_BYTE_CAPACITY {
            return Err(BoundedActionFault::StringCredits);
        }
        bytes = bytes.checked_add(part.len()).ok_or(BoundedActionFault::ByteCredits)?;
        if bytes > ACTION_ITEM_BYTE_CAPACITY {
            return Err(BoundedActionFault::ByteCredits);
        }
    }
    Ok(bytes)
}

#[derive(Clone, Copy, Debug)]
pub(super) struct TextSpan {
    pub(super) start: u16,
    pub(super) len: u16,
}

/// 🔢️ Carries `semio_framework_value::Number` variant-for-variant rather than a widened `f64`, so an integer queued
/// through the ring rehydrates as the same integer `copy_value` flattened — `{"value": 7}` came back
/// as `7.0` while this held `f64`.
#[derive(Clone, Copy, Debug)]
pub(super) enum FlatValue {
    Null,
    Bool(bool),
    Number(Number),
    String(TextSpan),
    Bytes(TextSpan),
    Array,
    Object,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct FlatNode {
    pub(super) key: Option<TextSpan>,
    pub(super) value: FlatValue,
    pub(super) first_child: Option<u16>,
    pub(super) last_child: Option<u16>,
    pub(super) next_sibling: Option<u16>,
}

impl FlatNode {
    pub(super) fn new(key: Option<TextSpan>, value: FlatValue) -> Self {
        Self { key, value, first_child: None, last_child: None, next_sibling: None }
    }
}

/// 🧾️ Selects the renderer-local owner that settles an admitted action receipt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionQueueReceiptSource {
    CanvasTextEditor,
    RetainedInput,
}

/// 🧾️ Identifies one member of an admitted renderer emission without changing its wire payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionQueueReceipt {
    pub source: ActionQueueReceiptSource,
    pub token: std::num::NonZeroU64,
    pub member: u8,
    pub abort_correlation_on_error: bool,
}

/// 📥️ Transfers an action and its local receipt to the runtime dispatcher.
#[derive(Debug)]
pub struct QueuedActionDescriptor {
    pub descriptor: ActionDescriptor,
    pub receipt: Option<ActionQueueReceipt>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedStringActionStep {
    Pending { copied_bytes: usize, total_bytes: usize },
    Complete,
}

/// 📦️ Builds one atomic action string over bounded UTF-8 pages while the caller retains the source.
pub struct RetainedStringAction {
    pub(super) descriptor: ActionDescriptor,
    argument: String,
    output: String,
    pub(super) total_bytes: usize,
}

impl RetainedStringAction {
    pub fn new(descriptor: ActionDescriptor, argument: String, total_bytes: usize) -> Result<Self, BoundedActionFault> {
        if descriptor.controller_id.is_empty() || descriptor.action.is_empty() || argument.is_empty() {
            return Err(BoundedActionFault::Structure);
        }
        checked_action_string_bytes(&[&descriptor.controller_id, &descriptor.action, &argument])?;
        if total_bytes > RETAINED_ACTION_STRING_BYTE_CAPACITY {
            return Err(BoundedActionFault::ByteCredits);
        }
        if descriptor.args.as_ref().is_some_and(|args| !matches!(args, DslValue::Object(_))) {
            return Err(BoundedActionFault::Structure);
        }
        let mut output = String::new();
        output.try_reserve_exact(total_bytes).map_err(|_| BoundedActionFault::ByteCredits)?;
        Ok(Self { descriptor, argument, output, total_bytes })
    }

    pub fn copied_bytes(&self) -> usize {
        self.output.len()
    }

    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn progress(&self) -> f32 {
        if self.total_bytes == 0 {
            1.0
        } else {
            self.output.len() as f32 / self.total_bytes as f32
        }
    }

    pub fn push_page(&mut self, page: &str) -> Result<RetainedStringActionStep, BoundedActionFault> {
        if page.len() > RETAINED_ACTION_STRING_PAGE_BYTES || page.is_empty() && self.output.len() != self.total_bytes {
            return Err(if page.len() > RETAINED_ACTION_STRING_PAGE_BYTES { BoundedActionFault::ByteCredits } else { BoundedActionFault::Structure });
        }
        let next = self.output.len().checked_add(page.len()).ok_or(BoundedActionFault::ByteCredits)?;
        if next > self.total_bytes {
            return Err(BoundedActionFault::ByteCredits);
        }
        self.output.push_str(page);
        Ok(if next == self.total_bytes { RetainedStringActionStep::Complete } else { RetainedStringActionStep::Pending { copied_bytes: next, total_bytes: self.total_bytes } })
    }

    pub fn finish(mut self, receipt: Option<ActionQueueReceipt>) -> Result<QueuedActionDescriptor, BoundedActionFault> {
        if self.output.len() != self.total_bytes {
            return Err(BoundedActionFault::Structure);
        }
        let mut entries = match self.descriptor.args.take() {
            Some(DslValue::Object(entries)) => entries,
            None => Vec::new(),
            _ => return Err(BoundedActionFault::Structure),
        };
        entries.retain(|(key, _)| key != &self.argument);
        entries.push((self.argument, DslValue::String(self.output)));
        self.descriptor.args = Some(DslValue::Object(entries));
        Ok(QueuedActionDescriptor { descriptor: self.descriptor, receipt })
    }
}

#[derive(Debug)]
pub struct BoundedAction {
    pub(super) controller_id: TextSpan,
    pub(super) action: TextSpan,
    pub(super) nodes: Box<[Option<FlatNode>; ACTION_NODE_CAPACITY]>,
    pub(super) bytes: Box<[u8; ACTION_ITEM_BYTE_CAPACITY]>,
    pub(super) node_len: usize,
    pub(super) byte_len: usize,
    pub(super) root: Option<u16>,
    pub(super) batch_remaining: u8,
    pub(super) receipt: Option<ActionQueueReceipt>,
}

impl BoundedAction {
    pub fn receipt(&self) -> Option<ActionQueueReceipt> {
        self.receipt
    }

    pub fn into_envelope(self) -> Result<QueuedActionDescriptor, BoundedActionFault> {
        let receipt = self.receipt;
        Ok(QueuedActionDescriptor { descriptor: self.into_descriptor()?, receipt })
    }

    pub fn owned_bytes(&self) -> usize {
        self.byte_len
    }

    pub(super) fn batch_remaining(&self) -> usize {
        usize::from(self.batch_remaining)
    }

    pub fn into_descriptor(self) -> Result<ActionDescriptor, BoundedActionFault> {
        let controller_id = self.text(self.controller_id)?.to_owned();
        let action = self.text(self.action)?.to_owned();
        let args = self.root.map(|root| self.materialize_node(root, 0)).transpose()?;
        Ok(ActionDescriptor { controller_id, action, args })
    }

    pub(super) fn text(&self, span: TextSpan) -> Result<&str, BoundedActionFault> {
        let start = usize::from(span.start);
        let end = start.checked_add(usize::from(span.len)).ok_or(BoundedActionFault::Structure)?;
        std::str::from_utf8(self.bytes.get(start..end).ok_or(BoundedActionFault::Structure)?).map_err(|_| BoundedActionFault::Structure)
    }

    fn materialize_node(&self, index: u16, depth: usize) -> Result<DslValue, BoundedActionFault> {
        if depth > ACTION_DEPTH_CAPACITY {
            return Err(BoundedActionFault::DepthCredits);
        }
        let node = self.nodes.get(index as usize).and_then(Option::as_ref).ok_or(BoundedActionFault::Structure)?;
        match node.value {
            FlatValue::Null => Ok(DslValue::Null),
            FlatValue::Bool(value) => Ok(DslValue::Bool(value)),
            FlatValue::Number(value) => Ok(DslValue::Number(value)),
            FlatValue::String(span) => Ok(DslValue::String(self.text(span)?.to_owned())),
            FlatValue::Bytes(span) => {
                let start = usize::from(span.start);
                let end = start.checked_add(usize::from(span.len)).ok_or(BoundedActionFault::Structure)?;
                Ok(DslValue::Bytes(self.bytes.get(start..end).ok_or(BoundedActionFault::Structure)?.to_vec()))
            }
            FlatValue::Array => {
                let mut values = Vec::with_capacity(self.child_count(node)?);
                let mut child = node.first_child;
                while let Some(index) = child {
                    let child_node = self.nodes[index as usize].as_ref().ok_or(BoundedActionFault::Structure)?;
                    values.push(self.materialize_node(index, depth + 1)?);
                    child = child_node.next_sibling;
                }
                Ok(DslValue::Array(values))
            }
            FlatValue::Object => {
                let mut entries = Vec::with_capacity(self.child_count(node)?);
                let mut child = node.first_child;
                while let Some(index) = child {
                    let child_node = self.nodes[index as usize].as_ref().ok_or(BoundedActionFault::Structure)?;
                    let key = self.text(child_node.key.ok_or(BoundedActionFault::Structure)?)?.to_owned();
                    entries.push((key, self.materialize_node(index, depth + 1)?));
                    child = child_node.next_sibling;
                }
                Ok(DslValue::Object(entries))
            }
        }
    }

    fn child_count(&self, node: &FlatNode) -> Result<usize, BoundedActionFault> {
        let mut count = 0usize;
        let mut child = node.first_child;
        while let Some(index) = child {
            count = count.checked_add(1).ok_or(BoundedActionFault::Structure)?;
            if count > ACTION_NODE_CAPACITY {
                return Err(BoundedActionFault::Structure);
            }
            child = self.nodes[index as usize].as_ref().ok_or(BoundedActionFault::Structure)?.next_sibling;
        }
        Ok(count)
    }
}

#[derive(Debug)]
pub struct BoundedActionBuilder {
    pub(super) action: BoundedAction,
    pub(super) parents: [Option<u16>; ACTION_DEPTH_CAPACITY],
    pub(super) depth: usize,
    pub(super) reserved_bytes: usize,
    pub(super) fault: Option<BoundedActionFault>,
}

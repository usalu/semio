//! 🧾 Fixed-credit action construction and FIFO ownership for interactive renderer input.

use crate::wgpu::ActionDescriptor;
use dsl::DslValue;

pub const ACTION_QUEUE_ITEM_CAPACITY: usize = 256;
pub const ACTION_BATCH_ITEM_CAPACITY: usize = 16;
pub const ACTION_NODE_CAPACITY: usize = 256;
pub const ACTION_DEPTH_CAPACITY: usize = 32;
pub const ACTION_STRING_BYTE_CAPACITY: usize = 4 * 1024;
pub const ACTION_ITEM_BYTE_CAPACITY: usize = 16 * 1024;
pub const ACTION_QUEUE_BYTE_CAPACITY: usize = 1024 * 1024;
pub const ACTION_CLAIM_CAPACITY: usize = 256;

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
struct TextSpan {
    start: u16,
    len: u16,
}

#[derive(Clone, Copy, Debug)]
enum FlatValue {
    Null,
    Bool(bool),
    Number(f64),
    String(TextSpan),
    Array,
    Object,
}

#[derive(Clone, Copy, Debug)]
struct FlatNode {
    key: Option<TextSpan>,
    value: FlatValue,
    first_child: Option<u16>,
    last_child: Option<u16>,
    next_sibling: Option<u16>,
}

impl FlatNode {
    fn new(key: Option<TextSpan>, value: FlatValue) -> Self {
        Self { key, value, first_child: None, last_child: None, next_sibling: None }
    }
}

#[derive(Debug)]
pub struct BoundedAction {
    controller_id: TextSpan,
    action: TextSpan,
    nodes: Box<[Option<FlatNode>; ACTION_NODE_CAPACITY]>,
    bytes: Box<[u8; ACTION_ITEM_BYTE_CAPACITY]>,
    node_len: usize,
    byte_len: usize,
    root: Option<u16>,
}

impl BoundedAction {
    pub fn owned_bytes(&self) -> usize {
        self.byte_len
    }

    pub fn into_descriptor(self) -> Result<ActionDescriptor, BoundedActionFault> {
        let controller_id = self.text(self.controller_id)?.to_owned();
        let action = self.text(self.action)?.to_owned();
        let args = self.root.map(|root| self.materialize_node(root, 0)).transpose()?;
        Ok(ActionDescriptor { controller_id, action, args })
    }

    fn text(&self, span: TextSpan) -> Result<&str, BoundedActionFault> {
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
    action: BoundedAction,
    parents: [Option<u16>; ACTION_DEPTH_CAPACITY],
    depth: usize,
    reserved_bytes: usize,
    fault: Option<BoundedActionFault>,
}

impl BoundedActionBuilder {
    fn new(controller_id: &str, action: &str, reserved_bytes: usize) -> Result<Self, BoundedActionFault> {
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
        self.push_leaf(key, FlatValue::Number(value))
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
            DslValue::Number(value) => self.number(key, *value),
            DslValue::String(value) => self.string(key, value),
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
        let end = self.action.byte_len.checked_add(value.len()).ok_or(BoundedActionFault::ByteCredits)?;
        if end > self.reserved_bytes || end > ACTION_ITEM_BYTE_CAPACITY {
            return Err(BoundedActionFault::ByteCredits);
        }
        let start = self.action.byte_len;
        self.action.bytes[start..end].copy_from_slice(value.as_bytes());
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

pub struct BoundedActionReservation<'a> {
    queue: &'a mut BoundedActionQueue,
    builder: BoundedActionBuilder,
}

pub struct BoundedActionBatchReservation<'a> {
    queue: &'a mut BoundedActionQueue,
    actions: [Option<BoundedAction>; ACTION_BATCH_ITEM_CAPACITY],
    item_credits: usize,
    byte_credits: usize,
    len: usize,
    declared_bytes: usize,
    bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundedActionClaim {
    slot: u16,
    epoch: u64,
    byte_credits: usize,
}

#[derive(Clone, Copy)]
struct BoundedActionClaimSlot {
    epoch: u64,
    byte_credits: usize,
}

pub struct BoundedClaimedActionReservation<'a> {
    queue: &'a mut BoundedActionQueue,
    claim: BoundedActionClaim,
    builder: BoundedActionBuilder,
}

impl BoundedActionBatchReservation<'_> {
    pub fn action(&mut self, controller_id: &str, action: &str, byte_credits: usize, build: impl FnOnce(&mut BoundedActionBuilder) -> Result<(), BoundedActionFault>) -> Result<(), BoundedActionFault> {
        if self.len == self.item_credits || self.len == ACTION_BATCH_ITEM_CAPACITY {
            return Err(BoundedActionFault::ItemCredits);
        }
        let next = self.declared_bytes.checked_add(byte_credits).ok_or(BoundedActionFault::ByteCredits)?;
        if byte_credits > ACTION_ITEM_BYTE_CAPACITY || next > self.byte_credits {
            return Err(BoundedActionFault::ByteCredits);
        }
        let mut builder = BoundedActionBuilder::new(controller_id, action, byte_credits)?;
        build(&mut builder)?;
        let action = builder.finish()?;
        self.declared_bytes = next;
        self.bytes = self.bytes.checked_add(action.owned_bytes()).ok_or(BoundedActionFault::ByteCredits)?;
        self.actions[self.len] = Some(action);
        self.len += 1;
        Ok(())
    }

    pub fn publish(mut self) -> Result<(), BoundedActionFault> {
        if self.len != self.item_credits || self.bytes > self.byte_credits {
            return Err(BoundedActionFault::ItemCredits);
        }
        self.publish_staged();
        Ok(())
    }

    pub fn publish_with(mut self, commit: impl FnOnce()) -> Result<(), BoundedActionFault> {
        if self.len != self.item_credits || self.bytes > self.byte_credits {
            return Err(BoundedActionFault::ItemCredits);
        }
        commit();
        self.publish_staged();
        Ok(())
    }

    pub fn publish_with_checked(mut self, commit: impl FnOnce() -> bool) -> Result<(), BoundedActionFault> {
        if self.len != self.item_credits || self.bytes > self.byte_credits {
            return Err(BoundedActionFault::ItemCredits);
        }
        if !commit() {
            return Err(BoundedActionFault::Structure);
        }
        self.publish_staged();
        Ok(())
    }

    pub fn publish_partial(mut self) -> Result<(), BoundedActionFault> {
        if self.len > self.item_credits || self.bytes > self.byte_credits {
            return Err(BoundedActionFault::ItemCredits);
        }
        self.publish_staged();
        Ok(())
    }

    pub fn publish_partial_with_checked(mut self, commit: impl FnOnce() -> bool) -> Result<(), BoundedActionFault> {
        if self.len > self.item_credits || self.bytes > self.byte_credits {
            return Err(BoundedActionFault::ItemCredits);
        }
        if !commit() {
            return Err(BoundedActionFault::Structure);
        }
        self.publish_staged();
        Ok(())
    }

    fn publish_staged(&mut self) {
        for index in 0..self.len {
            self.queue.push_reserved(self.actions[index].take().expect("reserved batch action"));
        }
    }
}

impl BoundedActionReservation<'_> {
    pub fn builder(&mut self) -> &mut BoundedActionBuilder {
        &mut self.builder
    }

    pub fn publish(self) -> Result<(), BoundedActionFault> {
        let action = self.builder.finish()?;
        self.queue.push_reserved(action);
        Ok(())
    }

    pub fn publish_with(self, commit: impl FnOnce()) -> Result<(), BoundedActionFault> {
        let action = self.builder.finish()?;
        commit();
        self.queue.push_reserved(action);
        Ok(())
    }
}

impl BoundedClaimedActionReservation<'_> {
    pub fn builder(&mut self) -> &mut BoundedActionBuilder {
        &mut self.builder
    }

    pub fn publish(self) -> Result<(), BoundedActionFault> {
        let action = self.builder.finish()?;
        self.queue.publish_claimed(self.claim, action)
    }

    pub fn publish_with_checked(self, commit: impl FnOnce() -> bool) -> Result<(), BoundedActionFault> {
        let action = self.builder.finish()?;
        self.queue.validate_claim(self.claim)?;
        if action.owned_bytes() > self.claim.byte_credits {
            return Err(BoundedActionFault::ByteCredits);
        }
        if !commit() {
            return Err(BoundedActionFault::Structure);
        }
        self.queue.publish_claimed(self.claim, action)
    }
}

pub struct BoundedActionQueue {
    slots: Box<[Option<BoundedAction>; ACTION_QUEUE_ITEM_CAPACITY]>,
    head: usize,
    len: usize,
    bytes: usize,
    claims: Box<[Option<BoundedActionClaimSlot>; ACTION_CLAIM_CAPACITY]>,
    claimed_items: usize,
    claimed_bytes: usize,
    next_claim_epoch: u64,
}

impl Default for BoundedActionQueue {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| None)), head: 0, len: 0, bytes: 0, claims: Box::new(std::array::from_fn(|_| None)), claimed_items: 0, claimed_bytes: 0, next_claim_epoch: 1 }
    }
}

impl BoundedActionQueue {
    pub fn reserve<'a>(&'a mut self, controller_id: &str, action: &str, byte_credits: usize) -> Result<BoundedActionReservation<'a>, BoundedActionFault> {
        if self.len.checked_add(self.claimed_items).is_none_or(|items| items >= ACTION_QUEUE_ITEM_CAPACITY) {
            return Err(BoundedActionFault::ItemCredits);
        }
        if byte_credits > ACTION_ITEM_BYTE_CAPACITY || self.bytes.checked_add(self.claimed_bytes).and_then(|bytes| bytes.checked_add(byte_credits)).is_none_or(|bytes| bytes > ACTION_QUEUE_BYTE_CAPACITY) {
            return Err(BoundedActionFault::ByteCredits);
        }
        let builder = BoundedActionBuilder::new(controller_id, action, byte_credits)?;
        Ok(BoundedActionReservation { queue: self, builder })
    }

    pub fn reserve_batch(&mut self, item_credits: usize, byte_credits: usize) -> Result<BoundedActionBatchReservation<'_>, BoundedActionFault> {
        if item_credits == 0 || item_credits > ACTION_BATCH_ITEM_CAPACITY || self.len.checked_add(self.claimed_items).and_then(|len| len.checked_add(item_credits)).is_none_or(|len| len > ACTION_QUEUE_ITEM_CAPACITY) {
            return Err(BoundedActionFault::ItemCredits);
        }
        if byte_credits > ACTION_QUEUE_BYTE_CAPACITY || self.bytes.checked_add(self.claimed_bytes).and_then(|bytes| bytes.checked_add(byte_credits)).is_none_or(|bytes| bytes > ACTION_QUEUE_BYTE_CAPACITY) {
            return Err(BoundedActionFault::ByteCredits);
        }
        Ok(BoundedActionBatchReservation { queue: self, actions: std::array::from_fn(|_| None), item_credits, byte_credits, len: 0, declared_bytes: 0, bytes: 0 })
    }

    pub fn claim(&mut self, byte_credits: usize) -> Result<BoundedActionClaim, BoundedActionFault> {
        if byte_credits > ACTION_ITEM_BYTE_CAPACITY || self.len.checked_add(self.claimed_items).is_none_or(|items| items >= ACTION_QUEUE_ITEM_CAPACITY) {
            return Err(if byte_credits > ACTION_ITEM_BYTE_CAPACITY { BoundedActionFault::ByteCredits } else { BoundedActionFault::ItemCredits });
        }
        if self.bytes.checked_add(self.claimed_bytes).and_then(|bytes| bytes.checked_add(byte_credits)).is_none_or(|bytes| bytes > ACTION_QUEUE_BYTE_CAPACITY) {
            return Err(BoundedActionFault::ByteCredits);
        }
        let slot = self.claims.iter().position(Option::is_none).ok_or(BoundedActionFault::ItemCredits)?;
        let epoch = self.next_claim_epoch;
        self.next_claim_epoch = self.next_claim_epoch.wrapping_add(1).max(1);
        self.claims[slot] = Some(BoundedActionClaimSlot { epoch, byte_credits });
        self.claimed_items += 1;
        self.claimed_bytes += byte_credits;
        Ok(BoundedActionClaim { slot: slot as u16, epoch, byte_credits })
    }

    pub fn reserve_claimed<'a>(&'a mut self, claim: BoundedActionClaim, controller_id: &str, action: &str) -> Result<BoundedClaimedActionReservation<'a>, BoundedActionFault> {
        self.validate_claim(claim)?;
        let builder = BoundedActionBuilder::new(controller_id, action, claim.byte_credits)?;
        Ok(BoundedClaimedActionReservation { queue: self, claim, builder })
    }

    pub fn release_claim(&mut self, claim: BoundedActionClaim) -> Result<(), BoundedActionFault> {
        let slot = self.validate_claim(claim)?;
        self.claims[slot] = None;
        self.claimed_items -= 1;
        self.claimed_bytes -= claim.byte_credits;
        Ok(())
    }

    fn validate_claim(&self, claim: BoundedActionClaim) -> Result<usize, BoundedActionFault> {
        let slot = usize::from(claim.slot);
        let Some(owned) = self.claims.get(slot).and_then(Option::as_ref) else {
            return Err(BoundedActionFault::Structure);
        };
        if owned.epoch != claim.epoch || owned.byte_credits != claim.byte_credits {
            return Err(BoundedActionFault::Structure);
        }
        Ok(slot)
    }

    fn publish_claimed(&mut self, claim: BoundedActionClaim, action: BoundedAction) -> Result<(), BoundedActionFault> {
        let slot = self.validate_claim(claim)?;
        if action.owned_bytes() > claim.byte_credits {
            return Err(BoundedActionFault::ByteCredits);
        }
        self.claims[slot] = None;
        self.claimed_items -= 1;
        self.claimed_bytes -= claim.byte_credits;
        self.push_reserved(action);
        Ok(())
    }

    fn push_reserved(&mut self, action: BoundedAction) {
        let bytes = self.bytes.checked_add(action.owned_bytes()).expect("reserved action byte credits");
        debug_assert!(self.len < ACTION_QUEUE_ITEM_CAPACITY && bytes <= ACTION_QUEUE_BYTE_CAPACITY);
        let index = (self.head + self.len) % ACTION_QUEUE_ITEM_CAPACITY;
        self.slots[index] = Some(action);
        self.len += 1;
        self.bytes = bytes;
    }

    pub fn pop_front(&mut self) -> Option<BoundedAction> {
        if self.len == 0 {
            return None;
        }
        let action = self.slots[self.head].take();
        self.head = (self.head + 1) % ACTION_QUEUE_ITEM_CAPACITY;
        self.len -= 1;
        if let Some(action) = action.as_ref() {
            self.bytes -= action.owned_bytes();
        }
        action
    }

    pub fn pop_back(&mut self) -> Option<BoundedAction> {
        if self.len == 0 {
            return None;
        }
        let index = (self.head + self.len - 1) % ACTION_QUEUE_ITEM_CAPACITY;
        let action = self.slots[index].take();
        self.len -= 1;
        if let Some(action) = action.as_ref() {
            self.bytes -= action.owned_bytes();
        }
        action
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0 && self.claimed_items == 0 && self.claimed_bytes == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn bytes(&self) -> usize {
        self.bytes
    }

    pub fn claimed_items(&self) -> usize {
        self.claimed_items
    }

    pub fn claimed_bytes(&self) -> usize {
        self.claimed_bytes
    }

    pub fn close_claim_step(&mut self) -> bool {
        let Some(slot) = self.claims.iter().position(Option::is_some) else {
            return true;
        };
        let claim = self.claims[slot].take().expect("claim slot found above");
        self.claimed_items -= 1;
        self.claimed_bytes -= claim.byte_credits;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn publish(queue: &mut BoundedActionQueue, index: usize) {
        let mut reservation = queue.reserve("controller", "dispatch", 128).expect("reservation");
        let builder = reservation.builder();
        builder.begin_object(None).expect("root");
        builder.number(Some("index"), index as f64).expect("number");
        builder.string(Some("value"), "owned").expect("string");
        builder.end_container().expect("root end");
        reservation.publish().expect("publication");
    }

    #[test]
    fn reservation_rejects_max_plus_one_before_allocating_action_storage() {
        let mut queue = BoundedActionQueue::default();
        assert!(matches!(queue.reserve("controller", "dispatch", ACTION_ITEM_BYTE_CAPACITY + 1), Err(BoundedActionFault::ByteCredits)));
        assert!(matches!(queue.reserve(&"x".repeat(ACTION_STRING_BYTE_CAPACITY + 1), "dispatch", ACTION_ITEM_BYTE_CAPACITY), Err(BoundedActionFault::StringCredits)));
        assert!(queue.is_empty());
    }

    #[test]
    fn hostile_depth_poison_has_only_flat_inline_storage_to_release() {
        let mut queue = BoundedActionQueue::default();
        let mut reservation = queue.reserve("controller", "dispatch", ACTION_ITEM_BYTE_CAPACITY).expect("reservation");
        for _ in 0..ACTION_DEPTH_CAPACITY {
            reservation.builder().begin_array(None).expect("admitted depth");
        }
        assert_eq!(reservation.builder().begin_array(None), Err(BoundedActionFault::DepthCredits));
        assert_eq!(reservation.publish(), Err(BoundedActionFault::DepthCredits));
        assert!(queue.is_empty());
    }

    #[test]
    fn full_queue_fails_before_builder_and_retry_preserves_fifo() {
        let mut queue = BoundedActionQueue::default();
        for index in 0..ACTION_QUEUE_ITEM_CAPACITY {
            publish(&mut queue, index);
        }
        assert!(matches!(queue.reserve("controller", "retry", 128), Err(BoundedActionFault::ItemCredits)));
        let first = queue.pop_front().expect("first").into_descriptor().expect("descriptor");
        assert_eq!(first.args.as_ref().and_then(|args| args.get("index")).and_then(DslValue::as_f64), Some(0.0));
        publish(&mut queue, ACTION_QUEUE_ITEM_CAPACITY);
        let mut last = None;
        while let Some(action) = queue.pop_front() {
            last = Some(action.into_descriptor().expect("descriptor"));
        }
        assert_eq!(last.and_then(|action| action.args).as_ref().and_then(|args| args.get("index")).and_then(DslValue::as_f64), Some(ACTION_QUEUE_ITEM_CAPACITY as f64));
        assert_eq!(queue.bytes(), 0);
    }

    #[test]
    fn detached_claim_reserves_exact_aggregate_credits_and_rejects_stale_epoch() {
        let mut queue = BoundedActionQueue::default();
        assert_eq!(queue.claim(ACTION_ITEM_BYTE_CAPACITY + 1), Err(BoundedActionFault::ByteCredits));
        let claim = queue.claim(ACTION_ITEM_BYTE_CAPACITY).expect("exact detached claim");
        assert_eq!(queue.claimed_items(), 1);
        assert_eq!(queue.claimed_bytes(), ACTION_ITEM_BYTE_CAPACITY);
        assert!(queue.reserve("controller", "dispatch", ACTION_QUEUE_BYTE_CAPACITY).is_err());
        queue.release_claim(claim).expect("claim release");
        let replacement = queue.claim(128).expect("reused slot with new epoch");
        assert_eq!(queue.release_claim(claim), Err(BoundedActionFault::Structure));

        let mut reservation = queue.reserve_claimed(replacement, "controller", "dispatch").expect("claimed builder");
        reservation.builder().begin_object(None).unwrap();
        reservation.builder().string(Some("value"), "owned").unwrap();
        reservation.builder().end_container().unwrap();
        reservation.publish().expect("claimed publication");
        assert_eq!(queue.claimed_items(), 0);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.pop_front().unwrap().into_descriptor().unwrap().action, "dispatch");
        assert!(queue.is_empty());
    }

    #[test]
    fn detached_claim_close_releases_one_fixed_owner_per_grant() {
        let mut queue = BoundedActionQueue::default();
        let _first = queue.claim(128).unwrap();
        let _second = queue.claim(256).unwrap();
        assert!(!queue.close_claim_step());
        assert_eq!(queue.claimed_items(), 1);
        assert!(!queue.close_claim_step());
        assert!(queue.close_claim_step());
        assert!(queue.is_empty());
    }

    #[test]
    fn batch_reservation_is_atomic_and_preserves_order() {
        let mut queue = BoundedActionQueue::default();
        let mut batch = queue.reserve_batch(2, 256).expect("batch");
        batch.action("controller", "first", 128, |_| Ok(())).expect("first");
        batch.action("controller", "second", 128, |_| Ok(())).expect("second");
        batch.publish().expect("publish");
        assert_eq!(queue.pop_front().expect("first").into_descriptor().expect("first descriptor").action, "first");
        assert_eq!(queue.pop_front().expect("second").into_descriptor().expect("second descriptor").action, "second");
    }

    #[test]
    fn incomplete_or_over_credit_batch_publishes_nothing() {
        let mut queue = BoundedActionQueue::default();
        let mut batch = queue.reserve_batch(2, 128).expect("batch");
        batch.action("controller", "first", 64, |_| Ok(())).expect("first");
        assert_eq!(batch.publish(), Err(BoundedActionFault::ItemCredits));
        assert!(queue.is_empty());
        assert!(matches!(queue.reserve_batch(ACTION_BATCH_ITEM_CAPACITY + 1, 1), Err(BoundedActionFault::ItemCredits)));
    }

    #[test]
    fn semantic_commit_runs_only_after_the_flat_owner_is_complete() {
        let mut queue = BoundedActionQueue::default();
        let mut committed = false;
        let mut reservation = queue.reserve("controller", "dispatch", 32).unwrap();
        reservation.builder().begin_object(None).unwrap();
        assert_eq!(reservation.publish_with(|| committed = true), Err(BoundedActionFault::Structure));
        assert!(!committed);
        assert!(queue.is_empty());

        let mut reservation = queue.reserve("controller", "dispatch", 32).unwrap();
        reservation.builder().begin_object(None).unwrap();
        reservation.builder().end_container().unwrap();
        reservation.publish_with(|| committed = true).unwrap();
        assert!(committed);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn rejected_revision_commit_publishes_no_flat_owner() {
        let mut queue = BoundedActionQueue::default();
        let mut batch = queue.reserve_batch(1, 32).unwrap();
        batch.action("controller", "dispatch", 32, |_| Ok(())).unwrap();
        assert_eq!(batch.publish_with_checked(|| false), Err(BoundedActionFault::Structure));
        assert!(queue.is_empty());

        let mut batch = queue.reserve_batch(2, 64).unwrap();
        batch.action("controller", "dispatch", 32, |_| Ok(())).unwrap();
        assert_eq!(batch.publish_partial_with_checked(|| false), Err(BoundedActionFault::Structure));
        assert!(queue.is_empty());
    }

    #[test]
    fn close_releases_one_flat_inline_owner_per_grant() {
        let mut queue = BoundedActionQueue::default();
        publish(&mut queue, 1);
        publish(&mut queue, 2);
        drop(queue.pop_back().expect("one owner"));
        assert_eq!(queue.len(), 1);
        drop(queue.pop_back().expect("one owner"));
        assert!(queue.is_empty());
    }

    #[test]
    fn production_boundary_has_no_forget_or_background_drop_escape() {
        const SOURCE: &str = include_str!("🦀️action.rs");
        assert!(!SOURCE.contains(concat!("mem::", "forget")));
        assert!(!SOURCE.contains(concat!("thread::", "spawn")));
        assert!(!SOURCE.contains(concat!("impl Clone", " for BoundedAction")));
        assert!(!SOURCE.contains(concat!("#[derive(Clone, Debug)]", "\npub struct BoundedAction")));
    }
}

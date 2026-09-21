//! 🧾 Fixed-credit action construction and FIFO ownership for interactive renderer input.
//!
//! 🎬️ Two channels leave this target, exactly as React's own Interpreter splits them
//! (`🗣️Interpreter/🟦️.tsx`'s `UiInterpreterContext`): every SEMANTIC control
//! (Button/Input/Select/Toggle/Slider/NumberStepper/Ring/IconSelect/Tree row) fires a
//! [`UiIntentCommand`] — the renderer-side twin of [`ui_contract::UiIntent`] — while the bare
//! [`ActionDescriptor`] stays reserved for the unowned scene-host elements and the chrome's own
//! immediate-mode widgets, which address no published node. An intent additionally carries the
//! addressing (`surface`/`revision`/`node`/`node_key`) that makes [`intent_is_stale`] and
//! [`BoundedActionQueue::admit_intent`] possible; a descriptor never could.

use crate::wgpu::ActionDescriptor;
use dsl::{DslValue, Number};

pub const ACTION_QUEUE_ITEM_CAPACITY: usize = 256;
pub const ACTION_BATCH_ITEM_CAPACITY: usize = 16;
pub const ACTION_NODE_CAPACITY: usize = 256;
pub const ACTION_DEPTH_CAPACITY: usize = 32;
pub const ACTION_STRING_BYTE_CAPACITY: usize = 4 * 1024;
pub const ACTION_ITEM_BYTE_CAPACITY: usize = 16 * 1024;
pub const ACTION_QUEUE_BYTE_CAPACITY: usize = 1024 * 1024;
pub const ACTION_CLAIM_CAPACITY: usize = 256;
pub const ACTION_CLAIM_BATCH_CAPACITY: usize = 16;

//#region 🎬️Intent
/// 🏷️ The argument field a trigger's scalar payload travels under — React's `uiInputField`
/// (`🛠️ShellHelpers/🟦️.tsx`). The name belongs to the TRIGGER, not to the control that fired it.
pub const INTENT_VALUE_FIELD: &str = "value";
pub const INTENT_DELTA_FIELD: &str = "delta";

/// 🎯️ Where a gesture happened, as the published document addressed it: the addressing half of
/// [`ui_contract::UiIntent`], stamped onto a retained node by the document reconcile and read back
/// when that node fires. `surface`/`revision` are what [`intent_is_stale`] compares; `node`/`node_key`
/// are the identity a replay or a log still resolves after id churn from an intervening
/// reconciliation. A node with no address (the chrome's immediate-mode widgets, a synthesized
/// recovery panel) is not addressable and never gets an intent.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UiIntentAddress {
    pub surface: String,
    pub revision: u64,
    pub node: u64,
    pub node_key: String,
}

/// 🔗️ One retained node's dispatch contract: where it lives plus the versioned [`ui_contract::ActionId`]
/// it binds per [`ui_contract::Trigger`]. The bindings list is the wgpu twin of
/// `UiNodeRecord::bindings` — carried whole rather than flattened into one action name per field, so
/// binding PRESENCE is answerable (React's `NumberStepperView` gates `onDelta` on exactly that) and so
/// the action's `scope`/`version` survive, which a stringly [`ActionDescriptor`] drops.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UiIntentBindings {
    pub address: UiIntentAddress,
    pub bindings: Vec<(ui_contract::Trigger, ui_contract::ActionId)>,
}

impl UiIntentBindings {
    pub fn action_for(&self, trigger: ui_contract::Trigger) -> Option<&ui_contract::ActionId> {
        self.bindings.iter().find(|(bound, _)| *bound == trigger).map(|(_, action)| action)
    }

    pub fn binds(&self, trigger: ui_contract::Trigger) -> bool {
        self.bindings.iter().any(|(bound, _)| *bound == trigger)
    }
}

/// 🎬️ One user gesture against one addressed node — the renderer-side twin of
/// [`ui_contract::UiIntent`], carrying the `controller_id` the host's own dispatch seam still needs
/// (the contract's intent resolves its controller from the surface instead; wgpu's host cannot yet).
/// `args` are the binding's AUTHORED arguments and `input` is the trigger's own payload, kept apart
/// exactly as the contract keeps them — [`Self::descriptor`] is the one place they merge.
#[derive(Clone, Debug, PartialEq)]
pub struct UiIntentCommand {
    pub address: UiIntentAddress,
    pub trigger: ui_contract::Trigger,
    pub action: ui_contract::ActionId,
    pub args: Option<DslValue>,
    pub input: Option<DslValue>,
    pub seq: u64,
}

impl UiIntentCommand {
    /// 🌉️ This intent on the plugin action address — React's `uiIntentToActionDescriptor`. A scalar
    /// payload is NAMED by its trigger and merged OVER the authored args (replacing them wholesale
    /// threw the authored `{windowId}` away and muted whole panels); a map payload merges key-wise.
    pub fn descriptor(&self) -> ActionDescriptor {
        ActionDescriptor { controller_id: self.controller_id().to_string(), action: self.action_name(), args: self.payload() }
    }

    /// 🆔️ Who answers this intent — React's `intent.action.scope` (`🛠️ShellHelpers/🟦️.tsx:2054`).
    pub fn controller_id(&self) -> &str {
        self.action.scope.as_str()
    }

    /// 🆔️ The verb as the host addresses it — the bare name at version one, `name@version` beyond it,
    /// so a renderer can never silently invoke a different version of the same verb.
    pub fn action_name(&self) -> String {
        if self.action.version == 1 {
            self.action.name.as_str().to_string()
        } else {
            format!("{}@{}", self.action.name.as_str(), self.action.version)
        }
    }

    pub fn payload(&self) -> Option<DslValue> {
        let Some(input) = self.input.as_ref() else { return self.args.clone() };
        let named = match input {
            DslValue::Object(_) | DslValue::Array(_) => input.clone(),
            scalar => DslValue::Object(vec![(self.input_field().to_string(), scalar.clone())]),
        };
        match (self.args.as_ref(), &named) {
            (Some(DslValue::Object(authored)), DslValue::Object(merged)) => {
                let mut entries: Vec<(String, DslValue)> = authored.iter().filter(|(key, _)| !merged.iter().any(|(name, _)| name == key)).cloned().collect();
                entries.extend(merged.iter().cloned());
                Some(DslValue::Object(entries))
            }
            _ => Some(named),
        }
    }

    pub fn input_field(&self) -> &'static str {
        if matches!(self.trigger, ui_contract::Trigger::Delta) {
            INTENT_DELTA_FIELD
        } else {
            INTENT_VALUE_FIELD
        }
    }
}

/// 🔢️ master.md's own rule ("Stale intents (revision < current − 1) are dropped"), the same
/// predicate `🖌️render/🖱️dispatch/🦀️.rs::is_stale` applies: a gesture recorded more than one revision
/// behind the surface it is now being resolved against was fired at geometry the user never saw.
pub fn intent_is_stale(recorded: u64, current: u64) -> bool {
    current > recorded.saturating_add(1)
}

/// 🔢️ Renderer-monotonic per surface — the `seq` a fired intent carries, minted once per gesture so
/// the receiving side can order and de-duplicate independently of transport delivery order.
#[derive(Clone, Debug, Default)]
pub struct UiIntentSequencer {
    surfaces: Vec<(String, u64)>,
}

impl UiIntentSequencer {
    pub fn next(&mut self, surface: &str) -> u64 {
        match self.surfaces.iter_mut().find(|(name, _)| name == surface) {
            Some((_, seq)) => {
                *seq += 1;
                *seq
            }
            None => {
                self.surfaces.push((surface.to_string(), 1));
                1
            }
        }
    }

    pub fn last(&self, surface: &str) -> u64 {
        self.surfaces.iter().find(|(name, _)| name == surface).map_or(0, |(_, seq)| *seq)
    }

    /// 🧹️ Retires one surface-name scalar without minting an intent.
    pub(crate) fn close_step(&mut self) -> bool {
        if let Some((name, _)) = self.surfaces.last_mut() {
            if name.pop().is_some() {
                return false;
            }
            if name.capacity() > 0 {
                *name = String::new();
                return false;
            }
            self.surfaces.pop();
            return false;
        }
        if self.surfaces.capacity() > 0 {
            self.surfaces = Vec::new();
            return false;
        }
        true
    }
}
//#endregion 🎬️Intent

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

/// 🔢️ Carries `dsl::Number` variant-for-variant rather than a widened `f64`, so an integer queued
/// through the ring rehydrates as the same integer `copy_value` flattened — `{"value": 7}` came back
/// as `7.0` while this held `f64`.
#[derive(Clone, Copy, Debug)]
enum FlatValue {
    Null,
    Bool(bool),
    Number(Number),
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
    batch_remaining: u8,
}

impl BoundedAction {
    pub fn owned_bytes(&self) -> usize {
        self.byte_len
    }

    fn batch_remaining(&self) -> usize {
        usize::from(self.batch_remaining)
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
                batch_remaining: 1,
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

pub struct BoundedClaimedActionDraft {
    claim: BoundedActionClaim,
    builder: BoundedActionBuilder,
}

pub struct PreparedClaimedAction {
    claim: BoundedActionClaim,
    action: BoundedAction,
}

pub struct BoundedActionClaimBatch {
    claims: [Option<BoundedActionClaim>; ACTION_CLAIM_BATCH_CAPACITY],
    len: u8,
}

pub struct PreparedClaimedActionBatch {
    claims: BoundedActionClaimBatch,
    actions: [Option<PreparedClaimedAction>; ACTION_CLAIM_BATCH_CAPACITY],
    len: u8,
}

impl BoundedClaimedActionDraft {
    pub fn builder(&mut self) -> &mut BoundedActionBuilder {
        &mut self.builder
    }

    pub fn finish(self) -> Result<PreparedClaimedAction, BoundedActionFault> {
        Ok(PreparedClaimedAction { claim: self.claim, action: self.builder.finish()? })
    }

    pub fn claim(&self) -> BoundedActionClaim {
        self.claim
    }
}

impl BoundedActionClaimBatch {
    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn claim(&self, index: usize) -> Option<BoundedActionClaim> {
        self.claims.get(index).copied().flatten()
    }

    pub fn take_last(&mut self) -> Option<BoundedActionClaim> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.claims[usize::from(self.len)].take()
    }
}

impl PreparedClaimedActionBatch {
    pub fn new(claims: BoundedActionClaimBatch) -> Self {
        Self { claims, actions: std::array::from_fn(|_| None), len: 0 }
    }

    pub fn claim(&self, index: usize) -> Option<BoundedActionClaim> {
        self.claims.claim(index)
    }

    pub fn push(&mut self, prepared: PreparedClaimedAction) -> Result<(), BoundedActionFault> {
        let index = usize::from(self.len);
        if index == self.claims.len() || self.claims.claim(index) != Some(prepared.claim) {
            return Err(BoundedActionFault::Structure);
        }
        self.actions[index] = Some(prepared);
        self.len += 1;
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        usize::from(self.len) == self.claims.len()
    }

    pub fn take_last_claim(&mut self) -> Option<BoundedActionClaim> {
        let claim = self.claims.take_last()?;
        let index = self.claims.len();
        self.actions[index] = None;
        self.len = self.len.min(self.claims.len() as u8);
        Some(claim)
    }

    pub fn restore_last_claim(&mut self, claim: BoundedActionClaim) -> Result<(), BoundedActionFault> {
        let index = self.claims.len();
        if index == ACTION_CLAIM_BATCH_CAPACITY || self.claims.claims[index].is_some() {
            return Err(BoundedActionFault::Structure);
        }
        self.claims.claims[index] = Some(claim);
        self.claims.len += 1;
        Ok(())
    }
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
            let mut action = self.actions[index].take().expect("reserved batch action");
            action.batch_remaining = (self.len - index) as u8;
            self.queue.push_reserved(action);
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
    admitted_seq: Vec<(String, u64)>,
}

impl Default for BoundedActionQueue {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| None)), head: 0, len: 0, bytes: 0, claims: Box::new(std::array::from_fn(|_| None)), claimed_items: 0, claimed_bytes: 0, next_claim_epoch: 1, admitted_seq: Vec::new() }
    }
}

/// 🚦️ Why an intent did not reach the queue — the two refusals React's runtime already applies to a
/// `UiIntent` before it becomes a command (`🧠️runtime/🎯️dispatch/🦀️.rs`'s `DispatchOutcome::Stale`
/// and the store's own per-surface `seq` ordering), named apart so a log or a metric can tell them
/// from a credit refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiIntentAdmission {
    Accepted,
    /// 🕰️ The gesture's revision trails the surface's current one by more than one.
    Stale,
    /// 🔁️ This surface already admitted this `seq` or a later one — a duplicate or a reorder.
    Duplicate,
}

impl BoundedActionQueue {
    /// 🚦️ Orders and de-duplicates one surface's intents by `seq`, and drops a stale one outright —
    /// the last gate before a gesture becomes a queued action. `current_revision` is the revision of
    /// the document the intent is being resolved against, not the one it was fired at.
    pub fn admit_intent(&mut self, intent: &UiIntentCommand, current_revision: u64) -> UiIntentAdmission {
        if intent_is_stale(intent.address.revision, current_revision) {
            return UiIntentAdmission::Stale;
        }
        self.admit_intent_seq(intent)
    }

    /// 🔁️ The ordering half of [`Self::admit_intent`] alone, for a host that reaches the queue
    /// without the live document in hand — the renderer's own `events::EventRouter::build_intent`
    /// already refused the stale ones against the tree it dispatched them on, which is the same place
    /// React's `Dispatcher` refuses them.
    pub fn admit_intent_seq(&mut self, intent: &UiIntentCommand) -> UiIntentAdmission {
        match self.admitted_seq.iter_mut().find(|(surface, _)| *surface == intent.address.surface) {
            Some((_, admitted)) if intent.seq <= *admitted => UiIntentAdmission::Duplicate,
            Some((_, admitted)) => {
                *admitted = intent.seq;
                UiIntentAdmission::Accepted
            }
            None => {
                self.admitted_seq.push((intent.address.surface.clone(), intent.seq));
                UiIntentAdmission::Accepted
            }
        }
    }

    pub fn admitted_seq(&self, surface: &str) -> u64 {
        self.admitted_seq.iter().find(|(name, _)| name == surface).map_or(0, |(_, seq)| *seq)
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

    pub fn claim_batch(&mut self, byte_credits: &[usize]) -> Result<BoundedActionClaimBatch, BoundedActionFault> {
        if byte_credits.is_empty() || byte_credits.len() > ACTION_CLAIM_BATCH_CAPACITY {
            return Err(BoundedActionFault::ItemCredits);
        }
        let total_bytes = byte_credits.iter().try_fold(0usize, |sum, bytes| {
            if *bytes > ACTION_ITEM_BYTE_CAPACITY {
                return Err(BoundedActionFault::ByteCredits);
            }
            sum.checked_add(*bytes).ok_or(BoundedActionFault::ByteCredits)
        })?;
        if self.len.checked_add(self.claimed_items).and_then(|items| items.checked_add(byte_credits.len())).is_none_or(|items| items > ACTION_QUEUE_ITEM_CAPACITY) {
            return Err(BoundedActionFault::ItemCredits);
        }
        if self.bytes.checked_add(self.claimed_bytes).and_then(|bytes| bytes.checked_add(total_bytes)).is_none_or(|bytes| bytes > ACTION_QUEUE_BYTE_CAPACITY) {
            return Err(BoundedActionFault::ByteCredits);
        }
        let mut free = [0usize; ACTION_CLAIM_BATCH_CAPACITY];
        let mut free_len = 0usize;
        for (index, claim) in self.claims.iter().enumerate() {
            if claim.is_none() {
                free[free_len] = index;
                free_len += 1;
                if free_len == byte_credits.len() {
                    break;
                }
            }
        }
        if free_len != byte_credits.len() {
            return Err(BoundedActionFault::ItemCredits);
        }
        let mut claims = [None; ACTION_CLAIM_BATCH_CAPACITY];
        for (index, bytes) in byte_credits.iter().copied().enumerate() {
            let slot = free[index];
            let epoch = self.next_claim_epoch;
            self.next_claim_epoch = self.next_claim_epoch.wrapping_add(1).max(1);
            self.claims[slot] = Some(BoundedActionClaimSlot { epoch, byte_credits: bytes });
            claims[index] = Some(BoundedActionClaim { slot: slot as u16, epoch, byte_credits: bytes });
        }
        self.claimed_items += byte_credits.len();
        self.claimed_bytes += total_bytes;
        Ok(BoundedActionClaimBatch { claims, len: byte_credits.len() as u8 })
    }

    pub fn reserve_claimed<'a>(&'a mut self, claim: BoundedActionClaim, controller_id: &str, action: &str) -> Result<BoundedClaimedActionReservation<'a>, BoundedActionFault> {
        self.validate_claim(claim)?;
        let builder = BoundedActionBuilder::new(controller_id, action, claim.byte_credits)?;
        Ok(BoundedClaimedActionReservation { queue: self, claim, builder })
    }

    pub fn draft_claimed(&self, claim: BoundedActionClaim, controller_id: &str, action: &str) -> Result<BoundedClaimedActionDraft, BoundedActionFault> {
        self.validate_claim(claim)?;
        Ok(BoundedClaimedActionDraft { claim, builder: BoundedActionBuilder::new(controller_id, action, claim.byte_credits)? })
    }

    pub fn publish_prepared_claimed(&mut self, prepared: PreparedClaimedAction) -> Result<(), BoundedActionFault> {
        self.publish_claimed(prepared.claim, prepared.action)
    }

    pub fn publish_prepared_claimed_batch(&mut self, mut prepared: PreparedClaimedActionBatch) -> Result<(), BoundedActionFault> {
        if !prepared.is_complete() {
            return Err(BoundedActionFault::ItemCredits);
        }
        for index in 0..prepared.claims.len() {
            let action = prepared.actions[index].as_ref().ok_or(BoundedActionFault::Structure)?;
            let claim = prepared.claims.claim(index).ok_or(BoundedActionFault::Structure)?;
            self.validate_claim(claim)?;
            if action.claim != claim || action.action.owned_bytes() > claim.byte_credits {
                return Err(BoundedActionFault::Structure);
            }
        }
        let len = prepared.claims.len();
        for index in 0..len {
            let mut action = prepared.actions[index].take().expect("validated claimed batch action");
            let slot = self.validate_claim(action.claim)?;
            self.claims[slot] = None;
            self.claimed_items -= 1;
            self.claimed_bytes -= action.claim.byte_credits;
            action.action.batch_remaining = (len - index) as u8;
            self.push_reserved(action.action);
        }
        prepared.claims.len = 0;
        prepared.len = 0;
        Ok(())
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

    pub fn front_batch_len(&self) -> Result<Option<usize>, BoundedActionFault> {
        if self.len == 0 {
            return Ok(None);
        }
        let remaining = self.slots[self.head].as_ref().ok_or(BoundedActionFault::Structure)?.batch_remaining();
        if remaining == 0 || remaining > ACTION_BATCH_ITEM_CAPACITY || remaining > self.len {
            return Err(BoundedActionFault::Structure);
        }
        for offset in 0..remaining {
            let index = (self.head + offset) % ACTION_QUEUE_ITEM_CAPACITY;
            if self.slots[index].as_ref().map(BoundedAction::batch_remaining) != Some(remaining - offset) {
                return Err(BoundedActionFault::Structure);
            }
        }
        Ok(Some(remaining))
    }

    fn shorten_batch_before(&mut self, index: usize) {
        if index >= self.len {
            return;
        }
        let slot = (self.head + index) % ACTION_QUEUE_ITEM_CAPACITY;
        let Some(mut expected) = self.slots[slot].as_ref().map(|action| action.batch_remaining.saturating_add(1)) else { return };
        let mut offset = index;
        while offset > 0 && usize::from(expected) <= ACTION_BATCH_ITEM_CAPACITY {
            offset -= 1;
            let previous = (self.head + offset) % ACTION_QUEUE_ITEM_CAPACITY;
            let Some(action) = self.slots[previous].as_mut() else { break };
            if action.batch_remaining != expected {
                break;
            }
            action.batch_remaining -= 1;
            expected = expected.saturating_add(1);
        }
    }

    pub fn pop_back(&mut self) -> Option<BoundedAction> {
        if self.len == 0 {
            return None;
        }
        self.shorten_batch_before(self.len - 1);
        let index = (self.head + self.len - 1) % ACTION_QUEUE_ITEM_CAPACITY;
        let action = self.slots[index].take();
        self.len -= 1;
        if let Some(action) = action.as_ref() {
            self.bytes -= action.owned_bytes();
        }
        action
    }

    /// 🧾️ Removes one owned action by logical FIFO index while preserving every remaining entry's
    /// order and byte accounting. Token-addressed host journals use this when a later publication
    /// becomes releasable before an earlier refusing surface.
    pub fn remove_at(&mut self, index: usize) -> Option<BoundedAction> {
        if index >= self.len {
            return None;
        }
        self.shorten_batch_before(index);
        let slot = (self.head + index) % ACTION_QUEUE_ITEM_CAPACITY;
        let action = self.slots[slot].take();
        for offset in index..self.len - 1 {
            let current = (self.head + offset) % ACTION_QUEUE_ITEM_CAPACITY;
            let next = (self.head + offset + 1) % ACTION_QUEUE_ITEM_CAPACITY;
            self.slots[current] = self.slots[next].take();
        }
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
#[path = "../../../🧪️tests/🔬️targets-wgpu-action-unit/🦀️.rs"]
mod tests;

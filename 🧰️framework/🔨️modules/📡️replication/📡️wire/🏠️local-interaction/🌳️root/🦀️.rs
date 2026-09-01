//! 🌳️ Exact persistent local interaction roots and byte-accounted final ownership.

use super::{DomainSelection, LocalInteractionState, SelectionMode};
use crate::value::ordered::{Grant, OrderedMap, Retirement, RetirementStep};
use std::mem::ManuallyDrop;

#[path = "🩹️update/🦀️.rs"]
mod update;
pub use update::{LocalInteractionRootPatch, LocalInteractionRootUpdate, LocalInteractionUpdateStep};

//#region 🌳️ImmutableRoot
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct LocalInteractionRoot {
    selection: OrderedMap<DomainSelection>,
    active_mode: OrderedMap<SelectionMode>,
    active_granularity: OrderedMap<String>,
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin in
/// `🎮️mutation/🦀️.rs` documents.
impl crate::value::ToValue for LocalInteractionRoot {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("selection".to_string(), crate::value::ToValue::to_value(&self.selection)),
            ("activeMode".to_string(), crate::value::ToValue::to_value(&self.active_mode)),
            ("activeGranularity".to_string(), crate::value::ToValue::to_value(&self.active_granularity)),
        ])
    }
}

impl LocalInteractionRoot {
    /// 🧊️ Explicit cold conversion from the transport schema; decoding/building earns no retained-work credit.
    pub fn from_cold(state: LocalInteractionState) -> Self {
        Self { selection: state.selection.into_iter().collect(), active_mode: state.active_mode.into_iter().collect(), active_granularity: state.active_granularity.into_iter().collect() }
    }

    pub fn selection(&self) -> &OrderedMap<DomainSelection> { &self.selection }
    pub fn active_mode(&self) -> &OrderedMap<SelectionMode> { &self.active_mode }
    pub fn active_granularity(&self) -> &OrderedMap<String> { &self.active_granularity }

    /// 📥️ Transfers three immutable roots without traversing or cloning their keys or payloads.
    pub fn retire(self) -> LocalInteractionRootRetirement {
        LocalInteractionRootRetirement { owned: ManuallyDrop::new(RetirementState { root: Some(self), ..Default::default() }) }
    }
}
//#endregion 🌳️ImmutableRoot

//#region ♻️FinalOwnership
enum MapRetirement {
    Selection(Retirement<DomainSelection>),
    Mode(Retirement<SelectionMode>),
    Granularity(Retirement<String>),
}

#[derive(Default)]
struct RetirementState {
    root: Option<LocalInteractionRoot>,
    map: Option<MapRetirement>,
    selection: Option<DomainSelection>,
    text: Option<String>,
    bytes: Vec<u8>,
    phase: u8,
}

impl RetirementState {
    fn is_empty(&self) -> bool { self.root.is_none() && self.map.is_none() && self.selection.is_none() && self.text.is_none() && self.bytes.capacity() == 0 }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalInteractionRootStep {
    Blocked,
    Progress { released_items: usize, released_bytes: usize },
    Complete,
}

#[must_use = "local interaction roots require exact retained retirement"]
pub struct LocalInteractionRootRetirement { owned: ManuallyDrop<RetirementState> }

impl LocalInteractionRootRetirement {
    pub fn terminal_is_empty(&self) -> bool { self.owned.is_empty() }

    /// 🪶️ Releases one exact owner or a bounded String-byte suffix; shared payloads remain untouched.
    pub fn advance(&mut self, grant: Grant) -> LocalInteractionRootStep {
        if self.terminal_is_empty() { return LocalInteractionRootStep::Complete; }
        if grant.maximum_items == 0 || grant.maximum_bytes == 0 { return LocalInteractionRootStep::Blocked; }
        let state = &mut *self.owned;
        let progress = |released_items, released_bytes| LocalInteractionRootStep::Progress { released_items, released_bytes };
        if !state.bytes.is_empty() {
            let bytes = state.bytes.len().min(grant.maximum_bytes);
            state.bytes.truncate(state.bytes.len() - bytes);
            return progress(0, bytes);
        }
        if state.bytes.capacity() != 0 { state.bytes = Vec::new(); return progress(1, 0); }
        if let Some(text) = state.text.take() { state.bytes = text.into_bytes(); return progress(1, 0); }
        if let Some(selection) = state.selection.as_mut() {
            if let Some(id) = selection.ids.pop() { state.text = Some(id); }
            else if selection.ids.capacity() != 0 { selection.ids = Vec::new(); }
            else if selection.granularity.capacity() != 0 { state.text = Some(std::mem::take(&mut selection.granularity)); }
            else if let Some(anchor) = selection.anchor_id.take() { state.text = Some(anchor); }
            else { state.selection = None; }
            return progress(1, 0);
        }
        if let Some(map) = state.map.as_mut() {
            let step = match map {
                MapRetirement::Selection(map) => match map.advance(grant) {
                    RetirementStep::Blocked => return LocalInteractionRootStep::Blocked,
                    RetirementStep::Progress { released_items, released_bytes } => return progress(released_items, released_bytes),
                    RetirementStep::OwnedValue(value) => { state.selection = Some(value); false }
                    RetirementStep::Complete => true,
                },
                MapRetirement::Mode(map) => match map.advance(grant) {
                    RetirementStep::Blocked => return LocalInteractionRootStep::Blocked,
                    RetirementStep::Progress { released_items, released_bytes } => return progress(released_items, released_bytes),
                    RetirementStep::OwnedValue(_) => false,
                    RetirementStep::Complete => true,
                },
                MapRetirement::Granularity(map) => match map.advance(grant) {
                    RetirementStep::Blocked => return LocalInteractionRootStep::Blocked,
                    RetirementStep::Progress { released_items, released_bytes } => return progress(released_items, released_bytes),
                    RetirementStep::OwnedValue(value) => { state.text = Some(value); false }
                    RetirementStep::Complete => true,
                },
            };
            if step { state.map = None; state.phase += 1; }
            return progress(1, 0);
        }
        if let Some(root) = state.root.as_mut() {
            state.map = match state.phase {
                0 => Some(MapRetirement::Selection(std::mem::take(&mut root.selection).retire())),
                1 => Some(MapRetirement::Mode(std::mem::take(&mut root.active_mode).retire())),
                2 => Some(MapRetirement::Granularity(std::mem::take(&mut root.active_granularity).retire())),
                _ => { state.root = None; None }
            };
            return progress(1, 0);
        }
        LocalInteractionRootStep::Complete
    }
}

impl Drop for LocalInteractionRootRetirement {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() { panic!("local interaction root retirement dropped before exact terminal ownership"); }
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.owned); }
    }
}
//#endregion ♻️FinalOwnership

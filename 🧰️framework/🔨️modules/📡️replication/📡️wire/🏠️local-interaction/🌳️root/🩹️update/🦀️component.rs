//! 🩹️ Atomic three-map candidates with retained key comparisons and exact input retirement.

use super::{DomainSelection, LocalInteractionRoot, LocalInteractionRootRetirement, LocalInteractionRootStep, MapRetirement, RetirementState, SelectionMode};
use super::super::LocalInteractionDomainPatch;
use crate::value::ordered::{Grant, RetirementStep, Step, UpdateCursor};
use std::mem::ManuallyDrop;
use std::sync::Arc;

//#region 📥️SharedInput
#[derive(Default)]
struct PatchState {
    selection: Option<Arc<DomainSelection>>,
    mode: Option<Arc<SelectionMode>>,
    granularity: Option<Arc<String>>,
}
impl PatchState { fn is_empty(&self) -> bool { self.selection.is_none() && self.mode.is_none() && self.granularity.is_none() } }

#[must_use = "shared interaction patch inputs require transfer or explicit retirement"]
pub struct LocalInteractionRootPatch { owned: ManuallyDrop<PatchState> }
impl LocalInteractionRootPatch {
    /// 📥️ Moves already-admitted immutable input pointers; null removes that field's domain entry.
    pub fn from_shared(selection: Option<Arc<DomainSelection>>, mode: Option<Arc<SelectionMode>>, granularity: Option<Arc<String>>) -> Self {
        Self { owned: ManuallyDrop::new(PatchState { selection, mode, granularity }) }
    }
    /// 🧊️ Allocates shared input headers only at an explicitly synchronous decoding boundary.
    pub fn from_cold(patch: LocalInteractionDomainPatch) -> Self {
        Self::from_shared(patch.selection.map(Arc::new), patch.active_mode.map(Arc::new), patch.active_granularity.map(Arc::new))
    }
    pub fn selection(&self) -> Option<&Arc<DomainSelection>> { self.owned.selection.as_ref() }
    pub fn retire(mut self) -> LocalInteractionRootUpdate {
        LocalInteractionRootUpdate { owned: ManuallyDrop::new(UpdateState { patch: std::mem::take(&mut *self.owned), closing: true, ..Default::default() }) }
    }
}
impl Drop for LocalInteractionRootPatch {
    fn drop(&mut self) {
        if !self.owned.is_empty() { assert!(std::thread::panicking(), "interaction patch input dropped before exact ownership transfer"); return; }
        unsafe { ManuallyDrop::drop(&mut self.owned); }
    }
}
//#endregion 📥️SharedInput

//#region 🧵️Candidate
enum MapUpdate { Selection(UpdateCursor<DomainSelection>), Mode(UpdateCursor<SelectionMode>), Granularity(UpdateCursor<String>) }
impl MapUpdate {
    fn begin_close(&mut self) { match self { Self::Selection(cursor) => cursor.begin_close(), Self::Mode(cursor) => cursor.begin_close(), Self::Granularity(cursor) => cursor.begin_close() } }
    fn is_complete(&self) -> bool { match self { Self::Selection(cursor) => cursor.is_complete(), Self::Mode(cursor) => cursor.is_complete(), Self::Granularity(cursor) => cursor.is_complete() } }
    fn advance(&mut self, grant: Grant) -> Step { match self { Self::Selection(cursor) => cursor.advance(grant), Self::Mode(cursor) => cursor.advance(grant), Self::Granularity(cursor) => cursor.advance(grant) } }
}

struct UpdateState {
    candidate: Option<LocalInteractionRoot>, domain: Option<Arc<String>>, patch: PatchState,
    current: Option<MapUpdate>, cleanup: LocalInteractionRootRetirement,
    phase: u8, current_closing: bool, closing: bool,
}
impl Default for UpdateState {
    fn default() -> Self {
        Self { candidate: None, domain: None, patch: PatchState::default(), current: None, cleanup: retirement(RetirementState::default()), phase: 0, current_closing: false, closing: false }
    }
}
impl UpdateState {
    fn terminal_is_empty(&self) -> bool { self.closing && self.candidate.is_none() && self.domain.is_none() && self.patch.is_empty() && self.current.is_none() && self.cleanup.terminal_is_empty() }
    fn close_current(&mut self, grant: Grant) -> LocalInteractionUpdateStep {
        let complete = match self.current.as_mut().unwrap() {
            MapUpdate::Selection(cursor) => match cursor.close_step(grant) {
                RetirementStep::Blocked => return LocalInteractionUpdateStep::Blocked,
                RetirementStep::Progress { released_items, released_bytes } => return released(released_items, released_bytes),
                RetirementStep::OwnedValue(value) => { self.cleanup = retirement(RetirementState { selection: Some(value), ..Default::default() }); false }
                RetirementStep::Complete => true,
            },
            MapUpdate::Mode(cursor) => match cursor.close_step(grant) {
                RetirementStep::Blocked => return LocalInteractionUpdateStep::Blocked,
                RetirementStep::Progress { released_items, released_bytes } => return released(released_items, released_bytes),
                RetirementStep::OwnedValue(_) => false,
                RetirementStep::Complete => true,
            },
            MapUpdate::Granularity(cursor) => match cursor.close_step(grant) {
                RetirementStep::Blocked => return LocalInteractionUpdateStep::Blocked,
                RetirementStep::Progress { released_items, released_bytes } => return released(released_items, released_bytes),
                RetirementStep::OwnedValue(value) => { self.cleanup = retirement(RetirementState { text: Some(value), ..Default::default() }); false }
                RetirementStep::Complete => true,
            },
        };
        if complete { self.current = None; self.current_closing = false; self.phase += 1; }
        released(1, 0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalInteractionUpdateStep {
    Blocked,
    Progress { completed_items: usize, compared_bytes: usize, released_items: usize, released_bytes: usize },
    Complete,
}
fn progressed(bytes: usize) -> LocalInteractionUpdateStep { LocalInteractionUpdateStep::Progress { completed_items: 1, compared_bytes: bytes, released_items: 0, released_bytes: 0 } }
fn released(items: usize, bytes: usize) -> LocalInteractionUpdateStep { LocalInteractionUpdateStep::Progress { completed_items: 0, compared_bytes: 0, released_items: items, released_bytes: bytes } }
fn retirement(state: RetirementState) -> LocalInteractionRootRetirement { LocalInteractionRootRetirement { owned: ManuallyDrop::new(state) } }
fn cleanup_step(owner: &mut LocalInteractionRootRetirement, grant: Grant) -> LocalInteractionUpdateStep {
    match owner.advance(grant) {
        LocalInteractionRootStep::Blocked => LocalInteractionUpdateStep::Blocked,
        LocalInteractionRootStep::Progress { released_items, released_bytes } => released(released_items, released_bytes),
        LocalInteractionRootStep::Complete => released(0, 0),
    }
}

#[must_use = "interaction candidates require complete root transfer and retained close"]
pub struct LocalInteractionRootUpdate { owned: ManuallyDrop<UpdateState> }
impl LocalInteractionRoot {
    /// 🩹️ Captures three immutable roots and exact shared inputs without copying any domain or value bytes.
    pub fn begin_domain_patch(&self, domain: Arc<String>, mut patch: LocalInteractionRootPatch) -> LocalInteractionRootUpdate {
        LocalInteractionRootUpdate { owned: ManuallyDrop::new(UpdateState { candidate: Some(self.clone()), domain: Some(domain), patch: std::mem::take(&mut *patch.owned), ..Default::default() }) }
    }
}
impl LocalInteractionRootUpdate {
    pub fn is_complete(&self) -> bool { !self.owned.closing && self.owned.phase == 3 && self.owned.current.is_none() && self.owned.cleanup.terminal_is_empty() }
    pub fn take(&mut self) -> Option<LocalInteractionRoot> { if self.is_complete() { self.owned.candidate.take() } else { None } }
    pub fn terminal_is_empty(&self) -> bool { self.owned.terminal_is_empty() }

    /// 🧵️ Advances exactly one map comparison, structural transition, or pending ownership release.
    pub fn advance(&mut self, grant: Grant) -> LocalInteractionUpdateStep {
        if self.owned.closing || grant.maximum_items == 0 || grant.maximum_bytes == 0 { return LocalInteractionUpdateStep::Blocked; }
        if self.is_complete() { return LocalInteractionUpdateStep::Complete; }
        let state = &mut *self.owned;
        if !state.cleanup.terminal_is_empty() { return cleanup_step(&mut state.cleanup, grant); }
        if state.current_closing { return state.close_current(grant); }
        if let Some(current) = state.current.as_mut() {
            if current.is_complete() {
                let root = state.candidate.as_mut().unwrap();
                let map = match current {
                    MapUpdate::Selection(cursor) => MapRetirement::Selection(std::mem::replace(&mut root.selection, cursor.take_result().unwrap()).retire()),
                    MapUpdate::Mode(cursor) => MapRetirement::Mode(std::mem::replace(&mut root.active_mode, cursor.take_result().unwrap()).retire()),
                    MapUpdate::Granularity(cursor) => MapRetirement::Granularity(std::mem::replace(&mut root.active_granularity, cursor.take_result().unwrap()).retire()),
                };
                state.cleanup = retirement(RetirementState { map: Some(map), ..Default::default() });
                current.begin_close(); state.current_closing = true;
                return progressed(0);
            }
            return match current.advance(grant) {
                Step::Blocked => LocalInteractionUpdateStep::Blocked,
                Step::Progress { completed_items, completed_bytes } => LocalInteractionUpdateStep::Progress { completed_items, compared_bytes: completed_bytes, released_items: 0, released_bytes: 0 },
                Step::Complete => progressed(0),
            };
        }
        let root = state.candidate.as_ref().unwrap();
        let domain = Arc::clone(state.domain.as_ref().unwrap());
        state.current = Some(match state.phase {
            0 => MapUpdate::Selection(match state.patch.selection.take() { Some(value) => root.selection.begin_set_shared(domain, value), None => root.selection.begin_remove_shared(domain) }),
            1 => MapUpdate::Mode(match state.patch.mode.take() { Some(value) => root.active_mode.begin_set_shared(domain, value), None => root.active_mode.begin_remove_shared(domain) }),
            2 => MapUpdate::Granularity(match state.patch.granularity.take() { Some(value) => root.active_granularity.begin_set_shared(domain, value), None => root.active_granularity.begin_remove_shared(domain) }),
            _ => unreachable!(),
        });
        progressed(0)
    }

    pub fn begin_close(&mut self) {
        self.owned.closing = true;
        if let Some(current) = self.owned.current.as_mut() { current.begin_close(); }
    }

    /// ♻️ Retains every partial candidate, shared input, comparison spine and final payload until exact close.
    pub fn close_step(&mut self, grant: Grant) -> LocalInteractionUpdateStep {
        if self.terminal_is_empty() { return LocalInteractionUpdateStep::Complete; }
        if !self.owned.closing || grant.maximum_items == 0 || grant.maximum_bytes == 0 { return LocalInteractionUpdateStep::Blocked; }
        let state = &mut *self.owned;
        if !state.cleanup.terminal_is_empty() { return cleanup_step(&mut state.cleanup, grant); }
        if state.current.is_some() { return state.close_current(grant); }
        if let Some(root) = state.candidate.take() { state.cleanup = root.retire(); }
        else if let Some(value) = state.patch.selection.take() {
            if let Some(value) = Arc::into_inner(value) { state.cleanup = retirement(RetirementState { selection: Some(value), ..Default::default() }); }
        } else if let Some(value) = state.patch.mode.take() { let _ = Arc::into_inner(value); }
        else if let Some(value) = state.patch.granularity.take().or_else(|| state.domain.take()) {
            if let Some(value) = Arc::into_inner(value) { state.cleanup = retirement(RetirementState { text: Some(value), ..Default::default() }); }
        }
        released(1, 0)
    }
}
impl Drop for LocalInteractionRootUpdate {
    fn drop(&mut self) {
        if !self.terminal_is_empty() { assert!(std::thread::panicking(), "interaction root update dropped before exact terminal ownership"); return; }
        unsafe { ManuallyDrop::drop(&mut self.owned); }
    }
}
//#endregion 🧵️Candidate

//! 📨️ A whole patch remains bound to its private typed retirement cursor through handback.

use crate::{UiPatch, UiTypedRetirementCursor, UiValueRetirementStep};
use std::mem::ManuallyDrop;

//#region 📨️WholePatchOwner
#[derive(Default)]
pub struct UiPendingPatch {
    patch: ManuallyDrop<Option<UiPatch>>,
    retirement: UiTypedRetirementCursor,
    closing: bool,
}

impl UiPendingPatch {
    /// 🎟️ The producer admits the fixed owner and inline patch move before filling this source.
    pub fn source_mut(&mut self) -> Result<&mut Option<UiPatch>, &'static str> {
        if self.closing { return Err("pending whole patch is closing"); }
        Ok(&mut self.patch)
    }
    pub fn get(&self) -> Option<&UiPatch> { if self.closing { None } else { self.patch.as_ref() } }

    /// 📏️ Exact operation-page backing only; inline ownership and nested payloads are separate credits.
    pub fn retained_operation_bytes(&self) -> usize { self.patch.as_ref().map_or(0, |patch| patch.ops.allocated_bytes()) }

    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if items == 0 || bytes == 0 { return Ok(Default::default()); }
        self.closing = true;
        self.retirement.advance(&mut *self.patch, items, bytes)
    }
    pub fn terminal_is_empty(&self) -> bool { self.patch.is_none() && (!self.closing || self.retirement.terminal_is_empty()) }
}

impl std::fmt::Debug for UiPendingPatch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("UiPendingPatch").field("owned", &self.patch.is_some()).field("closing", &self.closing).finish()
    }
}

impl Drop for UiPendingPatch {
    fn drop(&mut self) { if !self.terminal_is_empty() && !std::thread::panicking() { panic!("whole patch requires exact typed retirement"); } }
}
//#endregion 📨️WholePatchOwner

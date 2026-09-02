//! 📨️ Unplaced inline patch ownership; cancellation never needs a payload allocation.

use super::*;

//#region 📨️UnplacedOwner
#[derive(Default)]
struct PendingOperation { operation: Option<UiPatchOp> }

#[derive(Default)]
pub struct UiPendingPatchOp {
    pending: PendingOperation,
    retirement: UiTypedRetirementCursor,
    closing: bool,
}

impl UiPendingPatchOp {
    /// 🎟️ The producer must account whole inline placement before writing this fixed owner slot.
    pub fn source_mut(&mut self) -> Result<&mut Option<UiPatchOp>, &'static str> {
        if self.closing { return Err("pending patch source is closing"); }
        Ok(&mut self.pending.operation)
    }

    pub fn get(&self) -> Option<&UiPatchOp> { if self.closing { None } else { self.pending.operation.as_ref() } }

    /// 📏️ The inline owner has no heap backing; descendants retain their own separately censused owners.
    pub fn allocated_bytes(&self) -> usize { 0 }

    pub fn place_into(&mut self, target: &mut UiPatchOps, physical_bytes: usize) -> Result<usize, &'static str> {
        if self.closing { return Err("pending patch source is closing"); }
        target.try_push_reserved(&mut self.pending.operation, physical_bytes)
    }

    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if items == 0 || bytes == 0 { return Ok(UiValueRetirementStep::default()); }
        self.closing = true;
        self.retirement.advance(&mut self.pending, items, bytes)
    }

    pub fn terminal_is_empty(&self) -> bool { self.pending.operation.is_none() && (!self.closing || self.retirement.terminal_is_empty()) }
}

impl std::fmt::Debug for UiPendingPatchOp {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("UiPendingPatchOp").field("owned", &self.pending.operation.is_some()).field("closing", &self.closing).finish()
    }
}

impl UiTypedRetire for PendingOperation {
    const DEPTH: usize = 1 + <UiPatchOp as UiTypedRetire>::DEPTH;
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let (phase, child_path) = path.split_first_mut().ok_or("pending patch retirement exceeds schema depth")?;
        let Some(operation) = self.operation.as_mut() else { return Ok(UiValueRetirementStep { complete: true, progressed: true, ..UiValueRetirementStep::default() }) };
        if *phase == 1 {
            self.operation = None;
            return Ok(UiValueRetirementStep { complete: true, progressed: true, released_items: 1, released_bytes: 0 });
        }
        let mut step = operation.retire_typed(child_path, value, bytes)?;
        if step.complete { *phase = 1; child_path.fill(0); }
        step.complete = false;
        Ok(step)
    }
}
//#endregion 📨️UnplacedOwner

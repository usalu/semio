//! ⚖️ Actual bounded component comparison execution.
use crate::*;
use semio_framework_ui_contract_values::compare_protocol::*;
use std::mem::ManuallyDrop;
#[derive(Default)]
pub struct UiComponentComparisonCursor {
    path: [usize; COMPARE_DEPTH],
    values: ValueComparison,
    result: Option<bool>,
}
impl UiComponentComparisonCursor {
    /// ⏩️ The enclosing owner must retain the same immutable operand pair until reset or completion.
    pub fn advance(&mut self, left: &Component, right: &Component, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        if bytes == 0 {
            return Ok(Default::default());
        }
        if let Some(equal) = self.result {
            return Ok(result(equal));
        }
        let step = left.compare_one(right, &mut self.path, &mut self.values, bytes)?;
        self.result = step.equal;
        Ok(step)
    }
    pub fn result(&self) -> Option<bool> {
        self.result
    }
    pub fn release_reads(&mut self) {
        self.values.reset();
    }
    pub fn reads_empty(&self) -> bool {
        self.values.reads_empty()
    }
}
struct OwnedComparison {
    left: Option<Component>,
    right: Option<Component>,
}
pub struct UiComponentCompare {
    owned: ManuallyDrop<OwnedComparison>,
    cursor: UiComponentComparisonCursor,
    retirement: UiTypedRetirementCursor,
    closing: bool,
}
impl UiComponentCompare {
    pub fn new(left: Component, right: Component) -> Self {
        Self { owned: ManuallyDrop::new(OwnedComparison { left: Some(left), right: Some(right) }), cursor: Default::default(), retirement: Default::default(), closing: false }
    }
    pub fn result(&self) -> Option<bool> {
        if self.closing {
            None
        } else {
            self.cursor.result()
        }
    }
    pub fn advance(&mut self, items: usize, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        if self.closing {
            return Err("component comparison is closing");
        }
        if items == 0 || bytes == 0 {
            return Ok(Default::default());
        }
        let owned = &*self.owned;
        self.cursor.advance(owned.left.as_ref().ok_or("component comparison left root missing")?, owned.right.as_ref().ok_or("component comparison right root missing")?, bytes)
    }
    pub fn take_completed(&mut self) -> Option<(Component, Component)> {
        if self.cursor.result().is_none() || self.closing {
            return None;
        }
        let owned = &mut *self.owned;
        Some((owned.left.take()?, owned.right.take()?))
    }
    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if self.terminal_is_empty() {
            return Ok(UiValueRetirementStep { complete: true, ..Default::default() });
        }
        if items == 0 || bytes == 0 {
            return Ok(Default::default());
        }
        self.closing = true;
        self.cursor.release_reads();
        self.retirement.advance(&mut *self.owned, items, bytes)
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.owned.left.is_none() && self.owned.right.is_none() && self.cursor.reads_empty() && (!self.closing || self.retirement.terminal_is_empty())
    }
}
impl UiTypedRetire for OwnedComparison {
    const DEPTH: usize = 1 + <Option<Component> as UiTypedRetire>::DEPTH;
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let (field, path) = path.split_first_mut().ok_or("component comparison retirement exceeds schema depth")?;
        let mut step = match *field {
            0 => self.left.retire_typed(path, value, bytes)?,
            1 => self.right.retire_typed(path, value, bytes)?,
            _ => return Ok(UiValueRetirementStep { complete: true, progressed: true, ..Default::default() }),
        };
        if step.complete {
            *field += 1;
            path.fill(0);
        }
        step.complete = *field == 2;
        Ok(step)
    }
}
impl Drop for UiComponentCompare {
    fn drop(&mut self) {
        if !self.terminal_is_empty() && !std::thread::panicking() {
            panic!("component comparison requires exact root retirement");
        }
    }
}
//#endregion 🎟️ExactRootOwner

#[cfg(test)]
#[path="🧪️tests/⚖️compare/🦀️.rs"]
mod tests;

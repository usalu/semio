//! 📊️ Observes committed retained receipts and execution fuel per work category for the retained-work tests.
use super::*;

#[cfg(feature = "testkit")]
impl CoreInstance {
    /// 📍️ Observes the current function index and instruction byte offset for testkit profiling.
    pub fn testkit_scalar_site(&self) -> Option<(u32, usize)> {
        let frame = self.machine.as_ref()?.frames.last()?;
        Some((frame.function, frame.pc))
    }
    /// 📍️ Observes frame count, outermost pc, operands and operand storage address to prove a refused step changed nothing.
    pub fn testkit_stack_site(&self) -> Option<(usize, usize, Vec<Value>, usize)> {
        let machine = self.machine.as_ref()?;
        Some((machine.frames.len(), machine.frames.first()?.pc, machine.values.clone(), machine.values.as_ptr() as usize))
    }
}

#[cfg(feature = "testkit")]
impl CoreInstance {
    /// 🏊️ Observes the pooled vectors per kind (locals, controls, types) and whether every pooled vector is empty.
    pub fn testkit_pool_shape(&self) -> ([usize; 3], bool) {
        let pool = &self.pool;
        let shape = [pool.locals.iter().map(Vec::len).sum(), pool.controls.iter().map(Vec::len).sum(), pool.types.iter().map(Vec::len).sum()];
        (shape, pool.locals.iter().flatten().all(Vec::is_empty) && pool.controls.iter().flatten().all(Vec::is_empty) && pool.types.iter().flatten().all(Vec::is_empty))
    }
}

#[cfg(feature = "testkit")]
impl CoreInstanceOpening {
    /// 🏊️ Fills every free pool slot so the next returned frame must release its storage instead of recycling it.
    pub fn testkit_fill_pool(&mut self) {
        if let Some(core) = self.core.as_mut() {
            for class in 0..POOL_CLASSES {
                core.pool.locals[class].resize_with(POOL_LIMIT, || Vec::with_capacity(1 << class));
                core.pool.controls[class].resize_with(POOL_LIMIT, || Vec::with_capacity(1 << class));
                core.pool.types[class].resize_with(POOL_LIMIT, || Vec::with_capacity(1 << class));
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️observation/🦀️.rs"]
mod observation;
#[cfg(test)]
pub(crate) use observation::*;

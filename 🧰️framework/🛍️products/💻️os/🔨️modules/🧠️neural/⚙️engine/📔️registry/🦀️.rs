//! 📔️ Exact shared registry ownership and incremental retirement.

use super::{Operator, OperatorImpl, Registry, ValueRetirement, ValueRetirementStep};
use std::collections::BTreeSet;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};

//#region 🔗️SharedOwnership
type FinalRoot = Arc<OnceLock<ManuallyDrop<Registry>>>;

/// 🔗️ Every registry reader participates in the exact last-reader handoff; raw Arc roots never escape.
#[derive(Clone)]
pub struct SharedRegistry { root: Option<Arc<Registry>>, final_root: FinalRoot }
impl SharedRegistry {
    /// 🎟️ Creates readers and their unique retirement authority before any shared alias can escape.
    pub fn new(registry: Registry) -> (Self, RegistryRetirement) {
        let final_root = Arc::new(OnceLock::new());
        let retirement = RegistryRetirement {
            final_root: ManuallyDrop::new(Some(Arc::clone(&final_root))),
            registry: ManuallyDrop::new(None),
            values: ValueRetirement::default(),
            implementations: ManuallyDrop::new(None),
            operator: ManuallyDrop::new(None),
            providers: ManuallyDrop::new(None),
        };
        (Self { root: Some(Arc::new(registry)), final_root }, retirement)
    }
}
impl Deref for SharedRegistry { type Target = Registry; fn deref(&self) -> &Registry { self.root.as_deref().expect("open registry reader") } }
impl AsRef<Registry> for SharedRegistry { fn as_ref(&self) -> &Registry { self } }
impl Drop for SharedRegistry {
    fn drop(&mut self) {
        if let Some(root) = self.root.take().and_then(Arc::into_inner) {
            assert!(self.final_root.set(ManuallyDrop::new(root)).is_ok(), "registry final root is handed off exactly once");
        }
    }
}
//#endregion 🔗️SharedOwnership

//#region 🧹️RegistryRetirement
/// 🧹️ Owns every collection, metadata/default domain and dynamic implementation until terminal-empty.
/// 🛟️ Supervising state must retain this cursor outside worker unwind boundaries; a live cursor Drop is not a recovery handoff.
#[must_use = "registry retirement must be driven to terminal-empty"]
pub struct RegistryRetirement {
    final_root: ManuallyDrop<Option<FinalRoot>>,
    registry: ManuallyDrop<Option<Registry>>,
    values: ValueRetirement,
    implementations: ManuallyDrop<Option<Vec<OperatorImpl>>>,
    operator: ManuallyDrop<Option<Box<dyn Operator>>>,
    providers: ManuallyDrop<Option<BTreeSet<String>>>,
}
impl RegistryRetirement {
    pub fn terminal_is_empty(&self) -> bool {
        self.final_root.is_none() && self.registry.is_none() && self.values.terminal_is_empty() && self.implementations.is_none() && self.operator.is_none() && self.providers.is_none()
    }

    /// 🎫️ Advances one owned structural frontier or a granted byte slice; waiting readers are never forced closed.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<ValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(ValueRetirementStep::Blocked); }
        let progress = ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 };
        if !self.values.terminal_is_empty() { return Ok(self.values.close_step(1, maximum_bytes)); }
        if let Some(operator) = self.operator.as_mut() {
            if operator.retirement_is_empty() {
                drop(self.operator.take());
                return Ok(progress);
            }
            let step = operator.retire_step(1, maximum_bytes, &mut self.values)?;
            match step {
                ValueRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => return Err("neural.operator-retirement-grant-exceeded"),
                ValueRetirementStep::Complete if !operator.retirement_is_empty() => return Err("neural.operator-retirement-not-empty"),
                ValueRetirementStep::Complete => return Ok(progress),
                _ => return Ok(step),
            }
        }
        if let Some(implementations) = self.implementations.as_mut() {
            if let Some(implementation) = implementations.pop() {
                self.values.push_strings(implementation.schemas);
                *self.operator = Some(implementation.operator);
            } else { drop(self.implementations.take()); }
            return Ok(progress);
        }
        if let Some(providers) = self.providers.as_mut() {
            if let Some(provider) = providers.pop_first() { self.values.text(provider); }
            else { drop(self.providers.take()); }
            return Ok(progress);
        }
        if let Some(registry) = self.registry.as_mut() {
            if let Some((key, schema)) = registry.schemas.pop_first() {
                self.values.text(key); self.values.push_schema(schema);
            } else if let Some((key, record)) = registry.operators.pop_first() {
                self.values.text(key); self.values.push_operator(record.info);
                *self.implementations = Some(record.implementations);
            } else if let Some((key, schemas)) = registry.operator_produces.pop_first() {
                self.values.text(key); self.values.push_strings(schemas);
            } else if let Some((key, providers)) = registry.schema_providers.pop_first() {
                self.values.text(key); *self.providers = Some(providers);
            } else { drop(self.registry.take()); }
            return Ok(progress);
        }
        if let Some(final_root) = self.final_root.as_mut() {
            let Some(slot) = Arc::get_mut(final_root) else { return Ok(ValueRetirementStep::Blocked); };
            let Some(registry) = slot.take() else { return Err("neural.registry-final-root-missing"); };
            *self.registry = Some(ManuallyDrop::into_inner(registry));
            drop(self.final_root.take());
            return Ok(progress);
        }
        Ok(ValueRetirementStep::Complete)
    }
}
impl Drop for RegistryRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(), "registry must finish explicit domain retirement before drop"); } }
}
//#endregion 🧹️RegistryRetirement

#[cfg(test)]
#[path = "🧪️component.rs"]
mod tests;

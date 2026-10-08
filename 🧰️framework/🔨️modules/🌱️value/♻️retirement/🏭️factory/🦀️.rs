//! 🏭️ Typed prefunded factory tickets consume shared aliases before bounded payload retirement.
use crate::{ErasedSnapshotRetirement, SnapshotRetirementStep, ValueError, ValueRefusalKind};
use std::{alloc::Layout, sync::Arc};

/// 🧬️ Concrete factory ownership supplies an inline close state before any alias retires.
pub trait FactoryPayloadRetirement: Send + Sync + 'static {
    type CloseState: Send + 'static;
    fn close_state_birth_bytes(&self) -> usize;
    fn prepare_close_state(&self) -> Self::CloseState;
    fn transfer_payload(value: Self, state: &mut Self::CloseState) where Self: Sized;
    fn close_state_byte_demand(state: &Self::CloseState) -> usize;
    fn close_state_step(state: &mut Self::CloseState, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError>;
    fn close_state_terminal_is_empty(state: &Self::CloseState) -> bool;
}

impl FactoryPayloadRetirement for std::sync::atomic::AtomicUsize {
    type CloseState = ();
    fn close_state_birth_bytes(&self) -> usize { 0 }
    fn prepare_close_state(&self) -> Self::CloseState { () }
    fn transfer_payload(value: Self, _: &mut Self::CloseState) { drop(value); }
    fn close_state_byte_demand(_: &Self::CloseState) -> usize { 0 }
    fn close_state_step(_: &mut Self::CloseState, _: usize, _: usize) -> Result<SnapshotRetirementStep, ValueError> { Ok(SnapshotRetirementStep::Complete) }
    fn close_state_terminal_is_empty(_: &Self::CloseState) -> bool { true }
}

impl FactoryPayloadRetirement for std::sync::atomic::AtomicBool {
    type CloseState = ();
    fn close_state_birth_bytes(&self) -> usize { 0 }
    fn prepare_close_state(&self) -> Self::CloseState { () }
    fn transfer_payload(value: Self, _: &mut Self::CloseState) { drop(value); }
    fn close_state_byte_demand(_: &Self::CloseState) -> usize { 0 }
    fn close_state_step(_: &mut Self::CloseState, _: usize, _: usize) -> Result<SnapshotRetirementStep, ValueError> { Ok(SnapshotRetirementStep::Complete) }
    fn close_state_terminal_is_empty(_: &Self::CloseState) -> bool { true }
}

/// 📬️ Object-safe factory ownership closes through its original concrete alias and inline payload.
pub trait FactoryRetirement: Send + Sync {
    fn factory_retirement_birth_bytes(&self) -> usize;
    fn preborn_factory_retirement(self: Arc<Self>) -> Box<dyn ErasedSnapshotRetirement>;
}

/// 🪆️ Child tickets occupy a fixed inline directory prepared before their parent releases.
pub struct FactoryChildTickets<const N: usize>(pub [Option<Box<dyn ErasedSnapshotRetirement>>; N]);

impl<const N: usize> FactoryChildTickets<N> {
    pub fn next_close_byte_demand(&self) -> usize { self.0.iter().flatten().next().map_or(0, factory_ticket_byte_demand) }
    pub fn terminal_is_empty(&self) -> bool { self.0.iter().all(Option::is_none) }
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        match self.0.iter_mut().find(|slot| slot.is_some()) {
            Some(slot) => match close_factory_ticket(slot, maximum_items, maximum_bytes)? {
                SnapshotRetirementStep::Complete => Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
                step => Ok(step),
            },
            None => Ok(SnapshotRetirementStep::Complete),
        }
    }
}

struct FactoryTicket<T: FactoryPayloadRetirement> {
    alias: Option<Arc<T>>,
    state: T::CloseState,
}

fn arc_extent<T>() -> usize {
    Layout::new::<[usize; 2]>().extend(Layout::new::<T>()).expect("factory Arc concrete layout").0.pad_to_align().size()
}

/// 🏭️ Prices the concrete original factory Arc control and payload layout without allocating.
pub fn factory_arc_birth_bytes<T: FactoryPayloadRetirement>() -> usize { arc_extent::<T>() }

/// 🎟️ Prices the concrete original ticket frame without constructing any factory or allocation.
pub const fn factory_retirement_frame_bytes<T: FactoryPayloadRetirement>() -> usize { std::mem::size_of::<FactoryTicket<T>>() }

/// 🌱️ Prices one concrete factory, its original ticket and declared nested factory constructors.
pub fn factory_constructor_birth_bytes<T: FactoryPayloadRetirement>(child_constructor_bytes: usize) -> usize {
    factory_arc_birth_bytes::<T>().checked_add(factory_retirement_frame_bytes::<T>()).and_then(|bytes| bytes.checked_add(child_constructor_bytes)).expect("factory constructor layout")
}

impl<T: FactoryPayloadRetirement> FactoryRetirement for T {
    fn factory_retirement_birth_bytes(&self) -> usize {
        factory_retirement_frame_bytes::<T>().checked_add(self.close_state_birth_bytes()).expect("factory ticket birth layout")
    }

    fn preborn_factory_retirement(self: Arc<Self>) -> Box<dyn ErasedSnapshotRetirement> {
        let state = self.prepare_close_state();
        Box::new(FactoryTicket { alias: Some(self), state })
    }
}

impl<T: FactoryPayloadRetirement> ErasedSnapshotRetirement for FactoryTicket<T> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if let Some(alias) = self.alias.as_ref() {
            if Arc::weak_count(alias) != 0 { return Ok(SnapshotRetirementStep::Blocked); }
            let extent = arc_extent::<T>();
            if maximum_bytes < extent { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
            let value = Arc::into_inner(self.alias.take().expect("factory alias remains present after its exact grant"));
            let released_bytes = if let Some(value) = value { T::transfer_payload(value, &mut self.state); extent } else { 0 };
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        match T::close_state_step(&mut self.state, maximum_items.min(1), maximum_bytes)? {
            SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
            SnapshotRetirementStep::Pending { .. } => Err(ValueError::new(ValueRefusalKind::InvariantViolated, "factory payload exceeded its exact close grant")),
            SnapshotRetirementStep::Complete if T::close_state_terminal_is_empty(&self.state) => Ok(SnapshotRetirementStep::Complete),
            SnapshotRetirementStep::Complete => Err(ValueError::new(ValueRefusalKind::InvariantViolated, "factory payload returned a false terminal witness")),
            SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
        }
    }

    fn terminal_is_empty(&self) -> bool { self.alias.is_none() && T::close_state_terminal_is_empty(&self.state) }
    fn next_close_byte_demand(&self) -> usize { if self.alias.is_some() { arc_extent::<T>() } else { T::close_state_byte_demand(&self.state) } }
}

impl<T: FactoryPayloadRetirement> Drop for FactoryTicket<T> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "factory ticket reached Drop before its original alias and payload were retired"); }
}

/// 📏️ Borrows the exact retained factory allocation or its final prefunded ticket frame.
pub fn factory_ticket_byte_demand(ticket: &Box<dyn ErasedSnapshotRetirement>) -> usize {
    if ticket.terminal_is_empty() { std::mem::size_of_val(ticket.as_ref()) } else { ticket.next_close_byte_demand() }
}

/// 🧹️ Keeps payload and terminal ticket release in separately funded physical turns.
pub fn close_factory_ticket(slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
    let Some(ticket) = slot.as_mut() else { return Ok(SnapshotRetirementStep::Complete); };
    if ticket.terminal_is_empty() {
        let bytes = std::mem::size_of_val(ticket.as_ref());
        if maximum_items == 0 || maximum_bytes < bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        drop(slot.take());
        return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
    }
    ticket.close_step(maximum_items, maximum_bytes)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

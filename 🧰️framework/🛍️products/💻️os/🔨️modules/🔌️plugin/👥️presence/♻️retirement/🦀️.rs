//! 👥️ Domain-authorized presence store closure preserves exact local and peer owner retirement.

use crate::{ArtifactOwnedDisposer, Fault, PluginCloseStep};
use crate::store::{Mutation, PresenceStore, PresenceStoreRetirement, SnapshotRetirementStep};
use std::sync::{Arc, Weak};

const _: () = assert!(std::mem::size_of::<crate::NoPresence>() == 0 && !std::mem::needs_drop::<crate::NoPresence>());

/// 🫧️ Explicit ownership for the framework's zero-payload presence type only.
pub struct NoPresenceRetirementFactory;

impl crate::store::SnapshotRetirementFactory<crate::NoPresence> for NoPresenceRetirementFactory {
    fn retire(&self, root: Arc<crate::NoPresence>) -> Box<dyn crate::store::ErasedSnapshotRetirement> {
        Box::new(NoPresenceRetirement(std::mem::ManuallyDrop::new(Some(root))))
    }
}

struct NoPresenceRetirement(std::mem::ManuallyDrop<Option<Arc<crate::NoPresence>>>);

impl crate::store::ErasedSnapshotRetirement for NoPresenceRetirement {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.0.is_none() { return Ok(SnapshotRetirementStep::Complete); }
        if items == 0 || bytes == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        drop(self.0.take());
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool { self.0.is_none() }
}

impl Drop for NoPresenceRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.0.is_none(), "empty presence must return its exact root before drop"); } }
}

/// 🛂️ A domain supplies its exact empty terminal root and predicate; the store supplies its installed factories.
pub struct PresenceStoreOwnedDisposer<P> {
    terminal: Option<Arc<P>>,
    terminal_root: Weak<P>,
    terminal_generation: Option<u64>,
    terminal_is_empty: fn(&P) -> bool,
    retirement: Option<PresenceStoreRetirement<P>>,
}

impl<P> PresenceStoreOwnedDisposer<P> {
    pub fn new(terminal: Arc<P>, terminal_is_empty: fn(&P) -> bool) -> Result<Self, Arc<P>> {
        if !terminal_is_empty(&terminal) { return Err(terminal); }
        Ok(Self { terminal_root: Arc::downgrade(&terminal), terminal: Some(terminal), terminal_generation: None, terminal_is_empty, retirement: None })
    }
}

impl<P: Clone + Send + Sync + 'static> PresenceStoreOwnedDisposer<P> {
    fn owns_terminal<M: Mutation<P>>(&self, owner: &PresenceStore<P, M>) -> bool {
        self.terminal_generation == Some(owner.generation_now()) && owner.retirement_started()
            && self.terminal_root.upgrade().is_some_and(|terminal| std::ptr::eq(terminal.as_ref(), owner.local()))
            && (self.terminal_is_empty)(owner.local()) && owner.peers_root().is_empty()
    }
}

impl<P: Clone + Send + Sync + 'static, M: Mutation<P>> ArtifactOwnedDisposer<PresenceStore<P, M>> for PresenceStoreOwnedDisposer<P> {
    fn close_step(&mut self, owner: &mut PresenceStore<P, M>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if self.retirement.is_some() && !self.owns_terminal(owner) { return Err(Fault::from("presence close terminal root or generation changed")); }
        if self.retirement.as_ref().is_some_and(PresenceStoreRetirement::terminal_is_empty) { return Ok(PluginCloseStep::Complete); }
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes).map_err(Fault::from)? {
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maximum_bytes => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
                SnapshotRetirementStep::Pending { .. } => Err(Fault::from("presence retirement exceeded its exact grant")),
                SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "presence retains a captured local or peer reader" }),
                SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => Ok(PluginCloseStep::Complete),
                SnapshotRetirementStep::Complete => Err(Fault::from("presence retirement completed with retained owners")),
            };
        }
        let terminal = self.terminal.take().ok_or_else(|| Fault::from("presence close lost its exact terminal root"))?;
        match owner.begin_retirement(terminal, self.terminal_is_empty) {
            Ok(retirement) => {
                self.terminal_generation = Some(owner.generation_now());
                self.retirement = Some(retirement);
                Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
            }
            Err((reason, terminal)) => { self.terminal = Some(terminal); Err(Fault::from(reason)) }
        }
    }

    fn terminal_is_empty(&self, owner: &PresenceStore<P, M>) -> bool {
        self.terminal.is_none() && self.owns_terminal(owner) && self.retirement.as_ref().is_some_and(PresenceStoreRetirement::terminal_is_empty)
    }
}

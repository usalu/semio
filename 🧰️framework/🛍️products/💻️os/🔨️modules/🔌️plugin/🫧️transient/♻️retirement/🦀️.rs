//! 🫧️ Exact zero-payload transient closure, fenced to its installed terminal root and generation.

use crate::{ArtifactOwnedDisposer, Fault, NoTransient, NoTransientMutation, PluginCloseStep};
use crate::store::TransientStore;
use std::sync::{Arc, Weak};

type Store = TransientStore<NoTransient, NoTransientMutation>;
const _: () = assert!(std::mem::size_of::<NoTransient>() == 0 && !std::mem::needs_drop::<NoTransient>());
const _: () = assert!(std::mem::size_of::<NoTransientMutation>() == 0 && !std::mem::needs_drop::<NoTransientMutation>());
const _: () = assert!(std::mem::size_of::<Store>() == std::mem::size_of::<(Arc<NoTransient>, u64)>());

/// 🧊️ Only the statically empty transient lane is admitted; arbitrary store payloads cannot use this adapter.
#[derive(Default)]
pub struct NoTransientStoreDisposer {
    terminal_root: Option<Weak<NoTransient>>,
    terminal_generation: u64,
}

impl NoTransientStoreDisposer {
    pub fn new() -> Self { Self::default() }

    fn owns_terminal(&self, owner: &Store) -> bool {
        self.terminal_generation == owner.generation_now() && self.terminal_root.as_ref().and_then(Weak::upgrade).is_some_and(|root| Arc::ptr_eq(&root, &owner.current_root()))
    }
}

impl ArtifactOwnedDisposer<Store> for NoTransientStoreDisposer {
    fn close_step(&mut self, owner: &mut Store, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if self.terminal_root.is_some() {
            return self.owns_terminal(owner).then_some(PluginCloseStep::Complete).ok_or_else(|| Fault::from("empty transient terminal owner or generation changed"));
        }
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
        let retired = std::mem::replace(owner, Store::new(NoTransient::default()));
        self.terminal_root = Some(Arc::downgrade(&owner.current_root()));
        self.terminal_generation = owner.generation_now();
        drop(retired);
        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self, owner: &Store) -> bool { self.owns_terminal(owner) }
}

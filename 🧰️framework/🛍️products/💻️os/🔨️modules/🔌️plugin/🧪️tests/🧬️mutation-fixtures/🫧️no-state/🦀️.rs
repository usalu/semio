//! 🫧️ Exact no-state store retirement owners for the keyed app fixture.

use crate::app::{ArtifactOwnedDisposer, NoPresence, NoPresenceMutation, PluginCloseStep};
use crate::store;
use semio_framework::Fault;

//#region 🧹️NoStateFixtureOwners
pub(crate) fn presence_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::PresenceStore<NoPresence, NoPresenceMutation>>> {
    Box::new(KeyedNoPresenceStoreDisposer(None))
}

pub(crate) fn transient_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::TransientStore<crate::app::NoTransient, crate::app::NoTransientMutation>>> {
    Box::new(KeyedNoTransientStoreDisposer(KeyedNoTransientStoreRetirement::Unstarted))
}

pub(crate) fn presence_peer_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<NoPresence>> {
    std::sync::Arc::new(super::super::BoundedConfigRetirementFactory::<NoPresence>::new())
}

pub(crate) fn presence_local_root_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<NoPresence>> {
    std::sync::Arc::new(super::super::BoundedConfigRetirementFactory::<NoPresence>::new())
}

pub(crate) fn transient_local_root_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<crate::app::NoTransient>> {
    std::sync::Arc::new(super::super::BoundedConfigRetirementFactory::<crate::app::NoTransient>::new())
}

struct KeyedNoPresenceStoreDisposer(Option<store::PresenceStoreRetirement<NoPresence>>);

impl ArtifactOwnedDisposer<store::PresenceStore<NoPresence, NoPresenceMutation>> for KeyedNoPresenceStoreDisposer {
    fn close_step(&mut self, owner: &mut store::PresenceStore<NoPresence, NoPresenceMutation>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(active) = self.0.as_mut() {
            return active.close_step(1, maximum_bytes).map_err(Fault::from).map(|step| match step {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
                store::SnapshotRetirementStep::Blocked => PluginCloseStep::Blocked { reason: "keyed no-state presence retains a reader" },
                store::SnapshotRetirementStep::Complete => PluginCloseStep::Complete,
            });
        }
        self.0 = Some(owner.begin_retirement(std::sync::Arc::new(NoPresence::default()), |_| true).map_err(|(reason, _)| Fault::from(reason))?);
        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self, owner: &store::PresenceStore<NoPresence, NoPresenceMutation>) -> bool {
        owner.retirement_started() && self.0.as_ref().is_some_and(store::PresenceStoreRetirement::terminal_is_empty) && owner.peers_root().is_empty()
    }
}

enum KeyedNoTransientStoreRetirement {
    Unstarted,
    Retiring { retirement: Box<dyn store::ErasedSnapshotRetirement>, terminal_root: std::sync::Weak<crate::app::NoTransient>, terminal_generation: u64 },
    Complete { terminal_root: std::sync::Weak<crate::app::NoTransient>, terminal_generation: u64 },
}

struct KeyedNoTransientStoreDisposer(KeyedNoTransientStoreRetirement);

fn keyed_no_transient_terminal_owner(owner: &store::TransientStore<crate::app::NoTransient, crate::app::NoTransientMutation>, terminal_root: &std::sync::Weak<crate::app::NoTransient>, terminal_generation: u64) -> bool {
    owner.generation_now() == terminal_generation && terminal_root.upgrade().is_some_and(|terminal| std::sync::Arc::ptr_eq(&terminal, &owner.current_root()))
}

impl ArtifactOwnedDisposer<store::TransientStore<crate::app::NoTransient, crate::app::NoTransientMutation>> for KeyedNoTransientStoreDisposer {
    fn close_step(&mut self, owner: &mut store::TransientStore<crate::app::NoTransient, crate::app::NoTransientMutation>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if let KeyedNoTransientStoreRetirement::Complete { terminal_root, terminal_generation } = &self.0 {
            return keyed_no_transient_terminal_owner(owner, terminal_root, *terminal_generation)
                .then_some(PluginCloseStep::Complete)
                .ok_or_else(|| Fault::from("keyed no-state transient terminal owner changed after completion"));
        }
        if maximum_items == 0 || maximum_bytes < store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if matches!(self.0, KeyedNoTransientStoreRetirement::Unstarted) {
            let retired = std::mem::replace(owner, store::TransientStore::new(crate::app::NoTransient::default()));
            let terminal_root = std::sync::Arc::downgrade(&owner.current_root());
            let terminal_generation = owner.generation_now();
            let factory = super::super::BoundedConfigRetirementFactory::<store::TransientStore<crate::app::NoTransient, crate::app::NoTransientMutation>>::new();
            let retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&factory, retired);
            self.0 = KeyedNoTransientStoreRetirement::Retiring { retirement, terminal_root, terminal_generation };
        }
        let (step, terminal_root, terminal_generation) = match &mut self.0 {
            KeyedNoTransientStoreRetirement::Retiring { retirement, terminal_root, terminal_generation } => (store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), 1, maximum_bytes).map_err(Fault::from)?, terminal_root.clone(), *terminal_generation),
            KeyedNoTransientStoreRetirement::Unstarted | KeyedNoTransientStoreRetirement::Complete { .. } => unreachable!("keyed no-state transient close state is resolved before retirement"),
        };
        match step {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
            store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "keyed no-state transient retirement is blocked" }),
            store::SnapshotRetirementStep::Complete => {
                let terminal_retirement_empty = matches!(&self.0, KeyedNoTransientStoreRetirement::Retiring { retirement, .. } if store::ErasedSnapshotRetirement::terminal_is_empty(retirement.as_ref()));
                if !terminal_retirement_empty || !keyed_no_transient_terminal_owner(owner, &terminal_root, terminal_generation) {
                    return Err(Fault::from("keyed no-state transient reported completion before its owned store and installed terminal root were exact"));
                }
                self.0 = KeyedNoTransientStoreRetirement::Complete { terminal_root, terminal_generation };
                Ok(PluginCloseStep::Complete)
            }
        }
    }

    fn terminal_is_empty(&self, owner: &store::TransientStore<crate::app::NoTransient, crate::app::NoTransientMutation>) -> bool {
        matches!(&self.0, KeyedNoTransientStoreRetirement::Complete { terminal_root, terminal_generation } if keyed_no_transient_terminal_owner(owner, terminal_root, *terminal_generation))
    }
}
//#endregion 🧹️NoStateFixtureOwners

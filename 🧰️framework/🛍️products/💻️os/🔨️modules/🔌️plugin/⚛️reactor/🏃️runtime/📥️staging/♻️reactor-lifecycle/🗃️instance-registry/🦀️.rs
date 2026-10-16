//! 🗃️ Close worker state, fixed instance registry, actor authority and the typed plugin runtime.

use super::*;
pub(super) struct RuntimeCloseWorkerState<PA: PluginApp> {
    pub(super) instance_id: u32,
    pub(crate) generation: semio_framework_job::Generation,
    pub(super) mounted_policy: MountedOwnerPolicyV1,
    pub(super) cell: std::sync::Mutex<std::mem::ManuallyDrop<Option<std::sync::Arc<RuntimeAppCell<PA>>>>>,
    pub(super) pump: std::sync::Mutex<RuntimeCloseCleanupPump<PA>>,
    pub(super) status: AtomicU8,
    pub(super) deadline_resume: AtomicU8,
    pub(super) deadline_elapsed_us: AtomicU64,
    pub(super) stalled_steps: AtomicU8,
    /// ⏱️ Monotonic instant the current zero-progress run began (`0` = the ladder is moving).
    pub(super) stall_since_us: AtomicU64,
    /// ⏱️ Stall credit actually spent when a `ZeroProgress` verdict was stored, so the emitted
    /// fault quotes the measured stall instead of the last turn's unrelated cost.
    pub(super) stall_credit_spent_us: AtomicU64,
    pub(super) last_callback_elapsed_us: AtomicU64,
    #[cfg(test)]
    pub(super) last_fault: std::sync::Mutex<[u8; 256]>,
    #[cfg(test)]
    pub(super) last_fault_origin: AtomicU8,
    #[cfg(test)]
    pub(super) physical_close_calls: AtomicU64,
    #[cfg(test)]
    pub(super) callback_phase_started_us: AtomicU64,
    #[cfg(test)]
    pub(super) callback_phase_us: [AtomicU64; 24],
}
pub(super) struct RuntimeCloseEntry<PA: PluginApp> {
    pub(super) state: std::sync::Arc<RuntimeCloseWorkerState<PA>>,
}
pub(super) mod instance_lifetime {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🚪️lifetime/🦀️.rs"));
}
pub use instance_lifetime::{plugin_capture_instance_close, PluginInstanceCloseLease};
pub(super) const PLUGIN_RUNTIME_INSTANCE_SLOTS: usize = 1_024;
const PLUGIN_RUNTIME_INSTANCE_WORDS: usize = PLUGIN_RUNTIME_INSTANCE_SLOTS / u64::BITS as usize;
const PLUGIN_RUNTIME_ACTOR_BYTES: usize = 4_096;
pub(super) struct RuntimeActorAuthority {
    len: u16,
    pub(super) bytes: [u8; PLUGIN_RUNTIME_ACTOR_BYTES],
}
impl RuntimeActorAuthority {
    pub(super) fn new(actor: &str) -> Result<Self, Fault> {
        if actor.is_empty() || actor.len() > PLUGIN_RUNTIME_ACTOR_BYTES {
            return Err(plugin_internal_fault("runtime actor identity is empty or exceeds its fixed byte cap"));
        }
        let mut bytes = [0; PLUGIN_RUNTIME_ACTOR_BYTES];
        bytes[..actor.len()].copy_from_slice(actor.as_bytes());
        Ok(Self { len: actor.len() as u16, bytes })
    }
}
impl std::fmt::Display for RuntimeActorAuthority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("admitted actor authority remains utf-8"))
    }
}
pub(super) struct RuntimeInstanceRegistry<T> {
    slots: Box<[std::mem::MaybeUninit<(u32, T)>]>,
    pub(super) occupied: [u64; PLUGIN_RUNTIME_INSTANCE_WORDS],
    pub(crate) allocation_admitted: bool,
}
impl<T> RuntimeInstanceRegistry<T> {
    /// 🈳️ Constructs the registry EMPTY — no slot backing at all. Allocator success is
    /// not an admission: a runtime carries four of these, and the actor one alone is
    /// `PLUGIN_RUNTIME_INSTANCE_SLOTS` × `RuntimeActorAuthority` ≈ 4 MiB, which an actor that
    /// never opens an instance would otherwise reserve contiguously up front — the exact shape
    /// a guest's `dlmalloc` never gives back. Backing is taken by {@link admit_backing}, on the
    /// first admission that actually needs a slot.
    pub(crate) fn new() -> Self {
        Self { slots: Vec::new().into_boxed_slice(), occupied: [0; PLUGIN_RUNTIME_INSTANCE_WORDS], allocation_admitted: false }
    }
    /// 🛂️ Takes the one fixed slot allocation, once, and reports whether this registry
    /// now has backing. Idempotent; a refused reservation leaves the registry exactly as empty
    /// as it was, so every admission gate answers "cannot insert" instead of panicking.
    fn admit_backing(&mut self) -> bool {
        if self.allocation_admitted {
            return true;
        }
        let mut slots = Vec::new();
        if slots.try_reserve_exact(PLUGIN_RUNTIME_INSTANCE_SLOTS).is_err() {
            return false;
        }
        slots.resize_with(PLUGIN_RUNTIME_INSTANCE_SLOTS, std::mem::MaybeUninit::uninit);
        self.slots = slots.into_boxed_slice();
        self.allocation_admitted = true;
        true
    }
    fn index(instance_id: u32) -> usize {
        instance_id as usize % PLUGIN_RUNTIME_INSTANCE_SLOTS
    }
    fn occupied(&self, index: usize) -> bool {
        self.occupied[index / u64::BITS as usize] & (1u64 << (index % u64::BITS as usize)) != 0
    }
    fn set_occupied(&mut self, index: usize, occupied: bool) {
        let word = &mut self.occupied[index / u64::BITS as usize];
        let mask = 1u64 << (index % u64::BITS as usize);
        if occupied {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }
    /// 📎️ SAFETY: occupancy is set only after `write` and cleared before `assume_init_read`.
    fn entry(&self, index: usize) -> Option<&(u32, T)> {
        if !self.occupied(index) {
            return None;
        }
        self.slots.get(index).map(|slot| unsafe { slot.assume_init_ref() })
    }
    #[cfg(test)]
    pub(super) fn insert(&mut self, instance_id: u32, value: T) -> Result<(), T> {
        if !self.admit_backing() {
            return Err(value);
        }
        let index = Self::index(instance_id);
        if self.occupied(index) {
            return Err(value);
        }
        self.slots[index].write((instance_id, value));
        self.set_occupied(index, true);
        Ok(())
    }
    pub(super) fn insert_admitted(&mut self, instance_id: u32, value: T) {
        let index = Self::index(instance_id);
        let admitted = self.admit_backing();
        debug_assert!(admitted && !self.occupied(index));
        self.slots[index].write((instance_id, value));
        self.set_occupied(index, true);
    }
    pub(super) fn get(&self, instance_id: u32) -> Option<&T> {
        self.entry(Self::index(instance_id)).and_then(|(candidate, value)| (*candidate == instance_id).then_some(value))
    }
    pub(super) fn get_mut(&mut self, instance_id: u32) -> Option<&mut T> {
        let index = Self::index(instance_id);
        if self.entry(index).is_none_or(|(candidate, _)| *candidate != instance_id) {
            return None;
        }
        Some(unsafe { &mut self.slots[index].assume_init_mut().1 })
    }
    /// 🚦️ The admission gate. It takes the backing itself (see {@link admit_backing}) so
    /// a preflight that answers `true` is a promise the following {@link insert_admitted} can
    /// keep, and a refused reservation is an ordinary "cannot insert" instead of a panic.
    pub(super) fn can_insert(&mut self, instance_id: u32) -> bool {
        self.admit_backing() && !self.occupied(Self::index(instance_id))
    }
    /// 🫴️ SAFETY: exact identity and occupancy were checked, and occupancy is now cleared.
    pub(super) fn take(&mut self, instance_id: u32) -> Option<T> {
        let index = Self::index(instance_id);
        if self.entry(index).is_none_or(|(candidate, _)| *candidate != instance_id) {
            return None;
        }
        self.set_occupied(index, false);
        Some(unsafe { self.slots[index].assume_init_read().1 })
    }
    pub(super) fn entry_at(&self, index: usize) -> Option<(u32, &T)> {
        self.entry(index).map(|(instance_id, value)| (*instance_id, value))
    }
    pub(super) fn next_entry_from(&self, start: usize) -> Option<(usize, u32, &T)> {
        let start = start % PLUGIN_RUNTIME_INSTANCE_SLOTS;
        let word = start / u64::BITS as usize;
        let bit = start % u64::BITS as usize;
        for offset in 0..=PLUGIN_RUNTIME_INSTANCE_WORDS {
            let index = (word + offset) % PLUGIN_RUNTIME_INSTANCE_WORDS;
            let mask = if offset == 0 {
                u64::MAX << bit
            } else if offset == PLUGIN_RUNTIME_INSTANCE_WORDS {
                (1u64 << bit) - 1
            } else {
                u64::MAX
            };
            let occupied = self.occupied[index] & mask;
            if occupied == 0 {
                continue;
            }
            let slot = index * u64::BITS as usize + occupied.trailing_zeros() as usize;
            return self.entry_at(slot).map(|(id, value)| (slot, id, value));
        }
        None
    }
    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.occupied.iter().all(|word| *word == 0)
    }
}
impl<T> Drop for RuntimeInstanceRegistry<T> {
    fn drop(&mut self) {}
}
pub(super) struct RuntimeDocumentBackboneBindingV1 {
    pub(crate) generation: u64,
    pub(super) uri: Option<String>,
    pub(super) channel: Option<store::os_store::backbone::ActorBackboneChannelOwner>,
}
impl RuntimeDocumentBackboneBindingV1 {
    pub(super) fn state(&self) -> crate::document_backbone_binding::DocumentBackboneBindingStateV1 {
        crate::document_backbone_binding::DocumentBackboneBindingStateV1 { generation: self.generation, uri: self.uri.clone() }
    }
}
#[cfg(test)]
include!("../../../../../🧪️tests/🔬️plugin-runtime-native-aggregate-backing/🦀️.rs");
#[cfg(test)]
include!("../../../../../🧪️tests/🔬️plugin-runtime-runtime-instance-registry/🦀️.rs");
/// 🧩️ Typed state owned by each embedding plugin's `plugin_exports!` expansion.
pub struct PluginRuntime<PA: PluginApp> {
    pub(super) mounted_policy: MountedOwnerPolicyV1,
    pub(crate) guest_lifetimes: RefCell<crate::reactor::instance_lifetime::NativeLifecycleRegistry<PA>>,
    pub(super) plugin: RefCell<Option<Plugin<PA>>>,
    pub(super) plugin_assembly_error: RefCell<Option<Fault>>,
    pub(super) instances: RefCell<RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>>,
    pub(super) close_quarantine: RefCell<RuntimeInstanceRegistry<RuntimeCloseEntry<PA>>>,
    pub(super) close_cleanup_cursor: Cell<usize>,
    pub(super) live_cleanup_cursor: Cell<usize>,
    /// 🌡️ Whether the instance the last [`plugin_step_live_cleanup`] stepped reported maintenance pressure.
    pub(super) live_cleanup_pressure: Cell<bool>,
    pub(super) typed_continuation_cursor: Cell<usize>,
    pub(super) close_generation: Cell<u64>,
    pub(super) local_interaction_query_generation: crate::local_interaction::live::LocalInteractionQueryGeneration,
    pub(super) instance_actors: RefCell<RuntimeInstanceRegistry<RuntimeActorAuthority>>,
    pub(super) document_backbones: RefCell<RuntimeInstanceRegistry<RuntimeDocumentBackboneBindingV1>>,
}
impl<PA: PluginApp> PluginRuntime<PA> {
    /// 🏇️ Borrows the embedding owner's unchanged phase authority for each real reactor turn.
    pub fn mounted_owner_policy(&self) -> MountedOwnerPolicyV1 { self.mounted_policy }
    /// 🧩️ Creates an empty typed runtime without erasing the plugin-owned app enum.
    pub fn new(mounted_policy: MountedOwnerPolicyV1) -> Result<Self, semio_framework_value::ValueError> {
        let mounted_policy = mounted_policy.validate()?;
        Ok(Self {
            mounted_policy,
            guest_lifetimes: RefCell::new(crate::reactor::instance_lifetime::NativeLifecycleRegistry::new()),
            plugin: RefCell::new(None),
            plugin_assembly_error: RefCell::new(None),
            instances: RefCell::new(RuntimeInstanceRegistry::new()),
            close_quarantine: RefCell::new(RuntimeInstanceRegistry::new()),
            close_cleanup_cursor: Cell::new(0),
            live_cleanup_cursor: Cell::new(0),
            live_cleanup_pressure: Cell::new(false),
            typed_continuation_cursor: Cell::new(0),
            close_generation: Cell::new(0),
            local_interaction_query_generation: Default::default(),
            instance_actors: RefCell::new(RuntimeInstanceRegistry::new()),
            document_backbones: RefCell::new(RuntimeInstanceRegistry::new()),
        })
    }
}

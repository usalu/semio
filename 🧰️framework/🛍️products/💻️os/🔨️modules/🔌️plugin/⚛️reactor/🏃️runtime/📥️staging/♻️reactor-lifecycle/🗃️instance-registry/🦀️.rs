//! 🗃️ Close worker state, fixed instance registry, actor authority and the typed plugin runtime.

use semio_framework_plugin_decl_app::ArtifactApp;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::cell::{Cell, RefCell};
use semio_framework::{Fault, FaultFrom};
use semio_framework_value::{FromValue, ToValue};
use semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1;
use semio_framework_plugin_plugin_program::{Plugin, PluginProgram};
use semio_framework_plugin_reactor_app::PluginApp;
use crate::life_app_cell::RuntimeAppCell;
use crate::life_cleanup_pumps::{RuntimeCloseCleanupPump, checked_runtime_close_generation, plugin_begin_instance_close};
use crate::life_cleanup_vocabulary::{RuntimeCloseStatus, runtime_cleanup_fault};
use serde_json::Value;
use semio_framework_plugin_reactor_app::app_runtime_helpers::plugin_internal_fault;
#[doc(hidden)]
pub struct RuntimeCloseWorkerState<PA: PluginApp> {
    pub instance_id: u32,
    pub generation: semio_framework_job::Generation,
    pub mounted_policy: MountedOwnerPolicyV1,
    pub cell: std::sync::Mutex<std::mem::ManuallyDrop<Option<std::sync::Arc<RuntimeAppCell<PA>>>>>,
    pub pump: std::sync::Mutex<RuntimeCloseCleanupPump<PA>>,
    pub status: AtomicU8,
    pub deadline_resume: AtomicU8,
    pub deadline_elapsed_us: AtomicU64,
    pub stalled_steps: AtomicU8,
    /// ⏱️ Monotonic instant the current zero-progress run began (`0` = the ladder is moving).
    pub stall_since_us: AtomicU64,
    /// ⏱️ Stall credit actually spent when a `ZeroProgress` verdict was stored, so the emitted
    /// fault quotes the measured stall instead of the last turn's unrelated cost.
    pub stall_credit_spent_us: AtomicU64,
    pub last_callback_elapsed_us: AtomicU64,
    #[cfg(test)]
    pub last_fault: std::sync::Mutex<[u8; 256]>,
    #[cfg(test)]
    pub last_fault_origin: AtomicU8,
    #[cfg(test)]
    pub physical_close_calls: AtomicU64,
    #[cfg(test)]
    pub callback_phase_started_us: AtomicU64,
    #[cfg(test)]
    pub callback_phase_us: [AtomicU64; 24],
}
pub(super) struct RuntimeCloseEntry<PA: PluginApp> {
    pub state: std::sync::Arc<RuntimeCloseWorkerState<PA>>,
}
#[doc(hidden)]
pub mod instance_lifetime {
    include!("../../../../../🚪️lifetime/🦀️.rs");
}
pub use crate::life_instance_registry::instance_lifetime::{plugin_capture_instance_close, PluginInstanceCloseLease};
#[doc(hidden)]
pub const PLUGIN_RUNTIME_INSTANCE_SLOTS: usize = 1_024;
const PLUGIN_RUNTIME_INSTANCE_WORDS: usize = PLUGIN_RUNTIME_INSTANCE_SLOTS / u64::BITS as usize;
#[doc(hidden)]
pub const PLUGIN_RUNTIME_ACTOR_BYTES: usize = 4_096;
#[doc(hidden)]
pub struct RuntimeActorAuthority {
    pub len: u16,
    pub bytes: [u8; PLUGIN_RUNTIME_ACTOR_BYTES],
}
impl RuntimeActorAuthority {
    pub fn new(actor: &str) -> Result<Self, Fault> {
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
#[doc(hidden)]
pub struct RuntimeInstanceRegistry<T> {
    pub slots: Box<[std::mem::MaybeUninit<(u32, T)>]>,
    pub occupied: [u64; PLUGIN_RUNTIME_INSTANCE_WORDS],
    pub allocation_admitted: bool,
}
impl<T> RuntimeInstanceRegistry<T> {
    /// 🈳️ Constructs the registry EMPTY — no slot backing at all. Allocator success is
    /// not an admission: a runtime carries four of these, and the actor one alone is
    /// `PLUGIN_RUNTIME_INSTANCE_SLOTS` × `RuntimeActorAuthority` ≈ 4 MiB, which an actor that
    /// never opens an instance would otherwise reserve contiguously up front — the exact shape
    /// a guest's `dlmalloc` never gives back. Backing is taken by {@link admit_backing}, on the
    /// first admission that actually needs a slot.
    pub fn new() -> Self {
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
    pub fn index(instance_id: u32) -> usize {
        instance_id as usize % PLUGIN_RUNTIME_INSTANCE_SLOTS
    }
    pub fn occupied(&self, index: usize) -> bool {
        self.occupied[index / u64::BITS as usize] & (1u64 << (index % u64::BITS as usize)) != 0
    }
    pub fn set_occupied(&mut self, index: usize, occupied: bool) {
        let word = &mut self.occupied[index / u64::BITS as usize];
        let mask = 1u64 << (index % u64::BITS as usize);
        if occupied {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }
    /// 📎️ SAFETY: occupancy is set only after `write` and cleared before `assume_init_read`.
    pub fn entry(&self, index: usize) -> Option<&(u32, T)> {
        if !self.occupied(index) {
            return None;
        }
        self.slots.get(index).map(|slot| unsafe { slot.assume_init_ref() })
    }
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn insert(&mut self, instance_id: u32, value: T) -> Result<(), T> {
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
    pub fn insert_admitted(&mut self, instance_id: u32, value: T) {
        let index = Self::index(instance_id);
        let admitted = self.admit_backing();
        debug_assert!(admitted && !self.occupied(index));
        self.slots[index].write((instance_id, value));
        self.set_occupied(index, true);
    }
    pub fn get(&self, instance_id: u32) -> Option<&T> {
        self.entry(Self::index(instance_id)).and_then(|(candidate, value)| (*candidate == instance_id).then_some(value))
    }
    pub fn get_mut(&mut self, instance_id: u32) -> Option<&mut T> {
        let index = Self::index(instance_id);
        if self.entry(index).is_none_or(|(candidate, _)| *candidate != instance_id) {
            return None;
        }
        Some(unsafe { &mut self.slots[index].assume_init_mut().1 })
    }
    /// 🚦️ The admission gate. It takes the backing itself (see {@link admit_backing}) so
    /// a preflight that answers `true` is a promise the following {@link insert_admitted} can
    /// keep, and a refused reservation is an ordinary "cannot insert" instead of a panic.
    pub fn can_insert(&mut self, instance_id: u32) -> bool {
        self.admit_backing() && !self.occupied(Self::index(instance_id))
    }
    /// 🫴️ SAFETY: exact identity and occupancy were checked, and occupancy is now cleared.
    pub fn take(&mut self, instance_id: u32) -> Option<T> {
        let index = Self::index(instance_id);
        if self.entry(index).is_none_or(|(candidate, _)| *candidate != instance_id) {
            return None;
        }
        self.set_occupied(index, false);
        Some(unsafe { self.slots[index].assume_init_read().1 })
    }
    pub fn entry_at(&self, index: usize) -> Option<(u32, &T)> {
        self.entry(index).map(|(instance_id, value)| (*instance_id, value))
    }
    pub fn next_entry_from(&self, start: usize) -> Option<(usize, u32, &T)> {
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
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn is_empty(&self) -> bool {
        self.occupied.iter().all(|word| *word == 0)
    }
}
impl<T> Drop for RuntimeInstanceRegistry<T> {
    fn drop(&mut self) {}
}
#[doc(hidden)]
pub struct RuntimeDocumentBackboneBindingV1 {
    pub generation: u64,
    pub uri: Option<String>,
    pub channel: Option<store::os_store::backbone::ActorBackboneChannelOwner>,
}
impl RuntimeDocumentBackboneBindingV1 {
    pub fn state(&self) -> semio_framework_plugin_actor_host::document_backbone_binding::DocumentBackboneBindingStateV1 {
        semio_framework_plugin_actor_host::document_backbone_binding::DocumentBackboneBindingStateV1 { generation: self.generation, uri: self.uri.clone() }
    }
}
#[cfg(test)]
include!("../../../../../🧪️tests/🔬️plugin-runtime-native-aggregate-backing/🦀️.rs");
#[cfg(test)]
include!("../../../../../🧪️tests/🔬️plugin-runtime-runtime-instance-registry/🦀️.rs");
/// 🧩️ Typed state owned by each embedding plugin's `plugin_exports!` expansion.
pub struct PluginRuntime<PA: PluginApp> {
    pub mounted_policy: MountedOwnerPolicyV1,
    pub guest_lifetimes: RefCell<crate::instance_lifetime::NativeLifecycleRegistry<PA>>,
    pub plugin: RefCell<Option<Plugin<PA>>>,
    pub plugin_assembly_error: RefCell<Option<Fault>>,
    pub instances: RefCell<RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>>,
    pub close_quarantine: RefCell<RuntimeInstanceRegistry<RuntimeCloseEntry<PA>>>,
    pub close_cleanup_cursor: Cell<usize>,
    pub live_cleanup_cursor: Cell<usize>,
    /// 🌡️ Whether the instance the last [`plugin_step_live_cleanup`] stepped reported maintenance pressure.
    pub live_cleanup_pressure: Cell<bool>,
    pub typed_continuation_cursor: Cell<usize>,
    pub close_generation: Cell<u64>,
    pub local_interaction_query_generation: semio_framework_plugin_interaction::live::LocalInteractionQueryGeneration,
    pub instance_actors: RefCell<RuntimeInstanceRegistry<RuntimeActorAuthority>>,
    pub document_backbones: RefCell<RuntimeInstanceRegistry<RuntimeDocumentBackboneBindingV1>>,
}
impl<PA: PluginApp> PluginRuntime<PA> {
    /// 🏇️ Borrows the embedding owner's unchanged phase authority for each real reactor turn.
    pub fn mounted_owner_policy(&self) -> MountedOwnerPolicyV1 { self.mounted_policy }
    /// 🧩️ Creates an empty typed runtime without erasing the plugin-owned app enum.
    pub fn new(mounted_policy: MountedOwnerPolicyV1) -> Result<Self, semio_framework_value::ValueError> {
        let mounted_policy = mounted_policy.validate()?;
        Ok(Self {
            mounted_policy,
            guest_lifetimes: RefCell::new(crate::instance_lifetime::NativeLifecycleRegistry::new()),
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

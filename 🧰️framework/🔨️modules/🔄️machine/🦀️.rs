//! 🎭️ Statechart kernel, actor runtime and hosts — XState-parity FSMs as static tables.
//!
//! Machine structure is compile-time (dense static tables addressed by `NodeId`s);
//! machine state and actor instances are runtime. See [`Machine`] and [`statechart!`].

#[path = "🌐️host/🦀️.rs"]
mod host;
#[path = "🔎️inspect/🦀️.rs"]
mod inspect;
#[path = "🧠️kernel/🦀️.rs"]
mod kernel;
#[path = "💾️persist/🦀️.rs"]
mod persist;
#[path = "🎬️runtime/🦀️.rs"]
mod runtime;
#[path = "🔁️step/🦀️.rs"]
mod step;
#[cfg(test)]
#[path = "🧭️testing/🦀️.rs"]
mod testing;

//#region 🔖️Ids

/// 🗂️ Dense index of a state node within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u16);

/// 📇️ Dense index of an event kind within a [`StatechartEvent`] enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId(pub u16);

/// 🔨️ Dense index of a transition within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransitionId(pub u16);

/// 🔒️ Dense index of a guard function within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuardId(pub u16);

/// 🎬️ Dense index of a reducer/emitter action within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionId(pub u16);

/// 🆔️ Dense index of an `invoke` declaration within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvokeId(pub u16);

/// ⏰️ Dense index of an `after` (delayed transition) timer within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerId(pub u16);

/// ⚙️ Runtime-assigned identity of a spawned actor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActorId(pub u32);

/// 🧮️ Hand-rolled fixed-size bitset over `W` `u64` words — one bit per [`NodeId`].
///
/// The `statechart!` macro emits a concrete `BitSet<W>` sized to the machine's
/// state count; the kernel only ever operates on it through [`Configuration`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BitSet<const W: usize> {
    words: [u64; W],
}

impl<const W: usize> BitSet<W> {
    /// 🪫️ An empty bitset with no bits set.
    pub const fn empty() -> Self {
        Self { words: [0u64; W] }
    }
}

impl<const W: usize> Default for BitSet<W> {
    fn default() -> Self {
        Self::empty()
    }
}

/// 🧩️ Active-state configuration operations, implemented by the macro-generated `BitSet<W>`.
///
/// The active configuration of a running machine is a set of atomic `NodeId`s —
/// never a single enum variant — so that parallel regions can hold multiple
/// simultaneously-active states.
pub trait Configuration: Clone + PartialEq + Default {
    /// ➕️ Marks `id` as part of the active configuration.
    fn set(&mut self, id: NodeId);
    /// ➖️ Removes `id` from the active configuration.
    fn clear(&mut self, id: NodeId);
    /// ❓️ Whether `id` is part of the active configuration.
    fn contains(&self, id: NodeId) -> bool;
    /// 🔁️ Iterates active `NodeId`s in ascending order.
    fn iter_ones(&self) -> ConfigurationIter<'_, Self>;
    /// 🧹️ Clears every bit.
    fn clear_all(&mut self);
    /// 🕳️ Whether no bit is set.
    fn is_empty(&self) -> bool;
}

/// 🎚️ Iterator over the active `NodeId`s of a [`Configuration`].
pub struct ConfigurationIter<'a, C: Configuration> {
    words: &'a [u64],
    word_index: usize,
    current: u64,
    _marker: core::marker::PhantomData<C>,
}

impl<'a, C: Configuration> Iterator for ConfigurationIter<'a, C> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        loop {
            if self.current != 0 {
                let bit = self.current.trailing_zeros();
                self.current &= self.current - 1;
                return Some(NodeId((self.word_index as u16 - 1) * 64 + bit as u16));
            }
            self.word_index += 1;
            self.current = *self.words.get(self.word_index - 1)?;
        }
    }
}

impl<const W: usize> Configuration for BitSet<W> {
    fn set(&mut self, id: NodeId) {
        let (word, bit) = (id.0 as usize / 64, id.0 as usize % 64);
        self.words[word] |= 1u64 << bit;
    }

    fn clear(&mut self, id: NodeId) {
        let (word, bit) = (id.0 as usize / 64, id.0 as usize % 64);
        self.words[word] &= !(1u64 << bit);
    }

    fn contains(&self, id: NodeId) -> bool {
        let (word, bit) = (id.0 as usize / 64, id.0 as usize % 64);
        self.words[word] & (1u64 << bit) != 0
    }

    fn iter_ones(&self) -> ConfigurationIter<'_, Self> {
        ConfigurationIter { words: &self.words, word_index: 0, current: 0, _marker: core::marker::PhantomData }
    }

    fn clear_all(&mut self) {
        self.words = [0u64; W];
    }

    fn is_empty(&self) -> bool {
        self.words.iter().all(|w| *w == 0)
    }
}

//#endregion 🔖️Ids

//#region 🔖️Schema

/// 🏷️ A consumer-defined event enum, reflected into a dense [`EventId`] space.
///
/// Implemented via `#[derive(StatechartEvent)]` — see `machine_derive`.
pub trait StatechartEvent: Clone {
    /// 🔢️ Number of distinct event variants.
    const EVENT_COUNT: u16;
    /// 🪪️ The dense id of this event's variant.
    fn event_id(&self) -> EventId;
    /// 🩺️ The variant's declared name, for diagnostics/inspection/manifest.
    fn event_name(id: EventId) -> &'static str;
}

/// 🖥️ A compiled statechart: consumer-owned types bound to a static [`MachineDefinition`].
///
/// Implemented by the marker type generated by `statechart! { machine name { … } }`.
pub trait Machine: Sized + 'static {
    /// 📦️ Consumer-owned, mutable data carried alongside the active configuration.
    type Context;
    /// 📨️ Consumer-owned event enum, reflected via [`StatechartEvent`].
    type Event: StatechartEvent;
    /// 📥️ Consumer-owned input to [`kernel::init`].
    type Input;
    /// 📤️ Consumer-owned output produced when the machine reaches a final state.
    type Output: Clone;
    /// 🎇️ Consumer-owned effect payloads requested via [`kernel::Command::Effect`].
    type Effect;
    /// 🎛️ The macro-sized `BitSet<W>` for this machine's state count.
    type Config: Configuration;

    /// 📐️ The compiled static definition backing this machine.
    fn definition() -> &'static MachineDefinition<Self>;
}

/// 🧬️ Struct-level field name/type metadata, embedded as JSON — feeds TypeScript
/// generation tooling. Implemented via `#[derive(StatechartSchema)]`.
pub trait StatechartSchema {
    /// 🔣️ `{"fields":[{"name":"..","type":".."}, ..]}` for this struct's named fields.
    const SCHEMA_JSON: &'static str;
}

//#endregion 🔖️Schema

//#region 🔖️Reexports

pub use host::{Host, NativeHost, TestHost};
pub use inspect::{InspectionEvent, Inspector, MicrostepTrace, NullInspector, TraceInspector};
pub use kernel::{init, macrostep, timer_elapsed, ActionFn, Command, GuardFn, InputFn, MachineDefinition, NodeDef, NodeKind, OutputFn, Snapshot, Status, StepReport, TransitionDef, TransitionKind, Trigger, MICROSTEP_LIMIT, ROOT};
pub use persist::{persist, restore, Migration, NoMigrations, PersistedSnapshot, RestoreError};
pub use runtime::{route_command, ActorLogic, ActorSystem, MachineLogic};
pub use step::{start, step, MachineStep};

#[cfg(test)]
pub use testing::{check_invariants, explore, run_conformance, ConformanceStep, Coverage, FsmError, Invariant, Model};

// 🪄️ `StatechartEvent` here re-exports the derive macro — it shares its name with the
// `StatechartEvent` trait above without conflict since macros and traits live in
// separate namespaces (the same pattern `serde`/`serde_derive` use).
pub use machine_derive::{statechart, StatechartEvent, StatechartSchema};

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use machine_derive::export_wasm_machine;

//#endregion 🔖️Reexports

//#region 🔖️WasmBridge

// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this bridge is browser-only
// (JS effect callback + JS Date clock) — wasip2 already has `NativeHost` (std::time::Instant,
// which WASI's clock backs) so it needs no bridge of its own here.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
mod wasm_bridge {
    use crate::{ActorId, Host, InvokeId, Machine, TimerId};

    /// 🌐️ A [`Host`] backed by browser timers and a JS effect callback. Timers are
    /// polled by the caller via [`WasmHost::due_timers`] (driven by the generated
    /// `export_wasm_machine!` wrapper's `tick()`), matching [`crate::NativeHost`] and
    /// [`crate::TestHost`]'s caller-driven clock — no `async fn` anywhere in the host.
    pub struct WasmHost<M: Machine> {
        effect_callback: Option<js_sys::Function>,
        pending_timers: Vec<(ActorId, TimerId, f64)>,
        started_tasks: Vec<(ActorId, InvokeId)>,
        _marker: core::marker::PhantomData<M>,
    }

    impl<M: Machine> WasmHost<M> {
        /// 🆕️ A fresh host with no effect callback registered yet.
        pub fn new() -> Self {
            Self { effect_callback: None, pending_timers: Vec::new(), started_tasks: Vec::new(), _marker: core::marker::PhantomData }
        }

        /// 📞️ Registers the JS function invoked as `(actorId: number, effectJson: string) => void`.
        pub fn set_effect_callback(&mut self, callback: js_sys::Function) {
            self.effect_callback = Some(callback);
        }

        /// 🚀️ Tasks started via `invoke`, still pending cancellation.
        pub fn started_tasks(&self) -> &[(ActorId, InvokeId)] {
            &self.started_tasks
        }

        /// ⏱️ Removes and returns every timer whose deadline has passed, per `js_sys::Date::now()`.
        pub fn due_timers(&mut self) -> Vec<(ActorId, TimerId)> {
            let now = js_sys::Date::now();
            let mut due = Vec::new();
            self.pending_timers.retain(|(actor, timer, at)| {
                if *at <= now {
                    due.push((*actor, *timer));
                    false
                } else {
                    true
                }
            });
            due
        }
    }

    impl<M: Machine> Default for WasmHost<M> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<M: Machine> Host<M> for WasmHost<M>
    where
        M::Effect: serde::Serialize,
    {
        fn execute_effect(&mut self, actor: ActorId, effect: M::Effect) {
            if let Some(callback) = &self.effect_callback {
                if let Ok(json) = serde_json::to_string(&effect) {
                    let _ = callback.call2(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_f64(actor.0 as f64), &wasm_bindgen::JsValue::from_str(&json));
                }
            }
        }

        fn schedule(&mut self, actor: ActorId, timer: TimerId, delay_ms: u64) {
            self.pending_timers.push((actor, timer, js_sys::Date::now() + delay_ms as f64));
        }

        fn cancel_timer(&mut self, actor: ActorId, timer: TimerId) {
            self.pending_timers.retain(|(a, t, _)| !(*a == actor && *t == timer));
        }

        fn start_task(&mut self, actor: ActorId, invoke: InvokeId) {
            self.started_tasks.push((actor, invoke));
        }

        fn cancel_task(&mut self, actor: ActorId, invoke: InvokeId) {
            self.started_tasks.retain(|(a, i)| !(*a == actor && *i == invoke));
        }

        fn now_ms(&self) -> u64 {
            js_sys::Date::now() as u64
        }
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use wasm_bridge::WasmHost;

/// 🎫️ A minimal always-compiled `export_wasm_machine!` consumer — proves the wasm
/// target and the whole DSL → kernel → `WasmHost` → JSON-boundary path continuously,
/// independent of any downstream consumer crate.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
mod wasm_smoke {
    #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
    pub struct ToggleContext;

    fn build_context(_input: ()) -> ToggleContext {
        ToggleContext
    }

    crate::statechart! {
        machine toggle {
            context: ToggleContext;
            event Event: serde { Flip }
            input: ();
            output: ();
            effect: ();
            context_from_input: build_context;
            initial: off;
            state off {
                on Flip => on;
            }
            state on {
                on Flip => off;
            }
        }
    }

    crate::export_wasm_machine!(toggle::Toggle, "ToggleMachine");
}

//#endregion 🔖️WasmBridge

//#region 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

/// ⏹️ End-to-end proof that `statechart!` DSL → kernel → runtime → [`TestHost`] timers/invoke
/// → persist/restore → inspection trace → model coverage all semio_compose_rs over one real machine.
#[cfg(test)]
#[path = "🧪️tests/🔬️checkout-integration/🦀️.rs"]
mod checkout_integration;

//#endregion 🧪️Tests

impl semio_framework_value::retirement::RetireOwned for NodeId{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::leaf(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::leaf_birth_bytes::<Self>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{std::mem::size_of::<Self>()}
}
impl<O:semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for Status<O>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{match self{Self::Done(output)=>output.retirement(),Self::Running|Self::Stopped=>semio_framework_value::retirement::leaf(())}}
 fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Done(output)=>output.retirement_birth_bytes(),Self::Running|Self::Stopped=>Some(semio_framework_value::retirement::leaf_birth_bytes::<()>())}}
 fn controlled_retirement_supported()->bool{O::controlled_retirement_supported()}
}
impl<M:Machine+'static> semio_framework_value::retirement::RetireOwned for Snapshot<M>
where M::Config:Copy+Send+'static,M::Context:semio_framework_value::retirement::RetireOwned,M::Output:semio_framework_value::retirement::RetireOwned{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{let Self{context,status,history,..}=self;semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(context),semio_framework_value::retirement::deferred(status),semio_framework_value::retirement::deferred(history)])}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.context),deferred_birth_bytes_for(&self.status),deferred_birth_bytes_for(&self.history)])}
 fn controlled_retirement_supported()->bool{M::Context::controlled_retirement_supported()&&M::Output::controlled_retirement_supported()}
}

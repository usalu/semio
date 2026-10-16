//! 🌐️ Host abstraction — hosts execute the commands the kernel only describes.

use crate::{ActorId, InvokeId, Machine, TimerId};

//#region 🔖️Host

/// 🌐️ Executes the side effects a [`crate::Command`] describes. No `async fn` —
/// hosts own their own tasks/timers and report completion back as ordinary events.
pub trait Host<M: Machine> {
    /// 🎇️ Executes a consumer-defined effect requested by a running actor.
    fn execute_effect(&mut self, actor: ActorId, effect: M::Effect);
    /// ⏱️ Schedules a delayed-transition timer for the given actor.
    fn schedule(&mut self, actor: ActorId, timer: TimerId, delay_ms: u64);
    /// ⏱️ Cancels a previously scheduled timer (invoked when its owning state exits).
    fn cancel_timer(&mut self, actor: ActorId, timer: TimerId);
    /// 🚀️ Starts the task/actor backing an `invoke` declaration.
    fn start_task(&mut self, actor: ActorId, invoke: InvokeId);
    /// 🛑️ Stops a previously started task (invoked when its owning state exits).
    fn cancel_task(&mut self, actor: ActorId, invoke: InvokeId);
    /// 🕰️ The host's current clock reading, in milliseconds.
    fn now_ms(&self) -> u64;
}

//#endregion 🔖️Host

//#region 🔖️NativeHost

/// 🖥️ A synchronous, wall-clock-backed [`Host`] for native (non-WASM) targets.
///
/// Timers are polled by the caller via [`NativeHost::due_timers`] rather than
/// firing on their own thread — keeping the whole runtime single-threaded per actor.
pub struct NativeHost<M: Machine> {
    start: std::time::Instant,
    effects: Vec<(ActorId, M::Effect)>,
    pending_timers: Vec<(ActorId, TimerId, u64)>,
    started_tasks: Vec<(ActorId, InvokeId)>,
    cancelled_tasks: Vec<(ActorId, InvokeId)>,
}

impl<M: Machine> NativeHost<M> {
    /// 🖥️ A fresh host whose clock starts at zero.
    pub fn new() -> Self {
        Self { start: std::time::Instant::now(), effects: Vec::new(), pending_timers: Vec::new(), started_tasks: Vec::new(), cancelled_tasks: Vec::new() }
    }

    /// 🎇️ Effects recorded so far, in emission order.
    pub fn effects(&self) -> &[(ActorId, M::Effect)] {
        &self.effects
    }

    /// 🎇️ Drains and returns every recorded effect.
    pub fn drain_effects(&mut self) -> Vec<(ActorId, M::Effect)> {
        core::mem::take(&mut self.effects)
    }

    /// 🚀️ Tasks started via `invoke`, still pending cancellation.
    pub fn started_tasks(&self) -> &[(ActorId, InvokeId)] {
        &self.started_tasks
    }

    /// ⏱️ Removes and returns every timer whose deadline has passed.
    pub fn due_timers(&mut self) -> Vec<(ActorId, TimerId)> {
        let now = self.now_ms();
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

impl<M: Machine> Default for NativeHost<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: Machine> Host<M> for NativeHost<M> {
    fn execute_effect(&mut self, actor: ActorId, effect: M::Effect) {
        self.effects.push((actor, effect));
    }

    fn schedule(&mut self, actor: ActorId, timer: TimerId, delay_ms: u64) {
        let at = self.now_ms() + delay_ms;
        self.pending_timers.push((actor, timer, at));
    }

    fn cancel_timer(&mut self, actor: ActorId, timer: TimerId) {
        self.pending_timers.retain(|(a, t, _)| !(*a == actor && *t == timer));
    }

    fn start_task(&mut self, actor: ActorId, invoke: InvokeId) {
        self.started_tasks.push((actor, invoke));
    }

    fn cancel_task(&mut self, actor: ActorId, invoke: InvokeId) {
        self.started_tasks.retain(|(a, i)| !(*a == actor && *i == invoke));
        self.cancelled_tasks.push((actor, invoke));
    }

    fn now_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}

//#endregion 🔖️NativeHost

//#region 🔖️TestHost

/// 🧪️ A [`Host`] with a caller-driven simulated clock — never sleeps in real time.
pub struct TestHost<M: Machine> {
    clock_ms: u64,
    effects: Vec<(ActorId, M::Effect)>,
    pending_timers: Vec<(ActorId, TimerId, u64)>,
    started_tasks: Vec<(ActorId, InvokeId)>,
    cancelled_tasks: Vec<(ActorId, InvokeId)>,
}

impl<M: Machine> TestHost<M> {
    /// 🧪️ A fresh simulated host whose clock starts at zero.
    pub fn new() -> Self {
        Self { clock_ms: 0, effects: Vec::new(), pending_timers: Vec::new(), started_tasks: Vec::new(), cancelled_tasks: Vec::new() }
    }

    /// 🎇️ Effects recorded so far, in emission order.
    pub fn effects(&self) -> &[(ActorId, M::Effect)] {
        &self.effects
    }

    /// 🚀️ Tasks currently started (not yet cancelled), for invoke-lifecycle assertions.
    pub fn started_tasks(&self) -> &[(ActorId, InvokeId)] {
        &self.started_tasks
    }

    /// 🛑️ Tasks that have been cancelled, for invoke-lifecycle assertions.
    pub fn cancelled_tasks(&self) -> &[(ActorId, InvokeId)] {
        &self.cancelled_tasks
    }

    /// ⏱️ Advances the simulated clock and returns timers that became due, removing them.
    pub fn advance(&mut self, delay_ms: u64) -> Vec<(ActorId, TimerId)> {
        self.clock_ms += delay_ms;
        let now = self.clock_ms;
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

impl<M: Machine> Default for TestHost<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: Machine> Host<M> for TestHost<M> {
    fn execute_effect(&mut self, actor: ActorId, effect: M::Effect) {
        self.effects.push((actor, effect));
    }

    fn schedule(&mut self, actor: ActorId, timer: TimerId, delay_ms: u64) {
        self.pending_timers.push((actor, timer, self.clock_ms + delay_ms));
    }

    fn cancel_timer(&mut self, actor: ActorId, timer: TimerId) {
        self.pending_timers.retain(|(a, t, _)| !(*a == actor && *t == timer));
    }

    fn start_task(&mut self, actor: ActorId, invoke: InvokeId) {
        self.started_tasks.push((actor, invoke));
    }

    fn cancel_task(&mut self, actor: ActorId, invoke: InvokeId) {
        self.started_tasks.retain(|(a, i)| !(*a == actor && *i == invoke));
        self.cancelled_tasks.push((actor, invoke));
    }

    fn now_ms(&self) -> u64 {
        self.clock_ms
    }
}

//#endregion 🔖️TestHost

//#region 🧪️Tests

#[cfg(test)]
include!("../🧪️tests/🔬️host-unit/🦀️.rs");

//#endregion 🧪️Tests

//! ⏲️ The single host timer wheel: pure `WheelCore` arithmetic plus the finite driver chain that posts firings to a `CompletionSink`.

use semio_framework_actor::PackageId;
use semio_framework_async::{Lane, WorkerPool};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use super::completion_sink::CompletionSink;

//#region ⏲️TimerWheel
/// 🔖️ Identity of one armed timer, minted by [`WheelCore::arm`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimerId(u64);

/// 🚫️ [`WheelCore::arm`]'s only failure mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimerError {
    QuotaExceeded { plugin: PackageId, limit: u32 },
}

impl std::fmt::Display for TimerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerError::QuotaExceeded { plugin, limit } => write!(f, "plugin {:?} already has {limit} timers armed", plugin.0),
        }
    }
}
impl std::error::Error for TimerError {}

/// 🔔️ What fired: enough to route a [`CompletionSink::complete`] call to the right actor without
/// `WheelCore` needing to know anything about actors beyond these four bare fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimerFired {
    pub id: TimerId,
    pub plugin: PackageId,
    pub actor: u64,
    pub generation: u16,
    pub lane: u8,
}

struct TimerEntry {
    pub(super) plugin: PackageId,
    pub(super) actor: u64,
    pub(super) generation: u16,
    pub(super) lane: u8,
    expiry_ms: u64,
    repeat_ms: Option<u64>,
    pub(super) cancelled: bool,
}

/// ⏲️ Pure timer-wheel arithmetic: no tokio, no clock of its own — every `now_ms` is caller-supplied
/// — and unit-testable directly with no runtime at all. Exactly ONE of these exists per host process
/// (owned by [`TimerWheel`]); plugins declare timers through it and never own a timer of their own.
pub struct WheelCore {
    pub(super) next_id: u64,
    next_seq: u64,
    entries: HashMap<TimerId, TimerEntry>,
    pub(super) order: BinaryHeap<Reverse<(u64, u64, TimerId)>>,
    per_plugin_counts: HashMap<PackageId, u32>,
    quota_per_plugin: u32,
}

impl WheelCore {
    pub async fn new(quota_per_plugin: u32) -> WheelCore {
        WheelCore { next_id: 1, next_seq: 0, entries: HashMap::new(), order: BinaryHeap::new(), per_plugin_counts: HashMap::new(), quota_per_plugin }
    }

    /// ⏲️ Arms a timer for `plugin`/`actor` firing at `at_ms`, optionally repeating every
    /// `repeat_ms`. The per-plugin quota is checked BEFORE any insertion, so a rejected call leaves
    /// the wheel completely untouched.
    #[allow(clippy::too_many_arguments)]
    pub fn arm(&mut self, plugin: PackageId, actor: u64, generation: u16, lane: u8, at_ms: u64, repeat_ms: Option<u64>) -> Result<TimerId, TimerError> {
        let count = self.per_plugin_counts.get(&plugin).copied().unwrap_or(0);
        if count >= self.quota_per_plugin {
            return Err(TimerError::QuotaExceeded { plugin, limit: self.quota_per_plugin });
        }
        let id = TimerId(self.next_id);
        self.next_id += 1;
        let seq = self.next_seq;
        self.next_seq += 1;
        self.entries.insert(id, TimerEntry { plugin: plugin.clone(), actor, generation, lane, expiry_ms: at_ms, repeat_ms, cancelled: false });
        self.order.push(Reverse((at_ms, seq, id)));
        *self.per_plugin_counts.entry(plugin).or_insert(0) += 1;
        Ok(id)
    }

    /// 🔕️ Disarms `id`, returning whether it was actually armed. Lazy: the heap entry is left in
    /// place and skipped when eventually popped by [`WheelCore::pop_expired`], since a `BinaryHeap`
    /// cannot remove an arbitrary interior element faster than O(n).
    pub fn disarm(&mut self, id: TimerId) -> bool {
        match self.entries.get_mut(&id) {
            Some(entry) if !entry.cancelled => {
                entry.cancelled = true;
                if let Some(count) = self.per_plugin_counts.get_mut(&entry.plugin) {
                    *count = count.saturating_sub(1);
                }
                true
            }
            _ => false,
        }
    }

    /// 🔔️ Pops every timer whose expiry is `<= now_ms`, rearming repeaters at `expiry + repeat_ms`
    /// and catching up (rather than drifting) if more than one repeat period has actually elapsed.
    /// Cancelled entries are dropped here rather than rearmed — this is where lazy
    /// [`WheelCore::disarm`] deletion actually reclaims heap/map space.
    pub fn pop_expired(&mut self, now_ms: u64) -> Vec<TimerFired> {
        self.pop_expired_batch(now_ms, usize::MAX)
    }

    /// ⏱️ Pops at most `max_items` due timers so one driver turn has a hard item bound.
    pub fn pop_expired_batch(&mut self, now_ms: u64, max_items: usize) -> Vec<TimerFired> {
        let mut fired = Vec::new();
        let mut processed_items = 0usize;
        while processed_items < max_items {
            let Some(&Reverse((expiry, _seq, id))) = self.order.peek() else { break };
            if expiry > now_ms {
                break;
            }
            self.order.pop();
            processed_items += 1;
            let Some(entry) = self.entries.get_mut(&id) else { continue };
            if entry.cancelled {
                self.entries.remove(&id);
                continue;
            }
            fired.push(TimerFired { id, plugin: entry.plugin.clone(), actor: entry.actor, generation: entry.generation, lane: entry.lane });
            match entry.repeat_ms {
                Some(repeat) if repeat > 0 => {
                    let mut next_expiry = entry.expiry_ms + repeat;
                    while next_expiry <= now_ms {
                        next_expiry += repeat;
                    }
                    entry.expiry_ms = next_expiry;
                    let seq = self.next_seq;
                    self.next_seq += 1;
                    self.order.push(Reverse((next_expiry, seq, id)));
                }
                _ => {
                    let plugin = entry.plugin.clone();
                    self.entries.remove(&id);
                    if let Some(count) = self.per_plugin_counts.get_mut(&plugin) {
                        *count = count.saturating_sub(1);
                    }
                }
            }
        }
        fired
    }

    /// ⏰️ The earliest heaped expiry. A cancelled head may cause one harmless early driver turn;
    /// bounded expiry processing discards it without an unbounded scan here.
    pub fn next_expiry_ms(&self) -> Option<u64> {
        self.order.peek().map(|Reverse((expiry, _, _))| *expiry)
    }

    pub fn armed_count(&self, plugin: &PackageId) -> u32 {
        self.per_plugin_counts.get(plugin).copied().unwrap_or(0)
    }
}

/// 🐌️ Maximum delay between finite timer-driver turns while the wheel is empty.
const TIMER_DRIVER_MAX_IDLE_MS: u64 = 4;
const TIMER_DRIVER_BATCH_ITEMS: usize = 8;

/// ⏲️ The ONE host timer wheel for every plugin's timers — see the crate doc. Owns a [`WheelCore`]
/// behind a `Mutex` plus the finite driver chain ([`TimerWheel::spawn_driver`]) that posts
/// firings to a [`CompletionSink`]. Plugins arm/disarm through here; they must never spin up a timer
/// of their own.
pub struct TimerWheel {
    core: Arc<Mutex<WheelCore>>,
    driver_started: AtomicBool,
}

impl TimerWheel {
    pub async fn new(quota_per_plugin: u32) -> TimerWheel {
        TimerWheel { core: Arc::new(Mutex::new(WheelCore::new(quota_per_plugin).await)), driver_started: AtomicBool::new(false) }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn arm(&self, plugin: PackageId, actor: u64, generation: u16, lane: u8, at_ms: u64, repeat_ms: Option<u64>) -> Result<TimerId, TimerError> {
        let id = self.core.lock().expect("TimerWheel core mutex poisoned").arm(plugin, actor, generation, lane, at_ms, repeat_ms)?;
        Ok(id)
    }

    pub async fn disarm(&self, id: TimerId) -> bool {
        self.core.lock().expect("TimerWheel core mutex poisoned").disarm(id)
    }

    pub async fn armed_count(&self, plugin: &PackageId) -> u32 {
        self.core.lock().expect("TimerWheel core mutex poisoned").armed_count(plugin)
    }

    /// ▶️ Starts a chain of finite timer-lane turns. Each turn handles at most
    /// [`TIMER_DRIVER_BATCH_ITEMS`] firings and schedules its successor without occupying a worker
    /// while it waits. The four-millisecond idle cadence bounds newly-armed-timer latency without a
    /// permanent polling task.
    pub fn spawn_driver(&self, pool: &WorkerPool, sink: Arc<dyn CompletionSink>) {
        if self.driver_started.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            schedule_timer_driver_turn(pool, self.core.clone(), sink);
        }
    }
}

fn schedule_timer_driver_turn(pool: &WorkerPool, core: Arc<Mutex<WheelCore>>, sink: Arc<dyn CompletionSink>) {
    let now_ms = pool.now_ms();
    let deadline_ms = core.lock().expect("TimerWheel core mutex poisoned").next_expiry_ms().unwrap_or(now_ms + TIMER_DRIVER_MAX_IDLE_MS).min(now_ms + TIMER_DRIVER_MAX_IDLE_MS);
    let turn_pool = pool.clone();
    pool.submit_at(
        deadline_ms,
        Lane::Timer,
        Box::new(move || {
            let now_ms = turn_pool.now_ms();
            let fired = core.lock().expect("TimerWheel core mutex poisoned").pop_expired_batch(now_ms, TIMER_DRIVER_BATCH_ITEMS);
            for timer in fired {
                sink.complete(timer.actor, timer.generation, timer.id.0.to_le_bytes().to_vec(), timer.lane);
            }
            schedule_timer_driver_turn(&turn_pool, core, sink);
        }),
    );
}
//#endregion ⏲️TimerWheel

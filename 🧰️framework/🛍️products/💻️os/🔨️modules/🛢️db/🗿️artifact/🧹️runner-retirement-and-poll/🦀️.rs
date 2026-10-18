//! 🧹️ Runner retirement and poll: the runner retirement hooks and slots, the close poll, the runner state, poll and wake. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Actor
/// 📏️ Live document authorities one process holds at once: each reserves a retirement slot when it spawns, so dropping
/// it can always hand its runner to a retirement cursor. A slot is a fixed cell, never a pool maintenance hook — the pool
/// has a few dozen of those in all, and every authority used to take one, which capped a hub at about 60 open documents.
#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_RUNNER_RETIREMENT_SLOTS: usize = 4_096;

/// 🪝️ Worker pools that retire artifact runners at once: each gets ONE maintenance hook, shared by every retirement slot
/// reserved on it, installed with its first reservation and retired with its last.
#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_RUNNER_RETIREMENT_POOLS: usize = 256;

/// 🔑️ One pool's shared retirement hook: its row, the row's generation and the pool's maintenance ticket.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy)]
struct ArtifactRunnerRetirementHookKey {
    pub row: usize,
    pub generation: u64,
    pub(super) ticket: semio_framework_async::WorkerMaintenanceTicket,
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunnerRetirementHook {
    pub generation: u64,
    pub(super) pool: Arc<semio_framework_async::WorkerPool>,
    pub(super) ticket: semio_framework_async::WorkerMaintenanceTicket,
    pub slots: usize,
}

#[cfg(not(target_arch = "wasm32"))]
static ARTIFACT_RUNNER_RETIREMENT_HOOKS: std::sync::Mutex<[Option<ArtifactRunnerRetirementHook>; ARTIFACT_RUNNER_RETIREMENT_POOLS]> = std::sync::Mutex::new([const { None }; ARTIFACT_RUNNER_RETIREMENT_POOLS]);

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunnerRetirementCursor {
    pub generation: u64,
    pub close: Arc<dyn Fn() -> bool + Send + Sync>,
    pub(super) handoff: Arc<ArtifactRunnerHandoff>,
    pub(super) pool: Arc<semio_framework_async::WorkerPool>,
    pub(super) _pool_use: Arc<semio_framework_async::WorkerPoolUse>,
    pub(super) hook: ArtifactRunnerRetirementHookKey,
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) static ARTIFACT_RUNNER_RETIREMENTS: [std::sync::Mutex<Option<ArtifactRunnerRetirementCursor>>; ARTIFACT_RUNNER_RETIREMENT_SLOTS] = [const { std::sync::Mutex::new(None) }; ARTIFACT_RUNNER_RETIREMENT_SLOTS];

/// 🔬️ How many retained artifact-runner retirement cursors still hold the `WorkerPoolUse` clone a
/// non-terminal `ArtifactAuthority::drop` handed them. A database shutdown stuck in its `PoolUse`
/// phase with every other registry empty is blocked by exactly these.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_runner_retirement_live_slots() -> usize {
    ARTIFACT_RUNNER_RETIREMENTS.iter().filter(|slot| slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()).count()
}

/// 📈️ The most advanced faulted-close retry among the dropped authorities retiring on `pool`: a blocked
/// (exhausted or cancelled) owner wins over one still retrying, so a shutdown can report exactly why
/// its pool use is still retained.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_close_retry_progress(pool: &Arc<semio_framework_async::WorkerPool>) -> Option<ArtifactCloseRetryProgress> {
    ARTIFACT_RUNNER_RETIREMENTS
        .iter()
        .filter_map(|slot| slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().filter(|owner| Arc::ptr_eq(&owner.pool, pool)).map(|owner| owner.handoff.clone()))
        .filter_map(|handoff| handoff.close_retry_progress())
        .max_by_key(|progress| (progress.is_blocked(), progress.attempts))
}

/// 🔂️ Re-admits every exhausted or cancelled faulted close retiring on `pool`; returns how many were re-admitted.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_close_retry_readmit(pool: &Arc<semio_framework_async::WorkerPool>) -> usize {
    artifact_close_retry_handoffs(pool).into_iter().filter(|handoff| handoff.readmit_close_retry()).count()
}

/// 🛑️ Cancels every scheduled faulted-close retry retiring on `pool`; returns how many were cancelled.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_close_retry_cancel(pool: &Arc<semio_framework_async::WorkerPool>) -> usize {
    artifact_close_retry_handoffs(pool).into_iter().filter(|handoff| handoff.cancel_close_retry()).count()
}

#[cfg(not(target_arch = "wasm32"))]
fn artifact_close_retry_handoffs(pool: &Arc<semio_framework_async::WorkerPool>) -> Vec<Arc<ArtifactRunnerHandoff>> {
    ARTIFACT_RUNNER_RETIREMENTS.iter().filter_map(|slot| slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().filter(|owner| Arc::ptr_eq(&owner.pool, pool)).map(|owner| owner.handoff.clone())).collect()
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) static ARTIFACT_AUTHORITY_POOL_USES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🔬️ How many LIVE `ArtifactAuthority` owners still hold the `WorkerPoolUse` clone
/// `spawn_with_pool_use` stored in `_pool_use`. A handoff surrenders its own clone at the runner's
/// terminal transition, so `artifact_runner_handoff_pool_use_live_slots() == 0` does NOT exclude a
/// live authority: the authority outlives its runner's terminal transition and keeps a second
/// clone until the authority itself is dropped.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_authority_live_pool_uses() -> usize {
    ARTIFACT_AUTHORITY_POOL_USES.load(std::sync::atomic::Ordering::Acquire)
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) static ARTIFACT_RUNNER_HANDOFF_POOL_USES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🔬️ How many LIVE artifact-runner handoffs still park the `WorkerPoolUse` clone
/// `spawn_with_pool_use` gave them. A handoff surrenders it only at the runner's terminal
/// transition, so an authority already gone from `open_artifacts` — and holding no retirement
/// cursor — can still pin the database's pool use through a runner that never went terminal. This
/// is the family no registry census can see, because handoffs live in their runner, not in a slot
/// table.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_runner_handoff_pool_use_live_slots() -> usize {
    ARTIFACT_RUNNER_HANDOFF_POOL_USES.load(std::sync::atomic::Ordering::Acquire)
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) static ARTIFACT_RUNNER_RETIREMENT_GENERATIONS: [std::sync::atomic::AtomicU64; ARTIFACT_RUNNER_RETIREMENT_SLOTS] = [const { std::sync::atomic::AtomicU64::new(0) }; ARTIFACT_RUNNER_RETIREMENT_SLOTS];

#[cfg(not(target_arch = "wasm32"))]
static ARTIFACT_RUNNER_RETIREMENT_NEXT_GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct ArtifactRunnerRetirementReservation {
    pub index: usize,
    pub generation: u64,
    pub(super) pool: Arc<semio_framework_async::WorkerPool>,
    pub(super) hook: Option<ArtifactRunnerRetirementHookKey>,
}

/// 🪝️ Joins `pool`'s shared retirement hook, installing it for the pool's first reservation.
#[cfg(not(target_arch = "wasm32"))]
fn artifact_runner_retirement_hook_join(pool: &Arc<semio_framework_async::WorkerPool>, generation: u64) -> Result<ArtifactRunnerRetirementHookKey, DbError> {
    let mut hooks = ARTIFACT_RUNNER_RETIREMENT_HOOKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some((row, hook)) = hooks.iter_mut().enumerate().find_map(|(row, hook)| hook.as_mut().filter(|hook| Arc::ptr_eq(&hook.pool, pool)).map(|hook| (row, hook))) {
        hook.slots += 1;
        return Ok(ArtifactRunnerRetirementHookKey { row, generation: hook.generation, ticket: hook.ticket });
    }
    let row = hooks.iter().position(Option::is_none).ok_or_else(|| DbError::Unavailable("artifact runner retirement pool capacity exhausted".to_string()))?;
    let ticket = pool
        .install_maintenance_hook(semio_framework_async::Lane::UserVisible, artifact_runner_retirement_step, [row as u64, generation])
        .map_err(|error| DbError::Unavailable(format!("artifact runner retirement maintenance admission: {error:?}")))?;
    hooks[row] = Some(ArtifactRunnerRetirementHook { generation, pool: pool.clone(), ticket, slots: 1 });
    Ok(ArtifactRunnerRetirementHookKey { row, generation, ticket })
}

/// 🪝️ One retirement slot of the hook's pool was released: the pool's last one removes the hook's row (the hook itself
/// retires on its next turn, or at once when it is not running).
#[cfg(not(target_arch = "wasm32"))]
fn artifact_runner_retirement_hook_leave(key: ArtifactRunnerRetirementHookKey) {
    let mut hooks = ARTIFACT_RUNNER_RETIREMENT_HOOKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(hook) = hooks[key.row].as_mut().filter(|hook| hook.generation == key.generation) else { return };
    hook.slots -= 1;
    if hook.slots != 0 {
        return;
    }
    let retired = hooks[key.row].take().expect("checked artifact runner retirement hook");
    drop(hooks);
    let _ = retired.pool.remove_maintenance_hook(retired.ticket);
}

/// 🚥️ How one retirement slot's turn ended.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArtifactRunnerRetirementTurn {
    Absent,
    Waiting,
    Faulted,
    Retired,
}

/// 🧹️ One turn of a pool's shared retirement hook: every slot of that pool whose authority was dropped before its runner
/// went terminal advances one close step. A reservation whose cursor is not committed yet is skipped — its commit requests
/// the hook again — so an early turn can never strand a cursor; the hook retires once its pool's last slot is released.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn artifact_runner_retirement_step([row, generation]: [u64; 2]) -> semio_framework_async::WorkerMaintenanceStep {
    let Ok(row) = usize::try_from(row) else { return semio_framework_async::WorkerMaintenanceStep::Retire };
    let pool = {
        let hooks = ARTIFACT_RUNNER_RETIREMENT_HOOKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match hooks.get(row).and_then(Option::as_ref) {
            Some(hook) if hook.generation == generation => hook.pool.clone(),
            _ => return semio_framework_async::WorkerMaintenanceStep::Retire,
        }
    };
    let mut faulted = false;
    for index in 0..ARTIFACT_RUNNER_RETIREMENT_SLOTS {
        if ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire) == 0 {
            continue;
        }
        faulted |= artifact_runner_retirement_slot_turn(index, &pool) == ArtifactRunnerRetirementTurn::Faulted;
    }
    let hooks = ARTIFACT_RUNNER_RETIREMENT_HOOKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    match hooks.get(row).and_then(Option::as_ref) {
        Some(hook) if hook.generation == generation && faulted => semio_framework_async::WorkerMaintenanceStep::Fault,
        Some(hook) if hook.generation == generation => semio_framework_async::WorkerMaintenanceStep::Idle,
        _ => semio_framework_async::WorkerMaintenanceStep::Retire,
    }
}

/// 🧹️ One close step of the cursor in slot `index` when it retires on `pool`: `Absent` for a free, uncommitted or
/// foreign slot, `Waiting` while the runner still closes, `Faulted` for a panicking or blocked close (the exact cursor
/// stays), `Retired` once terminal — the slot and its share of the pool's hook are released.
#[cfg(not(target_arch = "wasm32"))]
fn artifact_runner_retirement_slot_turn(index: usize, pool: &Arc<semio_framework_async::WorkerPool>) -> ArtifactRunnerRetirementTurn {
    let slot = &ARTIFACT_RUNNER_RETIREMENTS[index];
    let mut row = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if !row.as_ref().is_some_and(|owner| Arc::ptr_eq(&owner.pool, pool)) {
        return ArtifactRunnerRetirementTurn::Absent;
    }
    let mut cursor = row.take();
    drop(row);
    let owner = cursor.as_mut().expect("checked artifact runner retirement owner");
    #[cfg(test)]
    owner.handoff.retirement_turns.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    let terminal = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (owner.close)())) {
        Ok(terminal) => terminal,
        Err(_) => {
            *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = cursor;
            return ArtifactRunnerRetirementTurn::Faulted;
        }
    };
    if !terminal {
        let blocked = owner.handoff.close_retry_progress().is_some_and(|progress| progress.is_blocked());
        *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = cursor;
        return if blocked { ArtifactRunnerRetirementTurn::Faulted } else { ArtifactRunnerRetirementTurn::Waiting };
    }
    owner.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    owner.handoff.retirement_maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    let hook = owner.hook;
    drop(cursor);
    ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].store(0, std::sync::atomic::Ordering::Release);
    artifact_runner_retirement_hook_leave(hook);
    ArtifactRunnerRetirementTurn::Retired
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactRunnerRetirementReservation {
    pub fn try_reserve(pool: Arc<semio_framework_async::WorkerPool>) -> Result<Self, DbError> {
        let generation = ARTIFACT_RUNNER_RETIREMENT_NEXT_GENERATION
            .try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0))
            .map_err(|_| DbError::LimitExceeded("artifact runner retirement generation"))?;
        for (index, slot) in ARTIFACT_RUNNER_RETIREMENT_GENERATIONS.iter().enumerate() {
            if slot.compare_exchange(0, generation, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
                continue;
            }
            let hook = match artifact_runner_retirement_hook_join(&pool, generation) {
                Ok(hook) => hook,
                Err(error) => {
                    slot.store(0, std::sync::atomic::Ordering::Release);
                    return Err(error);
                }
            };
            return Ok(Self { index, generation, pool, hook: Some(hook) });
        }
        Err(DbError::Unavailable("artifact runner retirement capacity exhausted".to_string()))
    }

    pub fn commit(mut self, close: Arc<dyn Fn() -> bool + Send + Sync>, handoff: Arc<ArtifactRunnerHandoff>, pool_use: Arc<semio_framework_async::WorkerPoolUse>) {
        let hook = self.hook.take().expect("artifact runner retirement reservation lost its hook");
        *ARTIFACT_RUNNER_RETIREMENTS[self.index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ArtifactRunnerRetirementCursor { generation: self.generation, close, handoff: handoff.clone(), pool: self.pool.clone(), _pool_use: pool_use, hook });
        *handoff.retirement_maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((self.pool.clone(), hook.ticket));
        let _ = self.pool.request_maintenance(hook.ticket);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactRunnerRetirementReservation {
    fn drop(&mut self) {
        let Some(hook) = self.hook.take() else { return };
        ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[self.index].store(0, std::sync::atomic::Ordering::Release);
        artifact_runner_retirement_hook_leave(hook);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct ArtifactRunnerClosePoll {
    pub(super) handoff: Arc<ArtifactRunnerHandoff>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactRunnerClosePoll {
    fn drop(&mut self) {
        use std::sync::atomic::Ordering;
        let faulted = self.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        loop {
            let driver = self.handoff.driver.load(Ordering::Acquire);
            if driver != ArtifactRunnerDriver::ClosingPolling as u8 && driver != ArtifactRunnerDriver::ClosingPollingWake as u8 {
                return;
            }
            let next = if driver == ArtifactRunnerDriver::ClosingPollingWake as u8 && !faulted { ArtifactRunnerDriver::ClosingReady } else { ArtifactRunnerDriver::ClosingParked };
            if self.handoff.driver.compare_exchange(driver, next as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                if next == ArtifactRunnerDriver::ClosingReady {
                    self.handoff.request_retirement_maintenance();
                }
                return;
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[must_use = "terminal artifact runner jobs retain the only retry or close authority"]
pub struct ArtifactRunnerTerminalJob {
    pub handoff: Arc<ArtifactRunnerHandoff>,
    pub owner: Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>,
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct ArtifactRunner {
    pub(super) pool: Arc<semio_framework_async::WorkerPool>,
    pub(super) address: db_actor::Address<ArtifactMessage>,
    pub(super) receiver: db_actor::Receiver<ArtifactMessage>,
    pub generation: u64,
    pub builder: std::sync::Mutex<Option<ArtifactBuildFuture>>,
    pub engine: std::sync::Mutex<Option<Box<ArtifactEngine>>>,
    pub turn: std::sync::Mutex<Option<ArtifactTurn>>,
    pub(super) ready: std::sync::Mutex<Option<db_actor::ReplySender<Result<(), ArtifactEngineOpenRejected>>>>,
    pub(super) done: std::sync::Mutex<Option<db_actor::ReplySender<()>>>,
    pub(super) handoff: Arc<ArtifactRunnerHandoff>,
    pub(super) retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub(super) retry_armed: std::sync::atomic::AtomicBool,
    pub(super) retry_generation: std::sync::atomic::AtomicU64,
    pub cancelled: std::sync::atomic::AtomicBool,
    pub(super) close_driving: std::sync::atomic::AtomicBool,
    pub terminal: std::sync::atomic::AtomicBool,
}

/// ⏱️ How long one worker turn of a document authority keeps re-polling a self-woken turn
/// future before it hands the worker back to the pool.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const ARTIFACT_RUNNER_TURN_MICROS: u64 = 2_000;

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct ArtifactRunnerPoll {
    pub(super) runner: Arc<ArtifactRunner>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactRunnerPoll {
    fn drop(&mut self) {
        use std::sync::atomic::Ordering;
        loop {
            let driver = self.runner.handoff.driver.load(Ordering::Acquire);
            let cancelled = self.runner.cancelled.load(Ordering::Acquire);
            let faulted = self.runner.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
            let next = match (driver, cancelled) {
                (value, true) if faulted && (value == ArtifactRunnerDriver::Polling as u8 || value == ArtifactRunnerDriver::PollingWake as u8) => ArtifactRunnerDriver::ClosingParked,
                (value, true) if value == ArtifactRunnerDriver::Polling as u8 || value == ArtifactRunnerDriver::PollingWake as u8 => ArtifactRunnerDriver::ClosingReady,
                (value, false) if value == ArtifactRunnerDriver::Polling as u8 => ArtifactRunnerDriver::RunnableIdle,
                (value, false) if value == ArtifactRunnerDriver::PollingWake as u8 => ArtifactRunnerDriver::Queued,
                _ => return,
            };
            if self.runner.handoff.driver.compare_exchange(driver, next as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                if next == ArtifactRunnerDriver::Queued {
                    self.runner.submit_scheduled();
                } else if next == ArtifactRunnerDriver::ClosingReady {
                    self.runner.handoff.request_retirement_maintenance();
                }
                return;
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct ArtifactRunnerWake {
    pub(super) runner: std::sync::Weak<ArtifactRunner>,
    pub generation: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl std::task::Wake for ArtifactRunnerWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(runner) = self.runner.upgrade() {
            if self.generation == runner.generation {
                runner.schedule();
            }
        }
    }
}
//#endregion 🔖️Actor
//#endregion 🔖️Ids

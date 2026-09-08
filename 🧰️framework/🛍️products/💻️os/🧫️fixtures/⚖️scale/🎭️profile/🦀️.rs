//! 🎭️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (F1-scale-fixture, design-workforce.md §3): the seven
//! synthetic behaviours, kept deliberately free of any `wit_bindgen`/`exports::…` type — this module
//! compiles and unit-tests on the host target exactly like `wasm32-wasip2` (`cargo check --all-targets`
//! runs both), and the crate root's `component::` module is the only place WIT<->plain conversion
//! happens (mirrors the SDK's kernel-SSOT/WIT-bridge split, `⚛️reactor/🦀️.rs`, without
//! pulling in that crate's much larger kernel dependency).

use std::cell::RefCell;
use std::collections::HashMap;

//#region 🔖️Config
/// 🎬️ Selects one of the seven handwritten behaviours below. `#[default] Idle` matches an actor the
/// generator never assigned a `scaleFixture.profile` (should not happen for a real generated
/// manifest, but keeps `FixtureConfig::default()` — used when `config` fails to parse — inert
/// rather than surprising).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Profile {
    #[default]
    Idle,
    Cpu,
    Ui,
    Io,
    Hang,
    Crash,
    Stateful,
}

/// 📋️ Decoded from the `instance-open` event's `config` pack — JSON, not `store::pack_rt` (this
/// crate has no dependency on the kernel pack codec; see the module doc). The generator
/// (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`'s `#region 🔖️ScaleFixture`) writes this exact
/// shape into each generated manifest's `scaleFixture` field.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FixtureConfig {
    pub profile: Profile,
    /// ⏱️ `cpu` profile: real wall-clock milliseconds of busy work per turn.
    pub cpu_busy_ms: u64,
    /// 🩹️ `ui` profile: revisioned patches emitted per turn (capped by the turn's `max_frames`).
    pub ui_patches_per_turn: u32,
    /// ⏰️ `hang` profile: how many multiples of its own declared `deadline_ms` it burns before
    /// returning — deliberately > 1 (the whole point is a host watchdog has to intervene).
    pub hang_overrun_multiplier: u32,
    /// 💥️ `crash` profile: which 1-indexed turn traps.
    pub crash_after_turns: u64,
    /// 🔑️ `io` profile: the capability id requested on activation.
    pub io_capability_id: String,
    /// 🧪️ `ui` boundary fixture: publish through the real `host-async.emit-patch` import.
    pub ui_import_sink: bool,
    /// 🧪️ `ui` boundary fixture: deliberately also return a patch after importing one.
    pub ui_return_with_import: bool,
    /// 🧪️ `ui` boundary fixture: make only the first imported patch non-canonical.
    pub ui_malformed_first: bool,
}

impl Default for FixtureConfig {
    fn default() -> Self {
        Self {
            profile: Profile::default(),
            cpu_busy_ms: 5,
            ui_patches_per_turn: 1,
            hang_overrun_multiplier: 3,
            crash_after_turns: 1,
            io_capability_id: "scale-fixture.io".to_string(),
            ui_import_sink: false,
            ui_return_with_import: false,
            ui_malformed_first: false,
        }
    }
}
//#endregion 🔖️Config

//#region 🔖️PlainWire
/// 🌉️ Mirrors WIT `effect` variants this crate actually emits — today only `request-capability`
/// (the `io` profile). `component::poll` converts this to the real
/// `exports::…::effects::RequestCapabilityEffect`; nothing here names a WIT type.
#[derive(Debug, Clone, PartialEq)]
pub enum PlainEffect {
    RequestCapability { req: u64, id: String, scope: String, reason: String, optional: bool },
}

/// 🩹️ One `ui`-profile patch — always a root-path `replace` (design-abi.md §2's node-identity-path
/// diffing is real follow-up work `A2b`'s own report already deferred; this fixture only needs a
/// revisioned patch stream to exist, not a minimal diff).
#[derive(Debug, Clone, PartialEq)]
pub struct PlainPatch {
    pub surface_instance: u32,
    pub surface: String,
    pub revision: u64,
    pub base_revision: u64,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct TurnOutcome {
    pub patches: Vec<PlainPatch>,
    pub effects: Vec<PlainEffect>,
    pub status_more_work: bool,
    pub fuel_used: u64,
    pub next_wake_ms: Option<u64>,
}

/// ⛽️ Plain mirror of WIT `budget` — `component::poll` builds this from
/// `exports::…::reactor::Budget` field-for-field.
#[derive(Debug, Clone, Copy)]
pub struct TurnBudget {
    pub fuel: u64,
    pub deadline_ms: u32,
    pub max_effects: u32,
    pub max_patch_bytes: u32,
    pub max_frames: u32,
}
//#endregion 🔖️PlainWire

//#region 🔖️Engine
struct EngineState {
    config: FixtureConfig,
    turns: u64,
    revision: u64,
    accumulated: Vec<u8>,
    req_seq: u64,
    io_req: Option<u64>,
    io_granted: bool,
    revocations: u64,
}

impl Default for EngineState {
    fn default() -> Self {
        Self { config: FixtureConfig::default(), turns: 0, revision: 0, accumulated: Vec::new(), req_seq: 0, io_req: None, io_granted: false, revocations: 0 }
    }
}

/// 🔎️ Test/bench observability only — how many times `on_capability_revoked` actually matched this
/// instance's configured capability id since the last `on_instance_open`.
pub fn revocation_count() -> u64 {
    STATE.with(|state| state.borrow().revocations)
}

/// 🧪️ Returns the three schema-owned UI boundary switches for the component bridge.
pub fn ui_boundary_mode() -> (bool, bool, bool) {
    STATE.with(|state| {
        let config = &state.borrow().config;
        (config.ui_import_sink, config.ui_return_with_import, config.ui_malformed_first)
    })
}

thread_local! {
    static STATE: RefCell<EngineState> = RefCell::new(EngineState::default());
}

/// 🐣️ `instance-open` — parses `config`; falls back to `FixtureConfig::default()` (⇒ `idle`) on any
/// decode error rather than trapping, since a malformed config is a host/generator bug this actor
/// should surface as inert-idle, not a crash indistinguishable from the real `crash` profile. Resets
/// every per-instance counter (not just `config`) — a fresh `InstanceOpen` genuinely means a new
/// instance, and this is also what keeps `STATE`'s `thread_local` safe to reuse across the test
/// harness's pooled threads (each `#[test]` below calls this first).
pub fn on_instance_open(config_bytes: &[u8]) {
    let config = serde_json::from_slice(config_bytes).unwrap_or_default();
    STATE.with(|state| *state.borrow_mut() = EngineState { config, ..EngineState::default() });
}

/// 🎬️ One `reactor::poll` turn's worth of profile-specific work. `now_ms` is read (repeatedly, for
/// `cpu`/`hang`'s busy-wait) rather than passed once, so this function measures real elapsed wall
/// time exactly like the guest export does via `pure::now-ms`.
pub fn turn(budget: TurnBudget, now_ms: impl Fn() -> i64) -> TurnOutcome {
    let profile = STATE.with(|state| state.borrow().config.profile);
    match profile {
        Profile::Idle => TurnOutcome::default(),
        Profile::Cpu => turn_cpu(budget, &now_ms),
        Profile::Ui => turn_ui(budget),
        Profile::Io => turn_io(),
        Profile::Hang => turn_hang(budget, &now_ms),
        Profile::Crash => turn_crash(),
        Profile::Stateful => turn_stateful(),
    }
}

//#region 🔖️Idle
// Handled inline above — `TurnOutcome::default()` is idle/no-effects/no-patches/zero-fuel.
//#endregion 🔖️Idle

//#region 🔖️Cpu
/// 🔥️ Busy-loops real wall-clock milliseconds — `min(config.cpu_busy_ms, budget.deadline_ms)`, so a
/// well-behaved `cpu` actor never itself blows its own deadline (that is `hang`'s job). Exercises
/// the scheduler's fuel/lane accounting: `fuel_used` scales with elapsed time.
fn turn_cpu(budget: TurnBudget, now_ms: &impl Fn() -> i64) -> TurnOutcome {
    let busy_ms = STATE.with(|state| state.borrow().config.cpu_busy_ms).min(u64::from(budget.deadline_ms));
    let start = now_ms();
    let mut spins: u64 = 0;
    while (now_ms().saturating_sub(start) as u64) < busy_ms {
        spins = spins.wrapping_add(spin_once());
    }
    STATE.with(|state| state.borrow_mut().turns += 1);
    std::hint::black_box(spins);
    TurnOutcome { status_more_work: true, fuel_used: busy_ms.max(1), next_wake_ms: Some(0), ..TurnOutcome::default() }
}

/// 🌀️ One arithmetic "unit of work" — kept out of the optimizer's reach by `black_box`ing the
/// accumulated result in the caller, so `-O` builds cannot prove the loop is dead and elide it.
fn spin_once() -> u64 {
    let mut acc: u64 = 0x9E3779B97F4A7C15;
    for _ in 0..64 {
        acc = acc.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    }
    acc
}
//#endregion 🔖️Cpu

//#region 🔖️Ui
/// 🩹️ Emits `min(config.ui_patches_per_turn, budget.max_frames)` revisioned root-replace patches —
/// exercises `⚛️reactor/🩹️patches`'s revision monotonicity, capped by the turn's own frame budget.
fn turn_ui(budget: TurnBudget) -> TurnOutcome {
    let count = STATE.with(|state| state.borrow().config.ui_patches_per_turn).min(budget.max_frames.max(1));
    let mut patches = Vec::with_capacity(count as usize);
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        for _ in 0..count {
            let base_revision = state.revision;
            state.revision += 1;
            let bytes = format!("{{\"revision\":{}}}", state.revision).into_bytes();
            let bytes = if (bytes.len() as u32) > budget.max_patch_bytes.max(1) { bytes[..(budget.max_patch_bytes.max(1) as usize).min(bytes.len())].to_vec() } else { bytes };
            patches.push(PlainPatch { surface_instance: 0, surface: "window".to_owned(), revision: state.revision, base_revision, bytes });
        }
        state.turns += 1;
    });
    TurnOutcome { patches, status_more_work: true, fuel_used: u64::from(count).max(1), next_wake_ms: Some(0), ..TurnOutcome::default() }
}
//#endregion 🔖️Ui

//#region 🔖️Io
/// 🔑️ Requests its configured capability exactly once on first turn, then goes idle — the grant/
/// denial arrives as an `event.completed` on a later `poll`, consumed by `on_completed` below.
fn turn_io() -> TurnOutcome {
    let (already_requested, effect) = STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.turns += 1;
        if state.io_req.is_some() || state.io_granted {
            (true, None)
        } else {
            state.req_seq += 1;
            let req = state.req_seq;
            state.io_req = Some(req);
            let id = state.config.io_capability_id.clone();
            (false, Some(PlainEffect::RequestCapability { req, id, scope: "read".to_string(), reason: "scale-fixture io profile".to_string(), optional: false }))
        }
    });
    let effects = effect.into_iter().collect::<Vec<_>>();
    TurnOutcome { effects, status_more_work: !already_requested, ..TurnOutcome::default() }
}

/// 🔻️ `event.capability-changed { change: revoked(id) }` — bench budget 8 (design-workforce.md §4:
/// "capability revoked at runtime -> denied completion, actor stays alive, quota counters zero").
/// Clears `io_granted` for a matching id so the NEXT `io` turn re-requests rather than silently
/// keeping on using a token the host already invalidated; does nothing (never traps) for a
/// non-matching id, matching "actor stays alive".
pub fn on_capability_revoked(id: &str) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.config.io_capability_id == id {
            state.io_granted = false;
            state.revocations += 1;
        }
    });
}

/// 📬️ `event.completed` for the `io` profile's outstanding capability request — `ok` mirrors
/// `completion-result::ok(_)` vs `fault(_)`; both clear `io_req` so the turn loop goes idle either
/// way (a denial is a valid, quota-preserving outcome, not a fault this actor should re-request).
pub fn on_completed(req: u64) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.io_req == Some(req) {
            state.io_req = None;
            state.io_granted = true;
        }
    });
}
//#endregion 🔖️Io

//#region 🔖️Hang
/// 🐌️ Deliberately burns `hang_overrun_multiplier × budget.deadline_ms` real wall-clock
/// milliseconds — MORE than its own declared deadline — before returning `more-work`, never
/// `idle`. This is the "ignores its budget" profile a host-side watchdog is supposed to preempt
/// (V1's bench, not this crate) — the overrun is bounded (never literally infinite) so this crate's
/// own `cargo check --all-targets`/unit tests stay finite even though nothing here calls `turn_hang`
/// itself.
fn turn_hang(budget: TurnBudget, now_ms: &impl Fn() -> i64) -> TurnOutcome {
    let overrun_ms = u64::from(budget.deadline_ms) * u64::from(STATE.with(|state| state.borrow().config.hang_overrun_multiplier).max(1));
    let start = now_ms();
    let mut spins: u64 = 0;
    while (now_ms().saturating_sub(start) as u64) < overrun_ms {
        spins = spins.wrapping_add(spin_once());
    }
    STATE.with(|state| state.borrow_mut().turns += 1);
    std::hint::black_box(spins);
    TurnOutcome { status_more_work: true, fuel_used: overrun_ms, next_wake_ms: Some(0), ..TurnOutcome::default() }
}
//#endregion 🔖️Hang

//#region 🔖️Crash
/// 💥️ Traps on turn `crash_after_turns` (1-indexed) — a real `panic!`, not a returned `Fault`: the
/// design point is a wasm TRAP (whole-actor kill, host restores the shard from the last checkpoint),
/// which only an unwind-past-the-export-boundary/abort achieves, not `Result::Err`.
fn turn_crash() -> TurnOutcome {
    let (turn, threshold) = STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.turns += 1;
        (state.turns, state.config.crash_after_turns.max(1))
    });
    if turn >= threshold {
        panic!("scale-fixture crash profile: intentional trap on turn {turn}");
    }
    TurnOutcome { status_more_work: true, ..TurnOutcome::default() }
}
//#endregion 🔖️Crash

//#region 🔖️Stateful
/// 📸️ Accumulates one byte per turn — `checkpoint`/`restore` (below) round-trip this byte-for-byte,
/// proving the LRU-suspend/resume path preserves state exactly.
fn turn_stateful() -> TurnOutcome {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.turns += 1;
        let next = (state.turns & 0xFF) as u8;
        state.accumulated.push(next);
    });
    TurnOutcome { status_more_work: true, ..TurnOutcome::default() }
}
//#endregion 🔖️Stateful

//#endregion 🔖️Engine

//#region 🔖️Checkpoint
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct CheckpointSnapshot {
    turns: u64,
    revision: u64,
    accumulated: Vec<u8>,
    io_granted: bool,
}

/// 📸️ `checkpoint::checkpoint` body — JSON, not `store::pack_rt` (same reasoning as `FixtureConfig`
/// above: this crate has no kernel pack-codec dependency).
pub fn checkpoint() -> Result<Vec<u8>, String> {
    let snapshot = STATE.with(|state| {
        let state = state.borrow();
        CheckpointSnapshot { turns: state.turns, revision: state.revision, accumulated: state.accumulated.clone(), io_granted: state.io_granted }
    });
    serde_json::to_vec(&snapshot).map_err(|error| error.to_string())
}

/// 📸️ `checkpoint::restore` body — `req.wit`'s `pending_requests are never re-parked` rule
/// (design-abi.md §4) applies here too: `io_req` is intentionally NOT restored, only `io_granted`
/// — an in-flight capability request is re-issued on the next `io` turn instead of assumed still
/// outstanding at the host.
pub fn restore(state_bytes: &[u8]) -> Result<(), String> {
    let snapshot: CheckpointSnapshot = serde_json::from_slice(state_bytes).map_err(|error| error.to_string())?;
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.turns = snapshot.turns;
        state.revision = snapshot.revision;
        state.accumulated = snapshot.accumulated;
        state.io_granted = snapshot.io_granted;
        state.io_req = None;
    });
    Ok(())
}
//#endregion 🔖️Checkpoint

//#region 🔖️Jobs
/// 💼️ `jobs.wit`'s `start-job`/`step-job`/`cancel-job` — no profile above places work on a job
/// (design-workforce.md §3 does not ask a scale-fixture profile to exercise the absorbed
/// `semio.io-run`/`semio.io-sniff` cold job kinds), so this is a minimal but genuinely functional
/// echo: `start-job` stores `input`, `step-job` immediately answers `Done(input)`.
pub mod jobs {
    use super::*;

    thread_local! {
        static JOBS: RefCell<HashMap<u64, Vec<u8>>> = RefCell::new(HashMap::new());
    }

    pub enum JobOutcome {
        Done(Vec<u8>),
        Failed(Vec<u8>),
    }

    pub fn start_job(job: u64, _kind: &str, input: Vec<u8>) {
        JOBS.with(|jobs| jobs.borrow_mut().insert(job, input));
    }

    pub fn step_job(job: u64) -> JobOutcome {
        JOBS.with(|jobs| match jobs.borrow_mut().remove(&job) {
            Some(input) => JobOutcome::Done(input),
            None => JobOutcome::Failed(b"scale-fixture: unknown job id".to_vec()),
        })
    }

    pub fn cancel_job(job: u64) {
        JOBS.with(|jobs| {
            jobs.borrow_mut().remove(&job);
        });
    }
}
//#endregion 🔖️Jobs

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

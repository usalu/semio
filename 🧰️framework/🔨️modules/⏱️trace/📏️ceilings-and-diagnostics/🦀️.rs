//! 📏️ Ceilings and diagnostics of the trace primitives: the interactive step and guest lifecycle ceilings, the soft targets, the stage enum and the runtime diagnostics switch. See [`super`].

use super::*;

//#region 🔖️Constants
/// 🚨️ Hard ceiling, in microseconds, for a single interactive step — [`Watchdog`] reports a
/// [`ContractViolation`] once elapsed time crosses this, regardless of which [`InteractiveStage`] the
/// site belongs to.
pub const INTERACTIVE_STEP_CEILING_US: u64 = 8_000;

/// 🚨️ Interactive callbacks must finish strictly below the shared eight-millisecond ceiling.
pub fn interactive_step_contract_violated(elapsed_us: u64) -> bool {
    elapsed_us >= INTERACTIVE_STEP_CEILING_US
}

/// 🚪️ Hard ceiling, in microseconds, for one GUEST LIFECYCLE turn — an instance open/close/ACK, which
/// mounts or tears down a whole app instance (its manifest, fixture and initial scene) in a single
/// one-shot turn. Deliberately NOT [`INTERACTIVE_STEP_CEILING_US`]: that budget bounds a per-frame
/// interactive step, and holding a one-shot mount to a frame budget is the same category error the
/// wgpu frame worker documents for browser-owned suspensions. Measured: `s.puzzle.puzzle3d@1/*#editor`
/// opens in ~446 ms, which no 8 ms frame budget can ever admit. Sized under the shard watchdog's own
/// `heartbeatTimeoutMs` (5 s) so a genuinely wedged open still surfaces as a clean guest fault here
/// rather than as the host killing the whole shard.
pub const GUEST_LIFECYCLE_TURN_CEILING_US: u64 = 5_000_000;

/// 🚪️ A guest lifecycle turn must finish strictly below [`GUEST_LIFECYCLE_TURN_CEILING_US`].
pub fn guest_lifecycle_turn_contract_violated(elapsed_us: u64) -> bool {
    elapsed_us >= GUEST_LIFECYCLE_TURN_CEILING_US
}

/// 🎯️ Soft target for a UI event handler (input → dispatch), in microseconds.
pub const UI_EVENT_SOFT_TARGET_US: u64 = 1_000;

/// 🎯️ Soft target for a UI present/paint, in microseconds.
pub const UI_PRESENT_SOFT_TARGET_US: u64 = 2_000;

/// 🎯️ Soft target for one interactive-lane job step, in microseconds.
pub const INTERACTIVE_STEP_SOFT_TARGET_US: u64 = 1_000;

/// 🎯️ Soft target for a user-visible simulation step, in microseconds.
pub const USER_VISIBLE_SIM_STEP_SOFT_TARGET_US: u64 = 2_000;

/// 🎯️ Soft target for a background-lane step, in microseconds.
pub const BACKGROUND_STEP_SOFT_TARGET_US: u64 = 4_000;

/// 🎭️ Which family of interactive contract a [`Watchdog`]-wrapped site belongs to — each maps to its
/// own [`InteractiveStage::soft_target_us`] below; every kind still shares the one hard
/// [`INTERACTIVE_STEP_CEILING_US`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InteractiveStage {
    UiEvent,
    UiPresent,
    InteractiveStep,
    UserVisibleSimStep,
    BackgroundStep,
}

impl InteractiveStage {
    /// 🎯️ This stage's own soft target, in microseconds — see the per-kind constants above.
    pub const fn soft_target_us(self) -> u64 {
        match self {
            InteractiveStage::UiEvent => UI_EVENT_SOFT_TARGET_US,
            InteractiveStage::UiPresent => UI_PRESENT_SOFT_TARGET_US,
            InteractiveStage::InteractiveStep => INTERACTIVE_STEP_SOFT_TARGET_US,
            InteractiveStage::UserVisibleSimStep => USER_VISIBLE_SIM_STEP_SOFT_TARGET_US,
            InteractiveStage::BackgroundStep => BACKGROUND_STEP_SOFT_TARGET_US,
        }
    }
}
//#endregion 🔖️Constants

//#region 🩺️Diagnostics
/// 🩺️ The ONE switch that arms every per-turn runtime perf trace in the repo — maintenance stage
/// cost, cooperative-callback overrun, reactor more-work streak, typed-operation publication. Off by
/// default: those sites are byte-proportional signal a perf run wants and an interactive boot must
/// never pay for. Named as a schema constant so a host that has no environment (bare wasm in a
/// browser tab) arms the same switch through [`set_runtime_diagnostics`] instead.
pub const RUNTIME_DIAGNOSTICS_ENV: &str = "SEMIO_RUNTIME_DIAGNOSTICS";

/// 🩺️ Tri-state so the environment is read at most once and an explicit host override always wins.
const RUNTIME_DIAGNOSTICS_UNRESOLVED: u32 = 0;

const RUNTIME_DIAGNOSTICS_OFF: u32 = 1;

const RUNTIME_DIAGNOSTICS_ON: u32 = 2;

static RUNTIME_DIAGNOSTICS: AtomicU32 = AtomicU32::new(RUNTIME_DIAGNOSTICS_UNRESOLVED);

/// 🩺️ Arms or disarms the runtime perf traces for this process, outranking the environment.
pub fn set_runtime_diagnostics(enabled: bool) {
    RUNTIME_DIAGNOSTICS.store(if enabled { RUNTIME_DIAGNOSTICS_ON } else { RUNTIME_DIAGNOSTICS_OFF }, Ordering::Relaxed);
}

/// 🩺️ Whether per-turn runtime perf traces may print. Resolves [`RUNTIME_DIAGNOSTICS_ENV`] once on
/// first ask (`1`/`true`/`on`/`yes` arm it, everything else including unset leaves it off).
pub fn runtime_diagnostics_enabled() -> bool {
    match RUNTIME_DIAGNOSTICS.load(Ordering::Relaxed) {
        RUNTIME_DIAGNOSTICS_ON => true,
        RUNTIME_DIAGNOSTICS_OFF => false,
        _ => {
            let armed = runtime_diagnostics_from_environment();
            RUNTIME_DIAGNOSTICS.store(if armed { RUNTIME_DIAGNOSTICS_ON } else { RUNTIME_DIAGNOSTICS_OFF }, Ordering::Relaxed);
            armed
        }
    }
}

/// 🌐️ Bare wasm owns no environment, so the switch there is host-set only.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
fn runtime_diagnostics_from_environment() -> bool {
    false
}

/// 🖥️ Native and WASI p2 read [`RUNTIME_DIAGNOSTICS_ENV`].
#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
fn runtime_diagnostics_from_environment() -> bool {
    matches!(std::env::var(RUNTIME_DIAGNOSTICS_ENV).ok().as_deref().map(str::trim), Some("1" | "true" | "on" | "yes"))
}
//#endregion 🩺️Diagnostics

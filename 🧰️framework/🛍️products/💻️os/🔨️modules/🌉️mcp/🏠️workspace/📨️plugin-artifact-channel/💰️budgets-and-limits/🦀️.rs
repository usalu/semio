//! 💰️ Budgets and limits of the plugin artifact channel: the inference ordinal and ceilings, the headless open, command and inference budgets, the wall budgets and the shared compiled component. See [`super`].

use super::*;

//#region 🔖️ArtifactChannel
/// 🧮️ The logical work units one headless turn may spend; the instruction fuel is disarmed separately.
#[cfg(not(target_arch = "wasm32"))]
const HEADLESS_WORK_FUEL: u64 = 1 << 30;

/// 🎫️ The standalone retained turn a headless gateway issues to itself for one guest turn of `wall_us`.
#[cfg(not(target_arch = "wasm32"))]
fn headless_budget(wall_us: u64) -> semio_framework::kernel::Budget {
    semio_framework_plugin_host::self_issued_budget(1, semio_framework_job::default_now_us, wall_us, HEADLESS_WORK_FUEL, u64::MAX, 4096, 1 << 24, 4096).expect("headless turn budget is issued on the process clock")
}

/// 🪪️ Runs `run` under the authoring identity a headless gateway issues to itself.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn with_headless_identity<T>(run: impl FnOnce(&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> T) -> T {
    let mut observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
    let observer: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_> = &mut observer;
    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1usize << 24, observer).expect("headless authoring identity admission");
    run(&mut identity)
}

#[cfg(not(target_arch = "wasm32"))]
/// 🔢️ The actor ordinal the inference-only activation uses. Distinct from every `ensure_instance`
/// ordinal (those are `instance + 1`, i.e. `1..=u16::MAX - 1`) so an inference guest can never
/// collide with a command-lane guest of the same plugin.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const INFERENCE_ACTOR_ORDINAL: u16 = u16::MAX;

/// ⏱️ The finite allocation/recursion limits every `AppCommand::Infer` carries. Work units come from
/// the caller (clamped to at least one); these two are host policy, not caller input, because an
/// agent must not be able to widen a guest's memory or dependency-recursion envelope.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const INFERENCE_ALLOCATION_BYTES: u64 = 1 << 20;

#[cfg(not(target_arch = "wasm32"))]
pub(super) const INFERENCE_RECURSION_DEPTH: u32 = 4;

/// 💡️ The budget the INFERENCE lane's own guest runs under. An 8 ms interpreter slice
/// exists to keep a UI frame responsive; the inference lane has no frame to keep — it activates its
/// own guest (`INFERENCE_ACTOR_ORDINAL`) and `ArtifactInferenceRouter` drives the cold `semio.infer`
/// job to completion inside ONE call this crate cannot resume from the outside, so an 8 ms epoch
/// deadline made every real declared inference answer `epoch deadline exceeded` (measured
/// 2026-09-19 against an installed owner service). Still a hard ceiling:
/// a runaway guest is stopped, just on a headless job's own time scale. `fuel` is disarmed for the
/// same reason the two budgets below disarm it: under the compiled runtime a fuel yield is as
/// unresumable as an epoch cut, so the time ceiling is the only one worth arming.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn headless_inference_budget() -> semio_framework::kernel::Budget {
    headless_budget(30_000_000)
}

/// 🎬️ The budget a COLD OPEN turn runs under. A ceiling, not a slice: under the compiled runtime a
/// turn is atomic — an epoch deadline cut leaves the component instance mid-call and every later
/// call answers `cannot enter component instance` — so this number is the point at which the guest
/// is declared wedged and its instance is thrown away, never a quantum the host resumes from. 60 s
/// is far above what the os dev host needs to boot the same components (~20 plugins in seconds) and
/// far below the MCP client's own 240 s per-call budget, so a wedged open still answers the agent.
///
/// ⛽️ `fuel` is deliberately disarmed. Under the JIT a fuel yield is exactly as unresumable as an
/// epoch cut, so arming both would only add a second, harder-to-read way to say "wedged"; the time
/// ceiling is the one an agent and an operator can both reason about. `🔬️owned-instance-open`'s
/// `wasmtime_runtime_instance_open_settles_against_a_real_plugin_component` opens a real component
/// under exactly this shape.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn headless_open_budget() -> semio_framework::kernel::Budget {
    headless_budget(60_000_000)
}

/// 📨️ The ceiling ONE `AppCommand` page turn runs under — same reasoning as
/// [`headless_open_budget`], on the scale of a bounded unit of work rather than a whole app
/// assembly. A command still takes several turns (one retained page per turn, then the guest's
/// terminal acknowledgement), but each of those turns runs to completion; the loop that drives them
/// is [`COMMAND_RESUME_WALL_BUDGET`]'s, and reaching THIS ceiling means one turn wedged.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn headless_command_budget() -> semio_framework::kernel::Budget {
    headless_budget(5_000_000)
}

/// ⏱️ How long the page loop for ONE `AppCommand` may keep taking turns before it reports the stall
/// to the caller. A wedged guest must still surface as a bounded, honest `budget.exceeded` rather
/// than a hung MCP tool call, so this is a wall clock, not an attempt count. Every turn inside it
/// completes, so this bounds the NUMBER of pages and host-side cleanup steps a command may need,
/// not the duration of any single guest call.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const COMMAND_RESUME_WALL_BUDGET: std::time::Duration = std::time::Duration::from_secs(60);

/// ⏱️ How long one whole-document load ([`PluginArtifactChannel::load_document_archive`]) may keep polling
/// before it cancels itself. A long history folds over many bounded polls, so this is longer than one
/// command's [`COMMAND_RESUME_WALL_BUDGET`], and it leaves that budget for settling the cancel inside the MCP
/// client's own 240 s per-call budget.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const DOCUMENT_LOAD_WALL_BUDGET: std::time::Duration = std::time::Duration::from_secs(150);

/// ⏱️ How long a cold open may keep taking turns. Larger than [`headless_open_budget`] because an
/// open is at least two turns — the `Event::InstanceOpen` itself and the `InstanceLifecycleAck` for
/// the receipt it publishes — and well under the MCP client's own 240 s per-call budget, so a guest
/// that cannot come up is reported as a bounded `budget.exceeded` instead of killing the tool call.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const INSTANCE_OPEN_WALL_BUDGET: std::time::Duration = std::time::Duration::from_secs(150);

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn shared_compiled_component(runtime: &GuestRuntimes, plugin_id: &str, bytes: &[u8]) -> Result<semio_framework_plugin_host::CompiledHandle, GatewayError> {
    scoped_compiled_component(runtime, plugin_id, ComponentBytes::Held(bytes), &ActivationScope::detached())
}
//#endregion 🔖️ArtifactChannel

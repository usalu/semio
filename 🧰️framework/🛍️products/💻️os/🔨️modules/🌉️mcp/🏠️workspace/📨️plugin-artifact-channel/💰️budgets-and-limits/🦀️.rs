//! 💰️ Budgets and limits of the plugin artifact channel: the inference ordinal and ceilings, the headless open, command and inference budgets, the wall budgets and the shared compiled component. See [`super`].

use super::*;

//#region 🔖️ArtifactChannel
/// 🧮️ The logical work units one headless turn may spend; the instruction fuel is disarmed separately.
#[cfg(not(target_arch = "wasm32"))]
const HEADLESS_WORK_FUEL: u64 = 1 << 30;

/// 🎫️ The standalone retained turn a headless gateway issues to itself for one guest turn of `wall_us`.
#[cfg(not(target_arch = "wasm32"))]
fn headless_budget(wall_us: u64) -> semio_framework::kernel::Budget {
    semio_plugin_host::self_issued_budget(1, semio_framework_job::default_now_us, wall_us, HEADLESS_WORK_FUEL, u64::MAX, 4096, 1 << 24, 4096).expect("headless turn budget is issued on the process clock")
}

/// 🪪️ Runs `run` under the authoring identity a headless gateway issues to itself.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn with_headless_identity<T>(run: impl FnOnce(&mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<T, semio_plugin_guest_contract::TurnFault>) -> Result<T, semio_plugin_guest_contract::TurnFault> {
    use semio_framework_value::native_encoding::{NativeEncodeControl, NativeEncodeRetirementRecipient};
    use semio_os_kernel::os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority, Observer};
    use semio_plugin_guest_contract::identity_turn_fault;

    let mut observer = |_| true;
    let observer: &mut Observer<'_> = &mut observer;
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let receipt = NativeEncodeControl::new(HEADLESS_ENCODING_BYTES, &mut *observer).pause().map_err(|(error, _)| identity_turn_fault(error))?;
    let receipt = receipt.with_retirement_recipient(&mut recipient).map_err(|(error, _)| identity_turn_fault(error))?;
    let mut identity = EntityIdentityAuthority::resume_retirement(receipt, observer);
    let result = run(&mut identity);
    identity.encode(close_headless_encoding).map_err(identity_turn_fault)?;
    result
}

#[cfg(not(target_arch = "wasm32"))]
const HEADLESS_ENCODING_BYTES: usize = 1 << 24;

/// ♻️ Funds each physical close step on the caller's explicit encoder slot.
#[cfg(not(target_arch = "wasm32"))]
fn close_headless_encoding(native: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), semio_framework_value::ValueError> {
    use semio_framework_value::{RetainedCloneGrant, ValueError, ValueRefusalKind, retained_clone::OriginalEffectReply};
    let grant = RetainedCloneGrant {
        maximum_items: 4096,
        maximum_copy_bytes: 65536,
        maximum_capacity_bytes: HEADLESS_ENCODING_BYTES,
        maximum_release_bytes: HEADLESS_ENCODING_BYTES,
        maximum_depth: 256,
    };
    let pump = semio_framework_job::OriginalRetirementPump::new();
    let mut work_left = u128::from(HEADLESS_WORK_FUEL);
    let mut reply = None;
    let mut recovering = false;
    let mut stalled = false;
    while native.has_retirement_owner() {
        let fuel = native.retirement_work_demand(grant.maximum_copy_bytes)?.max(1);
        if u128::from(fuel) > work_left { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "headless encoding retirement exceeded caller work")); }
        let (issued, work) = pump.turn(fuel, u64::MAX, semio_framework_job::default_now_us, grant)?.with_effect(|effect| {
            let output = OriginalEffectReply::borrow(&mut reply)?;
            if recovering { native.recover_close_retirement_recipient_into(grant.maximum_copy_bytes, effect, output)?; }
            else { native.close_retirement_recipient_into(grant.maximum_copy_bytes, effect, output)?; }
            Ok::<(), ValueError>(())
        })?.ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit, "headless encoding retirement turn was not admitted"))?;
        issued?;
        let step = reply.take().transpose().map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)?;
        recovering = step.is_none();
        let no_progress = work == 0 && step.is_none_or(|step| step.progress() == Default::default());
        if stalled && no_progress { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "headless encoding retirement did not advance")); }
        stalled = no_progress;
        work_left = work_left.saturating_sub(work.max(1));
    }
    Ok(())
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

/// 🎬️ The budget a COLD OPEN turn runs under. A ceiling, not a slice: under the compiled runtime a [Design notes](README.md#source-notes-41).
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
pub(crate) fn shared_compiled_component(runtime: &GuestRuntimes, plugin_id: &str, bytes: &[u8]) -> Result<semio_plugin_host::CompiledHandle, GatewayError> {
    scoped_compiled_component(runtime, plugin_id, ComponentBytes::Held(bytes), &ActivationScope::detached())
}
//#endregion 🔖️ArtifactChannel

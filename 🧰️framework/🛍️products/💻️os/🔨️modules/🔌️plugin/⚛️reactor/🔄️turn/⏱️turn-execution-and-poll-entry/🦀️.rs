//! ⏱️ Turn execution and poll entry of the reactor turn: the guest turn execution witness, the kernel poll entry points, the boxed plugin exchange and the turn patch budget and page. See [`super`].

use super::*;

#[path="⏱️execution.rs"]
mod execution;
pub use execution::{guest_turn_executing_us,with_turn_execution,with_turn_execution_clock};

/// 🧠️ Repository-owned actor ABI entrypoint. The component-model wrapper above and the native
/// interpreter both call this exact kernel reducer, so WIT lifting is no longer the production
/// host's semantic authority.
pub async fn poll_kernel<PA: semio_plugin_reactor_app::PluginApp + 'static>(
    runtime: &semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,
    events: &mut Option<Event>,
    command_page: &mut Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<&semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
    identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx:&mut semio_framework_job::StepContext<'_>,
) -> Result<semio_framework::kernel::TurnResult, semio_framework_job::OriginalTurnFailure> {
    poll_kernel_output(runtime, events, command_page, cold_pair_page, budget, |_| Ok(()), |result, ()| result, identity,cx).await
}

/// ⛽️ The paid owned scalar dispatch can schedule a bounded opening independently of the interactive quiet slice.
#[doc(hidden)]
pub async fn poll_kernel_owned<PA: semio_plugin_reactor_app::PluginApp + 'static>(
    runtime: &semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,
    events: &mut Option<Event>,
    command_page: &mut Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<&semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
    execution_clock: fn() -> Option<u64>,
    identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx: &mut semio_framework_job::StepContext<'_>,
) -> Result<semio_framework::kernel::TurnResult, semio_framework_job::OriginalTurnFailure> {
    with_turn_execution_clock(std::pin::pin!(poll_kernel_turn(runtime, events, command_page, cold_pair_page, budget, OpeningPacing::Metered, |_| Ok(()), |result, ()| result, identity, cx)), execution_clock).await
}

/// 🧠️ One interactive guest turn under [`with_turn_execution`] accounting.
#[doc(hidden)]
#[expect(clippy::result_large_err, reason = "Patch reservation and publication callbacks return the original fixed surface or patch owner on refusal; these transfers must not allocate an error wrapper.")]
pub async fn poll_kernel_output<PA: semio_plugin_reactor_app::PluginApp, T, Prepared>(
    runtime: &semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,
    events: &mut Option<Event>,
    command_page: &mut Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<&semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
    prepare: impl FnOnce(&semio_framework::kernel::TurnResult) -> Result<Prepared, semio_framework::Fault>,
    publish: impl FnOnce(semio_framework::kernel::TurnResult, Prepared) -> T,
    identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx:&mut semio_framework_job::StepContext<'_>,
) -> Result<T, semio_framework_job::OriginalTurnFailure> {
    with_turn_execution(std::pin::pin!(poll_kernel_turn(runtime, events, command_page, cold_pair_page, budget, OpeningPacing::Interactive, prepare, publish, identity, cx))).await
}

/// 🔀️ Borrows the retained command and partial output; its data remains in the actual Source home.
pub(super) async fn plugin_exchange_turn<'a, 'b, 'context, PA: semio_plugin_reactor_app::PluginApp>(
    runtime: &'a semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,
    instance_id: u32,
    command: &'a mut Option<(u64, semio_plugin_runtime_ops::ops_ingress::PluginCommandIngress)>,
    state:&'a mut semio_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState,
    identity: &'a mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,
    cx:&'a mut semio_framework_job::StepContext<'context>,
) -> Result<semio_plugin_runtime_ops::ops_ingress::PluginExchangeOutput,semio_framework_job::OriginalTurnFailure> {
        let mut output=semio_plugin_runtime_exchange::exch_plugin_exchange::plugin_exchange(runtime,instance_id,command,state,identity,cx).await?;
        if let Err(refusal)=collect_original_publication_receipts(&mut output,cx){return Err(retain_refused_publication(instance_id,output,refusal,cx));}
        Ok(output)
}

/// 📏️ The per-turn patch BYTE budget this turn admits against — the host's own declaration
/// (`Budget::max_patch_bytes`), clamped into the window the guest's declared linear-memory budget
/// leaves open: never below one contiguous request ceiling (a lane that declared less could not carry
/// one document-scaled publication at all) and never above
/// [`semio_framework::kernel::UI_TURN_PATCH_BUDGET_BYTES`].
#[doc(hidden)]
pub fn turn_patch_budget_bytes(budget: semio_framework::kernel::Budget) -> usize {
    usize::try_from(budget.max_patch_bytes).unwrap_or(semio_framework::kernel::UI_TURN_PATCH_BUDGET_BYTES).clamp(semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, semio_framework::kernel::UI_TURN_PATCH_BUDGET_BYTES)
}

/// 📤️ Fills this turn's patch page with EVERY publication that is ready and fits, and mints the one
/// receipt that authorizes the whole batch.
///
/// 🐛️ Until 2026-09-15 this took exactly one patch per turn, because `UI_TURN_PATCHES_MAXIMUM` was 1:
/// N published surfaces cost N + 1 host round trips by construction, and the React renderer publishes
/// about six per hop (`📓️reactor-reconcile-spin-2026-09-14.md` §7 named this floor and could not move
/// it). The admission rule is now a BYTE budget, not a count:
///
/// 1. the FIRST ready patch is always admitted, whatever it costs — otherwise a publication larger
///    than the whole budget could never leave the guest and the surface would never converge;
/// 2. every further patch joins only while the page's own [`semio_framework::kernel::ui_patch_turn_bytes`]
///    sum stays inside the budget — the one that does not fit is handed straight back to its reserved
///    slot and travels on the next turn, in order, still exactly once;
/// 3. the page's fixed capacity stops the loop at [`semio_framework::kernel::UI_TURN_PATCHES_MAXIMUM`],
///    which is itself derived so the page never becomes a contiguous request past the guest's ceiling.
#[doc(hidden)]
#[expect(clippy::result_large_err, reason = "A refused publication returns its exact patch owner to the reserved pending slot without allocating an error wrapper.")]
pub fn take_turn_patch_page(budget_bytes: usize) -> Result<semio_framework::kernel::UiTurnPatches, semio_framework::Fault> {
    let mut page = semio_framework::kernel::UiTurnPatches::default();
    let mut spent = 0usize;
    while page.len() < semio_framework::kernel::UI_TURN_PATCHES_MAXIMUM {
        let taken = with_pending_patches(|pending| pending.borrow_mut().take_one(semio_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES))
            .map_err(|reason| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.pending-patch-authority"), reason))?;
        let Some(patch) = taken else { break };
        let cost = semio_framework::kernel::ui_patch_turn_bytes(&patch);
        if !page.is_empty() && spent.saturating_add(cost) > budget_bytes {
            with_pending_patches(|pending| pending.borrow_mut().hand_back_turn(patch)).expect("a patch over the turn byte budget returns to its reserved slot");
            break;
        }
        match page.try_push_ui_patch(patch) {
            Ok(()) => spent = spent.saturating_add(cost),
            Err(patch) => {
                with_pending_patches(|pending| pending.borrow_mut().hand_back_turn(patch)).expect("exact unpublished patch returns to its reserved slot");
                break;
            }
        }
    }
    Ok(page)
}

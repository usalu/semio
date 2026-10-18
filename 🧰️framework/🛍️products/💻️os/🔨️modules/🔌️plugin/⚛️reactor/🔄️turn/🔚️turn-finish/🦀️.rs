//! 🔚️ Publication phase of the reactor turn: validates and stages the patch receipt, prepares the caller's output and either publishes the turn result or hands every transferred patch back. See [`super`].

use super::*;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;

/// 🔚️ Stages the patch emission of one built turn result, finishes the focused lifecycle turn and publishes the result; a failed preparation returns every transferred patch to its pending slot.
pub(super) fn publish_turn_result<PA: PluginApp, T, Prepared>(
    runtime: &PluginRuntime<PA>,
    mut result: semio_framework::kernel::TurnResult,
    focus: Option<u32>,
    retryable_lifecycle: bool,
    prepare: impl FnOnce(&semio_framework::kernel::TurnResult) -> Result<Prepared, semio_framework::Fault>,
    publish: impl FnOnce(semio_framework::kernel::TurnResult, Prepared) -> T,
) -> Result<T, semio_framework_job::OriginalTurnFailure> {
    with_pending_patches(|pending| {
        let mut pending = pending.borrow_mut();
        let prepared = (|| {
            result.validate_ui_patch_receipt().map_err(reactor_close_fault)?;
            if let Some(receipt) = result.ui_patch_receipt {
                pending.stage_emission(receipt, result.ui_patches.iter()).map_err(reactor_close_fault)?;
            }
            let prepared = prepare(&result)?;
            if let Some(instance) = focus {
                runtime.guest_lifetimes.borrow_mut().finish_turn(instance, guest_turn_executing_us()).map_err(|reason| {
                    if reason == instance_lifetime::GUEST_LIFECYCLE_TURN_DEADLINE {
                        semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-turn-deadline"), reason).with_retryable(retryable_lifecycle)
                    } else {
                        reactor_close_fault(reason)
                    }
                })?;
            }
            Ok(prepared)
        })();
        match prepared {
            Err(fault) => {
                loop {
                    let returned = result.ui_patches.try_transfer_one(|patch| pending.hand_back_turn(patch));
                    assert!(!matches!(returned, semio_framework::kernel::UiTurnPatchTransfer::Refused), "failed output retains its exact pending patch slot");
                    if matches!(returned, semio_framework::kernel::UiTurnPatchTransfer::Empty) {
                        break;
                    }
                }
                Err(fault)
            }
            Ok(prepared) => {
                if result.ui_patch_receipt.is_some() {
                    pending.commit_emission();
                }
                let published = publish(result, prepared);
                trace_turn_phase_retention("publish");
                Ok(published)
            }
        }
    })
}

//! 🎨️ Dirty surface render and reconcile phases of the reactor turn: renders every dirty surface under its reconcile reservation and drives the reconcile opportunities of the turn. See [`super`].

use super::*;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;

/// 🎨️ Renders each dirty surface of the turn under a reserved reconcile grant, defers a surface that was refused its reservation and records presence updates.
///
/// 👥️ M2 (ticket 26/08/17 `design-unified.md`): `now_ms` is read ONCE for both `record_peer`'s
/// expiry stamping below and `PRESENCE.expire` at the end of this turn — a single wall-clock
/// reading per poll, not one per presence update.
///
/// 🕹️ A refused commit THROWS AWAY the tree this turn just rendered and leaves the
/// surface at its previous revision, which the host then reads back as `unchanged` —
/// indistinguishable from a healthy refresh. It is the one exit on this path that can
/// lose a render the host explicitly asked for, so it is recorded as a shell fault
/// naming the surface rather than a `let _ =` (ticket 26/09/02/PUZZLE-3D-END-TO-END
/// wave B46 §8.1, wave B48 §3). Not a turn-fatal `?`: a commit refused because the
/// instance is closing is an ordinary teardown race, and faulting the whole turn for
/// it would break the close ladder.
///
/// 🕹️ A surface whose previous reconcile is still in flight cannot be re-rendered now, so the
/// dirty render is DEFERRED — and a deferred ring with no room left would drop it, which is a
/// render the host asked for that no later turn ever performs. That is exactly the shape of
/// "the guest never re-publishes its selection lane after a pick" (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B46 §8.1: `interactionSelect` settles, no verdict is
/// recorded, and `data-guest-selection-json` stays `selectedIds:[]` for 150 s), so it is a
/// typed fault naming the surface rather than a `let _ =`.
///
/// 🎟️ A refusal by one of the PROCESS-WIDE reconcile tables is not the ordinary "previous
/// reconcile still in flight" race the defer ring exists for: it says this surface cannot be
/// admitted at all until some other surface hands credit back, and while the reservation was
/// priced at the per-surface CEILING that was structural — three surfaces of a thirteen-surface
/// session, refused/deferred/re-dirtied for ever with `effects=0` (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B56 §1.1, wave B58). Priced by its own body it should
/// never happen, so it is reported by NAME, once per refusal, instead of deferring in silence.
pub(super) async fn render_dirty_surfaces<PA: PluginApp>(runtime: &PluginRuntime<PA>, dirty: DirtyPollOwners, now_ms: u64, effects: &mut Vec<Effect>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    for (instance, surface) in dirty.surfaces {
        if native_close_key(runtime, instance).is_err() {
            continue;
        }
        let surface_key = surface.as_ref().to_owned();
        let _ = semio_framework_plugin_window_kits_rich::tree_kit::take_arena_unbuilt_rows();
        let mounted = match native_close_key(runtime, instance) {
            Ok(key) => PATCHES.with(|patches| patches.reserve_mounted(surface, key)),
            Err(_) => Err(surface),
        };
        match mounted {
            Ok(mut grant) => match semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(runtime, instance, &surface_key,&mut |source,cx|grant.commit_source(source,cx),cx).await {
                Ok(Some(presence)) => {
                    let unbuilt = semio_framework_plugin_window_kits_rich::tree_kit::take_arena_unbuilt_rows();
                    ARENA_STARVED.with(|starved| starved.borrow_mut().note(instance, surface_key.as_str(), unbuilt));
                    for update in presence {
                        PRESENCE.with(|hub| {
                            let mut hub = hub.borrow_mut();
                            hub.record_own(update.surface.clone(), update.node_key.clone(), update.own, update.ttl_ms);
                            for peer in update.peers {
                                hub.record_peer(update.surface.clone(), update.node_key.clone(), peer, update.ttl_ms, now_ms);
                            }
                        });
                    }
                }
                Ok(None)=>{
                    grant.cancel_and_defer();
                }
                Err(failure)=>{
                    grant.cancel();
                    return Err(failure);
                }
            },
            Err(surface) => {
                if let Some(reason) = PATCHES.with(|patches| patches.take_unreported_reserve_refusal()).filter(|reason| reason.starts_with("registry-")) {
                    effects.push(shell_fault_effect(
                        instance,
                        &semio_framework::Fault::new(
                            semio_framework::FaultOrigin::Os,
                            semio_framework::FaultCode::new("ui.surface-reconcile-unadmitted"),
                            format!("dirty surface {surface_key} was refused its reconcile reservation by {reason}, so its render is deferred until another surface returns credit"),
                        ),
                    ));
                }
                PATCHES.with(|patches| patches.defer(surface)).map_err(|surface| {
                    semio_framework::Fault::new(
                        semio_framework::FaultOrigin::Os,
                        semio_framework::FaultCode::new("ui.dirty-surface-deferred-capacity"),
                        format!("deferred render ring is full, so dirty surface {} would never re-render", surface.as_ref()),
                    )
                })?;
            }
        }
    }
    Ok(())
}

/// 🩹️ Drives the reconcile opportunities of the turn, reports whether reconcile arms another turn and answers a render fault with a shell fault effect.
pub(super) fn drive_turn_reconcile(effects: &mut Vec<Effect>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    let reconcile_work = PATCHES
        .with(|patches| -> Result<bool,semio_framework_job::OriginalTurnFailure> {
            drive_reconcile_within(patches, reconcile_step_opportunities(cx.fuel_remaining()), cx)?;
            Ok(reconcile_arms_turn(patches))
        })
        ?;
    if let Some((instance, message)) = PATCHES.with(patches::PatchTracker::take_render_fault) {
        effects.push(shell_fault_effect(instance, &semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new(SURFACE_RENDER_FAULT_CODE), message)));
    }
    Ok(reconcile_work)
}

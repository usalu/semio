//! 🎯️ Dirty poll surfaces of the reactor turn: the dirty render and intent capacities, the dirty poll owners, the background and document surface collection and the arena-starved redirty. See [`super`].

use super::*;

pub(super) const DIRTY_RENDER_CAPACITY: usize = 64;

pub(super) const DIRTY_INTENT_INSTANCE_CAPACITY: usize = 64;

const DIRTY_INTENT_CAPACITY: usize = 64;

pub(super) struct DirtyIntentBatch {
    pub instance: u32,
    pub intents: ui_contract::UiFixedList<ui_contract::UiIntent, DIRTY_INTENT_CAPACITY>,
}

#[doc(hidden)]
pub struct DirtyPollOwners {
    pub surfaces: ui_contract::UiFixedList<(u32, ui_contract::SurfaceId), DIRTY_RENDER_CAPACITY>,
    pub intents: ui_contract::UiFixedList<DirtyIntentBatch, DIRTY_INTENT_INSTANCE_CAPACITY>,
}

/// 🕹️ Re-dirties EVERY deferred surface whose earlier reconcile the host has already acknowledged, and
/// answers how many this turn took. A surface is deferred when a dirty render arrived while its own
/// previous reconcile was still in flight (`PatchTracker::reserve_mounted` refuses an occupied slot),
/// which for a document-scale world body is the ordinary case rather than the rare one.
///
/// 🐢️ Taking ONE of them per turn priced every refused render at a whole host round trip: one
/// `refreshUi` for a narrowed `UiDirtyScope::Partial` dirties three world bodies plus three panel
/// bodies, so a pass that found them occupied needed five further turns before the last of them
/// re-rendered — while the settle driving that refresh
/// (`🧰️framework/…/🔌️PluginRuntime/🟦️.tsx` `settlePluginTurn`) names NO required surface for a body
/// that already has a root, and answers the host's cached hash from the still-unrendered retained tree.
/// That is the starvation half of "the guest never re-publishes its selection lane after a pick"
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B46 §8.1: `interactionSelect` settles twice, no
/// `interaction selection lost` verdict is recorded, and `data-guest-selection-json` stays
/// `selectedIds:[]` for 150 s).
///
/// 🧯️ Bounded twice over: the deferred ring is fixed-capacity, and every entry handed back is one this
/// same turn then renders or re-defers, so the loop cannot revisit an entry it has taken.
/// 🪟️ Dirties every REAL surface one instance's background work must repaint — its mounted window
/// instances, or its app-level panels when it owns no window.
///
/// 🐛️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56: a job's progress, its completion and a
/// document-backbone message each minted `"<instance>:window"` and dirtied THAT. No pane reads that
/// surface, so the host had to mount a synthetic alias for it
/// (`windowHostContextBindings`, `🔌️PluginRuntime/🟦️.tsx`) carrying the last window's body — a fourteenth
/// resident surface competing for the process-wide reconcile credit, whose reservation refusal left every
/// real surface deferred and the reactor answering `more-work` for ever (wave B54 §6.4) while the real
/// panes never received the update this dirty exists to deliver.
pub(super) async fn dirty_background_surfaces<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance: u32, dirty: &mut DirtyPollOwners) -> Result<(), semio_framework::Fault> {
    for surface in semio_framework_plugin_runtime_ops::ops_surface_render::plugin_instance_background_surfaces(runtime, instance).await {
        let surface = ui_contract::SurfaceId::try_from(surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
        dirty.try_surface(instance, surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
    }
    Ok(())
}

/// 📡️ Dirties every surface of `instance` after remote edits were merged into its document.
pub(super) async fn dirty_document_surfaces<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance: u32, dirty: &mut DirtyPollOwners) -> Result<(), semio_framework::Fault> {
    for surface in semio_framework_plugin_runtime_ops::ops_surface_render::plugin_instance_document_surfaces(runtime, instance).await {
        let surface = ui_contract::SurfaceId::try_from(surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
        dirty.try_surface(instance, surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
    }
    Ok(())
}

#[doc(hidden)]
pub fn redirty_acknowledged_deferred_surfaces(patches: &patches::PatchTracker, dirty: &mut DirtyPollOwners,cx:&mut semio_framework_job::StepContext<'_>) -> Result<usize, semio_framework_job::OriginalTurnFailure> {
    let mut taken = 0usize;
    while let Some(surface)=patches.take_pending_render_on_original(cx)?{
        let Some(instance)=parse_surface_instance(surface.as_ref())else{continue};
        dirty.try_surface(instance,surface).map_err(|_|reactor_close_fault("fixed dirty surface authority saturated"))?;taken+=1;
    }
    while let Some(surface) = patches.take_deferred_ready() {
        let Some(instance) = parse_surface_instance(surface.as_ref()) else { continue };
        dirty.try_surface(instance, surface).map_err(|_| reactor_close_fault("fixed dirty surface authority saturated"))?;
        taken += 1;
    }
    Ok(taken)
}

/// 🎟️ Surfaces whose last render stopped short for want of `UiValue` arena credit, each with the rows it left unbuilt
/// ([`crate::app::take_arena_unbuilt_rows`]). The arena is ONE page shared by every panel and by the generations the
/// close ladder is still retiring, so a quick edit → undo → redo renders against whatever is left; its windows then
/// stamp their full extent with a shorter run, and the host — whose window request depends on geometry only — never
/// asks again. [`redirty_arena_starved_surfaces`] renders such a surface once more as soon as the arena can price the
/// rows it is missing, so the rows arrive with the returned credit and no surface re-renders while it cannot gain.
pub(super) struct ArenaStarvedSurfaces {
    pub slots: [Option<(u32, ui_contract::SurfaceId, usize)>; DIRTY_RENDER_CAPACITY],
}

impl ArenaStarvedSurfaces {
    /// 🎟️ Records `surface`'s latest render: short by `unbuilt` rows, or complete (`0`) — which forgets it.
    pub fn note(&mut self, instance: u32, surface: &str, unbuilt: usize) {
        let Ok(surface) = ui_contract::SurfaceId::try_from(surface) else { return };
        for slot in &mut self.slots {
            if slot.as_ref().is_some_and(|(owner, starved, _)| *owner == instance && *starved == surface) {
                *slot = None;
            }
        }
        if unbuilt == 0 {
            return;
        }
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some((instance, surface, unbuilt));
        }
    }
}

/// 🎟️ Re-dirties every arena-starved surface whose missing rows the `UiValue` arena can price again, and answers how many.
pub(super) fn redirty_arena_starved_surfaces(dirty: &mut DirtyPollOwners) -> Result<usize, semio_framework::Fault> {
    let rows = ui_contract::ui_value_headroom().rows();
    ARENA_STARVED.with(|starved| {
        let mut taken = 0usize;
        for slot in starved.borrow_mut().slots.iter_mut() {
            let Some((instance, surface, _)) = slot.take_if(|(_, _, unbuilt)| *unbuilt <= rows) else { continue };
            dirty.try_surface(instance, surface).map_err(|_| reactor_close_fault("fixed dirty surface authority saturated"))?;
            taken += 1;
        }
        Ok(taken)
    })
}

impl DirtyPollOwners {
    pub fn new() -> Self {
        Self { surfaces: ui_contract::UiFixedList::default(), intents: ui_contract::UiFixedList::default() }
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    pub(super) fn try_surface(&mut self, instance: u32, surface: ui_contract::SurfaceId) -> Result<(), ui_contract::SurfaceId> {
        if self.surfaces.iter().any(|queued| queued.0 == instance && queued.1 == surface) {
            return Ok(());
        }
        self.surfaces.try_push((instance, surface)).map_err(|(_, surface)| surface)
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    pub(super) fn try_intent(&mut self, instance: u32, intent: ui_contract::UiIntent) -> Result<(), ui_contract::UiIntent> {
        if let Some(batch) = self.intents.iter_mut().find(|batch| batch.instance == instance) {
            return batch.intents.try_push(intent);
        }
        if self.intents.len() == DIRTY_INTENT_INSTANCE_CAPACITY {
            return Err(intent);
        }
        let mut intents = ui_contract::UiFixedList::default();
        intents.try_push(intent)?;
        let _ = self.intents.try_push(DirtyIntentBatch { instance, intents });
        Ok(())
    }
}

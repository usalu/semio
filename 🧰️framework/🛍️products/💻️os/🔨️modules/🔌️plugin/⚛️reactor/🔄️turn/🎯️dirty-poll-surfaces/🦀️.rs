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
pub(super) async fn dirty_background_surfaces<PA: semio_plugin_reactor_app::PluginApp>(runtime: &semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance: u32, dirty: &mut DirtyPollOwners) -> Result<(), semio_framework::Fault> {
    for surface in semio_plugin_runtime_ops::ops_surface_render::plugin_instance_background_surfaces(runtime, instance).await {
        let surface = ui_contract::SurfaceId::try_from(surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
        dirty.try_surface(instance, surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
    }
    Ok(())
}

/// 📡️ Dirties every surface of `instance` after remote edits were merged into its document.
pub(super) async fn dirty_document_surfaces<PA: semio_plugin_reactor_app::PluginApp>(runtime: &semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance: u32, dirty: &mut DirtyPollOwners) -> Result<(), semio_framework::Fault> {
    for surface in semio_plugin_runtime_ops::ops_surface_render::plugin_instance_document_surfaces(runtime, instance).await {
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

/// 🎟️ One starved surface: the rows its last render left unbuilt and, once it was re-dirtied for them, the count it was re-dirtied at.
pub(super) struct StarvedSurface {
    pub instance: u32,
    pub surface: ui_contract::SurfaceId,
    pub unbuilt: usize,
    pub retried_at: Option<usize>,
}

/// 🎟️ Surfaces whose last render stopped short for want of `UiValue` arena credit, each with the rows it left unbuilt
/// ([`crate::app::take_arena_unbuilt_rows`]). The arena is ONE page shared by every panel and by the generations the
/// close ladder is still retiring, so a quick edit → undo → redo renders against whatever is left; its windows then
/// stamp their full extent with a shorter run, and the host — whose window request depends on geometry only — never
/// asks again. [`redirty_arena_starved_surfaces`] renders such a surface once more as soon as the arena can price the
/// rows it is missing, so the rows arrive with the returned credit and no surface re-renders while it cannot gain.
pub(super) struct ArenaStarvedSurfaces {
    pub slots: [Option<StarvedSurface>; DIRTY_RENDER_CAPACITY],
}

impl ArenaStarvedSurfaces {
    /// 🎟️ Records `surface`'s latest render: short by `unbuilt` rows, or complete (`0`) — which forgets it.
    ///
    /// 🐛️ A surface re-dirtied for its missing rows that comes back short by as many or more is one no retry can finish: its rows exceed what the arena can ever hold. It is forgotten instead of re-dirtied again, because its own retirement returns exactly the credit that triggers the next retry. FEM 3D measured the artifact panel at revision 1 704 and both canvas windows at generation 1 775 within three minutes, every turn `reconcile` or `reconcile,ui_retirement`.
    pub fn note(&mut self, instance: u32, surface: &str, unbuilt: usize) {
        let Ok(surface) = ui_contract::SurfaceId::try_from(surface) else { return };
        let previous = self.slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|starved| starved.instance == instance && starved.surface == surface)).and_then(Option::take);
        if unbuilt == 0 || previous.as_ref().and_then(|starved| starved.retried_at).is_some_and(|retried_at| unbuilt >= retried_at) {
            return;
        }
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(StarvedSurface { instance, surface, unbuilt, retried_at: None });
        }
    }

    /// 🎟️ Hands every starved surface whose missing rows `rows` can price again to `dirty` once, remembering the count it was re-dirtied at, and answers how many.
    pub fn redirty(&mut self, rows: usize, dirty: &mut DirtyPollOwners) -> Result<usize, semio_framework::Fault> {
        let mut taken = 0usize;
        for starved in self.slots.iter_mut().flatten().filter(|starved| starved.retried_at.is_none() && starved.unbuilt <= rows) {
            dirty.try_surface(starved.instance, starved.surface.clone()).map_err(|_| reactor_close_fault("fixed dirty surface authority saturated"))?;
            starved.retried_at = Some(starved.unbuilt);
            taken += 1;
        }
        Ok(taken)
    }
}

/// 🎟️ Re-dirties every arena-starved surface whose missing rows the `UiValue` arena can price again, and answers how many.
pub(super) fn redirty_arena_starved_surfaces(dirty: &mut DirtyPollOwners) -> Result<usize, semio_framework::Fault> {
    let rows = ui_contract::ui_value_headroom().rows();
    let taken = ARENA_STARVED.with(|starved| starved.borrow_mut().redirty(rows, dirty))?;
    if taken > 0 {
        trace_guest_line(&format!("[TRACE] reactor arena-starved surfaces re-dirtied taken={taken} rows={rows}"));
    }
    Ok(taken)
}

#[cfg(test)]
mod arena_starved_tests {
    use super::*;

    fn starved() -> ArenaStarvedSurfaces {
        ArenaStarvedSurfaces { slots: [const { None }; DIRTY_RENDER_CAPACITY] }
    }

    fn occupied(table: &ArenaStarvedSurfaces) -> usize {
        table.slots.iter().flatten().count()
    }

    /// ⚖️ LAW: a surface that a retry leaves as short as before is forgotten, not re-dirtied for ever.
    #[test]
    fn a_starved_surface_whose_retry_gains_nothing_is_not_redirtied_again() {
        let mut table = starved();
        let mut dirty = DirtyPollOwners::new();
        table.note(1, "framework.panel.artifact", 5);
        assert_eq!(table.redirty(4, &mut dirty).unwrap(), 0, "the arena cannot price the missing rows yet");
        assert_eq!(table.redirty(10, &mut dirty).unwrap(), 1);
        assert_eq!(table.redirty(10, &mut dirty).unwrap(), 0, "a surface waiting for its retry is not re-dirtied");
        table.note(1, "framework.panel.artifact", 5);
        assert_eq!(occupied(&table), 0, "the retry gained no rows, so the surface is forgotten");
        assert_eq!(table.redirty(10, &mut dirty).unwrap(), 0);
        assert_eq!(dirty.surfaces.len(), 1);
    }

    /// ⚖️ LAW: a retry that gains rows is followed by another retry for the rest, and a complete render forgets the surface.
    #[test]
    fn a_starved_surface_whose_retry_gains_rows_is_retried_until_complete() {
        let mut table = starved();
        let mut dirty = DirtyPollOwners::new();
        table.note(1, "fem3d-model", 8);
        assert_eq!(table.redirty(8, &mut dirty).unwrap(), 1);
        table.note(1, "fem3d-model", 3);
        assert_eq!(occupied(&table), 1);
        assert_eq!(table.redirty(3, &mut dirty).unwrap(), 1);
        table.note(1, "fem3d-model", 0);
        assert_eq!(occupied(&table), 0);
        assert_eq!(dirty.surfaces.len(), 1, "one surface, queued once");
    }
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

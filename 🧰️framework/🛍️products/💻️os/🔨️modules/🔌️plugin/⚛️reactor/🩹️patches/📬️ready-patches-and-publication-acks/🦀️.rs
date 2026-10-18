//! 📬️ Ready patches and publication acks: the ready patch keys and takes, the rejection marks, the published native resource loans, the publication ack and the oldest admissible ready output. See [`super`].

use super::*;

impl PatchTracker {
    pub fn ready_patch_key(&self) -> Result<Option<(NativeCloseKey, u64)>, &'static str> {
        let state = self.state.try_borrow().map_err(|_| "patch publication target is busy")?;
        Ok(next_ready_index(&state).map(|index| {
            let ready = state.ready[index].as_ref().expect("selected output");
            (ready.key, ready.generation)
        }))
    }

    /// 🎚️ The generations of every slot still holding a producer or a job — what [`next_ready_index`]
    /// used to compare a ready output against GLOBALLY. A law reads it to prove the widened gate
    /// admits an output while a DIFFERENT allocation's lower-generation job is still running, which is
    /// exactly the state the global gate refused.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn in_flight_generations(&self) -> Vec<(NativeCloseKey, u64)> {
        let state = self.state.borrow();
        state.slots.iter().flatten().filter(|slot| slot.producer.is_some() || slot.job.is_some()).map(|slot| (slot.key, slot.generation)).collect()
    }

    pub fn take_ready_patch_into(&self, key: NativeCloseKey, generation: u64, target: &mut Option<SurfaceReconcileReadyPatch>, admitted_bytes: usize) -> Result<bool, &'static str> {
        if target.is_some() {
            return Ok(false);
        }
        let metadata = size_of::<ReadySlot>();
        let Some(bytes) = admitted_bytes.checked_sub(metadata) else { return Ok(false) };
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch publication target is busy")?;
        let Some(index) = next_ready_index(&state) else { return Ok(false) };
        let output = state.ready[index].as_mut().expect("selected retained output");
        if output.key != key || output.generation != generation {
            return Err("patch publication belongs to another allocation or generation");
        }
        if !output.outputs.take_front_into(target, bytes)? {
            return Ok(false);
        }
        output.published = false;
        output.closing = true;
        Ok(true)
    }

    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn take_ready_patch(&self) -> Option<SurfaceReconcileReadyPatch> {
        let (key, generation) = self.ready_patch_key().unwrap()?;
        let mut target = None;
        self.take_ready_patch_into(key, generation, &mut target, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES).unwrap();
        target
    }

    pub fn mark_rejected(&self, surface: &str, expected_generation: u64) -> bool {
        let mut state = self.state.borrow_mut();
        let Some(index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.surface.as_ref() == surface)) else { return false };
        let current_generation = state.slots[index].as_ref().expect("matched rejection surface").generation;
        if current_generation != expected_generation {
            return expected_generation != 0 && current_generation > expected_generation;
        }
        let Some(target_index) = state.terminals.iter().position(Option::is_none) else { return false };
        if state.slots[index].as_ref().is_some_and(|slot| slot.producer.is_some() || slot.job.is_some()) {
            let Some(slot) = state.slots[index].as_mut() else { return false };
            slot.cancelled = true;
            if slot.producer.is_some() {
                return false;
            }
            let Some(job) = slot.job.take() else { return false };
            let terminal = job.into_terminal();
            if slot.reconciler.is_none() {
                slot.reconciler = Some(SurfaceReconciler::new(slot.surface.clone()));
            }
            state.terminals[target_index] = Some(TerminalSlot { key: slot.key, instance: surface_instance(surface), authority: terminal, close: true });
            close_output(&mut state, index);
            return true;
        }
        let Some(generation) = next_generation(&state) else { return false };
        let Some(reconciler) = state.slots[index].as_mut().and_then(|slot| slot.reconciler.take()) else { return false };
        let terminal = match SurfaceReconcileTerminal::try_from_reconciler(reconciler, generation) {
            Ok(terminal) => terminal,
            Err(reconciler) => {
                if let Some(slot) = state.slots[index].as_mut() {
                    slot.reconciler = Some(reconciler);
                } else {
                    drop(reconciler);
                }
                return false;
            }
        };
        commit_generation(&mut state, generation);
        let Some(slot) = state.slots[index].as_mut() else {
            drop(terminal);
            return false;
        };
        slot.cancelled = true;
        slot.reconciler = Some(SurfaceReconciler::new(slot.surface.clone()));
        state.terminals[target_index] = Some(TerminalSlot { key: slot.key, instance: surface_instance(surface), authority: terminal, close: true });
        true
    }

    pub fn with_published_native_resource_loans_on_original<R>(&self,surface:&str,generation:u64,revision:ui_contract::UiRevision,cx:&mut StepContext<'_>,receive:impl FnOnce(&mut semio_framework_ui_runtime::UiNativeResourceLoans,&mut StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<Option<R>,semio_framework_job::OriginalTurnFailure>{
        let Ok(mut state)=self.state.try_borrow_mut()else{return Ok(None)};
        let Some(index)=state.slots.iter().position(|slot|slot.as_ref().is_some_and(|slot|slot.surface.as_ref()==surface && slot.generation==generation && slot.acknowledged_revision.0<revision.0))else{return Ok(None)};
        let key=state.slots[index].as_ref().expect("selected original native surface").key;
        if !state.ready.iter().flatten().any(|ready|ready.key==key && ready.published && !ready.closing && ready.generation==generation){return Ok(None)}
        let Some(reconciler)=state.slots[index].as_mut().and_then(|slot|slot.reconciler.as_mut()).filter(|reconciler|reconciler.revision()==revision)else{return Ok(None)};
        reconciler.with_native_resource_loans_on_original(cx,receive)
    }

    pub fn mark_published_ack(&self, ack: &SurfaceReconcilePublishedAck) -> Result<bool, &'static str> {
        let generation = ack.generation();
        let revision = ack.revision().0;
        let Some(surface) = ack.surface().map(|surface| surface.0.as_str()) else { return Ok(false) };
        let mut state = self.state.try_borrow_mut().map_err(|_| "published ACK target is busy")?;
        let Some(slot) = state.slots.iter_mut().flatten().find(|slot| slot.surface.0.as_str() == surface) else { return Ok(false) };
        if slot.generation != generation {
            return Ok(generation != 0 && slot.generation > generation);
        }
        let current = slot.reconciler.as_ref().map_or_else(
            || slot.producer.as_ref().and_then(|producer| producer.reconciler.as_ref()).map_or_else(|| slot.job.as_ref().map_or(ui_contract::UiRevision::default(), SurfaceReconcileJob::base_revision), SurfaceReconciler::revision),
            SurfaceReconciler::revision,
        );
        if slot.reconciler.as_ref().is_some_and(|reconciler|!reconciler.native_resource_loans_terminal_is_empty()){return Ok(false)}
        if revision == current.0 && revision > slot.acknowledged_revision.0 {
            slot.acknowledged_revision = ui_contract::UiRevision(revision);
            return Ok(true);
        }
        Ok(false)
    }

    pub fn revision(&self, surface: &str) -> ui_contract::UiRevision {
        self.state
            .borrow()
            .slots
            .iter()
            .flatten()
            .find(|slot| slot.surface.as_ref() == surface)
            .map(|slot| {
                slot.reconciler.as_ref().map_or_else(
                    || slot.producer.as_ref().and_then(|producer| producer.reconciler.as_ref()).map_or_else(|| slot.job.as_ref().map_or(ui_contract::UiRevision::default(), SurfaceReconcileJob::base_revision), SurfaceReconciler::revision),
                    SurfaceReconciler::revision,
                )
            })
            .unwrap_or_default()
    }

}

/// 🎚️ The oldest ready output that may leave the guest NOW — the whole of what "readiness" means to
/// [`super::turn::drive_reconcile_within`].
///
/// 🐛️ The refusal used to be GLOBAL: the minimum generation over EVERY slot still holding a producer
/// or a job was compared against the chosen output, so one surface's running reconcile job blocked
/// every other surface's finished one. That is the term
/// `📓️ui-turn-patch-batching-2026-09-15.md` §7.1 names — "widening the batch means widening
/// READINESS" — and it is why 18 of 25 measured boot turns had exactly ONE surface ready while the
/// wire could already carry sixteen.
///
/// Ordering is a per-ALLOCATION contract, never a global one. A retained patch is addressed by its own
/// `(surface, revision)` and applied into that surface's own retained document; two surfaces share no
/// revision line, no document and no reader, so nothing about surface B's in-flight job constrains
/// when surface A's finished output may be published. What DOES constrain it is the same allocation's
/// own earlier work, so the gate asks exactly that question and nothing wider.
fn next_ready_index(state: &PatchTrackerState) -> Option<usize> {
    state
        .ready
        .iter()
        .enumerate()
        .filter_map(|(index, ready)| ready.as_ref().filter(|ready| ready.published && !ready.closing && !state.closing_instances.iter().flatten().any(|closing| closing.key == ready.key)).map(|ready| (index, ready)))
        .filter(|(_, ready)| ready_output_is_admissible(ready.key, ready.generation, state.slots.iter().flatten().filter(|slot| slot.producer.is_some() || slot.job.is_some()).map(|slot| (slot.key, slot.generation))))
        .min_by_key(|(_, ready)| ready.generation)
        .map(|(index, _)| index)
}

/// 🎚️ Whether one finished reconcile output may leave the guest while `in_flight` allocations are
/// still producing — the whole ordering contract of [`next_ready_index`], as a function so it can be
/// stated as a law instead of inferred from a ladder.
///
/// An output waits only for ITS OWN allocation's earlier work. A retained patch is addressed by its
/// own `(surface, revision)` and applied into that surface's own retained document; two allocations
/// share no revision line, no document and no reader, so surface B's in-flight job says nothing about
/// when surface A's finished output may be published.
///
/// 🐛️ The refusal used to be GLOBAL — the minimum generation over EVERY producing slot, compared
/// against the chosen output — so one slow surface serialized every other surface's publication. That
/// is the term `📓️ui-turn-patch-batching-2026-09-15.md` §7.1 names: 18 of 25 measured boot turns had
/// exactly ONE surface ready while the wire could already carry sixteen.
#[doc(hidden)]
pub fn ready_output_is_admissible(ready_key: NativeCloseKey, ready_generation: u64, in_flight: impl Iterator<Item = (NativeCloseKey, u64)>) -> bool {
    !in_flight.into_iter().any(|(key, generation)| key == ready_key && generation < ready_generation)
}

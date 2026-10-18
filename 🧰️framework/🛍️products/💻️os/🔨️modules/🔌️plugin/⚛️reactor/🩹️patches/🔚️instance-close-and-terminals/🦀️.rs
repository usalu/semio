//! 🔚️ Instance close and terminals: the close instance preflight, reservation, activation and release, the close step, the terminal and rejected takes and resumes and the stranded output closes. See [`super`].

use super::*;

impl PatchTracker {
    pub fn preflight_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
        let state = self.state.try_borrow().map_err(|_| "patch close reservation is busy")?;
        if let Some(closing) = state.closing_instances.iter().flatten().find(|closing| closing.instance == key.instance()) {
            return if closing.key == key { Ok(()) } else { Err("patch close reservation belongs to another allocation") };
        }
        if state.slots.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.ready.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.rejected.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.terminals.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.producer_terminals.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.unadmitted.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
        {
            return Err("patch descendants belong to another allocation");
        }
        if state.closing_instances.iter().all(Option::is_some) {
            return Err("patch close reservation is full");
        }
        Ok(())
    }

    pub fn reserve_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch close reservation is busy")?;
        if let Some(closing) = state.closing_instances.iter().flatten().find(|closing| closing.instance == key.instance()) {
            return if closing.key == key { Ok(()) } else { Err("patch close reservation belongs to another allocation") };
        }
        if state.slots.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.ready.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.rejected.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.terminals.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.producer_terminals.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.unadmitted.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
        {
            return Err("patch descendants belong to another allocation");
        }
        let slot = state.closing_instances.iter_mut().find(|slot| slot.is_none()).ok_or("patch close reservation is full")?;
        *slot = Some(ClosingInstance { instance: key.instance(), key, active: false, complete: false });
        Ok(())
    }

    pub fn activate_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch close reservation is busy")?;
        let closing = state.closing_instances.iter_mut().flatten().find(|closing| closing.key == key).ok_or("exact patch close reservation missing")?;
        closing.active = true;
        Ok(())
    }

    pub fn close_instance_complete(&self, key: NativeCloseKey) -> Result<bool, &'static str> {
        let state = self.state.try_borrow().map_err(|_| "patch close receipt is busy")?;
        state.closing_instances.iter().flatten().find(|closing| closing.key == key).map(|closing| closing.complete).ok_or("exact patch close receipt missing")
    }

    pub fn release_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch close receipt is busy")?;
        let closing = state.closing_instances.iter_mut().find(|closing| closing.is_some_and(|closing| closing.key == key && closing.complete)).ok_or("exact patch close receipt is not terminal")?;
        *closing = None;
        Ok(())
    }

    /// 🧹️ Retires one unit of the oldest closing owner against the caller's grant — see
    /// [`super::pending::PendingPatchAuthority::close_step`] for why a retirement unit must be priced
    /// per PAGE and not per item: every unit the reactor cannot finish this turn is answered as
    /// `MoreWork`, and every `MoreWork` is one host round trip the user waits through.
    pub fn close_step(&self, cx: &mut StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.should_yield() { return Ok(false); }
        let grant = cx.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(false); }
        let items = 1;
        let bytes = grant.maximum_copy_bytes.min(grant.maximum_release_bytes);
        let Ok(mut state) = self.state.try_borrow_mut() else { return Ok(false) };
        if state.output_fault.is_some() {
            return Ok(false);
        }
        close_stranded_outputs(&mut state);
        if let Some(index) = state.ready.iter().position(|output| output.as_ref().is_some_and(|output| output.closing)) {
            match state.ready[index].as_mut().expect("retained output close").close_step(items, bytes) {
                Ok(true) => state.ready[index] = None,
                Ok(false) => {}
                Err(fault) => state.output_fault = Some((state.ready[index].as_ref().expect("faulted retained output").key, fault, false)),
            }
            return Ok(false);
        }
        if let Some(closing_index) = state.closing_instances.iter().position(|closing| closing.is_some_and(|closing| closing.active && !closing.complete)) {
            let closing = state.closing_instances[closing_index].expect("closing instance existed");
            let terminal_target = state.terminals.iter().position(Option::is_none);
            if terminal_target.is_none() {
                if let Some(index) = state.terminals.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.instance == Some(closing.instance))) {
                    let terminal = state.terminals[index].as_mut().expect("matching capacity-producing terminal");
                    terminal.close = true;
                    if terminal.authority.close_step_on_original(cx)? && terminal.authority.terminal_is_empty() {
                        state.terminals[index] = None;
                    }
                    return Ok(false);
                }
            }
            if let Some(ready_index) = state.ready.iter().position(|ready| ready.as_ref().is_some_and(|ready| ready.key == closing.key)) {
                let ready = state.ready[ready_index].as_mut().expect("matching ready patch");
                ready.closing = true;
                return Ok(false);
            }
            if let Some(index) = state.deferred.iter().position(|entry| entry.as_ref().is_some_and(|surface| surface_instance(surface.as_ref()) == Some(closing.instance))) {
                state.deferred[index].take();
                return Ok(false);
            }
            if state.unadmitted.iter().flatten().any(|entry| entry.key == closing.key) {
                return Ok(false);
            }
            if let Some(index) = state.rejected.iter().position(|entry| entry.as_ref().is_some_and(|entry| surface_instance(entry.surface.as_ref()) == Some(closing.instance))) {
                if let Some(target) = terminal_target {
                    let rejected = state.rejected[index].take().expect("matching rejected owner");
                    state.terminals[target] = Some(TerminalSlot { key: rejected.key, instance: Some(closing.instance), authority: rejected.authority.into_terminal(), close: true });
                }
                return Ok(false);
            }
            if let Some(index) = state.producer_terminals.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.instance == Some(closing.instance))) {
                let terminal = state.producer_terminals[index].as_mut().expect("matching producer terminal");
                terminal.close = true;
                if terminal.close_step(cx)? && terminal.terminal_is_empty() {
                    state.producer_terminals[index] = None;
                }
                return Ok(false);
            }
            if let Some(surface_index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.key == closing.key)) {
                if state.slots[surface_index].as_ref().is_some_and(|slot| slot.job.is_some() && slot.reconciler.is_some()) {
                    let Some(target) = terminal_target else { return Ok(false) };
                    let PatchTrackerState { slots, terminals, .. } = &mut *state;
                    let surface = slots[surface_index].as_mut().expect("transferred job and canonical root retained");
                    terminals[target] = Some(TerminalSlot { key: surface.key, instance: Some(closing.instance), authority: surface.job.take().expect("retained transferred job shell").into_terminal(), close: true });
                    surface.output_index = None;
                    return Ok(false);
                }
                if state.slots[surface_index].as_ref().is_some_and(|slot| slot.producer.is_some()) {
                    let Some(target) = state.producer_terminals.iter().position(Option::is_none) else { return Ok(false) };
                    let Some(mut surface) = state.slots[surface_index].take() else { return Ok(false) };
                    surface.cancelled = true;
                    let Some(mut producer) = surface.producer.take() else {
                        state.slots[surface_index] = Some(surface);
                        return Ok(false);
                    };
                    if state.rejected_reserved[producer.rejected_index] == Some(surface.generation) {
                        state.rejected_reserved[producer.rejected_index] = None;
                    }
                    state.producer_terminals[target] = Some(MountedTreeTerminal {
                        key: surface.key,
                        instance: Some(closing.instance),
                        surface_index: None,
                        surface: surface.surface,
                        reconciler: producer.reconciler.take(),
                        reservation: producer.reservation.take(),
                        authority: Some(producer.authority),
                        close: true,
                    });
                    return Ok(false);
                }
                let Some(target) = terminal_target else { return Ok(false) };
                let mut surface = state.slots[surface_index].take().expect("matching closing surface");
                surface.cancelled = true;
                let terminal = if let Some(job) = surface.job.take() {
                    Some(job.into_terminal())
                } else if let Some(reconciler) = surface.reconciler.take() {
                    match SurfaceReconcileTerminal::try_from_reconciler(reconciler, surface.generation) {
                        Ok(terminal) => Some(terminal),
                        Err(reconciler) => {
                            surface.reconciler = Some(reconciler);
                            state.slots[surface_index] = Some(surface);
                            return Ok(false);
                        }
                    }
                } else {
                    state.rejected.iter_mut().find(|entry| entry.as_ref().is_some_and(|rejected| rejected.authority.generation() == surface.generation)).and_then(Option::take).map(|rejected| rejected.authority.into_terminal())
                };
                if let Some(terminal) = terminal {
                    state.terminals[target] = Some(TerminalSlot { key: surface.key, instance: Some(closing.instance), authority: terminal, close: true });
                }
                return Ok(false);
            }
            if let Some(index) = state.terminals.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.instance == Some(closing.instance))) {
                let terminal = state.terminals[index].as_mut().expect("matching terminal");
                if terminal.authority.close_step_on_original(cx)? && terminal.authority.terminal_is_empty() {
                    state.terminals[index] = None;
                }
                return Ok(false);
            }
            state.closing_instances[closing_index].as_mut().expect("exact retained close receipt").complete = true;
            return Ok(false);
        }
        let Some(index) = (0..SURFACE_RECONCILE_ADMISSION_SLOTS)
            .map(|offset| (state.close_cursor + offset) % SURFACE_RECONCILE_ADMISSION_SLOTS)
            .find(|index| state.producer_terminals[*index].as_ref().is_some_and(|slot| slot.close) || state.terminals[*index].as_ref().is_some_and(|slot| slot.close))
        else {
            return Ok(true);
        };
        state.close_cursor = (index + 1) % SURFACE_RECONCILE_ADMISSION_SLOTS;
        if let Some(terminal) = state.producer_terminals[index].as_mut().filter(|slot| slot.close) {
            if terminal.close_step(cx)? && terminal.terminal_is_empty() {
                let Some(terminal) = state.producer_terminals[index].take() else { return Ok(false) };
                if let Some(surface_index) = terminal.surface_index {
                    if let Some(surface) = state.slots[surface_index].as_mut().filter(|slot| slot.surface == terminal.surface && slot.reconciler.is_none() && slot.producer.is_none() && slot.job.is_none()) {
                        surface.reconciler = Some(SurfaceReconciler::new(surface.surface.clone()));
                    }
                }
            }
            return Ok(!state.terminals.iter().flatten().any(|slot| slot.close) && !state.producer_terminals.iter().flatten().any(|slot| slot.close));
        }
        let Some(terminal) = state.terminals[index].as_mut().filter(|slot| slot.close) else {
            return Ok(!state.terminals.iter().flatten().any(|slot| slot.close) && !state.producer_terminals.iter().flatten().any(|slot| slot.close));
        };
        if terminal.authority.close_step_on_original(cx)? && terminal.authority.terminal_is_empty() {
            state.terminals[index] = None;
        }
        Ok(!state.terminals.iter().flatten().any(|slot| slot.close) && !state.producer_terminals.iter().flatten().any(|slot| slot.close))
    }

    pub fn take_terminal(&self, generation: u64) -> Option<SurfaceReconcileTerminal> {
        let mut state = self.state.borrow_mut();
        let slot = state.terminals.iter_mut().find(|slot| slot.as_ref().is_some_and(|slot| slot.authority.generation() == generation))?;
        slot.take().map(|slot| slot.authority)
    }

    pub fn take_rejected(&self, generation: u64) -> Option<SurfaceReconcileRejected> {
        let mut state = self.state.borrow_mut();
        let slot = state.rejected.iter_mut().find(|slot| slot.as_ref().is_some_and(|slot| slot.authority.generation() == generation))?;
        slot.take().map(|slot| slot.authority)
    }

    pub fn resume_rejected(&self, rejected: SurfaceReconcileRejected) -> Result<(), SurfaceReconcileTerminal> {
        let generation = rejected.generation();
        let job = match rejected.retry(Default::default()) {
            Ok(job) => job,
            Err(rejected) => return Err(rejected.into_terminal()),
        };
        let mut state = self.state.borrow_mut();
        let Some(slot) = state.slots.iter_mut().flatten().find(|slot| slot.generation == generation && slot.job.is_none() && slot.reconciler.is_none()) else { return Err(job.into_terminal()) };
        slot.job = Some(job);
        Ok(())
    }

    pub fn resume_terminal(&self, terminal: SurfaceReconcileTerminal) -> Result<(), SurfaceReconcileTerminal> {
        let generation = terminal.generation();
        let job = terminal.resume(generation)?;
        let mut state = self.state.borrow_mut();
        let Some(slot) = state.slots.iter_mut().flatten().find(|slot| slot.generation == generation && slot.job.is_none()) else { return Err(job.into_terminal()) };
        slot.job = Some(job);
        Ok(())
    }

    pub fn terminal_is_empty(&self) -> bool {
        let state = self.state.borrow();
        state.slots.iter().all(Option::is_none)
            && state.rejected.iter().all(Option::is_none)
            && state.terminals.iter().all(Option::is_none)
            && state.producer_terminals.iter().all(Option::is_none)
            && state.rejected_reserved.iter().all(Option::is_none)
            && state.unadmitted.iter().all(Option::is_none)
            && state.deferred.iter().all(Option::is_none)
            && state.ready.iter().all(Option::is_none)
            && state.closing_instances.iter().all(Option::is_none)
            && state.output_fault.is_none()
    }
}

/// 🧹️ An unpublished, not-yet-closing output whose surface slot holds no producer or job any more can
/// never be published (its job was retired into a terminal by an instance close or a supersession), so
/// it is closed here — otherwise `has_work` reports it forever and every drain spins on `reconcile`.
fn close_stranded_outputs(state: &mut PatchTrackerState) {
    for output_index in 0..state.ready.len() {
        let Some(ready) = state.ready[output_index].as_ref() else { continue };
        if ready.published || ready.closing {
            continue;
        }
        let publisher_alive = state.slots.iter().flatten().any(|slot| slot.output_index == Some(output_index) && (slot.producer.is_some() || slot.job.is_some()));
        if publisher_alive {
            continue;
        }
        for slot in state.slots.iter_mut().flatten() {
            if slot.output_index == Some(output_index) {
                slot.output_index = None;
            }
        }
        state.ready[output_index].as_mut().expect("stranded output").closing = true;
    }
}

pub(super) fn close_output(state: &mut PatchTrackerState, index: usize) {
    if let Some(output) = state.slots[index].as_mut().and_then(|slot| slot.output_index.take()).and_then(|output| state.ready[output].as_mut()) {
        output.closing = true;
    }
}

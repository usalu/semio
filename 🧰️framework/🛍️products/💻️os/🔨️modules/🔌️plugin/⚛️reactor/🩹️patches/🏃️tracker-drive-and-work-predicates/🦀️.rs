//! 🏃️ Tracker drive and work predicates: the drive of one surface, the debug state, the publishable, drivable and general work predicates, the render fault take, the job drive helper and the generation commit. See [`super`].

use super::*;

impl PatchTracker {
    pub fn drive_one(&self, cx: &mut StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let mut state = self.state.borrow_mut();
        if state.output_fault.is_some() {
            return Ok(true);
        }
        let Some(index) = (0..SURFACE_RECONCILE_ADMISSION_SLOTS)
            .map(|offset| (state.drive_cursor + offset) % SURFACE_RECONCILE_ADMISSION_SLOTS)
            .find(|index| state.slots[*index].as_ref().is_some_and(|slot| !state.closing_instances.iter().flatten().any(|closing| closing.key == slot.key) && (slot.producer.is_some() || slot.job.is_some())))
        else {
            return Ok(has_work(&state));
        };
        state.drive_cursor = (index + 1) % SURFACE_RECONCILE_ADMISSION_SLOTS;
        if state.slots[index].as_ref().is_some_and(|slot| slot.job.is_some()) {
            drive_job_one(&mut state, index, cx)?;
            return Ok(has_work(&state));
        }
        let outcome = state.slots[index].as_ref().and_then(|slot| slot.producer.as_ref()).and_then(|producer| producer.outcome);
        let Some(outcome) = outcome else {
            let slot = state.slots[index].as_mut().expect("selected structural producer slot");
            let Some(producer) = slot.producer.as_mut() else { return Ok(has_work(&state)) };
            if cx.should_yield() { return Ok(has_work(&state)); }
            let outcome = producer.authority.step_on_original(slot.generation,slot.cancelled,cx)?;
            #[cfg(test)]
            tests::after_producer_step();
            if outcome != ComponentTreeProducerStep::MoreWork {
                producer.outcome = Some(outcome);
            }
            return Ok(has_work(&state));
        };
        let grant=cx.retained_grant();
        if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||state.producer_terminals.iter().all(Option::is_some){return Ok(has_work(&state));}
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
        let Some(mut slot) = state.slots[index].take() else { return Ok(has_work(&state)) };
        if let Some(mut producer) = slot.producer.take() {
            match outcome {
                ComponentTreeProducerStep::MoreWork => slot.producer = Some(producer),
                ComponentTreeProducerStep::Complete => {
                    let Some(target)=state.producer_terminals.iter().position(Option::is_none)else{slot.producer=Some(producer);state.slots[index]=Some(slot);return Ok(has_work(&state));};
                    let sources = if producer.reconciler.is_some() && producer.reservation.is_some() && producer.authority.has_complete() {
                        Some((producer.reconciler.take().expect("preflight current root"), producer.reservation.take().expect("preflight producer reservation"), producer.authority.take_complete().expect("preflight completed tree")))
                    } else {
                        None
                    };
                    if let Some((reconciler, reservation, tree)) = sources {
                        match SurfaceReconcileJob::try_new_reserved(reconciler, tree, reservation) {
                            Ok(job) => {
                                if state.rejected_reserved[producer.rejected_index] == Some(slot.generation) {
                                    state.rejected_reserved[producer.rejected_index] = None;
                                }
                                slot.job = Some(job);
                            }
                            Err(authority) => {
                                state.rejected_reserved[producer.rejected_index] = None;
                                state.rejected[producer.rejected_index] = Some(RejectedSlot { key: slot.key, surface: slot.surface.clone(), authority });
                            }
                        }
                        state.producer_terminals[target]=Some(MountedTreeTerminal{key:slot.key,instance:surface_instance(slot.surface.as_ref()),surface_index:None,surface:slot.surface.clone(),reconciler:None,reservation:None,authority:Some(producer.authority),close:true});
                    } else {
                        state.rejected_reserved[producer.rejected_index] = None;
                        state.producer_terminals[target] = Some(MountedTreeTerminal {
                            key: slot.key,
                            instance: surface_instance(slot.surface.as_ref()),
                            surface_index: Some(index),
                            surface: slot.surface.clone(),
                            reconciler: producer.reconciler.take(),
                            reservation: producer.reservation.take(),
                            authority: Some(producer.authority),
                            close: true,
                        });
                    }
                }
                ComponentTreeProducerStep::Fault(_) => {
                    if let Some(target) = state.producer_terminals.iter().position(Option::is_none) {
                        state.rejected_reserved[producer.rejected_index] = None;
                        state.producer_terminals[target] = Some(MountedTreeTerminal {
                            key: slot.key,
                            instance: surface_instance(slot.surface.as_ref()),
                            surface_index: Some(index),
                            surface: slot.surface.clone(),
                            reconciler: producer.reconciler.take(),
                            reservation: producer.reservation.take(),
                            authority: Some(producer.authority),
                            close: false,
                        });
                    } else {
                        slot.producer = Some(producer);
                    }
                }
            }
            if slot.producer.is_none() && slot.job.is_none() && state.rejected.iter().flatten().all(|rejected| rejected.authority.generation() != slot.generation) {
                if let Some(output) = slot.output_index.take().and_then(|index| state.ready[index].as_mut()) {
                    output.closing = true;
                }
            }
            state.slots[index] = Some(slot);
            return Ok(has_work(&state));
        }
        state.slots[index] = Some(slot);
        Ok(has_work(&state))
    }

    /// 🐞️ Temporary trace summary of every retained slot family, read by the reactor's more-work streak trace.
    /// 📊️ WHAT the two process-wide reconcile tables hold, printed beside every refusal so a
    /// `registry-*` refusal can be read against the occupancy that caused it rather than guessed at
    /// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56).
    pub fn debug_state(&self) -> String {
        let state = self.state.borrow();
        let slots: Vec<String> = state
            .slots
            .iter()
            .flatten()
            .map(|slot| {
                format!(
                    "{}#g{}:{}{}{}:ack{}/rev{}:out{:?}",
                    slot.surface.as_ref(),
                    slot.generation,
                    if slot.producer.is_some() { "P" } else { "-" },
                    if slot.job.is_some() { "J" } else { "-" },
                    if slot.reconciler.is_some() { "R" } else { "-" },
                    slot.acknowledged_revision.0,
                    slot.reconciler.as_ref().map_or(0, |reconciler| reconciler.revision().0),
                    slot.output_index
                )
            })
            .collect();
        let ready: Vec<String> = state
            .ready
            .iter()
            .flatten()
            .map(|ready| {
                format!("g{}:{}{}{}{}", ready.generation, if ready.published { "p" } else { "-" }, if ready.closing { "c" } else { "-" }, if ready.reservation.is_some() { "r" } else { "-" }, if ready.outputs.terminal_is_empty() { "e" } else { "-" })
            })
            .collect();
        let terminals: Vec<String> = state
            .terminals
            .iter()
            .flatten()
            .map(|terminal| format!("g{}:{}{}{}", terminal.authority.generation(), if terminal.close { "c" } else { "-" }, if terminal.authority.fault().is_some() { "F" } else { "-" }, if terminal.authority.terminal_is_empty() { "e" } else { "-" }))
            .collect();
        let producer_terminals: Vec<String> = state
            .producer_terminals
            .iter()
            .flatten()
            .map(|terminal| {
                format!(
                    "{}:{}{}{}{}",
                    terminal.surface.as_ref(),
                    if terminal.close { "c" } else { "-" },
                    if terminal.authority.is_some() { "A" } else { "-" },
                    if terminal.reconciler.is_some() { "R" } else { "-" },
                    if terminal.reservation.is_some() { "V" } else { "-" }
                )
            })
            .collect();
        let deferred: Vec<String> = state.deferred.iter().flatten().map(|surface| surface.as_ref().to_owned()).collect();
        format!(
            "slots=[{}] ready=[{}] terminals=[{}] producer_terminals=[{}] deferred=[{}] rejected={} unadmitted={} closing={} output_fault={} reserve_refusal={} registry={} generation_exhausted={} close_cursor={}",
            slots.join(","),
            ready.join(","),
            terminals.join(","),
            producer_terminals.join(","),
            deferred.join(","),
            state.rejected.iter().flatten().count(),
            state.unadmitted.iter().flatten().count(),
            state.closing_instances.iter().flatten().count(),
            state.output_fault.as_ref().map_or("none", |fault| fault.1),
            state.reserve_refusal.as_ref().map_or_else(|| "none".to_string(), |(index, reason, _)| format!("{}:{reason}", state.slots.get(usize::from(*index)).and_then(Option::as_ref).map_or("unmounted", |slot| slot.surface.as_ref())),),
            registry_census_line(),
            state.generation_exhausted,
            state.close_cursor
        )
    }

    /// 📤️ Work that can still produce or publish a patch (producers, jobs, ready outputs, deferred
    /// surfaces whose slot is ready to re-admit them, unadmitted surfaces, a pending output fault).
    /// Terminal and producer-terminal retirement is background maintenance driven by `close_step`
    /// every turn and must not hold the actor in `more-work`, otherwise a host drain waits for the
    /// retirement of trees it will never see. A deferred surface whose slot still awaits the host's
    /// acknowledgement of an earlier revision is host-blocked the same way: the acknowledgement
    /// arrives as its own lifecycle turn, so counting it here only makes the drain spin on itself.
    pub fn has_publishable_work(&self) -> bool {
        let state = self.state.borrow();
        state.output_fault.is_some()
            || state.slots.iter().flatten().any(|slot| slot.producer.is_some() || slot.job.is_some() || pending_render_ready(slot))
            || state.deferred.iter().flatten().any(|surface| deferred_surface_ready(&state, surface))
            || state.unadmitted.iter().any(Option::is_some)
            || state.closing_instances.iter().any(Option::is_some)
            || state.ready.iter().flatten().any(|ready| ready.published && !ready.closing)
    }

    /// 🚗️ Work this guest can advance BY ITSELF — [`Self::has_publishable_work`] minus the one term
    /// that is not the guest's to advance: a ready output already marked published, whose extraction
    /// needs a free publication slot and therefore the host's acknowledgement of an earlier one.
    ///
    /// 🐛️ The reactor's reconcile loop used to spin on `has_publishable_work` alone, so a turn with a
    /// ready output and a full [`super::pending::PendingPatchAuthority`] burnt its whole 1 024-step
    /// opportunity budget (or the turn's wall deadline) calling `drive_one` on a tracker with nothing
    /// to drive, answered `MoreWork`, and the host paid a round trip to watch it do that again —
    /// 6.0 ms of guest per empty crossing, 59 such crossings per `flowEvalTick` hop
    /// (`📓️reactor-reconcile-spin-2026-09-14.md` §1, family 2).
    pub fn has_drivable_work(&self) -> bool {
        let state = self.state.borrow();
        state.output_fault.is_some()
            || state.slots.iter().flatten().any(|slot| slot.producer.is_some() || slot.job.is_some() || pending_render_ready(slot))
            || state.deferred.iter().flatten().any(|surface| deferred_surface_ready(&state, surface))
            || state.unadmitted.iter().any(Option::is_some)
            || state.closing_instances.iter().any(Option::is_some)
    }

    pub fn has_work(&self) -> bool {
        has_work(&self.state.borrow())
    }

    /// 🚨️ Returns one mounted surface failure while retaining its incremental cleanup owner.
    ///
    /// 🔎️ The message names the AUTHORITY that reported the fault, because the two authorities know
    /// their surface differently: a producer terminal carries its own `SurfaceId`, while a reconcile
    /// terminal is matched back to a slot BY GENERATION and names `unknown surface` once that slot is
    /// gone. A reader that cannot tell the two apart cannot tell an exact attribution from a derived
    /// one. A `DuplicateSiblingKey` additionally names the parent and the key it refused
    /// ([`ComponentTreeProducer::duplicate_sibling`]), so the author is pointed at the node rather
    /// than at the whole surface.
    pub fn take_render_fault(&self) -> Option<(u32, String)> {
        let mut state = self.state.borrow_mut();
        if let Some((key, fault, reported)) = state.output_fault.as_mut() {
            if !*reported {
                *reported = true;
                return Some((key.instance(), (*fault).to_owned()));
            }
        }
        for terminal in state.producer_terminals.iter_mut().flatten().filter(|terminal| !terminal.close) {
            if let Some(fault) = terminal.authority.as_ref().and_then(|authority| authority.fault()) {
                let detail = terminal.authority.as_ref().and_then(|authority| authority.duplicate_sibling()).map_or_else(String::new, |(parent, key)| format!(" parent={parent} key={key}"));
                terminal.close = true;
                return Some((terminal.instance.unwrap_or(0), format!("{} [producer]: {fault:?}{detail}", terminal.surface.as_ref())));
            }
        }
        let terminal = state.terminals.iter_mut().flatten().find(|terminal| !terminal.close && terminal.authority.fault().is_some())?;
        let fault = terminal.authority.fault().cloned()?;
        let generation = terminal.authority.generation();
        let instance = terminal.instance.unwrap_or(0);
        terminal.close = true;
        let surface = state.slots.iter().flatten().find(|slot| slot.generation == generation).map_or("unknown surface", |slot| slot.surface.as_ref());
        Some((instance, format!("{surface} [reconciler g{generation}]: {fault:?}")))
    }

}

fn drive_job_one(state: &mut PatchTrackerState, index: usize, cx: &mut StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    let Some(slot) = state.slots[index].as_ref() else { return Ok(()) };
    let Some(job) = slot.job.as_ref() else { return Ok(()) };
    if slot.reconciler.is_some() && !job.is_ready() {
        let Some(target) = state.terminals.iter().position(Option::is_none) else { return Ok(()) };
        let stranded = slot.output_index.and_then(|output| state.ready[output].as_ref()).is_some_and(|ready| !ready.published);
        if stranded {
            close_output(state, index);
        }
        let slot = state.slots[index].as_mut().expect("retained transferred job");
        state.terminals[target] = Some(TerminalSlot { key: slot.key, instance: surface_instance(slot.surface.as_ref()), authority: slot.job.take().expect("retained empty job shell").into_terminal(), close: true });
        slot.output_index = None;
        return Ok(());
    }
    let Some(output_index) = slot.output_index else { return Ok(()) };
    let Some(_) = state.ready[output_index].as_ref().filter(|output| output.key == slot.key && output.generation == slot.generation) else { return Ok(()) };
    if job.is_ready() {
        let slot = state.slots[index].as_mut().expect("retained ready job slot");
        if slot.job.as_mut().expect("retained ready job authority").drive_one(slot.generation, cx)? != SurfaceReconcileJobStep::Ready {
            return Ok(());
        }
        let receiver_bytes = size_of::<ReadySlot>();
        let Some(grant) = semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES.checked_sub(receiver_bytes) else { return Ok(()) };
        let output = state.ready[output_index].as_mut().expect("output reserved before producer");
        let result = output.outputs.receive_job_into(&mut output.reservation, slot.job.as_mut().expect("structural job receiver"), &mut slot.reconciler, grant);
        match result {
            Ok(SurfaceReconcileOutputTransfer::Published) => output.published = true,
            Ok(SurfaceReconcileOutputTransfer::Empty) => {
                slot.acknowledged_revision = slot.reconciler.as_ref().expect("transferred canonical root").revision();
                output.closing = true;
            }
            Ok(SurfaceReconcileOutputTransfer::Pending) => return Ok(()),
            Err(fault) => {
                state.output_fault = Some((slot.key, fault, false));
                return Ok(());
            }
        }
        #[cfg(test)]
        tests::after_output_transfer();
        return Ok(());
    }
    let slot = state.slots[index].as_mut().expect("retained job slot");
    let outcome = slot.job.as_mut().expect("retained job authority").drive_one(slot.generation, cx)?;
    if outcome == SurfaceReconcileJobStep::Fault {
        if let Some(target) = state.terminals.iter_mut().find(|slot| slot.is_none()) {
            *target = Some(TerminalSlot { key: slot.key, instance: surface_instance(slot.surface.as_ref()), authority: slot.job.take().expect("faulted job remains retained").into_terminal(), close: false });
        }
    }
    Ok(())
}

/// 🎟️ Whether a deferred surface can be re-admitted now: no slot holds it, or its slot has no live
/// producer/job and the host has acknowledged the slot's current revision (the predicate
/// `take_deferred_ready` releases on).
pub(super) fn deferred_surface_ready(state: &PatchTrackerState, surface: &ui_contract::SurfaceId) -> bool {
    state.slots.iter().flatten().find(|slot| slot.surface == *surface).is_none_or(|slot| slot.producer.is_none() && slot.job.is_none() && slot.reconciler.as_ref().is_some_and(|reconciler| slot.acknowledged_revision.0 >= reconciler.revision().0))
}

pub(super) fn pending_render_ready(slot: &SurfaceSlot) -> bool {
    slot.pending_render && slot.producer.is_none() && slot.job.is_none() && slot.reconciler.as_ref().is_some_and(|reconciler| slot.acknowledged_revision.0 >= reconciler.revision().0)
}

#[doc(hidden)]
pub fn has_work(state: &PatchTrackerState) -> bool {
    state.output_fault.is_some()
        || state.slots.iter().flatten().any(|slot| slot.producer.is_some() || slot.job.is_some() || pending_render_ready(slot))
        || state.terminals.iter().any(Option::is_some)
        || state.producer_terminals.iter().any(Option::is_some)
        || state.deferred.iter().any(Option::is_some)
        || state.unadmitted.iter().any(Option::is_some)
        || state.closing_instances.iter().any(Option::is_some)
        || state.ready.iter().any(Option::is_some)
}

#[doc(hidden)]
pub fn next_generation(state: &PatchTrackerState) -> Option<u64> {
    if state.generation_exhausted {
        return None;
    }
    state.next_generation.checked_add(1)
}

pub(super) fn commit_generation(state: &mut PatchTrackerState, generation: u64) {
    debug_assert_eq!(state.next_generation.checked_add(1), Some(generation));
    state.next_generation = generation;
    state.generation_exhausted = generation == u64::MAX;
}

/// 📊️ One line of both reconcile registries' live occupancy: `resident=<slots>s/<bytes>B of
/// <aggregate>B` and `handback=<free>/<slots>`. A `registry-resident-credit-exhausted` refusal beside
/// `resident=4s/33550336B of 33554432B` is a credit accounting fact, not a guess.
fn registry_census_line() -> String {
    let census = semio_framework_ui_runtime::surface_reconcile_registry_census();
    format!("resident={}s/{}i/{}B of {}B handback={}/{}", census.resident_slots, census.resident_items, census.resident_bytes, census.resident_aggregate_bytes, census.handback_free, census.handback_slots)
}

pub(super) fn surface_instance(surface: &str) -> Option<u32> {
    surface.split(':').next()?.parse().ok()
}

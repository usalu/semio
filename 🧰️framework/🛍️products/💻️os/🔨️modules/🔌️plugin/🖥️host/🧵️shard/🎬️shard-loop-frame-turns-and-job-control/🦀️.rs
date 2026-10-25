//! 🎬️ Shard loop frame turns and job control: the original mesh and event frames, the turn execution, frame consumption and validation, envelope dispatch and the suspend, resume, cancel, job turn and outcome sends. See [`super`].

use super::*;

impl ShardLoop {
    /// 🚦 One actor's turn: takes its collected `events` out of `events_by_actor`, runs
    /// [`super::GuestRuntime::execute_turn`], admits `SpawnJob`/`CancelJob` effects, and sends the
    /// resulting [`ShardOutcome`]. A failed guest cancellation retires the actor before reuse.
    ///
    /// 🔀️ Computed BEFORE `get_mut` below — `self.granted_budget(actor_id)`/`self.actor_lane(..)`
    /// need `&self` (the whole struct), which conflicts with the `&mut self.instances` borrow
    /// `instance` holds for the rest of this call (E0502).
    ///
    /// 👶️ host-dedyn: `GuestRuntime::execute_turn` is plain AFIT now (double-future collapsed)
    /// — `.await`ed directly. `ShardLoop` is driven by a `WorkerPool` job now (P1c) rather than a
    /// dedicated OS thread — that job's own `block_on` (`🧵️executor/🦀️.rs`'s `ShardExecutor::run`) is
    /// the executor boundary; every impl `ShardLoop` is ever handed resolves on its first poll
    /// (see `GuestRuntime`'s own doc comment), so this never actually parks.
    ///
    /// 🐕️ P1c: `Watchdog` wraps ONLY the guest call itself (not the effect-admission bookkeeping
    /// below) — `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` is 8ms; every lane's own
    /// `lane_defaults::budget_for` grants MORE than that to UserVisible (16ms)/Background
    /// (50ms)/Maintenance (200ms) turns BY DESIGN (the epoch-interruption ceiling, not a soft
    /// target), so those lanes are EXPECTED to record a violation on a turn that spends its full
    /// grant — see `📓️p1c-actor-shards.md`'s "turn paths exceeding 8ms" section. This packet only
    /// wires the recording; making a single guest call internally resumable within 8ms slices is
    /// Phase 2's job-protocol work, not this one's.
    ///
    /// 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, placement routing added K1): the
    /// generic `Effect::SpawnJob`/`Effect::CancelJob` admission this packet closes — see
    /// `running_jobs`'s own doc comment. `placement` (inline/isolated/exclusive) is
    /// captured into `job_placement` and `Exclusive` is routed to the FRONT of
    /// `to_step`'s per-pump order (below) — every placement still runs on the SAME
    /// instance that spawned it (routing to a DIFFERENT pooled/exclusive INSTANCE needs
    /// the actor pool `Kernel::activate`/`ShardTable` builds, `design-runtime.md` §1,
    /// `🎭️actor` territory a single `ShardLoop` cannot reach on its own — documented gap,
    /// not a silently faked one, see the K1 report's lease-request).
    ///
    /// 🛑️ terra-shard-lane piece 2: a background/maintenance turn that ran past its
    /// epoch-armed `budget.wall_ms` (`turn_budget_from_grant`'s `deadline_ms`, armed in
    /// `WasmtimeRuntime::execute_turn`/`step_job` via `EpochDeadlines::arm`, which advances
    /// the epoch when that deadline arrives) must be RE-GRANTED next tick, not treated as a failure — an
    /// epoch interrupt lands at a wasm-bytecode safe point, so the wasmtime `Store` inside
    /// `self.instances[&actor_id]` stays perfectly usable and nothing here unregisters it or
    /// clears its state. Sending `ShardOutcome::Fault` for this (the OLD behavior, still
    /// correct for every OTHER `TurnFault` variant below) would have the kernel's
    /// failure-escalation path quarantine an actor purely for being preempted by the exact
    /// per-turn wall budget this ticket's own DRR scheduler assigned it — see
    /// `📓️terra-shard-lane-report.md`.
    ///
    /// 👥️ `presence: Vec::new()` — a deadline-exceeded turn never finished, so there is
    /// no guest-computed presence (or effects/ui_patches) to carry, unlike the two
    /// wire-shape-mismatch sites this packet's report flags (`🦀️.rs`'s
    /// `execute_turn`, `⏳️runtime/🦀️.rs`'s `convert_poll_success`): nothing was dropped
    /// here, there was simply nothing produced.
    fn receive_original_mesh_frame(&mut self,actor:ActorId,input:RetainedTurnInput,sequence:u64,spent:RetainedCloneProgress,work:u64,accepted:bool)->Result<(),PluginHostError>{let budget=self.granted_budgets.get(&actor.0).ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original mesh peer has no existing actor batch")))?.budget;for ring in[&mut self.pending_interactive,&mut self.pending_background]{for offset in 0..ring.len{let Some(slot)=ring.slots[ring.order[offset]].as_mut()else{continue};if slot.owner.authority.actor()!=actor.0{continue}let Some(original)=slot.owner.original_event.as_mut()else{continue};if !original.mesh_awaits_host(){continue}original.receive_mesh_progress(input,sequence,spent,work,accepted).map_err(PluginHostError::NativeIo)?;if slot.owner.budget.is_some(){return Err(PluginHostError::NativeIo(IssuedShardTurn::refusal("original mesh peer would overwrite its actual source budget")))}slot.owner.budget=Some(budget);slot.owner.retry=lifecycle::LifecycleRetry::Ready;return Ok(())}}Err(PluginHostError::NativeIo(IssuedShardTurn::refusal("original mesh receive delta has no retained event checkout")))}
    async fn send_original_mesh_step(&mut self,actor:u64,original:&mut OriginalShardEvent,policy:RetainedCloneGrant,work_fuel:u64,deadline_us:u64)->Result<bool,PluginHostError>{if !original.has_mesh_receiving(){return Ok(false)}if original.mesh_awaits_host(){return Ok(true)}for _ in 0..64{if original.mesh_is_accepted(){let before=original.input.work_units_spent();original.close_mesh_effect_step(policy).map_err(PluginHostError::NativeIo)?;if !original.has_mesh_receiving(){return Ok(false)}if before==original.input.work_units_spent(){return Err(PluginHostError::NativeIo(IssuedShardTurn::refusal("original mesh source close preserves its denied physical frontier")))}continue}let binding=original.input.issued_input().ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original mesh source lost its actual input")))?.work.clock_binding;let now=self.instances.get(&actor).and_then(|instance|instance.original_clock_observation(binding));if let Some(packet)=original.prepare_mesh_packet_step(actor,policy,work_fuel,deadline_us,now).map_err(PluginHostError::NativeIo)?{self.send_outcome(&packet).await?;return Ok(true)}}Err(PluginHostError::NativeIo(IssuedShardTurn::refusal("original mesh source remains under its finite short policy or expired clock")))}

    fn receive_original_event_input(&mut self,actor:u64)->Result<(),PluginHostError>{
        let supplied=self.granted_budgets.get(&actor).ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original continuation has no Kernel supplier")))?;let input=*supplied.original_input();let budget=supplied.budget;
        for ring in [&mut self.pending_interactive,&mut self.pending_background]{for offset in 0..ring.len{let Some(slot)=ring.slots[ring.order[offset]].as_mut()else{continue};if slot.owner.authority.actor()!=actor{continue}if let Some(original)=slot.owner.original_event.as_mut(){original.receive_next_issued(input).map_err(PluginHostError::NativeIo)?;if slot.owner.budget.is_some(){return Err(PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event already holds a supplied budget")))}slot.owner.budget=Some(budget);slot.owner.retry=lifecycle::LifecycleRetry::Ready;return Ok(())}}}
        Err(PluginHostError::NativeIo(IssuedShardTurn::refusal("Kernel continuation has no retained original Event")))
    }

    fn receive_original_event(&mut self,actor:u64,receipt:semio_framework_actor::RetainedTurnReceipt)->Result<(),PluginHostError>{let batch=self.granted_budgets.get_mut(&actor).ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event batch is absent")))?;let issued=batch.issue_envelope(0).map_err(PluginHostError::NativeIo)?;receipt.validate_for(issued).map_err(PluginHostError::NativeIo)?;let returned=batch.settle_envelope(0,receipt).map_err(PluginHostError::NativeIo)?;if returned!=Some(receipt){return Err(PluginHostError::NativeIo(IssuedShardTurn::refusal("one original Event requires one complete batch return")))}Ok(())}

    async fn send_refused_original_call(&mut self,actor:u64,original:&mut OriginalShardEvent,error:semio_framework_value::ValueError,fallback:Option<OriginalActorGuestReturn>,call:Option<semio_framework_plugin_guest_contract::original_actor_context::OriginalActorFixedCall>)->Result<bool,PluginHostError>{let returned=original.input.refused_receipt().copied().or(fallback);original.retain_receiving_refusal(error);if let Some(call)=call{original.retain_fixed_call(call);}let returned=returned.ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original receiving refusal holds no actual raw return")))?;self.send_outcome(&ShardOutcome::OriginalCall{actor,returned,continuation:false,call}).await?;Err(PluginHostError::NativeIo(IssuedShardTurn::refusal("original raw call forwarded before its held local receiving refusal")))}

    pub async fn execute_turn_for(&mut self, actor_id:u64, original:&mut OriginalShardEvent,granted:semio_framework_actor::Budget,actor_lane:semio_framework_actor::Lane)->Result<bool,PluginHostError>{
        let input=*original.input.issued_input().ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event awaits its next Kernel input")))?;
        let turn_budget=turn_budget_from_grant(&granted,input,self.granted_budgets.get(&actor_id).ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event held policy is absent")))?.retained_policy,self.granted_budgets.get(&actor_id).unwrap().work_fuel,self.granted_budgets.get(&actor_id).unwrap().deadline_us).await;
        let allocation=*self.allocations.get(&actor_id).ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event actor allocation is absent")))?;
        if original.input.phase()==ActorGuestPhase::Pending{
            let instance=self.instances.get_mut(&actor_id).ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event guest instance is absent")))?;
            let admission=self.identities.get_mut(&allocation).expect("registered actor owns original identity").admit_actor_original(&self.runtime,instance,&mut original.input,turn_budget,turn_budget.retained_policy).await.map_err(PluginHostError::NativeIo)?.ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event admission has no funded checkout")))?;
            let call=match admission.call{Ok(reply)=>Some(semio_framework_plugin_guest_contract::original_actor_context::OriginalActorFixedCall{input:reply.input,spent:reply.spent,work_units_spent:reply.work_units_spent,kind:semio_framework_plugin_guest_contract::original_actor_context::OriginalActorFixedKind::Admission{ready:reply.ready,refusal:reply.refusal}}),Err(TurnFault::ActorCall(call))=>Some(call),Err(fault)=>{original.retain_trap(fault);None}};
            let raw=call.map(|call|OriginalActorGuestReturn{input:call.input,spent:call.spent,work_units_spent:call.work_units_spent,claimed_remaining:None,claimed_remaining_fuel:None});
            let receipt=match admission.receiving{Ok(receipt)=>receipt,Err(error)=>return self.send_refused_original_call(actor_id,original,error,raw,call).await};
            if let Err(error)=self.receive_original_event(actor_id,receipt){return match error{PluginHostError::NativeIo(error)=>self.send_refused_original_call(actor_id,original,error,Some(OriginalActorGuestReturn{input:receipt.input,spent:receipt.spent,work_units_spent:receipt.work_units_spent,claimed_remaining:Some(receipt.remaining),claimed_remaining_fuel:Some(receipt.remaining_fuel)}),call).await,error=>Err(error)}}
            let Some(call)=call else{return self.send_refused_original_call(actor_id,original,IssuedShardTurn::refusal("original admission trap remains held after its actual raw receipt"),Some(OriginalActorGuestReturn{input:receipt.input,spent:receipt.spent,work_units_spent:receipt.work_units_spent,claimed_remaining:Some(receipt.remaining),claimed_remaining_fuel:Some(receipt.remaining_fuel)}),None).await};
            self.send_outcome(&ShardOutcome::OriginalCall{actor:actor_id,returned:OriginalActorGuestReturn{input:receipt.input,spent:receipt.spent,work_units_spent:receipt.work_units_spent,claimed_remaining:Some(receipt.remaining),claimed_remaining_fuel:Some(receipt.remaining_fuel)},continuation:true,call:Some(call)}).await?;
            return Ok(true)
        }
        if original.is_retiring()&&self.send_original_mesh_step(actor_id,original,turn_budget.retained_policy,turn_budget.work_fuel,turn_budget.deadline_us).await?{return Ok(true)}
        let turn_outcome=if original.is_retiring(){
            let checked=original.input.checkout_guest(turn_budget.retained_policy).map_err(PluginHostError::NativeIo)?.ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event retirement has no funded checkout")))?;
            let policy=original.input.guest_policy(checked).map_err(PluginHostError::NativeIo)?;let mut source_work=0;let step=if checked.work.fuel==0{Ok(Default::default())}else{match original.release_empty_cursor(policy,&mut source_work){Some(progress)=>Ok(progress),None=>original.source_close_step(policy,&mut source_work)}};
            let progress=match step{Ok(progress)=>progress,Err(error)=>{let progress=error.retained_progress();original.retain_source_refusal(error);let receiving=original.input.receive_guest_spent(checked,progress,source_work,ActorGuestPhase::Dispatched);let error=receiving.err().unwrap_or_else(||IssuedShardTurn::refusal("original source close failure remains held with its whole Event"));return self.send_refused_original_call(actor_id,original,error,Some(OriginalActorGuestReturn{input:checked,spent:progress,work_units_spent:source_work,claimed_remaining:None,claimed_remaining_fuel:None}),None).await}};
            let raw=OriginalActorGuestReturn{input:checked,spent:progress,work_units_spent:source_work,claimed_remaining:None,claimed_remaining_fuel:None};if let Err(error)=original.input.receive_guest_spent(checked,progress,source_work,ActorGuestPhase::Dispatched){return self.send_refused_original_call(actor_id,original,error,Some(raw),None).await}
            let receipt=match original.input.return_actor_turn(){Ok(receipt)=>receipt,Err(error)=>return self.send_refused_original_call(actor_id,original,error,Some(raw),None).await};if let Err(error)=self.receive_original_event(actor_id,receipt){return match error{PluginHostError::NativeIo(error)=>self.send_refused_original_call(actor_id,original,error,Some(OriginalActorGuestReturn{input:receipt.input,spent:receipt.spent,work_units_spent:receipt.work_units_spent,claimed_remaining:Some(receipt.remaining),claimed_remaining_fuel:Some(receipt.remaining_fuel)}),None).await,error=>Err(error)}}
            match original.take_closed_result(receipt){Some(result)=>Ok(result),None=>{self.send_outcome(&ShardOutcome::OriginalCall{actor:actor_id,returned:OriginalActorGuestReturn{input:receipt.input,spent:receipt.spent,work_units_spent:receipt.work_units_spent,claimed_remaining:Some(receipt.remaining),claimed_remaining_fuel:Some(receipt.remaining_fuel)},continuation:true,call:None}).await?;return Ok(true)}}
        }else{
            let checked=original.input.checkout_guest(turn_budget.retained_policy).map_err(PluginHostError::NativeIo)?.ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event dispatch has no funded checkout")))?;
            let first=original.input.phase()==ActorGuestPhase::Ready;
            let event=original.input.original().ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event source is absent")))?;
            let events=if first{std::slice::from_ref(event)}else{&[]};
            let instance=self.instances.get_mut(&actor_id).ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event guest instance is absent")))?;
            let call=self.identities.get_mut(&allocation).expect("registered actor owns original identity").execute_turn(&self.runtime,instance,events,Budget{retained:checked,retained_policy:original.input.guest_policy(checked).map_err(PluginHostError::NativeIo)?,..turn_budget}).await;
            match call{
                Ok(result)=>{if let Err(error)=original.retain_dispatched_result(result){let raw=original.pending_raw_return();return self.send_refused_original_call(actor_id,original,error,raw,None).await}if self.send_original_mesh_step(actor_id,original,turn_budget.retained_policy,turn_budget.work_fuel,turn_budget.deadline_us).await?{return Ok(true)}let receipt=match original.input.return_actor_turn(){Ok(receipt)=>receipt,Err(error)=>{let raw=original.pending_raw_return();return self.send_refused_original_call(actor_id,original,error,raw,None).await}};if let Err(error)=self.receive_original_event(actor_id,receipt){return match error{PluginHostError::NativeIo(error)=>{let raw=original.pending_raw_return();self.send_refused_original_call(actor_id,original,error,raw,None).await},error=>Err(error)}}
                    match original.take_received_more_work(receipt){Some(result)=>Ok(result),None=>{self.send_outcome(&ShardOutcome::OriginalCall{actor:actor_id,returned:OriginalActorGuestReturn{input:receipt.input,spent:receipt.spent,work_units_spent:receipt.work_units_spent,claimed_remaining:Some(receipt.remaining),claimed_remaining_fuel:Some(receipt.remaining_fuel)},continuation:true,call:None}).await?;return Ok(true)}}},
                Err(TurnFault::ActorCall(call))=>{let raw=OriginalActorGuestReturn{input:call.input,spent:call.spent,work_units_spent:call.work_units_spent,claimed_remaining:None,claimed_remaining_fuel:None};let receipt=match original.receive_failed_call(call){Ok(receipt)=>receipt,Err(error)=>return self.send_refused_original_call(actor_id,original,error,Some(raw),Some(call)).await};if let Err(error)=self.receive_original_event(actor_id,receipt){return match error{PluginHostError::NativeIo(error)=>self.send_refused_original_call(actor_id,original,error,Some(raw),Some(call)).await,error=>Err(error)}}self.send_outcome(&ShardOutcome::OriginalCall{actor:actor_id,returned:raw,continuation:false,call:Some(call)}).await?;return Ok(true)},
                Err(fault)=>{original.retain_trap(fault);return Err(PluginHostError::NativeIo(IssuedShardTurn::refusal("original guest trap has no raw return; its owner and checkout remain held")))},
            }
        };
        let retry=turn_outcome.as_ref().is_ok_and(|result|matches!(result.status,semio_framework::kernel::TurnStatus::MoreWork));
        let mut cancel_funds=self.job_funds(actor_id);
        let Some(instance)=self.instances.get_mut(&actor_id)else{return Err(PluginHostError::NativeIo(IssuedShardTurn::refusal("received original Event guest instance is absent")))};
        let outcome = match turn_outcome {
            Ok(mut result) => {
                let base_revision = result.ui_patches.iter().map(|patch| patch.revision.0).max().unwrap_or_default();
                let bridged = to_actor_turn_result_in_place(&mut result, actor_id, 0, 0).await;
                let mut admitted_jobs = Vec::new();
                for effect in std::mem::take(&mut result.effects) {
                    match effect {
                        Effect::SpawnJob { job, kind, input, placement } => {
                            if self.next_job_operation == u64::MAX {
                                defer_completion(
                                    &mut self.pending_interactive,
                                    &mut self.pending_background,
                                    &mut self.terminal_authorities,
                                    actor_lane,
                                    self.allocations.get(&actor_id).copied(),
                                    self.granted_budgets.get(&actor_id).map(|turn| turn.budget),
                                    actor_id,
                                    Event::JobCompleted { job, result: RequestOutcome::Err(b"checked replay operation identity exhausted".to_vec()) },
                                )?;
                                continue;
                            }
                            let operation = self.next_job_operation;
                            self.next_job_operation += 1;
                            let authority =
                                JobTurn { job, operation: JobOperation { operation, base_revision, generation: u64::from(ActorId(actor_id).generation()) + 1, preview_sequence: 0, seed: operation.rotate_left(17) ^ actor_id ^ job }, step_sequence: 0 };
                            let request = JobReplayRequest::from_spawn(&kind, &input);
                            let mounted = if self.replay_seeds.iter().any(Option::is_none) { MountedReplaySeed::new(actor_id, job, authority, request, placement, kind, input) } else { Err((kind, input)) };
                            match mounted {
                                Ok(seed) => {
                                    let slot = self.replay_seeds.iter_mut().find(|slot| slot.is_none()).expect("preflighted fixed replay seed slot");
                                    *slot = Some(seed);
                                    admitted_jobs.push(authority);
                                }
                                Err((kind, input)) => {
                                    let reason: &'static [u8] = if self.replay_seeds.iter().any(Option::is_none) { b"fixed replay seed exceeds admitted page capacity" } else { b"fixed replay seed registry refused the exact spawn owner" };
                                    let refusal = ReplaySpawnRefusal::new(actor_id, job, reason, kind, input);
                                    let slot = self.replay_seed_refusals.iter_mut().find(|slot| slot.is_none()).expect("one empty refusal slot per granted maximum effect");
                                    *slot = Some(refusal);
                                }
                            }
                        }
                        Effect::CancelJob { job } => {
                            if let Some(index) = self.replay_seeds.iter().position(|seed| seed.as_ref().is_some_and(|seed| seed.actor == actor_id && seed.job == job && !matches!(seed.phase, ReplaySeedPhase::Retained))) {
                                Self::begin_replay_seed_close_owned(&mut self.replay_seeds, &mut self.running_jobs, &mut self.job_turns, &mut self.job_authorities, &mut self.job_placement, index, ReplaySeedCloseReason::Cancelled);
                            } else if self.running_jobs.contains(&(actor_id, job)) {
                                let cancelled = match cancel_funds.as_mut() {
                                    Ok(funds) => self.runtime.cancel_job_funded(instance, funds, job).await,
                                    Err(_) => Err(TurnFault::Trapped("cancel-job has no Kernel-issued actor batch".to_string())),
                                };
                                match cancelled {
                                    Ok(()) => {
                                        if let Some(index) = self.replay_seeds.iter().position(|seed| seed.as_ref().is_some_and(|seed| seed.actor == actor_id && seed.job == job)) {
                                            Self::begin_replay_seed_close_owned(&mut self.replay_seeds, &mut self.running_jobs, &mut self.job_turns, &mut self.job_authorities, &mut self.job_placement, index, ReplaySeedCloseReason::Cancelled);
                                        } else {
                                            self.running_jobs.remove(&(actor_id, job));
                                            self.job_turns.remove(&(actor_id, job));
                                            self.job_authorities.remove(&(actor_id, job));
                                            self.job_placement.remove(&(actor_id, job));
                                        }
                                    }
                                    Err(fault) => {
                                        let message = format!("ShardLoop::pump: cancel-job {job} failed; actor {actor_id} retired: {}", turn_fault_message(&fault));
                                        self.unregister(ActorId(actor_id)).await;
                                        self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message }).await?;
                                        return Ok(false);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                match bridged {
                    Ok(result) => ShardOutcome::Turn { actor: actor_id, result, jobs: admitted_jobs },
                    Err(fault) => ShardOutcome::Fault { actor: actor_id, message: fault.message },
                }
            }
            Err(fault)=>return Err(PluginHostError::Plugin(turn_fault_message(&fault))),
        };
        self.send_outcome(&outcome).await?;
        Ok(retry)
    }

    /// 📨️ Decodes one [`ShardFrame`] and dispatches it — the drain loop's per-frame body, factored
    /// out so both [`Self::pump_primed`]'s "one primed frame, then the non-blocking drain" shape
    /// and `ShardFrame::Grant`'s own per-envelope loop (below) can share it.
    pub async fn consume_frame(&mut self, bytes: Vec<u8>) -> Result<FrameAdmission, FrameAdmissionError> {
        if bytes.len() > SHARD_FRAME_MAX_BYTES {
            let byte_len = bytes.len();
            return Err(self.retain_terminal_frame(bytes, PluginHostError::Plugin(format!("ShardLoop: raw frame exceeds {SHARD_FRAME_MAX_BYTES} bytes ({byte_len}); exact bytes retained for terminal close"))));
        }
        let mut pos = 0usize;
        let frame = match ShardFrame::pack_decode(&bytes, &mut pos).await {
            Ok(frame) if pos == bytes.len() => frame,
            Ok(_) => {
                return Err(self.retain_terminal_frame(bytes, PluginHostError::Plugin("ShardLoop::pump: malformed frame has trailing bytes; exact bytes retained for terminal close".to_string())));
            }
            Err(error) => {
                return Err(self.retain_terminal_frame(bytes, PluginHostError::Plugin(format!("ShardLoop::pump: malformed frame: {error:?}; exact bytes retained for terminal close"))));
            }
        };
        let target = match &frame {
            ShardFrame::Grant { actor, .. } | ShardFrame::MeshReceived{actor,..} | ShardFrame::Unregister { actor } => Some(*actor),
            ShardFrame::Envelope(envelope) => Some(envelope.to),
            ShardFrame::Register { .. } => None,
        };
        if let Err(limit) = self.preflight_frame(&frame, bytes.len()) {
            let deferred_empty = self.pending_interactive.is_empty() && self.pending_background.is_empty();
            if deferred_empty {
                let byte_len = bytes.len();
                return Err(self.retain_terminal_frame(bytes, PluginHostError::Plugin(format!("ShardLoop: one frame permanently exceeds deferred {limit:?} capacity ({byte_len} bytes); exact frame retained for terminal close"))));
            }
            return Err(FrameAdmissionError::Full { bytes });
        }
        if let Some(actor) = target.filter(|actor| !self.actor_generation_is_current(*actor)) {
            self.send_outcome(&ShardOutcome::Fault { actor: actor.0, message: "actor is not registered on this shard".into() }).await.map_err(FrameAdmissionError::Fault)?;
            return Ok(FrameAdmission::Answered);
        }
        if let Err(error) = self.validate_frame(&frame) {
            return Err(self.retain_terminal_frame(bytes, error));
        }
        match frame {
            ShardFrame::Register { actor } => self.enqueue_authority(semio_framework_actor::Lane::Maintenance, DeferredAuthority::Register { actor }, bytes.len()).map_err(FrameAdmissionError::Fault)?,
            ShardFrame::Unregister { actor } => self.enqueue_authority(semio_framework_actor::Lane::Maintenance, DeferredAuthority::Unregister { actor }, bytes.len()).map_err(FrameAdmissionError::Fault)?,
            ShardFrame::Grant { actor, budget, retained, retained_policy,work_fuel,deadline_us,envelopes } => {
                if envelopes.len()!=1 {return Err(self.retain_terminal_frame(bytes,PluginHostError::NativeIo(IssuedShardTurn::refusal("one Kernel input admits exactly one original export envelope"))))}
                let issued = match IssuedShardTurn::new(budget, retained, retained_policy,work_fuel,deadline_us,envelopes.len()) {
                    Ok(issued) => issued,
                    Err(error) => return Err(self.retain_terminal_frame(bytes, PluginHostError::NativeIo(error))),
                };
                self.granted_budgets.insert(actor.0, issued);
                let item_count = envelopes.len();
                for (index, envelope) in envelopes.into_iter().enumerate() {
                    if let Err(error) = self.dispatch_envelope(envelope, split_frame_credit(bytes.len(), item_count, index)).await {
                        return Err(self.retain_terminal_frame(bytes, error));
                    }
                }
            }
            ShardFrame::MeshReceived{actor,input,receiving_sequence,spent,work_units_spent,accepted}=>{if let Err(error)=self.receive_original_mesh_frame(actor,input,receiving_sequence,spent,work_units_spent,accepted){return Err(self.retain_terminal_frame(bytes,error))}},
            ShardFrame::Envelope(envelope) => {
                if let Err(error) = self.dispatch_envelope(envelope, bytes.len()).await {
                    return Err(self.retain_terminal_frame(bytes, error));
                }
            }
        }
        Ok(FrameAdmission::Deferred)
    }

    fn retain_terminal_frame(&mut self, bytes: Vec<u8>, error: PluginHostError) -> FrameAdmissionError {
        let byte_len = bytes.len();
        match self.terminal_frames.try_push(bytes, byte_len) {
            Ok(_) => FrameAdmissionError::Fault(error),
            Err(rejected) => FrameAdmissionError::TerminalCapacity { bytes: rejected.owner, error },
        }
    }

    fn preflight_frame(&self, frame: &ShardFrame, raw_bytes: usize) -> Result<(), AdmissionLimit> {
        let envelopes: &[Envelope] = match frame {
            ShardFrame::Grant { envelopes, .. } => envelopes,
            ShardFrame::Envelope(envelope) => std::slice::from_ref(envelope),
            ShardFrame::MeshReceived{..}=>return Ok(()),
            ShardFrame::Register { .. } | ShardFrame::Unregister { .. } => return self.pending_background.can_admit(1, raw_bytes),
        };
        let mut interactive_items = 0usize;
        let mut interactive_bytes = 0usize;
        let mut background_items = 0usize;
        let mut background_bytes = 0usize;
        for (index, envelope) in envelopes.iter().enumerate() {
            let credit = split_frame_credit(raw_bytes, envelopes.len(), index);
            if Self::is_high_priority_lane(envelope.lane) {
                interactive_items = interactive_items.saturating_add(1);
                interactive_bytes = interactive_bytes.saturating_add(credit);
            } else {
                background_items = background_items.saturating_add(1);
                background_bytes = background_bytes.saturating_add(credit);
            }
        }
        self.pending_interactive.can_admit(interactive_items, interactive_bytes)?;
        self.pending_background.can_admit(background_items, background_bytes)
    }

    fn validate_frame(&self, frame: &ShardFrame) -> Result<(), PluginHostError> {
        let envelopes: &[Envelope] = match frame {
            ShardFrame::Grant { envelopes, .. } => envelopes,
            ShardFrame::Envelope(envelope) => std::slice::from_ref(envelope),
            ShardFrame::Register { .. } | ShardFrame::Unregister { .. } | ShardFrame::MeshReceived{..} => return Ok(()),
        };
        for envelope in envelopes {
            match &envelope.payload {
                Payload::Event { bytes } => {
                    serde_json::from_slice::<Event>(bytes).map_err(|error| PluginHostError::Json(error.to_string()))?;
                }
                Payload::JobStep { turn } => self.validate_job_turn(envelope.to.0, *turn)?,
                Payload::JobReplay { turn, request, .. } => self.validate_replay_request(envelope.to.0, *turn, *request)?,
                Payload::ContinueOriginalCall|Payload::Suspend { .. } | Payload::Resume { .. } | Payload::Cancel { .. } => {}
            }
        }
        Ok(())
    }

    pub fn enqueue_authority(&mut self, lane: semio_framework_actor::Lane, authority: DeferredAuthority, owner_bytes: usize) -> Result<(), PluginHostError> {
        let authority = AdmittedAuthority::new(self.current_allocation(authority.actor()), self.granted_budget(authority.actor()).copied(), authority, lane, owner_bytes);
        let result = if Self::is_high_priority_lane(lane) { self.pending_interactive.try_push(authority, owner_bytes) } else { self.pending_background.try_push(authority, owner_bytes) };
        if let Err(rejected) = result {
            let _ = self.terminal_authorities.try_push(rejected.owner.authority, owner_bytes).expect("ShardLoop: terminal authority ring owns every rejected authority");
            return Err(PluginHostError::Plugin(format!("ShardLoop: release admission rejected a preflighted {lane:?} authority at {:?}", rejected.limit)));
        }
        Ok(())
    }

    pub(super) fn actor_generation_is_current(&self, actor: ActorId) -> bool {
        self.instances.contains_key(&actor.0) && self.allocations.contains_key(&actor.0)
    }

    /// ✉️ One [`Envelope`]'s payload, dispatched — the exact per-envelope body `pump()` used to run
    /// directly off `Envelope::pack_decode`'s output before `ShardFrame` wrapped it; unchanged
    /// behavior, just reachable from BOTH `ShardFrame::Envelope` and each of `ShardFrame::Grant`'s
    /// bundled envelopes now.
    ///
    /// `Payload::Event`'s bytes are this file's own JSON encoding of `semio_framework::kernel::Event`
    /// — `semio_framework_actor::Payload`'s own doc comment calls this "pack-encoded", the eventual
    /// intended format once `🎠️kernel` grows a `pack_encode`/`pack_decode` for `Event`/`TurnResult`
    /// (not yet built, `🎠️kernel` is out of this packet's `path_scope`); JSON is what every OTHER
    /// wire boundary in this crate already uses (`IoRouter`/`EffectEventMarshal`), so this is a
    /// documented, consistent placeholder, not an invented one-off.
    ///
    /// 🚦 terra-shard-lane piece 1: records the LAST-seen lane for `envelope.to`, covering both
    /// standalone `ShardFrame::Envelope` frames and every envelope bundled inside a
    /// `ShardFrame::Grant` (`Self::consume_frame`'s `Grant` arm calls this per envelope) — see
    /// `Self::actor_lane`'s own doc for why this, not a `ShardFrame::Grant`-level field, is where
    /// the lane classification comes from.
    async fn dispatch_envelope(&mut self, envelope: Envelope, owner_bytes: usize) -> Result<(), PluginHostError> {
        if !self.actor_generation_is_current(envelope.to) {
            return Ok(());
        }
        let actor = envelope.to.0;
        let lane = envelope.lane;
        self.actor_lanes.insert(actor, lane);
        if matches!(envelope.payload,Payload::ContinueOriginalCall){return self.receive_original_event_input(actor)}
        let authority = match envelope.payload {
            Payload::ContinueOriginalCall=>unreachable!("original continuation was received before event dispatch"),
            Payload::Event { bytes: event_bytes } => DeferredAuthority::Event { actor, event:Some(serde_json::from_slice(&event_bytes).map_err(|error| PluginHostError::Json(error.to_string()))?) },
            Payload::JobStep { turn } => DeferredAuthority::JobStep { actor, turn },
            Payload::JobReplay { turn, request, worker_count, worker_slot } => DeferredAuthority::JobReplay { actor, turn, request, worker_count, worker_slot },
            Payload::Suspend { operation, applied_progress } => DeferredAuthority::Suspend { actor, operation, applied_progress },
            Payload::Resume { operation, checkpoint } => DeferredAuthority::Resume { actor, operation, checkpoint },
            Payload::Cancel { seq: _ } => DeferredAuthority::Cancel(CancelCursor { actor, after_job: None, owner_bytes }),
        };
        self.enqueue_authority(lane, authority, owner_bytes)
    }

    pub(super) async fn suspend_one(&mut self, actor_id: u64, operation: JobOperation, applied_progress: u64) -> Result<(), PluginHostError> {
        if !self.actor_generation_is_current(ActorId(actor_id)) {
            return Ok(());
        }
        let outcome = match self.instances.get_mut(&actor_id) {
            None => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Suspend for actor {actor_id} which is not registered on this shard") },
            Some(instance) => match self.identities.get_mut(self.allocations.get(&actor_id).expect("registered actor allocation")).expect("registered actor owns original identity").checkpoint(&self.runtime,instance).await {
                Ok(state) => ShardOutcome::Checkpoint { actor: actor_id, operation, checkpoint: JobCheckpoint { state, applied_progress } },
                Err(error) => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Suspend checkpoint failed for actor {actor_id}: {error}") },
            },
        };
        self.send_outcome(&outcome).await
    }

    pub(super) async fn resume_one(&mut self, actor_id: u64, operation: JobOperation, checkpoint: JobCheckpoint) -> Result<(), PluginHostError> {
        if !self.actor_generation_is_current(ActorId(actor_id)) {
            return Ok(());
        }
        let outcome = match self.instances.get_mut(&actor_id) {
            None => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Resume for actor {actor_id} which is not registered on this shard") },
            Some(instance) => match self.identities.get_mut(self.allocations.get(&actor_id).expect("registered actor allocation")).expect("registered actor owns original identity").restore(&self.runtime,instance,&checkpoint.state).await {
                Ok(()) => ShardOutcome::Resumed { actor: actor_id, operation },
                Err(error) => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Resume restore failed for actor {actor_id}: {error}") },
            },
        };
        self.send_outcome(&outcome).await
    }

    pub(super) async fn cancel_one(&mut self, cursor: CancelCursor, lane: semio_framework_actor::Lane) -> Result<(), PluginHostError> {
        let actor_id = cursor.actor;
        if !self.actor_generation_is_current(ActorId(actor_id)) {
            return Ok(());
        }
        let job = self.running_jobs.range((actor_id, cursor.after_job.map_or(0, |job| job.saturating_add(1)))..).next().copied().filter(|(job_actor, _)| *job_actor == actor_id).map(|(_, job)| job);
        if let Some(job) = job {
            let funds = self.job_funds(actor_id);
            let result = match self.instances.get_mut(&actor_id) {
                Some(instance) => match funds { Ok(mut funds) => self.runtime.cancel_job_funded(instance, &mut funds, job).await, Err(fault) => Err(fault) },
                None => {
                    self.retire_actor_replay_owners(actor_id);
                    self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Cancel for actor {actor_id} lost its registered instance") }).await?;
                    return Ok(());
                }
            };
            if let Err(fault) = result {
                self.unregister(ActorId(actor_id)).await;
                let message = format!("ShardLoop::pump: actor cancel-job {job} failed before retirement: {}", turn_fault_message(&fault));
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message }).await?;
                return Ok(());
            }
            if let Some(index) = self.replay_seeds.iter().position(|seed| seed.as_ref().is_some_and(|seed| seed.actor == actor_id && seed.job == job)) {
                self.begin_replay_seed_close(index, ReplaySeedCloseReason::Cancelled);
            } else {
                self.running_jobs.remove(&(actor_id, job));
                self.job_turns.remove(&(actor_id, job));
                self.job_authorities.remove(&(actor_id, job));
                self.job_placement.remove(&(actor_id, job));
            }
            let authority = DeferredAuthority::Cancel(CancelCursor { actor: actor_id, after_job: Some(job), owner_bytes: cursor.owner_bytes });
            let authority = AdmittedAuthority::new(self.current_allocation(actor_id), self.granted_budget(actor_id).copied(), authority, lane, cursor.owner_bytes);
            let result = if Self::is_high_priority_lane(lane) { self.pending_interactive.try_push(authority, cursor.owner_bytes) } else { self.pending_background.try_push(authority, cursor.owner_bytes) };
            if let Err(rejected) = result {
                let _ = self.terminal_authorities.try_push(rejected.owner.authority, cursor.owner_bytes).expect("ShardLoop: terminal authority ring owns every rejected close cursor");
                return Err(PluginHostError::Plugin(format!("ShardLoop: interrupted close handback rejected at {:?}; exact cursor retained", rejected.limit)));
            }
            return Ok(());
        }
        self.unregister(ActorId(actor_id)).await;
        self.send_outcome(&ShardOutcome::Cancelled { actor: actor_id }).await
    }

    /// 🔐️ Validates a requested turn against the active replay cursor before publication.
    pub(super) fn accept_job_turn(&mut self, actor: u64, turn: JobTurn) -> Result<(), PluginHostError> {
        self.validate_job_turn(actor, turn)?;
        let key = (actor, turn.job);
        self.job_turns.entry(key).or_insert(turn);
        Ok(())
    }

    fn validate_job_turn(&self, actor: u64, turn: JobTurn) -> Result<(), PluginHostError> {
        match self.job_turns.get(&(actor, turn.job)) {
            None if turn.step_sequence == 0 => Ok(()),
            None => Err(PluginHostError::Plugin(format!("ShardLoop::job bridge: first step for actor {actor}, job {} had sequence {}, expected 0", turn.job, turn.step_sequence))),
            Some(active)
                if active.operation.operation == turn.operation.operation
                    && active.operation.base_revision == turn.operation.base_revision
                    && active.operation.generation == turn.operation.generation
                    && active.operation.preview_sequence == turn.operation.preview_sequence
                    && active.operation.seed == turn.operation.seed
                    && active.step_sequence == turn.step_sequence =>
            {
                Ok(())
            }
            Some(active) => Err(PluginHostError::Plugin(format!("ShardLoop::job bridge: stale or non-deterministic turn for actor {actor}, job {}: active={active:?}, requested={turn:?}", turn.job))),
        }
    }

    pub async fn send_outcome(&mut self, outcome: &ShardOutcome) -> Result<(), PluginHostError> {
        let mut bytes = Vec::new();
        outcome.pack_encode(&mut bytes).await.map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        self.transport.send(&bytes).await;
        Ok(())
    }

    pub async fn heartbeat(&self) -> u64 {
        self.transport.heartbeat().await
    }
}

//! 🔄️ Exchange and turn settling: the exchange, the command exchange, the turn run and settle, the cancelled turn retirement, the serving compile and the reserved job admission. See [`super`].

use super::*;
use semio_framework_os_renderer_wgpu_kernel_protocol::CreateAppSourceFailureKind;

//#region 🎠️KernelRuntime
//#region 🔖️KernelPoolState
impl KernelPoolState {
    pub(super) fn close_realm_progress_step(&mut self) -> bool {
        if let Some(index) = self.product_replay_claims.iter().position(Option::is_some) {
            self.product_replay_claims[index].take().expect("realm product replay claim remains exact").retire();
            return false;
        }
        if let Some(index) = self.product_replay_authorities.iter().position(Option::is_some) {
            self.product_replay_authorities[index].take().expect("realm product replay authority remains exact").retire();
            return false;
        }
        if mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_any() {
            let _ = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false);
            return false;
        }
        if mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work() {
            let _ = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one();
            return false;
        }
        if let Some(index) = self.pending_job_progress_presentations.iter().position(Option::is_some) {
            let token = self.pending_job_progress_presentations[index].as_ref().expect("realm pending presentation").token;
            if !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").cancel(token) {
                return false;
            }
            let pending = self.pending_job_progress_presentations[index].take().expect("cancelled realm presentation");
            let actor = pending.receipt.identity().actor;
            if self.job_progress.abort(pending.receipt).is_err() {
                let _ = self.job_progress.begin_close_actor(actor);
            }
            return false;
        }
        if let Some(entry) = self.job_replays.iter_mut().flatten().find(|entry| !entry.log.close_started()) {
            entry.log.begin_close();
            return false;
        }
        if self.job_replays.iter().any(Option::is_some) {
            let _ = self.command_maintenance_step();
            return false;
        }
        if let Some(index) = self.replay_routes.iter().position(Option::is_some) {
            self.replay_routes[index] = None;
            return false;
        }
        if !self.realm_progress_close_started {
            self.job_progress.begin_close_all();
            self.realm_progress_close_started = true;
            return false;
        }
        if !self.job_progress.terminal_is_empty() || self.rejected_job_progress.iter().any(Option::is_some) || !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").terminal_is_empty() {
            let _ = self.command_maintenance_step();
            return false;
        }
        if !self.retained.advance_realm_close_one() {
            return false;
        }
        if self.pending_rejections.take_any_one().is_some() {
            return false;
        }
        if !self.pending_rejections.terminal_is_empty() {
            return false;
        }
        self.realm_progress_close_started = false;
        true
    }

    /// 🎫️ `Event::PatchRejected` carries the issued-patch authority the guest minted with the
    /// patch. A rejection with no authority behind it (a surface the registry could not admit
    /// at all, so no turn ever reached a slot) has nothing to reject back and stays a local
    /// registry entry rather than a fabricated receipt.
    pub(super) async fn exchange(&mut self, instance: u32, mut events: Vec<Event>) -> Result<ExchangeOutcome, ProgramFault> {
        let Some(&actor) = self.instances.get(&instance) else {
            return Err((format!("kernel: instance {instance} is not registered")).into());
        };
        if let Some(rejection) = self.pending_rejections.take_instance_one(instance) {
            if let Some(receipt) = rejection.receipt {
                events.insert(0, Event::PatchRejected { receipt, surface: rejection.surface.0.as_str().to_string(), revision: rejection.revision.0, reason: rejection.reason.to_string() });
            }
        }
        let mut outcome = self.run_turn(actor, instance, events).await?;
        self.settle_reserved_jobs(actor, instance, &mut outcome).await?;
        Ok(outcome)
    }

    pub(super) async fn exchange_commands(&mut self, instance: u32, driver: semio_framework::kernel::CommandBatchDriver) -> Result<ExchangeOutcome, ProgramFault> {
        let key = u64::from(instance);
        let generation = driver.generation();
        if !self.retained_command_closes.terminal_is_empty() {
            if !self.queued_command_closes.can_insert(key) {
                return Err(("kernel: retained and queued command close registries are saturated; caller owner remains in the queue close lane".to_string()).into());
            }
            self.queued_command_closes.insert_admitted(key, generation, driver);
            self.queued_command_closes.begin_close(key, generation).map_err(|fault| fault.describe())?;
            let _ = self.command_maintenance_step();
            return Err(("kernel: previous cancelled command owner is closing; incoming exact batch moved to the queued close lane".to_string()).into());
        }
        if !self.retained_command_closes.can_insert(key) {
            return Err(("kernel: retained command close registry is saturated or collided".to_string()).into());
        }
        self.retained_command_closes.insert_admitted(key, generation, driver);
        let Some(&actor) = self.instances.get(&instance) else {
            self.retained_command_closes.begin_close(key, generation).map_err(|fault| fault.describe())?;
            let _ = self.command_maintenance_step();
            return Err((format!("kernel: instance {instance} is not registered; exact command owner entered bounded close")).into());
        };
        let mut combined = ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle, typed_results: Vec::new() };
        loop {
            let mut destinations = match self.command_document_closes.try_reserve_page(key, generation) {
                Ok(destinations) => destinations,
                Err(()) => {
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err(("kernel: command document destinations saturated before the next turn; retained batch and exact documents entered bounded close".to_string()).into());
                }
            };
            let page = self.retained_command_closes.with_driver_mut(key, generation, |driver| driver.next_page());
            let page = match page {
                Ok(Ok(page)) => page,
                Ok(Err(fault)) => {
                    self.command_document_closes.release(&mut destinations);
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err((fault.describe()).into());
                }
                Err(fault) => {
                    self.command_document_closes.release(&mut destinations);
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err((fault.describe()).into());
                }
            };
            let events = match page {
                Some((cursor, bytes)) => vec![Event::CommandIngressPage { cursor, bytes }],
                None => vec![Event::Wake],
            };
            if let Err(fault) = self.retained_command_closes.prepare_suspend(key, generation) {
                self.command_document_closes.release(&mut destinations);
                self.command_document_closes.begin_close_batch(key, generation);
                let _ = self.retained_command_closes.begin_close(key, generation);
                let _ = self.command_maintenance_step();
                return Err((fault.describe()).into());
            }
            let outcome = match self.run_turn(actor, instance, events).await {
                Ok(outcome) => outcome,
                Err(fault) => {
                    self.command_document_closes.release(&mut destinations);
                    let _ = self.retained_command_closes.resume(key, generation);
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err((fault).into());
                }
            };
            let ExchangeOutcome { frames, surfaces, effects, command_ingress, typed_results } = outcome;
            for surface in surfaces {
                self.command_document_closes.admit(&mut destinations, surface);
            }
            self.command_document_closes.release(&mut destinations);
            if let Err(fault) = self.retained_command_closes.resume(key, generation) {
                self.command_document_closes.begin_close_batch(key, generation);
                let _ = self.retained_command_closes.begin_close(key, generation);
                let _ = self.command_maintenance_step();
                return Err((fault.describe()).into());
            }
            let progress = match self.retained_command_closes.with_driver_mut(key, generation, |driver| driver.observe(&command_ingress, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)) {
                Ok(Ok(progress)) => progress,
                Ok(Err(fault)) => {
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err((fault.describe()).into());
                }
                Err(fault) => {
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err((fault.describe()).into());
                }
            };
            combined.frames.extend(frames);
            combined.effects.extend(effects);
            combined.typed_results.extend(typed_results);
            combined.command_ingress = command_ingress;
            match progress {
                semio_framework::kernel::CommandBatchProgress::Complete => {
                    if let Err(fault) = self.retained_command_closes.remove_terminal(key, generation) {
                        self.command_document_closes.begin_close_batch(key, generation);
                        let _ = self.retained_command_closes.begin_close(key, generation);
                        let _ = self.command_maintenance_step();
                        return Err((fault.describe()).into());
                    }
                    self.command_document_closes.publish_batch(key, generation, &mut combined.surfaces);
                    self.settle_reserved_jobs(actor, instance, &mut combined).await?;
                    return Ok(combined);
                }
                semio_framework::kernel::CommandBatchProgress::Faulted => {
                    self.command_document_closes.begin_close_batch(key, generation);
                    self.retained_command_closes.begin_close(key, generation).map_err(|fault| fault.describe())?;
                    let _ = self.command_maintenance_step();
                    return Err(("kernel: command ingress faulted; exact driver and surface-document owners entered incremental close".to_string()).into());
                }
                semio_framework::kernel::CommandBatchProgress::PageReady | semio_framework::kernel::CommandBatchProgress::Waiting => {}
            }
        }
    }

    /// ⏯️ One request's turn for `actor`, settled the way the React host settles one
    /// (`🔌️PluginRuntime`'s `settlePluginTurn`, ticket 26/09/23 slice WG8, B1):
    ///
    /// - a grant the shard ended with the guest mid-call ([`ShardOutcome::Preempted`]) is resumed
    ///   inside [`Self::run_turn_once`] before anything else reaches the actor, so this method never
    ///   hands a mid-flight guest back to its caller;
    /// - a settled turn that published an `ActorInstanceLifecycleReceipt` is acknowledged on the next
    ///   turn (an open the host never acknowledges is re-offered on every later turn);
    /// - a completed turn answering `MoreWork` is continued with empty-event turns while its command
    ///   and cold-pair ingress are idle — a non-idle ingress belongs to the caller's own page driver
    ///   ([`Self::exchange_commands`] observes every ingress status, so a settle that swallowed one
    ///   would resend a page the guest already owns: measured as block2d's `addHandleKind` spinning
    ///   `MoreWork` until the settle budget, ticket 26/09/18 slice G7w run 13);
    /// - it stops once [`run_turn_quiescent_continuations`] continuations in a row carried nothing,
    ///   because a guest waiting on a host round trip (a typed-operation ACK, a backbone reply) is
    ///   answering `MoreWork` for something no empty turn can deliver.
    ///
    /// 🧵️ Events cross ONE per granted turn and each settles before the next is submitted: the shard
    /// executes one event per guest turn anyway, and a batch's later envelopes must not reach a guest
    /// the first one left owning ingress. [`RUN_TURN_SETTLE_BUDGET`] bounds the whole request.
    pub(super) async fn run_turn(&mut self, actor: ActorId, instance: u32, events: Vec<Event>) -> Result<ExchangeOutcome, ProgramFault> {
        let deadline = std::time::Instant::now() + RUN_TURN_SETTLE_BUDGET;
        let mut batches = events.into_iter().map(|event| vec![event]).collect::<std::collections::VecDeque<_>>();
        if batches.is_empty() {
            batches.push_back(Vec::new());
        }
        let mut settled: Option<ExchangeOutcome> = None;
        while let Some(batch) = batches.pop_front() {
            let first = self.run_turn_once(actor, instance, batch).await?;
            let outcome = self.settle_turn(actor, instance, first, deadline).await?;
            match settled.as_mut() {
                Some(earlier) => earlier.absorb(outcome)?,
                None => settled = Some(outcome),
            }
        }
        settled.ok_or_else(|| ProgramFault::from("kernel: a turn with no batch produced no outcome"))
    }

    /// 🔁️ Continues one granted turn with what it still owes (see [`Self::run_turn`]) until it settled,
    /// [`run_turn_quiescent_continuations`] continuations in a row carried nothing, or `deadline` passed.
    async fn settle_turn(&mut self, actor: ActorId, instance: u32, (mut outcome, continuation): (ExchangeOutcome, Option<Vec<Event>>), deadline: std::time::Instant) -> Result<ExchangeOutcome, ProgramFault> {
        let mut owed = run_turn_continuation_turns(continuation);
        let mut quiet = 0usize;
        while let Some(events) = owed.pop_front() {
            if std::time::Instant::now() >= deadline || quiet >= run_turn_quiescent_continuations() {
                break;
            }
            let (later, next) = self.run_turn_once(actor, instance, events).await?;
            quiet = if later.carries_nothing() { quiet + 1 } else { 0 };
            outcome.absorb(later)?;
            owed.extend(run_turn_continuation_turns(next));
        }
        Ok(outcome)
    }

    /// 🧰️ Drives every live [`ReservedToolJob`] of `actor` to its end once the request that spawned it
    /// settled, folding what the drive produced into `outcome`: the native host's half of the one
    /// reserved-job mechanism (React: `driveReservedToolJob`; guest: `admit_reserved_spawned_job`, ticket
    /// 26/09/23 slices C8 and WG8 §1.4).
    ///
    /// - The shard started the job when it admitted the `Effect::SpawnJob` and reported its exact
    ///   [`JobTurn`] with that turn ([`ShardOutcome::Turn`]'s `jobs`); one `Payload::JobStep` is granted
    ///   per turn until a step publication ends the job.
    /// - The shard then runs the guest's deferred `Event::JobCompleted` turn by itself; this host waits
    ///   for it and settles it like any turn, so the guest commits the job one unit per `MoreWork`
    ///   continuation and answers the verb (`Invocation { in_reply_to: 0 }`, `OperationCompleted`).
    async fn settle_reserved_jobs(&mut self, actor: ActorId, instance: u32, outcome: &mut ExchangeOutcome) -> Result<(), ProgramFault> {
        while self.reserved_jobs.iter().flatten().any(|job| job.actor == actor) {
            if self.turn_cancelled() {
                return Err((self.retire_cancelled_turn(actor).await).into());
            }
            let first = self.run_reserved_job_once(actor, instance).await?;
            let later = self.settle_turn(actor, instance, first, std::time::Instant::now() + RUN_TURN_SETTLE_BUDGET).await?;
            outcome.absorb(later)?;
            if !self.reserved_jobs.iter().flatten().any(|job| job.actor == actor && job.step.is_none()) {
                self.yield_slice(actor, instance).await;
            }
        }
        Ok(())
    }

    /// 🛑️ Whether the turn being driven must end now: its requester cancelled it (a cancellable request whose
    /// response slot was abandoned — a document open's create) or the realm is closing (a realm close waits
    /// at the head of the queue). Nothing else ends a turn: a slow guest is never killed by wall-clock time.
    pub(super) fn turn_cancelled(&self) -> bool {
        self.serving.last().is_some_and(|(slot, cancellable)| *cancellable && slot.abandoned()) || self.request_queue.head_is(|request| matches!(request, KernelRequest::CloseRealm { .. }))
    }

    /// ✂️ Retires an actor whose turn was cancelled mid-call: a mid-flight guest can never be handed back,
    /// so its shard instance is dropped, the scheduler stops granting it, and every host record of it goes.
    /// Answers the error its request reports.
    pub(super) async fn retire_cancelled_turn(&mut self, actor: ActorId) -> ProgramFault {
        let _ = self.runtime.kernel_mut().suspend(actor, None).await;
        if let Err(error) = self.runtime.unregister(actor).await { return error.into(); }
        self.instances.retain(|_, instance_actor| *instance_actor != actor);
        for route in self.replay_routes.iter_mut().filter(|route| route.is_some_and(|route| route.actor == actor)) {
            *route = None;
        }
        for job in self.reserved_jobs.iter_mut().filter(|job| job.is_some_and(|job| job.actor == actor)) {
            *job = None;
        }
        self.begin_fault_close(actor);
        #[cfg(any(test, feature = "testkit"))]
        CANCELLED_KERNEL_TURNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("kernel: actor {}'s turn was cancelled by its owner", actor.0).into()
    }

    /// 🧩️ Compiles `bytes` on the worker pool while this loop keeps serving the other instances' requests — a
    /// large component parses for minutes in a debug host, and the synchronous parse used to hold the whole
    /// loop — reusing the handle already compiled for the same content hash. The open's owner may cancel it
    /// while the initiating original compiler caller retains the sealed future, source loan and complete result through publication.
    pub(super) async fn compile_serving_others(&mut self,owner:&CreateAppRequestOwner,hash:PackageHash,bytes:semio_framework_job::OriginalSuppliedJobBytes)->Result<semio_framework_plugin_host::CompiledHandle,CreateAppSourceFailureKind>{
        if let Some((_,compiled))=self.compiled_components.iter().find(|(key,_)|*key==hash){return Ok(compiled.clone())}
        let mut original=Some(bytes);let mut compiled=None;
        loop{
            let gap=std::future::poll_fn(|cx|{
                if self.turn_cancelled(){owner.cancel_compiler();return Poll::Ready(Some(Err(CreateAppSourceFailureKind::Cancelled)))}
                match owner.start_compiler(&self.guest_runtime,hash,&mut original){Ok(true)=>{},Ok(false)=>{cx.waker().wake_by_ref();return Poll::Pending},Err(kind)=>return Poll::Ready(Some(Err(kind)))}
                match owner.poll_compiler(cx.waker(),&self.guest_runtime,&mut compiled){Ok(true)=>return Poll::Ready(Some(Ok(compiled.take().expect("original compiler result transferred once after paid frame close")))),Ok(false)=>{},Err(kind)=>return Poll::Ready(Some(Err(kind)))}
                if !self.interleaving&&self.request_queue.head_is(|request|kernel_request_interleavable(request,u32::MAX)){return Poll::Ready(None)}
                self.request_queue.watch(cx.waker());Poll::Pending
            }).await;
            match gap{Some(result)=>{let compiled=result?;if self.compiled_components.len()==COMPILED_COMPONENT_CAPACITY{self.compiled_components.pop();}self.compiled_components.insert(0,(hash,compiled.clone()));return Ok(compiled)},None=>self.serve_head_between(u32::MAX).await}
        }
    }

    /// ⚖️ Serves the head request when it may run beside work mid-flight on instance `busy`, marking the gap so a
    /// request served in it never opens another.
    async fn serve_head_between(&mut self, busy: u32) {
        let Some((request, slot)) = self.request_queue.try_next_if(|request| kernel_request_interleavable(request, busy)) else { return };
        self.interleaving = true;
        let outcome = self.serve(request, &slot).await;
        self.interleaving = false;
        if let Some(outcome) = outcome {
            slot.deliver(outcome);
        }
    }

    /// ⚖️ Gives the other actors one request between two slices of `actor`'s turn on instance `busy` (the
    /// fairness contract's `requestsBetweenSlices`): the head of the queue runs when it may
    /// ([`kernel_request_interleavable`]) while `actor` is suspended in the scheduler, so no grant of that
    /// request can reach the mid-flight guest; then `actor` is resumed. A request running in a slice gap
    /// never opens another gap.
    pub(super) async fn yield_slice(&mut self, actor: ActorId, busy: u32) {
        if self.interleaving || !self.request_queue.head_is(|request| kernel_request_interleavable(request, busy)) {
            return;
        }
        let suspended = self.runtime.kernel_mut().suspend(actor, None).await.is_ok();
        for _ in 0..TURN_REQUESTS_BETWEEN_SLICES {
            self.serve_head_between(busy).await;
        }
        if suspended {
            let _ = self.runtime.kernel_mut().resume(actor).await;
        }
    }

    /// 🧾️ Registers every reserved-kind `Effect::SpawnJob` of `actor`'s settled turn with the
    /// shard-minted [`JobTurn`] its admission reported; a spawn the shard refused has none and only owes
    /// its deferred `Event::JobCompleted` refusal.
    pub(super) fn admit_reserved_jobs(&mut self, actor: ActorId, effects: &[Effect], jobs: &[JobTurn]) -> Result<(), String> {
        for effect in effects {
            let Effect::SpawnJob { job, kind, .. } = effect else { continue };
            if kind != semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND {
                continue;
            }
            let slot = self.reserved_jobs.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| format!("kernel: actor {}'s reserved tool job {job} exceeds {JOB_PROGRESS_ACTIVE_CAPACITY} live reserved jobs", actor.0))?;
            *slot = Some(ReservedToolJob { actor, job: *job, step: jobs.iter().find(|turn| turn.job == *job).copied() });
        }
        Ok(())
    }

}
//#endregion
//#endregion 🎠️KernelRuntime

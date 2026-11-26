/** 🚚️ Phases 6 and 7 of `loadPluginModule`: the spawned-job driver with host-effect routing, and the turn pump with the typed-operation continuation drain.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { decodeAppFrame, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { FRAMEWORK_RESERVED_JOB_KIND, SemioFaultError } from "@semio-tech/framework";
import { type PluginDispatchHintV1 } from "@semio-tech/framework";
import { createShardCommandIngressPages, type ShardEventEnvelope, type ShardJobStep } from "@semio-tech/framework/actor/shard-client/wire";
import { hostContinuations } from "@semio-tech/framework/async/continuation";
import { drainTypedOperationBatches, wireTurnStatusTag } from "@semio-tech/framework/actor/wire-turn";
import { type ActorDocumentMessagePortV1 } from "@semio-tech/framework/backbone/binding";
import { beginSpawnedJobV1 } from "../💼️job-ledger/🟦️.ts";
import {
  coerceWireBytes,
  commandIngressFaultDisplay,
  documentPortHotBytes,
  shellFrameBytes,
  type WireTurnResult,
  type WireVariant
} from "../🪝️wire-codec/🟦️.ts";
import { createOriginalMediaExchange } from "./original-media-exchange.ts";
import { originalMediaRequest } from "./original-media-wire.ts";
import { publishOperationFault, typedOperationAcknowledgements, withTypedOperationCall, type TypedOperationCall } from "../📬️typed-result/🟦️.ts";
import {
  beginIsolatedJobDrive,
  DEFAULT_SHARD_BUDGET,
  endIsolatedJobDrive,
  isolatedJobStepsPerSerializedAdmission,
  isolatedJobUiPollEverySteps,
  requestIsolatedJobUiPoll
} from "../🛰️runtime-singletons/🟦️.ts";
import { commandStallWatch, serializeCommandIngressForActor } from "../🧵️turn-dispatch/🟦️.ts";
import { commandStalledMessageV1, isCommandStalledFaultV1 } from "../⏱️command-stall/🟦️.ts";
import { compactAcknowledgedTurns } from "../🔁️turn-settle/📥️acknowledged/🟦️.ts";
import { commandIngressContinuationCeilingV1, commandIngressNeedsReplyStampV1, commandIngressUnownedV1, PLUGIN_UI_CONTINUATION_LIMIT, settleAcknowledgedPluginTurns, settlePluginTurn } from "../🔁️turn-settle/🟦️.ts";
import { yieldPluginUiContinuation } from "../⏩️continuation-yield/🟦️.ts";
import {
  pendingCompletionEffects,
  pendingTurnEffects,
  PLUGIN_OPERATION_DRAIN_BUDGET,
  PLUGIN_OPERATION_EFFECT_CAPACITY,
  PLUGIN_OPERATION_WAKE_MAX_MS
} from "../🏁️operation-completion/🟦️.ts";
import { encodedFrameReplySequence, inspectEncodedAppCommand, promoteShellSendMessages } from "../📥️invocation-frames/🟦️.ts";
import { createInstanceTable, type PluginHandleScope } from "../🪢️handle-scope/🟦️.ts";
import { createRetainedUiIntake } from "../🧺️handle-ui-intake/🟦️.ts";

//#region 🔖️plugin-runtime
/** 🎞️ The spawned-job progress lane of Phase 6: listeners per instance and the per-job coalescing clock. */
function createSpawnedJobProgressLane() {
  /** 🎞️ Coalesces a running job's progress into the shell's operation-progress refresh lane: at most one
   * full refresh per {@link PLUGIN_JOB_PROGRESS_REFRESH_MS} per job, so a job publishing on every slice
   * repaints at a bounded cadence while a job that reports nothing costs no repaint at all. */
  const PLUGIN_JOB_PROGRESS_REFRESH_MS = 120;
  /** 🪙️ How many times its own cost a progress refresh leaves to the job before the next one, so showing progress takes at most a fifth of the drive. */
  const PLUGIN_JOB_PROGRESS_SHARE = 4;
  const nextJobProgressRefreshMs = new Map<string, number>();
  const spawnedJobProgressListeners = new Map<number, Set<() => void>>();
  const subscribeSpawnedJobProgress = (instanceId: number, listener: () => void): (() => void) => {
    const listeners = spawnedJobProgressListeners.get(instanceId) ?? new Set<() => void>();
    listeners.add(listener);
    spawnedJobProgressListeners.set(instanceId, listeners);
    return () => {
      listeners.delete(listener);
      if (listeners.size === 0) spawnedJobProgressListeners.delete(instanceId);
    };
  };
  const notifySpawnedJobProgress = (instanceId: number): void => {
    for (const listener of [...(spawnedJobProgressListeners.get(instanceId) ?? [])]) {
      try { listener(); }
      catch (error) {  }
    }
  };
  const publishSpawnedJobProgress = (instanceId: number, key: string): boolean => {
    const now = performance.now();
    if (now < (nextJobProgressRefreshMs.get(key) ?? -Infinity)) return false;
    nextJobProgressRefreshMs.set(key, now + PLUGIN_JOB_PROGRESS_REFRESH_MS);
    notifySpawnedJobProgress(instanceId);
    return true;
  };
  /** ⏳️ The lane time a batch waited is what the refreshes before it cost the job; the next one is held back by a multiple of it. */
  const settleSpawnedJobLaneWait = (key: string, waitedMs: number): void => {
    nextJobProgressRefreshMs.set(key, Math.max(nextJobProgressRefreshMs.get(key) ?? -Infinity, performance.now() + PLUGIN_JOB_PROGRESS_SHARE * waitedMs));
  };
  /** 🏁️ A job's terminal is its last progress: once its completion turn has installed the surfaces it dirtied, the shell repaints them
   * unthrottled. That turn routes no shell frame, so without this the final frame waited for an unrelated later refresh. */
  const publishSpawnedJobTerminal = (instanceId: number, key: string): void => {
    nextJobProgressRefreshMs.delete(key);
    notifySpawnedJobProgress(instanceId);
  };
  return { nextJobProgressRefreshMs, subscribeSpawnedJobProgress, publishSpawnedJobProgress, settleSpawnedJobLaneWait, publishSpawnedJobTerminal };
}

/** 🛑️ Consecutive `backpressure` answers while idle (no deferred events, no command owner, nothing to advance) after which a command page is refused for good: offering it again cannot change the answer, and spinning holds the actor's whole command lane, which also starves its job slices. Measured 2026-11-24: healthy refusals repeat a few times at most. */
export const PAGE_IDLE_REFUSAL_LIMIT = 256;

/** 💼️ Phase 6 of {@link loadPluginModule}: the host half of `Effect::SpawnJob` (isolated and reserved-tool job drives),
 * the shell-facing spawned-job progress lane and the one choke point every turn's host effects pass through.
 * `turns` is the late-bound door back into the turn pump: a job completion re-arms the typed-operation drain, and
 * the pump in turn routes its effects through this driver. */
export function createSpawnedJobDriver(scope: PluginHandleScope, instances: ReturnType<typeof createInstanceTable>, ui: ReturnType<typeof createRetainedUiIntake>, turns: { readonly drainTypedOperations: (instanceId: number) => Promise<void> }) {
  const { pluginId, shardClient, state, closingInstances, actorIdByInstance, documentBindings, turnOutcomes } = scope;
  const { requireActorId, submitTurn } = instances;
  const { acceptUiPatches } = ui;
  const { nextJobProgressRefreshMs, subscribeSpawnedJobProgress, publishSpawnedJobProgress, settleSpawnedJobLaneWait, publishSpawnedJobTerminal } = createSpawnedJobProgressLane();
  /** 💼️ In-flight `spawn-job` drives, keyed `actorId#job`. A guest re-emitting the same `job` id
   * (a replayed turn, a checkpoint restore) must never open a second `start-job` for one owner. */
  const drivingJobs = new Set<string>();
  const finishedReservedJobs = new Set<string>();
  /** 🏃️ The jobs the guest still owns: admitted by `start-job` and not yet terminal, in the actor's own ingress order. */
  const runningJobs = new Set<string>();
  /** 🛑️ Cancels a job only while the guest owns it, as `ShardLoop::pump` does with its `running_jobs`: a terminal reached ahead
   * of the cancel in the actor's ingress order already released the job, and the guest refuses a cancel without an owner. */
  const cancelOwnedJob = (actorId: string, job: bigint): Promise<boolean> => serializeCommandIngressForActor(actorId, async () => {
    const key = `${actorId}#${job}`;
    if (!runningJobs.has(key)) return false;
    await shardClient.cancelJob(actorId, job);
    runningJobs.delete(key);
    return true;
  });
  const cancelSpawnedJob = (instanceId: number, actorId: string, job: bigint): void => {
    void cancelOwnedJob(actorId, job).catch(error => turnOutcomes.push({ instanceId, error }));
  };
  /** 🧹️ A drive that ends before its job's terminal (its instance closed, or a step failed) cancels the job it still owns:
   * an orphaned guest job is never stepped, closed or released again, so the actor never quiesces and its stock is never renewed
   * (`🎭️actor/🎟️retained-turn` `OriginalStockSuccession`). */
  const releaseSpawnedJob = (instanceId: number, actorId: string, job: bigint): void => {
    if (state.disposing) runningJobs.delete(`${actorId}#${job}`);
    else cancelSpawnedJob(instanceId, actorId, job);
  };
  const pendingReservedJobDrives: Promise<void>[] = [];
  const flushReservedJobDrives = async (): Promise<void> => {
    const batch = pendingReservedJobDrives.splice(0);
    if (batch.length === 0) return;
    await Promise.all(batch);
  };

  /** ⛽️ The budget one `step-job` is granted. `fuel` is a WIT `u64`, hence a `bigint` — the same
   * reshaping `ShardLoop::pump` does natively (`🖥️host/🧵️shard/🦀️.rs:146-147`,
   * `job_budget_from_grant`), so a job slice on this target costs what it costs on the native one. */
  const jobStepBudget = { fuel: BigInt(DEFAULT_SHARD_BUDGET.fuel), deadlineMs: DEFAULT_SHARD_BUDGET.wallMs } as const;

  /** 📏️ Steps driven back to back before yielding the main thread, so a long plan never blocks a
   * frame. `yieldPluginUiContinuation` is deliberately not a `setTimeout` chain (its own doc: a
   * hidden tab throttles those to one tick a second). */
  const PLUGIN_JOB_STEPS_PER_YIELD = 32;

  /** 🏁️ Feeds one job's terminal back into the guest as `events::job-completed` — the event
   * `⚛️reactor/🔄️turn/🦀️.rs:327-344` resolves the guest's own parked `spawn_job` request with
   * (`RequestId(job)` IS the job id). Without this the guest waits forever on a job the host already
   * finished. `job-progress` is deliberately NOT delivered per slice: the guest ignores its payload
   * (`⚛️reactor/🔄️turn/🦀️.rs:317` only marks the surface dirty) and one whole actor turn per slice
   * would cost more than the repaint is worth. */
  const deliverJobCompletionTurn = async (instanceId: number, actorId: string, job: bigint, outcome: ShardJobStep): Promise<void> => {
    if (outcome.status === "running") throw new Error("plugin.job-completion-not-terminal");
    const val = Array.from(outcome.value);
    const payload = { job, outcome: outcome.status === "done" ? { tag: "ok", val } : { tag: "fault", val } };
    const activation = shardClient.captureActorActivation(actorId);
    const settled = await withTypedOperationCall(actorId, `job-completion#${instanceId}`, async (call) => {
      activation.assertActive();
      return settlePluginTurn(actorId, await submitTurn(actorId, [{ kind: "job-completed", payload }], { lane: "Background", activation }), "Background", new Set(), (turn) => acceptUiPatches(instanceId, turn), true, activation, call);
    });
    const frames: Uint8Array[] = [];
    const leftover: WireVariant[] = [];
    for (const effect of routeHostEffects(instanceId, settled.effects, documentBindings.get(instanceId)?.port)) {
      const frame = shellFrameBytes(effect, instanceId);
      if (frame) frames.push(frame);
      else leftover.push(effect);
    }
    const pending = frames.some(frame => "OperationCompleted" in decodeAppFrame(frame)) ? pendingCompletionEffects : pendingTurnEffects;
    pending.set(instanceId, [...(pending.get(instanceId) ?? []), ...leftover]);
    if (frames.length > 0) turnOutcomes.push({ instanceId, frames });
    if (wireTurnStatusTag(settled.status) === "more-work") hostContinuations.schedule(() => void turns.drainTypedOperations(instanceId), 0, `operation-wake#${instanceId}`);
  };
  const deliverJobCompletion = async (instanceId: number, actorId: string, job: bigint, outcome: ShardJobStep): Promise<void> => {
    await serializeCommandIngressForActor(actorId, () => deliverJobCompletionTurn(instanceId, actorId, job, outcome));
    await flushReservedJobDrives();
  };

  /** 💼️ The host half of `Effect::SpawnJob` on this target: `start-job` once, then one `step-job`
   * per turn of this loop until the guest answers `done`/`failed`, then one `job-completed` event
   * back into the actor. This is the browser counterpart of `ShardLoop::pump`'s `running_jobs` walk
   * (`🔌️plugin/🖥️host/🧵️shard/🦀️.rs:359`, :1714-1765). Before ticket 26/09/02 W-J nothing on this
   * target started or stepped an isolated job at all — the effect was dropped unmapped in
   * `wireEffectToFriendly`, so every plugin-authored job silently never ran.
   *
   * ⛽️ A job is bounded PER SLICE and never over its lifetime, exactly as `ShardLoop::pump` bounds it:
   * every `step-job` carries {@link jobStepBudget} (`job_budget_from_grant`'s web twin) and the loop
   * ends only when the guest's own `BoundedJob::step` answers `done`/`failed`, or when the instance
   * this drive belongs to goes away. Ticket 26/09/02/PUZZLE-3D-END-TO-END W-J shipped a fixed
   * 65 536-step LIFETIME cap here instead: a real `semio.puzzle3d.fill` plan (up to
   * `PUZZLE3D_FILL_COUNT_MAX = 1000` placements over a whole document census) reaches that in ~40 s of
   * ticking, and the cancel it then issued tore the guest's live envelope down mid-run — browser-
   * measured 2026-09-10 as `unreachable` → `shard 0 lost` → `actor-activation.revoked` on every later
   * call. A host may not decide a plan is over; only its owner may. */

  const driveSpawnedJob = async (instanceId: number, actorId: string, job: bigint, kind: string, input: Uint8Array): Promise<void> => {
    const key = `${actorId}#${job}`;
    if (drivingJobs.has(key)) return;
    drivingJobs.add(key);
    const live = (): boolean => !state.disposing && !closingInstances.has(instanceId) && actorIdByInstance.get(instanceId) === actorId;
    const ledgerEntry = beginSpawnedJobV1({
      key,
      pluginId,
      actorId,
      instanceId,
      job,
      kind,
      startedAtMs: Date.now(),
      cancel: () => cancelSpawnedJob(instanceId, actorId, job),
    });
    try {
      beginIsolatedJobDrive();
      await serializeCommandIngressForActor(actorId, async () => {
        await shardClient.startJob(actorId, job, kind, input);
        runningJobs.add(key);
      });
      let step = 0;
      while (live()) {
        const batch = isolatedJobStepsPerSerializedAdmission(PLUGIN_JOB_STEPS_PER_YIELD);
        const progressed = { value: false };
        const requestedMs = performance.now();
        const outcome = await serializeCommandIngressForActor(actorId, async () => {
          settleSpawnedJobLaneWait(key, performance.now() - requestedMs);
          let last: ShardJobStep | undefined;
          for (let i = 0; i < batch; i += 1) {
            last = await shardClient.stepJob(actorId, job, jobStepBudget);
            step += 1;
            if (last.status === "running" && last.progress && last.progress.byteLength > 0) progressed.value = true;
            if (last.status !== "running") {
              runningJobs.delete(key);
              return last;
            }
          }
          return last ?? { status: "failed" as const, value: new TextEncoder().encode("plugin.job-step-empty") };
        });
        if (outcome.status !== "running") {
          if (live()) {
            await deliverJobCompletion(instanceId, actorId, job, outcome);
            if (live()) publishSpawnedJobTerminal(instanceId, key);
          }
          return;
        }
        ledgerEntry.step(step, progressed.value);
        const published = progressed.value && live() && publishSpawnedJobProgress(instanceId, key);
        if (published || isolatedJobUiPollEverySteps(step)) requestIsolatedJobUiPoll();
        await yieldPluginUiContinuation();
      }
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
      try {
        if (live() && await cancelOwnedJob(actorId, job) && live()) await deliverJobCompletion(instanceId, actorId, job, { status: "failed", value: new TextEncoder().encode("plugin.job-drive-failed") });
      } catch (failure) {
        turnOutcomes.push({ instanceId, error: failure });
      }
    } finally {
      ledgerEntry.end();
      endIsolatedJobDrive();
      releaseSpawnedJob(instanceId, actorId, job);
      drivingJobs.delete(key);
      nextJobProgressRefreshMs.delete(key);
    }
  };

  /** 💼️ Drives a reserved job under its owning command ingress through terminal handoff so
   * subsequent commands observe its commit and completion in actor order. */
  const commitReservedToolJobWhileSerialized = async (instanceId: number, actorId: string, job: bigint, kind: string, input: Uint8Array): Promise<void> => {
    const key = `${actorId}#${job}`;
    if (drivingJobs.has(key) || finishedReservedJobs.has(key)) return;
    const live = (): boolean => !state.disposing && !closingInstances.has(instanceId) && actorIdByInstance.get(instanceId) === actorId;
    if (!live()) return;
    drivingJobs.add(key);
    try {
      beginIsolatedJobDrive();
      await shardClient.startJob(actorId, job, kind, input);
      runningJobs.add(key);
      while (live()) {
        const batch = isolatedJobStepsPerSerializedAdmission(PLUGIN_JOB_STEPS_PER_YIELD);
        for (let i = 0; i < batch && live(); i += 1) {
          const outcome = await shardClient.stepJob(actorId, job, jobStepBudget);
          if (outcome.status === "running") continue;
          runningJobs.delete(key);
          finishedReservedJobs.add(key);
          if (live()) await deliverJobCompletionTurn(instanceId, actorId, job, outcome);
          return;
        }
        await yieldPluginUiContinuation();
      }
    } finally {
      endIsolatedJobDrive();
      releaseSpawnedJob(instanceId, actorId, job);
      drivingJobs.delete(key);
    }
  };
  const commitReservedToolSpawnsWhileSerialized = async (instanceId: number, actorId: string, effects: readonly WireVariant[]): Promise<void> => {
    for (const effect of effects) {
      if (effect.tag !== "spawn-job") continue;
      const value = effect.val as { job?: unknown; kind?: unknown; input?: unknown } | undefined;
      if (typeof value?.job !== "bigint" || value.kind !== FRAMEWORK_RESERVED_JOB_KIND) continue;
      await commitReservedToolJobWhileSerialized(instanceId, actorId, value.job, value.kind, coerceWireBytes(value.input));
    }
  };

  const driveReservedToolJob = async (instanceId: number, actorId: string, job: bigint, kind: string, input: Uint8Array): Promise<void> => {
    try {
      await serializeCommandIngressForActor(actorId, () => commitReservedToolJobWhileSerialized(instanceId, actorId, job, kind, input));
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    }
  };

    /** 🚦 The one choke point every consumer of a turn's `effects` passes through: backbone
   * `send-message`s go to the document port, `spawn-job`/`cancel-job` go to {@link driveSpawnedJob}
   * (they are host WORK, never a `requestedEffects` entry for `applyHostEffects` to branch on), and
   * everything else is returned for the caller's own shell-frame/leftover split. */
  const routeHostEffects = (instanceId: number, effects: readonly WireVariant[], port: ActorDocumentMessagePortV1 | undefined): WireVariant[] => effects.filter(effect => {
    if (effect.tag === "spawn-job") {
      const value = effect.val as { job?: unknown; kind?: unknown; input?: unknown } | undefined;
      if (typeof value?.job !== "bigint" || typeof value.kind !== "string") throw new Error("plugin.spawn-job-authority-invalid");
      const actorId = requireActorId(instanceId);
      if (drivingJobs.has(`${actorId}#${value.job}`) || finishedReservedJobs.has(`${actorId}#${value.job}`)) return false;
      const input = coerceWireBytes(value.input);
      const drive = value.kind === FRAMEWORK_RESERVED_JOB_KIND
        ? driveReservedToolJob(instanceId, actorId, value.job, value.kind, input)
        : driveSpawnedJob(instanceId, actorId, value.job, value.kind, input);
      if (value.kind === FRAMEWORK_RESERVED_JOB_KIND) pendingReservedJobDrives.push(drive);
      else void drive;
      return false;
    }
    if (effect.tag === "cancel-job") {
      const value = effect.val as { job?: unknown } | undefined;
      if (typeof value?.job !== "bigint") throw new Error("plugin.cancel-job-authority-invalid");
      cancelSpawnedJob(instanceId, requireActorId(instanceId), value.job);
      return false;
    }
    const value = effect.val as { target?: WireVariant; payload?: unknown } | undefined;
    if (effect.tag !== "send-message" || value?.target?.tag !== "backbone") return true;
    if (!port) throw new Error("actor-document-port.unbound");
    if (value.target.val !== port.uri) throw new Error("actor-document-port.foreign-uri");
    if (!port.send(port.uri, documentPortHotBytes(value.payload))) throw new Error("actor-document-port.retired");
    return false;
  });
  return { routeHostEffects, flushReservedJobDrives, commitReservedToolSpawnsWhileSerialized, subscribeSpawnedJobProgress };
}

/** 🚚️ Phase 7 of {@link loadPluginModule}: the queued command turn and the typed-operation continuation drain. */
export function createTurnPump(scope: PluginHandleScope, instances: ReturnType<typeof createInstanceTable>, ui: ReturnType<typeof createRetainedUiIntake>, jobs: ReturnType<typeof createSpawnedJobDriver>) {
  const { pluginId, shardClient, state, closingInstances, actorIdByInstance, documentBindings, turnOutcomes } = scope;
  const { requireActorId, submitTurn } = instances;
  const { acceptUiPatches } = ui;
  const { routeHostEffects, flushReservedJobDrives, commitReservedToolSpawnsWhileSerialized } = jobs;
  /** 🔀️ The real turn-submission body `enqueue` used to run synchronously inline and return — now
   * run fire-and-forget from `enqueue`, pushing its settlement onto {@link turnOutcomes} instead of
   * resolving a caller's promise directly (a caller's own promise now lives one layer up, in
   * `AppChannelClient.sendCommand`, correlated against this broadcast). A turn-submission failure
   * (NOT an `AppFrame::Error` — that is still an ordinary decoded frame) becomes an `error`-shaped
   * outcome rather than an uncaught rejection, since nothing here awaits this function's own promise.
   * `dispatch?.order` (the ledger's causal key, see {@link serializeCommandIngressForActor}) is the
   * only thing that steers WHERE in the actor's ingress queue this turn lands.
   * The natural import's signal stops new command crossings after the current turn's patch receipts
   * settle. Its fresh owner's lifecycle close then retires the retained guest command. */
  const runQueuedTurn = async (instanceId: number, events: readonly Uint8Array[], dispatch?: PluginDispatchHintV1, queuedImportSignal?: AbortSignal): Promise<void> => {
    try {
      const actorId = requireActorId(instanceId);
      const activation = shardClient.captureActorActivation(actorId);
      const documentPort = documentBindings.get(instanceId)?.port;
      const inspected = inspectEncodedAppCommand(events);
      const importSignal = inspected.mediaIn ? queuedImportSignal : undefined;
      let stage = "queued behind the actor's ingress lane";
      const result = await withTypedOperationCall(actorId, `command#${instanceId}`, (call) => serializeCommandIngressForActor(actorId, () => commandStallWatch.watch({ actorId, programId: pluginId, commandId: inspected.actionId, operation: inspected.operation, stage: () => stage }, async (): Promise<WireTurnResult> => {
        activation.assertActive();
        importSignal?.throwIfAborted();
        const results: WireTurnResult[] = [];
        let acknowledgements: readonly ShardEventEnvelope[] = [];
        const acceptTurn = async (turn: WireTurnResult): Promise<void> => {
          results.push(turn);
          const accepted = await acceptUiPatches(instanceId, turn);
          results.push(...accepted.turns);
          acknowledgements = [...accepted.acknowledgements, ...typedOperationAcknowledgements(turn), ...accepted.turns.flatMap(typedOperationAcknowledgements)];
        };
        const cancelImportAtBoundary = async (): Promise<void> => {
          let consumed = 0;
          let deferred = results.findLast((turn) => turn.deferredEvents !== undefined)?.deferredEvents ?? 0;
          for (let step = 0; ; step += 1) {
            while (consumed < results.length) await call.accept(results[consumed++]!.effects);
            if (acknowledgements.length === 0 && deferred === 0) break;
            if (step >= PLUGIN_UI_CONTINUATION_LIMIT) throw new Error("natural-file.import-acknowledgement-stalled");
            const receipt = await submitTurn(actorId, acknowledgements, { activation });
            deferred = receipt.deferredEvents ?? 0;
            await acceptTurn(receipt);
          }
          importSignal?.throwIfAborted();
        };
        for (let commandIndex = 0; commandIndex < events.length; commandIndex += 1) {
          state.eventSeq += 1;
          // ⏱️ The reactor keeps the command while it makes progress (a slow example load + evaluation takes many slices), so the turn-count ceiling is a floor and the continuation is bounded by wall time.
          const COMMAND_CONTINUATION_WALL_MS = 300_000;
          const pages = createShardCommandIngressPages({
            owner: BigInt(instanceId),
            generation: 1n,
            commandIndex,
            commandCount: events.length,
            instance: instanceId,
            seq: BigInt(state.eventSeq),
            command: events[commandIndex]!,
          });
          const crossingStartedMs = performance.now();
          const observedStatuses = new Set<string>();
          let collected = results.length;
          const foldIngress = (): string | undefined => {
            let folded: string | undefined;
            for (const turn of results.slice(collected)) {
              const tag = turn.commandIngress?.tag;
              observedStatuses.add(tag ?? "missing");
              if (tag === "command-complete" || tag === "fault" || tag === "backpressure" || folded === undefined) folded = tag;
            }
            compactAcknowledgedTurns(results, collected);
            collected = results.length;
            return folded;
          };
          let terminal: string | undefined;
          let idleRefusals = 0;
          for (let pageIndex = 0; pageIndex < pages.length;) {
            if (importSignal?.aborted) await cancelImportAtBoundary();
            if (performance.now() - crossingStartedMs >= COMMAND_CONTINUATION_WALL_MS) throw new Error(`plugin ${pluginId}: command ingress plugin.command-page-stalled: page ${pageIndex}/${pages.length} stayed refused (action=${inspected.actionId ?? "none"})`);
            stage = `offering ingress page ${pageIndex + 1}/${pages.length}`;
            const offered = await submitTurn(actorId, acknowledgements, { commandPage: pages[pageIndex]!, activation });
            await acceptTurn(offered);
            if (importSignal?.aborted) await cancelImportAtBoundary();
            terminal = foldIngress();
            // 🚦️ A refused page is NOT consumed: the guest keeps its assembly at the pages it admitted, so offer the SAME page again (the native driver's `CommandBatchDriver::observe` does the same) instead of moving on and tripping `plugin.command-page-order`.
            if ((offered.deferredEvents ?? 0) > 0 || terminal === "backpressure") {
              idleRefusals = terminal === "backpressure" && (offered.deferredEvents ?? 0) === 0 && wireTurnStatusTag(offered.status) === "idle" ? idleRefusals + 1 : 0;
              if (idleRefusals >= PAGE_IDLE_REFUSAL_LIMIT) throw new Error(`plugin ${pluginId}: command ingress plugin.command-page-refused: the reactor answered backpressure while idle ${idleRefusals} times in a row for page ${pageIndex}/${pages.length} (action=${inspected.actionId ?? "none"}); it owns no command and has no work, so its retained stock cannot fund the page`);
              await yieldPluginUiContinuation();
              continue;
            }
            if (terminal !== "page-accepted") break;
            pageIndex += 1;
          }
          const COMMAND_CONTINUATION_PAUSE_MS = 25;
          const COMMAND_CONTINUATION_BACKOFF_MAX_MS = 250;
          const COMMAND_CONTINUATION_BURST_MS = 15_000;
          let lastObservableMs = performance.now();
          const ceiling = commandIngressContinuationCeilingV1(pages.length);
          let lastTurnStatus = wireTurnStatusTag(results.at(-1)?.status);
          let continuations = 0;
          for (; terminal !== "command-complete" && (continuations < ceiling || performance.now() - crossingStartedMs < COMMAND_CONTINUATION_WALL_MS); continuations += 1) {
            if (terminal === "fault") {
              const display = commandIngressFaultDisplay(results.at(-1)?.commandIngress);
              if (isCommandStalledFaultV1(display)) throw new Error(commandStalledMessageV1(pluginId, inspected.operation));
              throw new Error(`plugin ${pluginId}: command ingress fault: ${display}`);
            }
            // 🚦️ backpressure is the guest asking for another turn (its slice ended or its single ingress owner is still assembling); the owner is retained, so keep draining within the ceiling instead of failing the command.
            if (commandIngressUnownedV1(terminal, lastTurnStatus)) break;
            // ⏳️ every guest turn pays a fixed ~4 KB / 12 items of lifetime grant (origin refresh), so a retained command that cannot advance must not be re-asked at millisecond cadence.
            // 🚥️ A turn that reports `more-work` has work ready, and a fixed 25 ms sleep capped a retained command at 40 turns/s (a hidden tab throttles timer chains to one tick a second), so it re-enters on the next macrotask. The guest advances a large command internally in small slices that show nothing in the result, so a command that answers `more-work` without anything observable (no effects, patches, presence, receipt, acknowledgement or ingress change) keeps that cadence for `COMMAND_CONTINUATION_BURST_MS`, then backs off 25 ms doubling up to 250 ms so a command that cannot advance does not spin.
            const quietMs = performance.now() - lastObservableMs;
            if (lastTurnStatus === "more-work" && quietMs < COMMAND_CONTINUATION_BURST_MS) await yieldPluginUiContinuation();
            else await new Promise<void>((resolve) => setTimeout(resolve, lastTurnStatus === "more-work" ? Math.min(COMMAND_CONTINUATION_BACKOFF_MAX_MS, COMMAND_CONTINUATION_PAUSE_MS * 2 ** Math.min(4, Math.floor((quietMs - COMMAND_CONTINUATION_BURST_MS) / 5_000))) : COMMAND_CONTINUATION_PAUSE_MS));
            if (importSignal?.aborted) await cancelImportAtBoundary();
            stage = `continuation ${continuations + 1}/${ceiling} (guest status ${lastTurnStatus}, ingress ${terminal ?? "none"}, ${acknowledgements.length} acknowledgements pending)`;
            const continued = await submitTurn(actorId, acknowledgements, { activation });
            const previousIngress = terminal;
            await acceptTurn(continued);
            if (importSignal?.aborted) await cancelImportAtBoundary();
            terminal = foldIngress();
            const observable = continued.effects.length > 0 || continued.uiPatches.length > 0 || (continued.presence?.length ?? 0) > 0 || continued.lifecycleReceipt !== undefined || continued.uiPatchReceipt !== undefined || acknowledgements.length > 0 || terminal !== previousIngress || (continued.deferredEvents ?? 0) > 0;
            if (observable) lastObservableMs = performance.now();
            lastTurnStatus = wireTurnStatusTag(continued.status);
          }
          if (terminal !== "command-complete") {
            const cause = commandIngressUnownedV1(terminal, lastTurnStatus) ? "plugin.command-ingress-unowned: the reactor retains no ingress owner for this command" : `plugin.command-ingress-stalled: the reactor still owns this command and never completed it (last status ${terminal ?? "missing"}, actor ${lastTurnStatus})`;
            throw new Error(`plugin ${pluginId}: command ingress ${cause} (action=${inspected.actionId ?? "none"}, seq=${inspected.seq ?? "none"}, pages=${pages.length}, continuations=${continuations}/${ceiling}, observed statuses: ${[...observedStatuses].join(", ")})`);
          }
        }
        if (importSignal?.aborted) await cancelImportAtBoundary();
        stage = "settling the acknowledged turns";
        const settled = await settleAcknowledgedPluginTurns(actorId, results, acknowledgements, (turn) => acceptUiPatches(instanceId, turn), activation, call);
        importSignal?.throwIfAborted();
        await commitReservedToolSpawnsWhileSerialized(instanceId, actorId, settled.effects);
        return settled;
      }), inspected.lane, dispatch?.order), inspected.adoptsTypedOperation);
      requireActorId(instanceId);
      activation.assertActive();
      const outFrames: Uint8Array[] = [];
      const leftover: WireVariant[] = [...(pendingTurnEffects.get(instanceId) ?? [])];
      for (const effect of routeHostEffects(instanceId, result.effects, documentPort)) {
        const frame = shellFrameBytes(effect, instanceId);
        if (frame) outFrames.push(frame);
        else leftover.push(effect);
      }
      const promoted = promoteShellSendMessages(instanceId, leftover, outFrames);
      pendingTurnEffects.set(instanceId, promoted);
      await flushReservedJobDrives();
      if (commandIngressNeedsReplyStampV1(outFrames.map(encodedFrameReplySequence), inspected.seq)) {
        outFrames.push(encodeAppFrame({ Done: { in_reply_to: inspected.seq! } }));
      }
      turnOutcomes.push({ instanceId, frames: outFrames });
      if (wireTurnStatusTag(result.status) === "more-work") void drainTypedOperations(instanceId);
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    }
  };

  /** 🔁️ Keeps an instance's retained typed operations advancing once NO host call is left to drive
   * them. A mounted operation returns its "started" `InvocationResult` on its very first turn and then
   * needs hundreds more reactor turns to reach its terminal result — but every turn in this file is
   * submitted by a host call, so the moment `runQueuedTurn` returns the actor simply stops, its
   * operation frozen mid-flight and its completion never published. This is the poll that replaces
   * that missing driver: one `UserVisible` continuation settle per macrotask (never a `setTimeout`
   * chain — `yieldPluginUiContinuation`'s own doc: a hidden tab throttles those to one tick a second),
   * stopping the instant the actor reports anything other than `more-work`, and re-armed by the next
   * command turn that ends `more-work` again. It shares `serializeCommandIngressForActor` with every
   * other caller, so a real command always wins the actor and this poll waits behind it. */
  const drainingInstances = new Set<number>();
  const rearmedDrains = new Set<number>();
  const drainTypedOperations = async (instanceId: number): Promise<void> => {
    const live = (): boolean => !state.disposing && !closingInstances.has(instanceId) && actorIdByInstance.has(instanceId);
    if (!live()) return;
    if (drainingInstances.has(instanceId)) { rearmedDrains.add(instanceId); return; }
    drainingInstances.add(instanceId);
    try {
      do {
        rearmedDrains.delete(instanceId);
        const outcome = await drainTypedOperationBatches(PLUGIN_OPERATION_DRAIN_BUDGET, live, async () => {
          const actorId = requireActorId(instanceId);
          const activation = shardClient.captureActorActivation(actorId);
          const documentPort = documentBindings.get(instanceId)?.port;
          const settled = await withTypedOperationCall(actorId, `operation-drain#${instanceId}`, (call) =>
            serializeCommandIngressForActor(actorId, async () => {
              activation.assertActive();
              return settlePluginTurn(actorId, await submitTurn(actorId, [], { lane: "UserVisible", activation }), "UserVisible", new Set(), (turn) => acceptUiPatches(instanceId, turn), false, activation, call);
            }, "UserVisible"), false,
          );
          const frames: Uint8Array[] = [];
          const leftover = pendingCompletionEffects.get(instanceId) ?? [];
          for (const effect of routeHostEffects(instanceId, settled.effects, documentPort)) {
            const frame = shellFrameBytes(effect, instanceId);
            if (frame) frames.push(frame);
            else leftover.push(effect);
          }
          if (leftover.length > PLUGIN_OPERATION_EFFECT_CAPACITY) throw new Error(`typed-operation host effects for instance ${instanceId} exceeded their ${PLUGIN_OPERATION_EFFECT_CAPACITY}-entry authority`);
        pendingCompletionEffects.set(instanceId, leftover);
          if (frames.length > 0) turnOutcomes.push({ instanceId, frames });
          return { status: settled.status, nextWake: settled.nextWake };
        }, yieldPluginUiContinuation);
        if (outcome.stopped === "idle" && outcome.nextWake !== null && live()) {
          hostContinuations.schedule(() => void drainTypedOperations(instanceId), Math.min(Math.max(outcome.nextWake, 0), PLUGIN_OPERATION_WAKE_MAX_MS), `operation-wake#${instanceId}`);
        }
      } while (rearmedDrains.has(instanceId) && live());
    } catch (error) {
      if (error instanceof SemioFaultError) publishOperationFault(instanceId, error);
      turnOutcomes.push({ instanceId, error });
    } finally {
      drainingInstances.delete(instanceId);
      rearmedDrains.delete(instanceId);
    }
  };

  const openOriginalMediaIngress = (instanceId: number, issued: unknown) => {
    const request = originalMediaRequest(issued, instanceId);
    const actorId = requireActorId(instanceId);
    const activation = shardClient.captureActorActivation(actorId);
    const binding = documentBindings.get(instanceId);
    const scopedDocument = binding?.port.owner.scope?.documentId;
    if (scopedDocument !== undefined && scopedDocument !== request.loan.documentId) throw new Error("original-media.wrong-document");
    if (!Number.isSafeInteger(state.eventSeq + 1)) throw new Error("original-media.sequence");
    const seq = BigInt(++state.eventSeq);
    const retired = () => {
      if (state.disposing || closingInstances.has(instanceId) || actorIdByInstance.get(instanceId) !== actorId) return true;
      try { activation.assertActive(); return false; } catch { return true; }
    };
    const current = () => !retired() && documentBindings.get(instanceId) === binding && !binding?.port.closing;
    let call: TypedOperationCall | undefined;
    const results: WireTurnResult[] = [];
    let acknowledgements: readonly ShardEventEnvelope[] = [];
    const settle = async () => {
      if (results.length === 0) return;
      const turns = results.splice(0);
      const acks = acknowledgements;
      acknowledgements = [];
      const settled = await settleAcknowledgedPluginTurns(actorId, turns, acks, (turn) => acceptUiPatches(instanceId, turn), activation, call);
      await commitReservedToolSpawnsWhileSerialized(instanceId, actorId, settled.effects);
      const frames: Uint8Array[] = [];
      const pending: WireVariant[] = [...pendingTurnEffects.get(instanceId) ?? []];
      for (const effect of routeHostEffects(instanceId, settled.effects, binding?.port)) {
        const frame = shellFrameBytes(effect, instanceId);
        if (frame) frames.push(frame);
        else pending.push(effect);
      }
      if (pending.length > PLUGIN_OPERATION_EFFECT_CAPACITY) throw new Error("original-media.effect-capacity");
      pendingTurnEffects.set(instanceId, promoteShellSendMessages(instanceId, pending, frames));
      if (frames.length > 0) turnOutcomes.push({ instanceId, frames });
      if (wireTurnStatusTag(settled.status) === "more-work") void drainTypedOperations(instanceId);
    };
    return createOriginalMediaExchange(request, BigInt(instanceId), seq, {
      current,
      retired,
      assertOriginalActivation: () => activation.assertActive(),
      serialize: (run) => withTypedOperationCall(actorId, `original-media#${instanceId}:${seq}`, owned =>
        serializeCommandIngressForActor(actorId, async () => {
          call = owned;
          try { return await run(); }
          finally {
            try { if (!retired()) await settle(); }
            finally { call = undefined; }
          }
        })).then(async (answer) => { await flushReservedJobDrives(); return answer; }),
      turn: async (crossing) => {
        const turn = await submitTurn(actorId, [...acknowledgements, ...(crossing?.events ?? [])], { commandPage: crossing?.page, activation });
        const accepted = await acceptUiPatches(instanceId, turn);
        const observations = [turn, ...accepted.turns];
        const failure = observations.find((value) => value.commandIngress?.tag === "fault");
        const ingress = observations.findLast((value) => value.commandIngress?.tag === "original-media-ack" || value.commandIngress?.tag === "backpressure")?.commandIngress;
        const collected = results.length;
        results.push(...observations);
        compactAcknowledgedTurns(results, collected);
        acknowledgements = [...accepted.acknowledgements, ...observations.flatMap(typedOperationAcknowledgements)];
        if (failure) throw new Error(commandIngressFaultDisplay(failure.commandIngress));
        return { ingress, deferredEvents: turn.deferredEvents };
      },
      settle,
      yieldTurn: yieldPluginUiContinuation,
      now: () => performance.now(),
    });
  };
  return { runQueuedTurn, drainTypedOperations, openOriginalMediaIngress };
}
//#endregion 🔖️plugin-runtime

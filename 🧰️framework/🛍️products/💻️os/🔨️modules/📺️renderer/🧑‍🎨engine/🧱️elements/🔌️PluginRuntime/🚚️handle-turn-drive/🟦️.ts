/** 🚚️ Phases 6 and 7 of `loadPluginModule`: the spawned-job driver with host-effect routing, and the turn pump with the typed-operation continuation drain.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { decodeAppFrame, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { FRAMEWORK_RESERVED_JOB_KIND } from "@semio-tech/framework";
import { type PluginDispatchHintV1 } from "@semio-tech/framework";
import { createShardCommandIngressPages, type ShardEventEnvelope, type ShardJobStep } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { hostContinuations } from "@semio-tech/framework/async/continuation";
import { wireTurnStatusTag } from "../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
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
import { typedOperationAcknowledgements, withTypedOperationCall } from "../📬️typed-result/🟦️.ts";
import {
  beginIsolatedJobDrive,
  DEFAULT_SHARD_BUDGET,
  endIsolatedJobDrive,
  isolatedJobStepsPerSerializedAdmission,
  isolatedJobUiPollEverySteps,
  requestIsolatedJobUiPoll
} from "../🛰️runtime-singletons/🟦️.ts";
import { commandStallWatch, serializeCommandIngressForActor } from "../🧵️turn-dispatch/🟦️.ts";
import {
  commandIngressContinuationCeilingV1,
  commandIngressNeedsReplyStampV1,
  commandIngressUnownedV1,
  settleAcknowledgedPluginTurns,
  settlePluginTurn,
  yieldPluginUiContinuation
} from "../🔁️turn-settle/🟦️.ts";
import {
  drainTypedOperationTurns,
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
  const lastJobProgressRefreshMs = new Map<string, number>();
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
  const publishSpawnedJobProgress = (instanceId: number, key: string): void => {
    const now = performance.now();
    if (now - (lastJobProgressRefreshMs.get(key) ?? -Infinity) < PLUGIN_JOB_PROGRESS_REFRESH_MS) return;
    lastJobProgressRefreshMs.set(key, now);
    for (const listener of [...(spawnedJobProgressListeners.get(instanceId) ?? [])]) {
      try { listener(); }
      catch (error) {  }
    }
  };
  return { lastJobProgressRefreshMs, subscribeSpawnedJobProgress, publishSpawnedJobProgress };
}

/** 💼️ Phase 6 of {@link loadPluginModule}: the host half of `Effect::SpawnJob` (isolated and reserved-tool job drives),
 * the shell-facing spawned-job progress lane and the one choke point every turn's host effects pass through.
 * `turns` is the late-bound door back into the turn pump: a job completion re-arms the typed-operation drain, and
 * the pump in turn routes its effects through this driver. */
export function createSpawnedJobDriver(scope: PluginHandleScope, instances: ReturnType<typeof createInstanceTable>, ui: ReturnType<typeof createRetainedUiIntake>, turns: { readonly drainTypedOperations: (instanceId: number) => Promise<void> }) {
  const { pluginId, shardClient, state, closingInstances, actorIdByInstance, documentBindings, turnOutcomes } = scope;
  const { requireActorId, submitTurn } = instances;
  const { acceptUiPatches } = ui;
  const { lastJobProgressRefreshMs, subscribeSpawnedJobProgress, publishSpawnedJobProgress } = createSpawnedJobProgressLane();
  /** 💼️ In-flight `spawn-job` drives, keyed `actorId#job`. A guest re-emitting the same `job` id
   * (a replayed turn, a checkpoint restore) must never open a second `start-job` for one owner. */
  const drivingJobs = new Set<string>();
  const finishedReservedJobs = new Set<string>();
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
      cancel: () => void serializeCommandIngressForActor(actorId, () => shardClient.cancelJob(actorId, job)).catch((error) => turnOutcomes.push({ instanceId, error })),
    });
    try {
      beginIsolatedJobDrive();
      await serializeCommandIngressForActor(actorId, () => shardClient.startJob(actorId, job, kind, input));
      let step = 0;
      while (live()) {
        const batch = isolatedJobStepsPerSerializedAdmission(PLUGIN_JOB_STEPS_PER_YIELD);
        const progressed = { value: false };
        const outcome = await serializeCommandIngressForActor(actorId, async () => {
          let last: ShardJobStep | undefined;
          for (let i = 0; i < batch; i += 1) {
            last = await shardClient.stepJob(actorId, job, jobStepBudget);
            step += 1;
            if (last.status === "running" && last.progress && last.progress.byteLength > 0) progressed.value = true;
            if (last.status !== "running") return last;
          }
          return last ?? { status: "failed" as const, value: new TextEncoder().encode("plugin.job-step-empty") };
        });
        if (outcome.status !== "running") {
          if (live()) await deliverJobCompletion(instanceId, actorId, job, outcome);
          return;
        }
        ledgerEntry.step(step, progressed.value);
        if (progressed.value || isolatedJobUiPollEverySteps(step)) requestIsolatedJobUiPoll();
        if (progressed.value && live()) publishSpawnedJobProgress(instanceId, key);
        await yieldPluginUiContinuation();
      }
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    } finally {
      ledgerEntry.end();
      endIsolatedJobDrive();
      drivingJobs.delete(key);
      lastJobProgressRefreshMs.delete(key);
    }
  };

  /** 💼️ Drives a reserved job under its owning command ingress through terminal handoff so
   * subsequent commands observe its commit and completion in actor order. */
  const commitReservedToolJobWhileSerialized = async (instanceId: number, actorId: string, job: bigint, kind: string, input: Uint8Array): Promise<void> => {
    const key = `${actorId}#${job}`;
    if (drivingJobs.has(key)) return;
    drivingJobs.add(key);
    try {
      beginIsolatedJobDrive();
      await shardClient.startJob(actorId, job, kind, input);
      let step = 0;
      let last: ShardJobStep | undefined;
      const batch = isolatedJobStepsPerSerializedAdmission(PLUGIN_JOB_STEPS_PER_YIELD);
      for (let i = 0; i < batch; i += 1) {
        last = await shardClient.stepJob(actorId, job, jobStepBudget);
        step += 1;
        if (last.status !== "running") break;
      }
      const outcome = last ?? { status: "failed" as const, value: new TextEncoder().encode("plugin.job-step-empty") };
      if (outcome.status === "running") throw new Error(`reserved-tool job ${job} still running after ${step} Isolated steps`);
      await deliverJobCompletionTurn(instanceId, actorId, job, outcome);
    } finally {
      endIsolatedJobDrive();
      finishedReservedJobs.add(key);
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
    const key = `${actorId}#${job}`;
    if (drivingJobs.has(key)) return;
    drivingJobs.add(key);
    const live = (): boolean => !state.disposing && !closingInstances.has(instanceId) && actorIdByInstance.get(instanceId) === actorId;
    try {
      beginIsolatedJobDrive();
      await serializeCommandIngressForActor(actorId, async () => {
        await shardClient.startJob(actorId, job, kind, input);
        let step = 0;
        let last: ShardJobStep | undefined;
        const batch = isolatedJobStepsPerSerializedAdmission(PLUGIN_JOB_STEPS_PER_YIELD);
        for (let i = 0; i < batch; i += 1) {
          last = await shardClient.stepJob(actorId, job, jobStepBudget);
          step += 1;
          if (last.status !== "running") break;
        }
        const outcome = last ?? { status: "failed" as const, value: new TextEncoder().encode("plugin.job-step-empty") };
        if (outcome.status === "running") throw new Error(`reserved-tool job ${job} still running after ${step} Isolated steps`);
        if (live()) await deliverJobCompletionTurn(instanceId, actorId, job, outcome);
      });
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    } finally {
      endIsolatedJobDrive();
      drivingJobs.delete(key);
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
      const job = value.job;
      const actorId = requireActorId(instanceId);
      void serializeCommandIngressForActor(actorId, () => shardClient.cancelJob(actorId, job)).catch(error => turnOutcomes.push({ instanceId, error }));
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
   * only thing that steers WHERE in the actor's ingress queue this turn lands. */
  const runQueuedTurn = async (instanceId: number, events: readonly Uint8Array[], dispatch?: PluginDispatchHintV1): Promise<void> => {
    try {
      const actorId = requireActorId(instanceId);
      const activation = shardClient.captureActorActivation(actorId);
      const documentPort = documentBindings.get(instanceId)?.port;
      const inspected = inspectEncodedAppCommand(events);
      const result = await withTypedOperationCall(actorId, `command#${instanceId}`, (call) => serializeCommandIngressForActor(actorId, () => commandStallWatch.watch({ actorId, programId: pluginId, commandId: inspected.actionId }, async (): Promise<WireTurnResult> => {
        activation.assertActive();
        const results: WireTurnResult[] = [];
        let acknowledgements: readonly ShardEventEnvelope[] = [];
        const acceptTurn = async (turn: WireTurnResult): Promise<void> => {
          results.push(turn);
          const accepted = await acceptUiPatches(instanceId, turn);
          results.push(...accepted.turns);
          acknowledgements = [...accepted.acknowledgements, ...typedOperationAcknowledgements(turn), ...accepted.turns.flatMap(typedOperationAcknowledgements)];
        };
        for (let commandIndex = 0; commandIndex < events.length; commandIndex += 1) {
          state.eventSeq += 1;
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
          // 🧾️ Every turn this drain submits reports the ingress, and `acceptTurn` submits turns of its
          // own (the batched `issued-ui-acks` lifecycle turn), so the terminal is folded over ALL of
          // them: a `command-complete` that landed on a supplemental turn used to be read past.
          let collected = results.length;
          const foldIngress = (): string | undefined => {
            let folded: string | undefined;
            for (const turn of results.slice(collected)) {
              const tag = turn.commandIngress?.tag;
              observedStatuses.add(tag ?? "missing");
              if (tag === "command-complete" || tag === "fault" || tag === "backpressure" || folded === undefined) folded = tag;
            }
            collected = results.length;
            return folded;
          };
          for (const commandPage of pages) {
            const pageTurn = await submitTurn(actorId, acknowledgements, { commandPage, activation });
            await acceptTurn(pageTurn);
          }
          const ceiling = commandIngressContinuationCeilingV1(pages.length);
          let terminal = foldIngress();
          let lastTurnStatus = wireTurnStatusTag(results.at(-1)?.status);
          let continuations = 0;
          for (; terminal !== "command-complete" && continuations < ceiling; continuations += 1) {
            if (terminal === "fault") throw new Error(`plugin ${pluginId}: command ingress fault: ${commandIngressFaultDisplay(results.at(-1)?.commandIngress)}`);
            if (terminal === "backpressure") throw new Error(`plugin ${pluginId}: command ingress backpressure after serialized submission`);
            if (commandIngressUnownedV1(terminal, lastTurnStatus)) break;
            const continued = await submitTurn(actorId, acknowledgements, { activation });
            await acceptTurn(continued);
            terminal = foldIngress();
            lastTurnStatus = wireTurnStatusTag(continued.status);
          }
          if (terminal !== "command-complete") {
            const cause = commandIngressUnownedV1(terminal, lastTurnStatus) ? "plugin.command-ingress-unowned: the reactor retains no ingress owner for this command" : `plugin.command-ingress-stalled: the reactor still owns this command and never completed it (last status ${terminal ?? "missing"}, actor ${lastTurnStatus})`;
            throw new Error(`plugin ${pluginId}: command ingress ${cause} (action=${inspected.actionId ?? "none"}, seq=${inspected.seq ?? "none"}, pages=${pages.length}, continuations=${continuations}/${ceiling}, observed statuses: ${[...observedStatuses].join(", ")})`);
          }
        }
        const settled = await settleAcknowledgedPluginTurns(actorId, results, acknowledgements, (turn) => acceptUiPatches(instanceId, turn), activation, call);
        await commitReservedToolSpawnsWhileSerialized(instanceId, actorId, settled.effects);
        return settled;
      }), inspected.lane, dispatch?.order));
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
  const drainTypedOperations = async (instanceId: number): Promise<void> => {
    const live = (): boolean => !state.disposing && !closingInstances.has(instanceId) && actorIdByInstance.has(instanceId);
    if (drainingInstances.has(instanceId) || !live()) return;
    drainingInstances.add(instanceId);
    try {
      const outcome = await drainTypedOperationTurns(PLUGIN_OPERATION_DRAIN_BUDGET, live, async () => {
        const actorId = requireActorId(instanceId);
        const activation = shardClient.captureActorActivation(actorId);
        const documentPort = documentBindings.get(instanceId)?.port;
        const settled = await withTypedOperationCall(actorId, `operation-drain#${instanceId}`, (call) =>
          serializeCommandIngressForActor(actorId, async () => {
            activation.assertActive();
            return settlePluginTurn(actorId, await submitTurn(actorId, [], { lane: "UserVisible", activation }), "UserVisible", new Set(), (turn) => acceptUiPatches(instanceId, turn), true, activation, call);
          }),
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
      });
      if (outcome.stopped === "idle" && outcome.nextWake !== null && live()) {
        // 🪃️ The guest's own requested wake. `nextWake: 0` means "immediately" and MUST cost one
        // event-loop turn, not a throttled timer tick — this is the same continuation lane the
        // `Effect::DispatchAction` re-arm rides. Coalesced per instance so a burst of idle turns
        // that each ask for a wake leaves exactly one pending, at the earliest deadline asked for.
        hostContinuations.schedule(() => void drainTypedOperations(instanceId), Math.min(Math.max(outcome.nextWake, 0), PLUGIN_OPERATION_WAKE_MAX_MS), `operation-wake#${instanceId}`);
      }
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    } finally {
      drainingInstances.delete(instanceId);
    }
  };
  return { runQueuedTurn, drainTypedOperations };
}
//#endregion 🔖️plugin-runtime

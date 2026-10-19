/** 🖼️ The serialized surface render, the spawned job and typed operation drains and the queued turn driver.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | loadPluginModule}: each function builds one group of the handle's closure state and helpers and returns them for the later groups. */
import { createTurnOutcomeBroadcast, type Effect, type PluginDispatchHintV1, type PluginWasmHandle as KernelPluginWasmHandle, type SpawnedJobCompletion, type TurnOutcome } from "@semio-tech/framework";
import { encodePackValue } from "@semio-tech/framework-os";
import { stashOperationProgress, stashOperationPublication, WGPU_TYPED_OPERATION_DRAIN_BUDGET, WGPU_TYPED_OPERATION_EFFECT_CAPACITY, WGPU_TYPED_OPERATION_SETTLE_LIMIT, WgpuTypedOperationDrive } from "../../🧾️typed-operations-and-handle/🟦️.ts";
import { admitSpawnedJob, declareWgpuBootSubphase, forgetSpawnedJobs, leftoverFriendlyEffects, pendingTurnEffects, shellFrameAnswersACaller, stashLeftoverHostEffects, takeSpawnedJobs, WGPU_SPAWNED_JOB_ROUNDS, wgpuHostEffect, type WgpuSpawnedJobDrain } from "../../🎛️contributions-and-effects/🟦️.ts";
import { getShardClient, serializeWgpuActorCall, submitTurn } from "../../🧵️shard-and-actor-calls/🟦️.ts";
import { RETAINED_DOCUMENT_OPPORTUNITIES, type WgpuOwnedUiProjection, yieldWgpuUi } from "../../🍽️ui-intake-and-route/🟦️.ts";
import { drainTypedOperationTurns, shellFrameBytes, type WireTurnResult, wireTurnStatusTag } from "../../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts";
import { driveSpawnedJob, spawnedJobCompletedEvent, type SpawnedJobPort } from "../../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🧵️spawned-job/🟦️.ts";
import { createShardCommandIngressPages } from "@semio-tech/framework-actor/shard-client/wire";
import type { createInstanceBookkeeping } from "../🪪️descriptor-and-instance-state/🟦️.ts";

/** 🎨️ Renders one surface through the owned ui route inside the actor chain. */
export function createRenderSurface({ requireActorId, requireUiRoute, executeFor, uiRouteByInstance }: Pick<ReturnType<typeof createInstanceBookkeeping>, "requireActorId" | "requireUiRoute" | "executeFor" | "uiRouteByInstance">) {
  const renderSurface = (instanceId: number, surfaceId: string, bodyKey: string, viewState: unknown): Promise<WgpuOwnedUiProjection> =>
    serializeWgpuActorCall(requireActorId(instanceId), () => renderSurfaceSerialized(instanceId, surfaceId, bodyKey, viewState));
  const renderSurfaceSerialized = async (instanceId: number, surfaceId: string, bodyKey: string, viewState: unknown): Promise<WgpuOwnedUiProjection> => {
    const actorId = requireActorId(instanceId);
    const route = requireUiRoute(instanceId);
    const execute = executeFor(actorId);
    const phase = declareWgpuBootSubphase(`shell-boot:render:${surfaceId}`);
    const turns: WireTurnResult[] = [];
    const carried = (pendingTurnEffects.get(instanceId) ?? []).map((effect) => wgpuHostEffect(effect)).filter((effect): effect is Effect => effect !== null);
    pendingTurnEffects.delete(instanceId);
    try {
      let current = await submitTurn(actorId, [{ kind: "surface-visible", payload: { surface: { instance: instanceId, surface: surfaceId }, bodyKey, viewState: encodePackValue(viewState) } }]);
      turns.push(current);
      for (let opportunity = 0; opportunity < RETAINED_DOCUMENT_OPPORTUNITIES; opportunity += 1) {
        const accepted = await route.accept(current, execute);
        turns.push(...accepted);
        requireActorId(instanceId);
        if (uiRouteByInstance.get(instanceId) !== route) throw new Error("wgpu-ui.owner-replaced");
        const projected = await route.project(surfaceId);
        const effects = [...carried, ...leftoverFriendlyEffects(instanceId, turns)];
        if (projected) return { ...projected, effects };
        const status = typeof current.status === "string" ? current.status : current.status && typeof current.status === "object" && "tag" in current.status ? String((current.status as { readonly tag?: unknown }).tag ?? "") : "";
        if (status.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase() !== "more-work") throw new Error(`wgpu-ui.surface-not-published:${surfaceId}`);
        current = await submitTurn(actorId, []);
        turns.push(current);
      }
      throw new Error(`wgpu-ui.render-budget-exhausted:${surfaceId}`);
    } finally {
      phase.leave();
    }
  };
  return { renderSurface };
}

/** 🚦️ Builds the outcome broadcast, the spawned job drain, the typed operation drain and the queued turn driver. */
export function createTurnDrivers({ requireUiRoute, executeFor, state, closingInstances, actorIdByInstance, requireActorId, pluginId }: Pick<ReturnType<typeof createInstanceBookkeeping>, "requireUiRoute" | "executeFor" | "state" | "closingInstances" | "actorIdByInstance" | "requireActorId"> & { readonly pluginId: string }) {
  /** 📤️📥️ Backs `channelHandle.enqueue`/`.outcomes` below — one broadcast per `loadPluginModule` call,
   * matching the handle's own lifetime: every instance this call's `createApp` ever opens shares it,
   * and each instance's `AppChannelClient` filters to its own `instanceId` (`pumpOutcomes`'s own doc in
   * `💻️os/🟦️.ts`). Mirrors `PluginRuntime`'s own `turnOutcomes`/`loadPluginModule` exactly. */
  const turnOutcomes = createTurnOutcomeBroadcast<TurnOutcome>();
  /** 🧵️ Runs every job this instance's guest asked for, and answers each one with the
   * `Event::JobCompleted` it is waiting on — the wgpu half of the shared `driveSpawnedJob` contract
   * (`🖼️wire-turn/🟦️.ts`, rule owned by `kernel::spawnedJobCompletion`).
   *
   * MUST be called from inside a {@link serializeWgpuActorCall} body: it submits turns on `actorId`,
   * so running it outside the actor's own chain would interleave with a command's continuations.
   *
   * A drive that refuses still answers — with the refusal bytes as a `fault` — because the guest has a
   * parked future on this job id and a host that says nothing leaves the interaction hanging forever,
   * which is the shape the dropped effect already had. */
  const drainSpawnedJobs = async (instanceId: number, actorId: string): Promise<WgpuSpawnedJobDrain> => {
    const frames: Uint8Array[] = [];
    let last: WireTurnResult | undefined;
    const client = getShardClient();
    const port: SpawnedJobPort = {
      startJob: (job, kind, input) => client.startJob(actorId, job, kind, input),
      stepJob: (job, budget) => client.stepJob(actorId, job, budget),
    };
    for (let round = 0; round < WGPU_SPAWNED_JOB_ROUNDS; round += 1) {
      const jobs = takeSpawnedJobs(instanceId);
      if (jobs.length === 0) return { frames, last };
      for (const job of jobs) {
        const started = performance.now();
        let completion: SpawnedJobCompletion;
        try {
          completion = await driveSpawnedJob({ job: job.job, kind: job.kind, input: job.input, port });
        } catch (error) {
          const detail = error instanceof Error ? error.message : String(error);
          completion = { steps: 0, outcome: { fault: new TextEncoder().encode(detail) } };
        }
        const drive = new WgpuTypedOperationDrive(instanceId);
        const route = requireUiRoute(instanceId);
        const execute = executeFor(actorId);
        const settled = [await submitTurn(actorId, [spawnedJobCompletedEvent(job.job, completion)])];
        settled.push(...await route.accept(settled[0]!, execute));
        drive.observe(settled);
        for (let settle = 0; drive.owesASettle(settled.at(-1)) && settle < WGPU_TYPED_OPERATION_SETTLE_LIMIT; settle += 1) {
          const next = await submitTurn(actorId, drive.takeAcknowledgements());
          const more = [next, ...await route.accept(next, execute)];
          settled.push(...more);
          drive.observe(more);
          if (!drive.progressed(more)) break;
          await yieldWgpuUi();
        }
        const hostEffects = drive.hostEffects(settled);
        for (const effect of hostEffects) {
          const frame = shellFrameBytes(effect, instanceId);
          if (frame && shellFrameAnswersACaller(frame)) frames.push(frame);
        }
        stashLeftoverHostEffects(instanceId, hostEffects);
        drive.report(`job=${job.job}`);
        last = settled.at(-1) ?? last;
      }
    }
    forgetSpawnedJobs(instanceId);
    return { frames, last };
  };
  /** 🔁️ Keeps an instance's mounted typed operations advancing once no host call is left to drive
   * them — the wgpu twin of `PluginRuntime`'s own `drainTypedOperations`, on the SAME shared
   * {@link drainTypedOperationTurns} loop. A `flowEvalTick` is a `Migrated` interactive job: it
   * returns its "started" reply on the first turn and then needs hundreds more reactor turns to reach
   * its terminal page, so without this poll the actor simply stops and `flowEvalResolve` never
   * arrives. Frames a poll uncovers are published on this instance's own outcome stream, where
   * `AppChannelClient`'s pump routes them exactly as a command reply's. */
  const drainingInstances = new Set<number>();
  const drainTypedOperations = async (instanceId: number): Promise<void> => {
    const live = (): boolean => !state.disposing && !closingInstances.has(instanceId) && actorIdByInstance.has(instanceId);
    if (drainingInstances.has(instanceId) || !live()) return;
    drainingInstances.add(instanceId);
    const drive = new WgpuTypedOperationDrive(instanceId);
    try {
      const outcome = await drainTypedOperationTurns(
        WGPU_TYPED_OPERATION_DRAIN_BUDGET,
        live,
        async () => serializeWgpuActorCall(requireActorId(instanceId), async () => {
          const actorId = requireActorId(instanceId);
          const route = requireUiRoute(instanceId);
          const execute = executeFor(actorId);
          const turn = await submitTurn(actorId, drive.takeAcknowledgements());
          const accepted = [turn, ...await route.accept(turn, execute)];
          drive.observe(accepted);
          const frames: Uint8Array[] = [];
          const leftover = pendingTurnEffects.get(instanceId) ?? [];
          for (const effect of drive.hostEffects(accepted)) {
            if (admitSpawnedJob(instanceId, effect)) continue;
            const frame = shellFrameBytes(effect, instanceId);
            if (frame && shellFrameAnswersACaller(frame)) frames.push(frame);
            else if (!frame || !stashOperationProgress(instanceId, frame)) leftover.push(effect);
          }
          if (leftover.length > WGPU_TYPED_OPERATION_EFFECT_CAPACITY) throw new Error(`wgpu-bridge typed-operation host effects for instance ${instanceId} exceeded their ${WGPU_TYPED_OPERATION_EFFECT_CAPACITY}-entry authority`);
          pendingTurnEffects.set(instanceId, leftover);
          const jobDrain = await drainSpawnedJobs(instanceId, actorId);
          frames.push(...jobDrain.frames);
          if (frames.length > 0) turnOutcomes.push({ instanceId, frames });
          const latest = jobDrain.last ?? accepted.at(-1);
          return { status: latest?.status, nextWake: latest?.nextWake ?? null };
        }),
        yieldWgpuUi,
      );
      drive.report(`drain polls=${outcome.polls} stopped=${outcome.stopped}`);
    } catch (error) {
      console.error(`wgpu-bridge typed-operation drain failed for instance ${instanceId}`, error);
      stashOperationPublication(instanceId, { completed: false, fault: error instanceof Error ? error.message : String(error) });
      turnOutcomes.push({ instanceId, error });
    } finally {
      drainingInstances.delete(instanceId);
    }
  };
  const runQueuedTurnSerialized = async (instanceId: number, actorId: string, events: readonly Uint8Array[]): Promise<void> => {
    try {
      requireActorId(instanceId);
      const route = requireUiRoute(instanceId);
      const execute = executeFor(actorId);
      const results: WireTurnResult[] = [];
      const drive = new WgpuTypedOperationDrive(instanceId);
      const accept = async (turn: WireTurnResult): Promise<void> => {
        const accepted = [turn, ...await route.accept(turn, execute)];
        results.push(...accepted);
        drive.observe(accepted);
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
        let terminal: string | undefined;
        for (const commandPage of pages) {
          const submitted = await submitTurn(actorId, drive.takeAcknowledgements(), commandPage);
          terminal = submitted.commandIngress?.tag;
          await accept(submitted);
        }
        for (let continuation = 0; terminal !== "command-complete" && continuation < 1_024; continuation += 1) {
          if (terminal === "fault") throw new Error(`plugin ${pluginId}: command ingress fault`);
          if (terminal === "backpressure") throw new Error(`plugin ${pluginId}: command ingress backpressure after serialized submission`);
          const continued = await submitTurn(actorId, drive.takeAcknowledgements());
          await accept(continued);
          terminal = continued.commandIngress?.tag;
        }
        if (terminal !== "command-complete") throw new Error(`plugin ${pluginId}: command ingress did not complete within 1024 continuations`);
      }
      for (let settle = 0; drive.owesASettle(results.at(-1)) && settle < WGPU_TYPED_OPERATION_SETTLE_LIMIT; settle += 1) {
        const before = results.length;
        await accept(await submitTurn(actorId, drive.takeAcknowledgements()));
        if (!drive.progressed(results.slice(before))) break;
        await yieldWgpuUi();
      }
      const outFrames: Uint8Array[] = [];
      for (const effect of drive.hostEffects(results)) {
        const frame = shellFrameBytes(effect, instanceId);
        if (frame && shellFrameAnswersACaller(frame)) outFrames.push(frame);
      }
      stashLeftoverHostEffects(instanceId, drive.hostEffects(results));
      const jobDrain = await drainSpawnedJobs(instanceId, actorId);
      outFrames.push(...jobDrain.frames);
      const leftover = pendingTurnEffects.get(instanceId) ?? [];
      const leftoverFriendly = leftover.map((effect) => wgpuHostEffect(effect)).filter((effect): effect is Effect => effect !== null);
      drive.report("command");
      turnOutcomes.push({ instanceId, frames: outFrames });
      if (wireTurnStatusTag((jobDrain.last ?? results.at(-1))?.status) === "more-work") void drainTypedOperations(instanceId);
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    }
  };
  /** 🔀️ Frames every `AppCommand`/`AppFrame` `AppChannelClient` sends through — one `"app-command"`
   * shard event per batched frame, demuxing the resulting turn's `Effect::SendMessage{Shell}` replies
   * back into frames and stashing everything else as this instance's leftover effects
   * (`performInvocation` drains them) — pushed onto {@link turnOutcomes} instead of returned, since
   * `PluginWasmHandle.enqueue` is fire-and-forget (channel v12/H1-react retired the old synchronous
   * `exchange(instanceId, frames) -> Promise<frames>` RPC shape this file was first ported against). A
   * turn-submission failure becomes an `error`-shaped outcome rather than an uncaught rejection, since
   * nothing here awaits this function's own promise. Mirrors `PluginRuntime`'s own `runQueuedTurn`. */
  const runQueuedTurn = async (instanceId: number, events: readonly Uint8Array[], dispatch?: PluginDispatchHintV1): Promise<void> => {
    try {
      const actorId = requireActorId(instanceId);
      await serializeWgpuActorCall(actorId, () => runQueuedTurnSerialized(instanceId, actorId, events), dispatch?.order);
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    }
  };
  const channelHandle: Pick<KernelPluginWasmHandle, "enqueue" | "outcomes"> = {
    enqueue: (instanceId, events, dispatch) => {
      void runQueuedTurn(instanceId, events, dispatch);
    },
    outcomes: turnOutcomes.stream,
  };
  return { drainSpawnedJobs, turnOutcomes, drainTypedOperations, channelHandle };
}

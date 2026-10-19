import { activationReasonForAppId, appCommandIdsV1, createTurnOutcomeBroadcast, type Effect, fetchPackageDescriptor, GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, GUEST_HOST_ANSWER_CEILING_BYTES, guestAnswerPages, hostEffectInvocationV1, type InvocationResponse, type PluginDispatchHintV1, type PluginWasmHandle as KernelPluginWasmHandle, SemioFaultError, type SpawnedJobCompletion, type TurnOutcome } from "@semio-tech/framework";
import { AppChannelClient, AppChannelRequestSequence, type AppFrameValue, decodeAppFrame, decodeDocumentArchiveBytes, decodeFaultFromWire, decodePackValue, decodePackWire, encodeDocumentArchiveBytes, encodePackValue, faultDisplayMessage } from "@semio-tech/framework-os";
import { decodeCausalEnvelopeBatch } from "@semio-tech/framework-replication";
import { operationCompletionPublication, pendingOperationPublications, performInvocation, stashOperationProgress, stashOperationPublication, takeOperationPublications, WGPU_TYPED_OPERATION_DRAIN_BUDGET, WGPU_TYPED_OPERATION_EFFECT_CAPACITY, WGPU_TYPED_OPERATION_SETTLE_LIMIT, wgpuEphemeralSnapshot, type WgpuPluginExtensionCompletion, type WgpuPluginHandle, type WgpuPluginInvokeContext, WgpuTypedOperationDrive } from "../🧾️typed-operations-and-handle/🟦️.ts";
import { componentPackageReceiptSource } from "@semio-tech/framework-plugin/receipt";
import { admitSpawnedJob, contributionManifests, declareWgpuBootSubphase, forgetSpawnedJobs, jsonEffects, leftoverFriendlyEffects, loadedWgpuHandles, pendingTurnEffects, shellFrameAnswersACaller, stashLeftoverHostEffects, takeSpawnedJobs, WGPU_SPAWNED_JOB_ROUNDS, wgpuBuildScopedContributionsPack, wgpuContributionsIngressSize, wgpuEffectDispatchScope, wgpuHostEffect, wgpuSetContributionsCommand, wgpuSlimContributionsView, type WgpuSpawnedJobDrain } from "../🎛️contributions-and-effects/🟦️.ts";
import { actorTurnChains, extensionRequestActivationReason, getActivationRegistry, getShardClient, serializeWgpuActorCall, submitActorWork, submitTurn } from "../🧵️shard-and-actor-calls/🟦️.ts";
import { ShardInstanceLifecycleLease } from "@semio-tech/framework-actor/shard-client/activation";
import { RETAINED_DOCUMENT_OPPORTUNITIES, retireWgpuOwnedUiInstanceLifecycle, settleInstanceLifecycle, type WgpuActorExecutor, WgpuOwnedUiInstanceRoute, type WgpuOwnedUiProjection, yieldWgpuUi } from "../🍽️ui-intake-and-route/🟦️.ts";
import { coerceTurnResult, drainTypedOperationTurns, driveInboundRequest, driveSpawnedJob, INBOUND_REQUEST_TURN_BUDGET, shellFrameBytes, spawnedJobCompletedEvent, type SpawnedJobPort, type WireTurnResult, wireTurnStatusTag, type WireVariant } from "../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import { createShardCommandIngressPages, SHARD_COMMAND_MAXIMUM_PAGES, type ShardEventEnvelope } from "@semio-tech/framework-actor/shard-client/wire";
import { DOCUMENT_BACKBONE_BINDING_SCHEMA_V1, encodeDocumentBackboneControlV1, requireDocumentBackboneReceiptV1, splitDocumentBackboneControlTurnV1 } from "../../../../../../../../../🔨️modules/🛰️backbone/🔗️binding/🟦️.ts";
import { DEFAULT_SHARD_BUDGET } from "@semio-tech/framework-actor/shard-runtime";
import { settleFailedInstanceOpen } from "@semio-tech/framework-actor/shard-client/heartbeat";
import { mediaTransportPort } from "../../../../🎬️media/📡️channel/🟦️.ts";

/** 🪪️ Instance ids are unique across EVERY plugin `loadPluginModule` loads, not just within one call —
 * `pendingTurnEffects` is keyed by `instanceId` alone and shared module-wide, mirroring the kernel's own
 * single global `next_instance_id`. */
let nextGlobalInstanceId = 1;

/** 🐚️ Acquires a real actor through `ActivationRegistry`/`ShardClient` (replacing the deleted
 * `acquirePluginModule`/per-plugin `Worker` — design-runtime.md §3, copying `PluginRuntime`'s own
 * `loadPluginModule` shape). `dispose()` disposes every instance's worker-side actor entry via
 * `ShardClient.dispose` — no shared module lease to refcount any more, one actor belongs to exactly
 * one instance. */
export async function loadPluginModule(pluginId: string, moduleUrl: string, signal: AbortSignal | undefined, admittedDescriptor: import("@semio-tech/framework").PluginPackageDescriptor | undefined, consumedTopics: readonly string[]): Promise<WgpuPluginHandle> {
  signal?.throwIfAborted();
  const { manifest, packageId, componentSha256 } = admittedDescriptor ?? await fetchPackageDescriptor(pluginId, moduleUrl, componentPackageReceiptSource, signal);
  signal?.throwIfAborted();
  if (manifest.pluginId !== pluginId) throw new Error(`plugin.descriptor-identity-mismatch: ${pluginId}`);
  contributionManifests.set(pluginId, manifest);
  const registry = getActivationRegistry();
  registry.registerManifest({ pluginId, moduleUrl, caps: [] });
  const shardClient = getShardClient();
  const actorIdByInstance = new Map<number, string>();
  const channelByInstance = new Map<number, AppChannelClient>();
  const lifecycleByInstance = new Map<number, ShardInstanceLifecycleLease>();
  const uiRouteByInstance = new Map<number, WgpuOwnedUiInstanceRoute>();
  const openingInstances = new Map<number, Promise<number>>();
  const retiringInstances = new Map<number, Promise<void>>();
  const closingInstances = new Set<number>();
  const channelRequests = new AppChannelRequestSequence();
  let disposing = false;
  let disposal: Promise<void> | null = null;
  let eventSeq = 0;

  const requireActorId = (instanceId: number): string => {
    const actorId = actorIdByInstance.get(instanceId);
    if (disposing) throw new Error("wgpu-plugin-handle.closed");
    if (!actorId || closingInstances.has(instanceId)) throw new Error(`program ${pluginId}: no actor for instance ${instanceId} (createApp not called, or already destroyed)`);
    return actorId;
  };
  const requireChannel = (instanceId: number): AppChannelClient => {
    const client = channelByInstance.get(instanceId);
    if (!client) throw new Error(`program ${pluginId}: no channel for instance ${instanceId} (createApp not called, or already destroyed)`);
    return client;
  };
  const requireUiRoute = (instanceId: number): WgpuOwnedUiInstanceRoute => {
    const route = uiRouteByInstance.get(instanceId);
    if (!route) throw new Error("wgpu-ui.native-owner-required");
    return route;
  };
  const executeFor = (actorId: string): WgpuActorExecutor => <T>(work: () => Promise<T>) => submitActorWork(actorId, work);
  const codecActorId = (): string => {
    if (disposing) throw new Error("wgpu-plugin-handle.closed");
    for (const [instanceId, actorId] of actorIdByInstance) if (channelByInstance.has(instanceId) && !closingInstances.has(instanceId)) return actorId;
    throw new Error(`wgpu-plugin-codec.no-mounted-instance:${pluginId}`);
  };
  const releaseInstance = (instanceId: number, actorId: string): void => {
    actorIdByInstance.delete(instanceId);
    channelByInstance.delete(instanceId);
    lifecycleByInstance.delete(instanceId);
    uiRouteByInstance.delete(instanceId);
    pendingTurnEffects.delete(instanceId);
    pendingOperationPublications.delete(instanceId);
    actorTurnChains.delete(actorId);
    closingInstances.delete(instanceId);
  };

  const renderSurface = (instanceId: number, surfaceId: string, bodyKey: string, viewState: unknown): Promise<WgpuOwnedUiProjection> =>
    serializeWgpuActorCall(requireActorId(instanceId), () => renderSurfaceSerialized(instanceId, surfaceId, bodyKey, viewState));

  const renderSurfaceSerialized = async (instanceId: number, surfaceId: string, bodyKey: string, viewState: unknown): Promise<WgpuOwnedUiProjection> => {
    const actorId = requireActorId(instanceId);
    const route = requireUiRoute(instanceId);
    const execute = executeFor(actorId);
    const phase = declareWgpuBootSubphase(`shell-boot:render:${surfaceId}`);
    const turns: WireTurnResult[] = [];
    // 🧾️ Claim whatever the instance's leftover ledger still owes the host. A typed-operation drain
    // poll that ran between two host calls parks its effects there, and a render is the next door
    // they can reach the shell through — `performInvocation` is the other.
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

  /** 📤️📥️ Backs `channelHandle.enqueue`/`.outcomes` below — one broadcast per `loadPluginModule` call,
   * matching the handle's own lifetime: every instance this call's `createApp` ever opens shares it,
   * and each instance's `AppChannelClient` filters to its own `instanceId` (`pumpOutcomes`'s own doc in
   * `💻️os/🟦️.ts`). Mirrors `PluginRuntime`'s own `turnOutcomes`/`loadPluginModule` exactly. */
  const turnOutcomes = createTurnOutcomeBroadcast<TurnOutcome>();

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
        // 🔁️ A reserved tool job's completion turn is a turn like any other: if the guest still has
        // work left, SOMETHING has to keep polling it. Nothing else will — the command's own
        // `more-work` check reads the command turns, which ended before this job ever started — so an
        // operation the completion re-armed would sit forever and the shell's deferred action would
        // never resolve, wedging the interaction state it holds checked out.
      }
    }
    forgetSpawnedJobs(instanceId);
    return { frames, last };
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
        eventSeq += 1;
        const pages = createShardCommandIngressPages({
          owner: BigInt(instanceId),
          generation: 1n,
          commandIndex,
          commandCount: events.length,
          instance: instanceId,
          seq: BigInt(eventSeq),
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
      // 🧵️ Before the command's own reply is published: a reserved tool job's completion IS the rest
      // of this action (`interactionSelect` answers with nothing but the job), so its frames belong on
      // the same outcome the shell is already waiting on, after the admitted reply and in order.
      const jobDrain = await drainSpawnedJobs(instanceId, actorId);
      outFrames.push(...jobDrain.frames);
      const leftover = pendingTurnEffects.get(instanceId) ?? [];
      const leftoverFriendly = leftover.map((effect) => wgpuHostEffect(effect)).filter((effect): effect is Effect => effect !== null);
      drive.report("command");
      turnOutcomes.push({ instanceId, frames: outFrames });
      // 🔁️ The status that decides whether to keep polling is the LAST turn this call drove — a
      // reserved tool job's completion turn, when one ran. Reading the command's own last turn would
      // stop the drain on work the job itself re-armed, and nothing else would ever poll the guest.
      if (wireTurnStatusTag((jobDrain.last ?? results.at(-1))?.status) === "more-work") void drainTypedOperations(instanceId);
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    }
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
    const live = (): boolean => !disposing && !closingInstances.has(instanceId) && actorIdByInstance.has(instanceId);
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
          // 🧵️ A poll can uncover a reserved tool job just as a command can — an `interactionHover`
          // the guest armed between two host calls arrives here and nowhere else.
          const jobDrain = await drainSpawnedJobs(instanceId, actorId);
          frames.push(...jobDrain.frames);
          if (frames.length > 0) turnOutcomes.push({ instanceId, frames });
          // 🔁️ Same rule as the command path: this poll's verdict is the last turn it actually drove.
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

  const channelHandle: Pick<KernelPluginWasmHandle, "enqueue" | "outcomes"> = {
    enqueue: (instanceId, events, dispatch) => {
      void runQueuedTurn(instanceId, events, dispatch);
    },
    outcomes: turnOutcomes.stream,
  };

  const emptyInvocation = (requestedEffects: readonly Effect[] = []): InvocationResponse => ({
    output: null,
    mutations: [],
    inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] },
    diagnostics: [],
    requestedEffects: [...requestedEffects],
    events: [],
  });

  const captureExtensionCompletion = (instanceId: number, req: bigint): WgpuPluginExtensionCompletion => {
    if (typeof req !== "bigint" || req <= 0n || req > 0xffffffffffffffffn) throw new Error("extension.request-id-invalid");
    const actorId = requireActorId(instanceId);
    let submitted = false;
    const assertActive = (): void => { requireActorId(instanceId); };
    const complete = async (outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array }): Promise<InvocationResponse> => {
      assertActive();
      if (submitted) throw new Error("extension.completion-already-submitted");
      submitted = true;
      const answer = "ok" in outcome ? outcome.ok : outcome.fault;
      if (answer.byteLength > GUEST_HOST_ANSWER_CEILING_BYTES) {
        throw new SemioFaultError({
          origin: "os", code: "extension.answer-too-large", severity: "error",
          message: `extension answer of ${answer.byteLength} B for request ${req} exceeds the ${GUEST_HOST_ANSWER_CEILING_BYTES}-byte host-answer ceiling`,
          scope: { instanceId: String(instanceId) }, retryable: false,
        });
      }
      const { prologue, terminal } = guestAnswerPages(answer);
      const events: ShardEventEnvelope[] = [
        ...prologue.map((page) => ({ kind: "http-chunk" as const, payload: { req, params: { bytes: Array.from(page), done: false } } })),
        { kind: "completed" as const, payload: { req, outcome: "ok" in outcome ? { tag: "ok", val: Array.from(terminal) } : { tag: "fault", val: Array.from(terminal) } } },
      ];
      // 🧾️ The answer turn is a host call like any other: it must hold the actor against the standing
      // drain, acknowledge the result pages the resumed operation publishes, and hand the shell BOTH
      // its own leftovers and anything the ledger still owed.
      const carried = (pendingTurnEffects.get(instanceId) ?? []).map((effect) => wgpuHostEffect(effect)).filter((effect): effect is Effect => effect !== null);
      pendingTurnEffects.delete(instanceId);
      const drive = new WgpuTypedOperationDrive(instanceId);
      const frames: Uint8Array[] = [];
      const leftoverWire: WireVariant[] = [];
      const settled = await serializeWgpuActorCall(actorId, async () => {
        const route = requireUiRoute(instanceId);
        const execute = executeFor(actorId);
        const turn = await submitTurn(actorId, events);
        const accepted = [turn, ...await route.accept(turn, execute)];
        drive.observe(accepted);
        for (let settle = 0; drive.owesASettle(accepted.at(-1)) && settle < WGPU_TYPED_OPERATION_SETTLE_LIMIT; settle += 1) {
          const next = await submitTurn(actorId, drive.takeAcknowledgements());
          const more = [next, ...await route.accept(next, execute)];
          accepted.push(...more);
          drive.observe(more);
          if (!drive.progressed(more)) break;
          await yieldWgpuUi();
        }
        for (const effect of drive.hostEffects(accepted)) {
          if (admitSpawnedJob(instanceId, effect)) continue;
          const frame = shellFrameBytes(effect, instanceId);
          if (frame && shellFrameAnswersACaller(frame)) frames.push(frame);
          else leftoverWire.push(effect);
        }
        // 🧵️ An extension answer resumes a guest operation that may itself admit a reserved tool job;
        // the pump belongs inside this serialized body, never in a second one behind it.
        const jobDrain = await drainSpawnedJobs(instanceId, actorId);
        frames.push(...jobDrain.frames);
        if (jobDrain.last) accepted.push(jobDrain.last);
        return accepted;
      });
      drive.report(`completion req=${req}`);
      if (frames.length > 0) turnOutcomes.push({ instanceId, frames });
      if (wireTurnStatusTag(settled.at(-1)?.status) === "more-work") void drainTypedOperations(instanceId);
      return emptyInvocation([...carried, ...leftoverWire.map((effect) => wgpuHostEffect(effect)).filter((effect): effect is Effect => effect !== null)]);
    };
    return Object.freeze({ instanceId, req, assertActive, complete });
  };

  /** 📥️ See {@link WgpuPluginHandle.invoke}. One request actor per handle, activated lazily and
   * retired with it — an extension program declares no app, so it never gets a `createApp` instance
   * and `Event::Request` needs none. The actor activates `on-extension-request:<capability>` with the
   * capability whose request pulled it up — the declared extension point, not the host's `manual`. */
  let requestActor: Promise<string> | null = null;
  let inboundRequestSeq = 0n;
  const ensureRequestActor = (capability: string): Promise<string> => {
    if (disposing) return Promise.reject(new Error("wgpu-plugin-handle.closed"));
    requestActor ??= (async () => {
      const actorId = `${pluginId}#request`;
      await registry.activate(pluginId, actorId, extensionRequestActivationReason(capability));
      return actorId;
    })().catch((error: unknown) => {
      requestActor = null;
      throw error;
    });
    return requestActor;
  };
  const retireRequestActor = async (): Promise<void> => {
    const pending = requestActor;
    requestActor = null;
    if (!pending) return;
    const actorId = await pending.catch(() => null);
    if (!actorId) return;
    registry.cancel(actorId);
    actorTurnChains.delete(actorId);
  };

  const invoke = async (capability: string, request: Uint8Array | string, context?: WgpuPluginInvokeContext): Promise<Uint8Array> => {
    if (typeof capability !== "string" || capability.length === 0) throw new Error("extension.capability-required");
    const payload = typeof request === "string" ? new TextEncoder().encode(request) : request;
    const refuse = (code: string, message: string): SemioFaultError =>
      new SemioFaultError({ origin: "os", code, severity: "error", message, scope: { pluginId }, retryable: false });
    if (payload.byteLength > GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES) throw refuse("extension.request-too-large", `${capability} request of ${payload.byteLength} B for ${pluginId} exceeds the ${GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES}-byte contiguous guest-request ceiling`);
    const actorId = await ensureRequestActor(capability);
    inboundRequestSeq += 1n;
    const req = inboundRequestSeq;
    const answer = await driveInboundRequest({
      req,
      capability,
      payload,
      originInstanceId: context?.originInstanceId ?? 0,
      signal: context?.signal,
      onProgress: context?.onProgress,
      submit: (events) => submitTurn(actorId, events as readonly ShardEventEnvelope[]),
    });
    if (answer.status === "cancelled") throw refuse("extension.request-cancelled", `extension request ${req} to ${pluginId} was cancelled after ${answer.turns} turn(s)`);
    if (answer.status === "unanswered") throw refuse("extension.request-unanswered", `extension ${pluginId} did not answer ${capability} within ${answer.turns} of ${INBOUND_REQUEST_TURN_BUDGET} turns`);
    if ("fault" in answer.result) {
      const fault = decodeFaultFromWire(Array.from(answer.result.fault), decodePackValue);
      throw fault ? new SemioFaultError(fault) : refuse("extension.answer-not-a-fault", `extension ${pluginId} refused ${capability} with ${answer.result.fault.byteLength} undecodable bytes`);
    }
    if (answer.result.ok.byteLength > GUEST_HOST_ANSWER_CEILING_BYTES) throw refuse("extension.answer-too-large", `extension answer of ${answer.result.ok.byteLength} B exceeds the ${GUEST_HOST_ANSWER_CEILING_BYTES}-byte host-answer ceiling`);
    return answer.result.ok;
  };

  const dispatchInvokeExtension = async (instanceId: number, extensionId: string, capability: string, requestJson: string, req: bigint): Promise<InvocationResponse> => {
    const completion = captureExtensionCompletion(instanceId, req);
    let outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array };
    try {
      const extension = loadedWgpuHandles.get(extensionId) as WgpuPluginHandle | undefined;
      if (typeof extension?.invoke !== "function") throw new SemioFaultError({
        origin: "os", code: "extension.invoke-unavailable", severity: "error",
        message: "extension.invoke-unavailable",
        scope: { pluginId: extensionId, instanceId: String(instanceId) }, retryable: false,
      });
      const raw = await extension.invoke(capability, requestJson, { originInstanceId: instanceId });
      completion.assertActive();
      outcome = { ok: encodePackValue(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(raw))) };
    } catch (error) {
      completion.assertActive();
      const fault = error instanceof SemioFaultError ? error.fault : {
        origin: "os", code: "extension.invoke-failed", severity: "error",
        message: error instanceof Error ? error.message : String(error),
        scope: { pluginId: extensionId, instanceId: String(instanceId) }, retryable: false,
      };
      outcome = { fault: encodePackValue(fault) };
    }
    return completion.complete(outcome);
  };

  const pushScopedContributions = async (instanceId: number, appId: string, viewStateJson: string): Promise<InvocationResponse> => {
    let viewState: unknown = {};
    try {
      viewState = JSON.parse(viewStateJson);
    } catch {
      viewState = {};
    }
    const pack = wgpuBuildScopedContributionsPack(pluginId, undefined, consumedTopics);
    if (!pack) {
      return emptyInvocation();
    }
    const command = wgpuSetContributionsCommand(pluginId, appId, pack.json);
    const slimView = wgpuSlimContributionsView(viewState);
    const sequenceOwner = channelRequests.checkpoint().sequence;
    if (sequenceOwner === Number.MAX_SAFE_INTEGER) throw new Error("app-channel.sequence-exhausted");
    const ingress = wgpuContributionsIngressSize(command, slimView, sequenceOwner + 1);
    if (ingress.ingressPages > SHARD_COMMAND_MAXIMUM_PAGES) {
      throw new Error(`contributions pack ingress ${ingress.ingressPages} pages exceeds ${SHARD_COMMAND_MAXIMUM_PAGES}`);
    }
    const result = await performInvocation(requireChannel(instanceId), instanceId, command, slimView);
    const ticks: InvocationResponse[] = [];
    for (const effect of result.requestedEffects ?? []) {
      if (!effect || typeof effect !== "object" || !("dispatchAction" in effect)) continue;
      const dispatch = effect.dispatchAction as { readonly action: string; readonly args?: unknown };
      try {
        const addressed = hostEffectInvocationV1(wgpuEffectDispatchScope(pluginId, appId, slimView), appCommandIdsV1(manifest, appId), dispatch.action, (dispatch.args ?? {}) as Record<string, unknown>);
        const tick = await performInvocation(requireChannel(instanceId), instanceId, addressed.invocation, slimView);
        ticks.push(tick);
      } catch (error) {
      }
    }
    if (!ticks.length) return result;
    return { ...ticks[ticks.length - 1]!, requestedEffects: [...(result.requestedEffects ?? []), ...ticks.flatMap((tick) => tick.requestedEffects ?? [])] };
  };

  /** 📡️ One document-backbone turn — a bind/retire control or a hot backbone payload — held on the
   * actor like every other host call: the typed-operation settle, the UI patch intake and the spawned
   * job drain all run inside ONE serialized body. Answers the `Shell{instance}` frames the turn
   * produced (the receipt lane) apart from the host effects left for the shell. */
  const settleDocumentBackboneTurn = (instanceId: number, events: readonly ShardEventEnvelope[]): Promise<{ readonly frames: readonly Uint8Array[]; readonly effects: readonly Effect[] }> => {
    const actorId = requireActorId(instanceId);
    return serializeWgpuActorCall(actorId, async () => {
      const route = requireUiRoute(instanceId);
      const execute = executeFor(actorId);
      const drive = new WgpuTypedOperationDrive(instanceId);
      const turn = await submitTurn(actorId, events);
      const accepted = [turn, ...await route.accept(turn, execute)];
      drive.observe(accepted);
      for (let settle = 0; drive.owesASettle(accepted.at(-1)) && settle < WGPU_TYPED_OPERATION_SETTLE_LIMIT; settle += 1) {
        const next = await submitTurn(actorId, drive.takeAcknowledgements());
        const more = [next, ...await route.accept(next, execute)];
        accepted.push(...more);
        drive.observe(more);
        if (!drive.progressed(more)) break;
        await yieldWgpuUi();
      }
      const frames: Uint8Array[] = [];
      const effects: Effect[] = [];
      for (const effect of drive.hostEffects(accepted)) {
        if (admitSpawnedJob(instanceId, effect)) continue;
        const frame = shellFrameBytes(effect, instanceId);
        if (frame) frames.push(frame);
        else {
          const host = wgpuHostEffect(effect);
          if (host) effects.push(host);
        }
      }
      const jobDrain = await drainSpawnedJobs(instanceId, actorId);
      if (jobDrain.frames.length > 0) turnOutcomes.push({ instanceId, frames: [...jobDrain.frames] });
      drive.report("document-backbone");
      return { frames, effects };
    });
  };

  const documentBackbone = async (instanceId: number, operation: "bind" | "retire", bindingGeneration: bigint, uri: string): Promise<InvocationResponse> => {
    const command = { schema: DOCUMENT_BACKBONE_BINDING_SCHEMA_V1, operation, instanceId, bindingGeneration, uri };
    const control = encodeDocumentBackboneControlV1(command);
    const settled = await settleDocumentBackboneTurn(instanceId, [{ kind: "message", payload: { source: { tag: "shell", val: String(instanceId) }, payload: Array.from(control) } }]);
    const { receipts, unsolicited } = splitDocumentBackboneControlTurnV1(settled.frames);
    if (receipts.length !== 1) throw new Error(`actor-document-control.receipt-count:${receipts.length}`);
    requireDocumentBackboneReceiptV1(receipts[0]!, command);
    if (unsolicited.length > 0) turnOutcomes.push({ instanceId, frames: [...unsolicited] });
    return emptyInvocation(settled.effects);
  };

  const receiveDocumentBackbone = async (instanceId: number, uri: string, payload: Uint8Array): Promise<InvocationResponse> => {
    const settled = await settleDocumentBackboneTurn(instanceId, [{ kind: "message", payload: { source: { tag: "backbone", val: uri }, payload: Array.from(payload) } }]);
    const failed = settled.frames.map(decodeAppFrame).find((frame) => "Error" in frame);
    if (failed && "Error" in failed) throw new Error(faultDisplayMessage(failed.Error.fault, decodePackValue));
    return emptyInvocation(settled.effects);
  };

  const handle: WgpuPluginHandle = {
    pluginId,
    manifest,
    packageId,
    componentSha256,
    createApp: (appId) => {
      if (disposing) return Promise.reject(new Error("wgpu-plugin-handle.closed"));
      const instanceId = nextGlobalInstanceId;
      nextGlobalInstanceId += 1;
      const actorId = `${pluginId}#${instanceId}`;
      actorIdByInstance.set(instanceId, actorId);
      const requireOpening = (): void => {
        if (disposing || closingInstances.has(instanceId) || actorIdByInstance.get(instanceId) !== actorId) throw new Error("wgpu-plugin-handle.closed");
      };
      const opening = Promise.resolve().then(async () => {
        requireOpening();
        const phase = declareWgpuBootSubphase(`shell-boot:create-app:${pluginId}`);
        try {
        await registry.activate(pluginId, actorId, activationReasonForAppId(appId));
        requireOpening();
        eventSeq += 1;
        const lifecycle = shardClient.captureInstanceLifecycle(actorId, instanceId);
        lifecycleByInstance.set(instanceId, lifecycle);
        const execute = executeFor(actorId);
        const opened = coerceTurnResult(await execute(() => lifecycle.open({ appId, actor: "local", config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})) }, DEFAULT_SHARD_BUDGET)));
        const captured = lifecycle.pendingReceipt;
        if (!captured || captured.kind !== "captured" || !lifecycle.lifetime) throw new Error("wgpu-ui.native-lifetime-required");
        const route = new WgpuOwnedUiInstanceRoute(lifecycle);
        uiRouteByInstance.set(instanceId, route);
        requireOpening();
        await settleInstanceLifecycle(lifecycle, route, opened, execute, instanceId);
        requireOpening();
        const channel = new AppChannelClient(channelHandle, channelRequests, instanceId, appId);
        channel.onOperationCompleted((completion) => stashOperationPublication(instanceId, operationCompletionPublication(completion)));
        channelByInstance.set(instanceId, channel);
        return instanceId;
        } finally {
          phase.leave();
        }
      });
      openingInstances.set(instanceId, opening);
      const forget = (): void => { if (openingInstances.get(instanceId) === opening) openingInstances.delete(instanceId); };
      return opening.then((value) => { forget(); return value; }, (error) => {
        forget();
        return settleFailedInstanceOpen(error, () => (retiringInstances.has(instanceId) ? Promise.resolve() : handle.destroyApp(instanceId)));
      });
    },
    /** 🧹 Retires one instance. An instance whose `open` never produced a lifetime owns NO retained UI
     * — there is nothing to retire, only an actor to cancel — so a missing route joins the missing
     * lifecycle branch instead of throwing. Throwing here is what turned every guest first-step trap
     * into `create_app promise failed: wgpu-ui.native-owner-required` at the shell boundary: the
     * failed-open cleanup below awaits this call, and its rejection replaced the real cause. */
    destroyApp: (instanceId) => {
      const previous = retiringInstances.get(instanceId);
      if (previous) return previous;
      const actorId = actorIdByInstance.get(instanceId);
      if (!actorId) return Promise.resolve();
      closingInstances.add(instanceId);
      const opening = openingInstances.get(instanceId);
      const retirement = Promise.resolve().then(async () => {
        await opening?.then(() => {}, () => {});
        if (actorIdByInstance.get(instanceId) !== actorId) return;
        channelByInstance.get(instanceId)?.dispose();
        const lifecycle = lifecycleByInstance.get(instanceId);
        const route = uiRouteByInstance.get(instanceId);
        if (!lifecycle || !route) {
          registry.cancel(actorId);
          releaseInstance(instanceId, actorId);
          return;
        }
        await retireWgpuOwnedUiInstanceLifecycle(lifecycle, route, executeFor(actorId));
        registry.cancel(actorId);
        releaseInstance(instanceId, actorId);
      });
      retiringInstances.set(instanceId, retirement);
      const forget = (): void => { if (retiringInstances.get(instanceId) === retirement) retiringInstances.delete(instanceId); };
      void retirement.then(forget, forget);
      return retirement;
    },
    handleAction: (instanceId, invocation, viewState, dispatch) => performInvocation(requireChannel(instanceId), instanceId, invocation, viewState, dispatch),
    handleCommand: (instanceId, invocation, viewState, dispatch) => performInvocation(requireChannel(instanceId), instanceId, invocation, viewState, dispatch),
    render: (instanceId, surfaceId, bodyKey, viewState) => renderSurface(instanceId, surfaceId, bodyKey, viewState).then((result) => result.node),
    renderDocument: (instanceId, surfaceId, bodyKey, viewState) => renderSurface(instanceId, surfaceId, bodyKey, viewState).then((result) => JSON.stringify({ document: result.document, effects: jsonEffects(result.effects) })),
    ...mediaTransportPort((instanceId) => { requireActorId(instanceId); return requireChannel(instanceId); }),
    readAppDocumentIdentity: async (instanceId) => {
      requireActorId(instanceId);
      const identity = await requireChannel(instanceId).readDocumentIdentity();
      return { appInstanceId: identity.app_instance_id, parentDocumentId: identity.parent_document_id };
    },
    captureExtensionCompletion,
    invoke,
    dispatchInvokeExtension,
    pushScopedContributions,
    contextMenu: (instanceId, request) => requireChannel(instanceId).contextMenu(request),
    readWindowConfigPacks: (instanceId) => requireChannel(instanceId).readWindowConfigs(),
    loadWindowConfigPack: (instanceId, entry) => requireChannel(instanceId).loadWindowConfig(entry),
    documentBackbone,
    receiveDocumentBackbone,
    applyMutations: async (instanceId, operations) => {
      const frames = await requireChannel(instanceId).applyEnvelopes(decodeCausalEnvelopeBatch(Array.from(operations), { encode: encodePackValue, decode: decodePackValue }));
      const failed = frames.find((frame) => "Error" in frame);
      if (failed && "Error" in failed) throw new Error(faultDisplayMessage(failed.Error.fault, decodePackValue));
    },
    loadAppDocumentArchive: (instanceId, archive) => requireChannel(instanceId).loadDocumentArchive(decodeDocumentArchiveBytes(archive)),
    readAppDocumentArchive: async (instanceId) => encodeDocumentArchiveBytes(await requireChannel(instanceId).readDocumentArchive()),
    readHistory: async (instanceId) => {
      const frames = await requireChannel(instanceId).readHistory();
      const snapshot = frames.find((frame): frame is Extract<AppFrameValue, { readonly HistorySnapshot: unknown }> => "HistorySnapshot" in frame);
      if (snapshot) return decodePackWire(new Uint8Array(snapshot.HistorySnapshot.history_patch), "readHistory.historyPatch");
      const failed = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      throw new Error(failed ? faultDisplayMessage(failed.Error.fault, decodePackValue) : `readHistory(${instanceId}): no HistorySnapshot among ${frames.length} reply frame(s)`);
    },
    ephemeralSnapshot: (instanceId) => wgpuEphemeralSnapshot(channelByInstance.get(instanceId)?.ephemeral() ?? null),
    takeOperationPublications,
    codec: async (request) => {
      const actorId = codecActorId();
      const answer = await submitActorWork(actorId, () => shardClient.codec(actorId, request));
      if ("fault" in answer) throw new Error(`actor-codec.${request.operation}(${request.artifactKind}): ${faultDisplayMessage(Array.from(answer.fault), decodePackValue)}`);
      return answer.ok;
    },
    dispose: () => {
      if (disposal) return disposal;
      disposing = true;
      const retirements = [...actorIdByInstance.keys()].map((instanceId) => handle.destroyApp(instanceId));
      disposal = Promise.allSettled([...retirements, retireRequestActor()]).then((results) => {
        const failures = results.filter((result): result is PromiseRejectedResult => result.status === "rejected");
        if (failures.length) throw new AggregateError(failures.map((result) => result.reason), "wgpu-plugin-handle.retirement-failed");
        turnOutcomes.complete();
      });
      return disposal;
    },
  };
  loadedWgpuHandles.set(pluginId, handle);
  return handle;
}

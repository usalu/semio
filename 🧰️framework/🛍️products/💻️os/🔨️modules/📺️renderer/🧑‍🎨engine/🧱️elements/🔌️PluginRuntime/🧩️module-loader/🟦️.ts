/** 🧩️ `loadPluginModule` and the phases that assemble the handle: descriptor resolution, the kernel handle and the refresh, extension-completion, inbound-invoke and document-port surfaces.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { publishedPageUrl } from "@semio-tech/framework/assets/published-page";
// #region 🔌️Adapters
import {
  type InvocationResponse,
  type PluginUiRefreshRequest,
  type PluginUiRefreshResponse,
  SemioFaultError,
  GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
  GUEST_HOST_ANSWER_CEILING_BYTES,
  guestAnswerPages
} from "@semio-tech/framework";
// #endregion 🔌️Adapters
import {
  decodePackValue,
  encodePackValue
} from "@semio-tech/framework/packs";
import {
  decodeAppFrame,
  decodeConflictsFromWire,
  decodeFaultFromWire,
  decodeMergeReportFromWire,
  faultDisplayMessage
} from "@semio-tech/framework/app-channel";
import { DOCUMENT_BACKBONE_RETENTION_LIMITS } from "@semio-tech/framework/replication";
import { DEFAULT_UI_DOCUMENT_LIMITS } from "../../📃️UiDocumentStore/🟦️.tsx";
import { activationReasonForAppId } from "@semio-tech/framework";
import { descriptorPackageAssets, fetchDescriptorManifest } from "@semio-tech/framework";
import { type PluginWasmHandle as KernelPluginWasmHandle } from "@semio-tech/framework";
import { settleFailedInstanceOpen } from "@semio-tech/framework/actor/shard-client/heartbeat";
import { type ShardEventEnvelope } from "@semio-tech/framework/actor/shard-client/wire";
import { OwnedUiInstance } from "@semio-tech/ui-contract/retained/instance";
import { hopTrace } from "@semio-tech/framework-trace";
import { driveInboundRequest, INBOUND_REQUEST_TURN_BUDGET } from "@semio-tech/framework/actor/wire-turn";
import { BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES } from "@semio-tech/framework/wire-contracts";
import {
  ActorDocumentBindingV1,
  type ActorDocumentMessagePortV1,
  bindActorDocumentV1,
  encodeDocumentBackboneControlV1,
  releaseActorDocumentBindingV1,
  splitDocumentBackboneControlTurnV1
} from "@semio-tech/framework/backbone/binding";
import { notePluginLoadProgress, notePluginLoadProgressForInFlightV1, sharedDescriptorManifestV1, withPluginLoadInFlightV1 } from "../🫀️load-progress/🟦️.ts";
import { type PluginDocumentBindingV1, type PluginExtensionCompletion, type PluginExtensionInvokeContext, type PluginWasmHandle } from "../🧷️handle-contract/🟦️.ts";
import { documentPortHotBytes, retainedSurfaceId, shellFrameBytes, type WireVariant } from "../🪝️wire-codec/🟦️.ts";
import { withTypedOperationCall } from "../📬️typed-result/🟦️.ts";
import { getActivationRegistry, rememberInstanceForRecovery } from "../🛰️runtime-singletons/🟦️.ts";
import { serializeCommandIngressForActor, submitPluginLifecycleTurn } from "../🧵️turn-dispatch/🟦️.ts";
import { settlePluginTurn } from "../🔁️turn-settle/🟦️.ts";
import { uiRefreshSurfaceEvents } from "../🖼️surface-refresh/🟦️.ts";
import { invocationFromFrames, noteGuestIngressV1 } from "../📥️invocation-frames/🟦️.ts";
import { adaptPluginHandle, currentPluginRuntimeActor } from "../🔗️channel-adapter/🟦️.ts";
import {
  createInstanceTable,
  createPluginHandleScope,
  createRequestActor,
  type PluginHandleScope,
  takeNextGlobalInstanceId
} from "../🪢️handle-scope/🟦️.ts";
import { createRetainedUiIntake } from "../🧺️handle-ui-intake/🟦️.ts";
import { createSpawnedJobDriver, createTurnPump } from "../🚚️handle-turn-drive/🟦️.ts";

//#region 🔖️plugin-runtime
/** 🚦️ Resolves the small descriptor request before starting the shard-worker module graph, keeping
 * cold browser connection capacity available for the request that lets plugin loading complete. */
export async function resolveDescriptorBeforeRuntime<TManifest, TRuntime>(loadDescriptor: () => Promise<TManifest>, initializeRuntime: () => TRuntime): Promise<{ readonly manifest: TManifest; readonly runtime: TRuntime }> {
  const manifest = await loadDescriptor();
  return { manifest, runtime: initializeRuntime() };
}

/** 🚦️ Phase 1 of {@link loadPluginModule}: resolves the descriptor manifest and registers it with the shared
 * `ActivationRegistry`. Returns the in-flight load promise itself (never an `async` wrapper around it), so
 * the caller's `await` costs exactly the microtask hops the inline form did. */
function resolvePluginDescriptorAndRegistry(pluginId: string, moduleUrl: string, signal: AbortSignal | undefined, manifestPluginId: string) {
  moduleUrl = publishedPageUrl(moduleUrl);
  return withPluginLoadInFlightV1(pluginId, async () => {
    const resolved = await resolveDescriptorBeforeRuntime(
      () => sharedDescriptorManifestV1(pluginId, moduleUrl, () => fetchDescriptorManifest(manifestPluginId, moduleUrl, signal, () => {notePluginLoadProgressForInFlightV1();})),
      getActivationRegistry,
    );
    notePluginLoadProgress(pluginId);
    const assets = descriptorPackageAssets(resolved.manifest);
    resolved.runtime.registerManifest({ pluginId, moduleUrl, caps: [], ...(manifestPluginId === pluginId ? {} : { manifestPluginId }), ...(assets.length === 0 ? {} : { assets }) });
    notePluginLoadProgress(pluginId);
    return resolved;
  });
}

/** 🧬️ Phase 8 of {@link loadPluginModule}: the kernel `PluginWasmHandle` this module constructs — instance creation,
 * retirement, the `enqueue`/`outcomes` turn ABI and disposal. */
function createKernelHandle(scope: PluginHandleScope, instances: ReturnType<typeof createInstanceTable>, ui: ReturnType<typeof createRetainedUiIntake>, requestActors: ReturnType<typeof createRequestActor>, turns: ReturnType<typeof createTurnPump>): KernelPluginWasmHandle {
  const { pluginId, manifest, registry, shardClient, turnOutcomes, state, actorIdByInstance, closingInstances, openingInstances, retiringInstances, documentBindings, lifecycleByInstance, uiOwnerByInstance, uiSurfaceByInstance, uiIntakesByInstance, uiReadsByInstance } = scope;
  const { requireActorId, releaseInstanceMaps } = instances;
  const { acceptUiPatches, retireInstanceLifecycle } = ui;
  const { retireRequestActor } = requestActors;
  const { runQueuedTurn } = turns;
  const handle: KernelPluginWasmHandle = {
    manifest,
    createApp: (appId) => {
      if (state.disposing) return Promise.reject(new Error("plugin-handle.closed"));
      const instanceId = takeNextGlobalInstanceId();
      const actorId = `${pluginId}#${instanceId}`;
      actorIdByInstance.set(instanceId, actorId);
      const requireOpening = () => {
        if (state.disposing || closingInstances.has(instanceId) || actorIdByInstance.get(instanceId) !== actorId) throw new Error("plugin-handle.closed");
      };
      const opening = Promise.resolve().then(async () => {
        requireOpening();
        await registry.activate(pluginId, actorId, activationReasonForAppId(appId));
        requireOpening();
        state.eventSeq += 1;
        const lease = shardClient.captureInstanceLifecycle(actorId, instanceId);
        lifecycleByInstance.set(instanceId, lease);
        registry.touch(actorId);
        const opened = await submitPluginLifecycleTurn(
          lease,
          { kind: "open", input: { appId, actor: currentPluginRuntimeActor, config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})) } },
          "Interactive",
        );
        const captured = lease.pendingReceipt;
        const lifetime = lease.lifetime;
        if (!captured || captured.kind !== "captured" || !lifetime) throw new Error("plugin-ui.native-lifetime-required");
        const owner = new OwnedUiInstance(lease.activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 32 });
        lease.bindHostRetirement(owner);
        uiOwnerByInstance.set(instanceId, owner);
        uiSurfaceByInstance.set(instanceId, new Map());
        uiIntakesByInstance.set(instanceId, new Set());
        rememberInstanceForRecovery({ pluginId, instanceId, actorId, release: () => releaseInstanceMaps(instanceId, actorId) });
        requireOpening();
        await withTypedOperationCall(actorId, `open#${instanceId}`, async (call) => {
          await settlePluginTurn(actorId, opened.turn, "Interactive", new Set(), (turn) => acceptUiPatches(instanceId, turn), false, undefined, call);
          requireOpening();
          const acknowledged = await submitPluginLifecycleTurn(lease, { kind: "receipt-ack", receipt: captured }, "Interactive");
          await settlePluginTurn(actorId, acknowledged.turn, "Interactive", new Set(), (turn) => acceptUiPatches(instanceId, turn), false, undefined, call);
        });
        requireOpening();
        return instanceId;
      });
      openingInstances.set(instanceId, opening);
      const forget = () => { if (openingInstances.get(instanceId) === opening) openingInstances.delete(instanceId); };
      return opening.then(value => { forget(); return value; }, error => {
        forget();
        return settleFailedInstanceOpen(error, () => handle.destroyApp(instanceId));
      });
    },
    /** 🧹 Retires one instance. An instance whose `open` never bound a UI owner holds NO retained UI
     * — there is nothing to retire, only an actor to cancel — so a missing owner joins the missing
     * lifecycle branch instead of throwing. Throwing here is what turned a mid-boot
     * `shard 0 terminated` into `Framework OS boot failed: plugin-ui.native-owner-required`: the
     * failed-open cleanup above awaited this call, and its rejection replaced the real cause. */
    destroyApp: (instanceId) => {
      const previous = retiringInstances.get(instanceId);
      if (previous) return previous;
      const actorId = actorIdByInstance.get(instanceId);
      if (!actorId) return Promise.resolve();
      closingInstances.add(instanceId);
      const opening = openingInstances.get(instanceId);
      const retirement = documentBindings.get(instanceId)?.port.retire();
      const retiring = Promise.resolve().then(async () => {
        ui.beginOwnedUiSurfaceClose(instanceId);
        await Promise.all([opening?.then(() => {}, () => {}), retirement]);
        if (actorIdByInstance.get(instanceId) === actorId) {
          const lease = lifecycleByInstance.get(instanceId);
          const owner = uiOwnerByInstance.get(instanceId);
          if (!lease || !owner) {
            registry.cancel(actorId);
            releaseInstanceMaps(instanceId, actorId);
            return;
          }
          await Promise.allSettled([...(uiReadsByInstance.get(instanceId) ?? [])]);
          await retireInstanceLifecycle(instanceId, lease, owner);
          registry.cancel(actorId);
          releaseInstanceMaps(instanceId, actorId);
        }
      });
      retiringInstances.set(instanceId, retiring);
      const forget = () => { if (retiringInstances.get(instanceId) === retiring) retiringInstances.delete(instanceId); };
      void retiring.then(forget, forget);
      return retiring;
    },
    takeSegmentedDownloadChunk: (instanceId, operationId) => shardClient.takeSegmentedDownloadChunk(requireActorId(instanceId), instanceId, operationId),
    enqueue: (instanceId, events, dispatch) => {
      requireActorId(instanceId);
      void runQueuedTurn(instanceId, events, dispatch);
    },
    outcomes: turnOutcomes.stream,
    dispose: () => {
      if (state.disposal) return state.disposal;
      state.disposing = true;
      const retirements = [...actorIdByInstance.keys()].map(instanceId => handle.destroyApp(instanceId));
      state.disposal = Promise.allSettled([...retirements, retireRequestActor()]).then(results => {
        const failures = results.filter((result): result is PromiseRejectedResult => result.status === "rejected");
        if (failures.length) throw new AggregateError(failures.map(result => result.reason), "plugin-handle.retirement-failed");
        turnOutcomes.complete();
      });
      return state.disposal;
    },
  };
  return handle;
}

/** 🔁️ Phase 9 of {@link loadPluginModule}: window-body refresh through `Event::SurfaceVisible` on the instance's actor. */
function createRefreshUi(scope: PluginHandleScope, instances: ReturnType<typeof createInstanceTable>, ui: ReturnType<typeof createRetainedUiIntake>, jobs: ReturnType<typeof createSpawnedJobDriver>) {
  const { shardClient, state, documentBindings, uiSurfaceByInstance } = scope;
  const { requireActorId, submitTurn } = instances;
  const { acceptUiPatches, ownedUiRefreshResponse } = ui;
  const { routeHostEffects } = jobs;
  /** 🔁️ H1-react item 2 — window-body refresh no longer goes through `AppCommand::RefreshUi`
   * (deleted, channel v12): it submits `Event::SurfaceVisible` directly and reads back whatever this
   * SAME turn's `TurnResult.uiPatches` produced (or the retained tree if nothing changed — the
   * `PatchTracker` on the guest side emits nothing for an unchanged body). Every requested window is
   * identified by its schema-owned body key; continuations drain until all surfaces missing from the
   * retained actor state publish their first patch. Panels use the same authored-body surface path. */
  const refreshUi = async (instanceId: number, request: PluginUiRefreshRequest): Promise<PluginUiRefreshResponse> => {
    const events = uiRefreshSurfaceEvents(instanceId, request);
    if (events.length === 0) return {};
    const actorId = requireActorId(instanceId);
    const activation = shardClient.captureActorActivation(actorId);
    const documentPort = documentBindings.get(instanceId)?.port;
    state.eventSeq += 1;
    const requestedSurfaceIds = new Set(events.map((event) => retainedSurfaceId(instanceId, (event.payload as { readonly surface: { readonly surface: string } }).surface.surface)));
    const missingAtSubmission = (): ReadonlySet<string> => {
      const surfaces = uiSurfaceByInstance.get(instanceId);
      return new Set([...requestedSurfaceIds].filter((surfaceId) => {
        const surface = surfaces?.get(surfaceId);
        return !surface || surface.view.root === null;
      }));
    };
    const closeTurn = hopTrace.open("refresh.turn", { instanceId, events: events.length, requested: requestedSurfaceIds.size });
    const result = await (async () => {
      try {
        return await withTypedOperationCall(actorId, `refresh-ui#${instanceId}`, (call) => serializeCommandIngressForActor(actorId, async () => {
          const missingSurfaceIds = missingAtSubmission();
          const settled = await settlePluginTurn(
            actorId,
            await submitTurn(
              actorId,
              events,
              { lane: "UserVisible", activation },
            ),
            "UserVisible",
            missingSurfaceIds,
            (turn) => acceptUiPatches(instanceId, turn),
            true,
            activation,
            call,
            requestedSurfaceIds,
            (outcome) => closeTurn({ stop: outcome.stop, continuations: outcome.continuations, patches: outcome.patches, owedAtStop: outcome.owed }),
          );
          return settled;
        }));
      } finally {
        closeTurn();
      }
    })();
    requireActorId(instanceId);
    activation.assertActive();
    return hopTrace.timeAsync("refresh.project", { instanceId, windows: (request.windows ?? []).length, panels: (request.panels ?? []).length }, () =>
      ownedUiRefreshResponse(instanceId, actorId, request, routeHostEffects(instanceId, result.effects, documentPort)),
    );
  };
  return refreshUi;
}

/** 🔁️ Phase 10 of {@link loadPluginModule}: retains one completion's exact activation across evaluation, queueing and publication. */
function createExtensionCompletionCapture(scope: PluginHandleScope, instances: ReturnType<typeof createInstanceTable>, ui: ReturnType<typeof createRetainedUiIntake>, jobs: ReturnType<typeof createSpawnedJobDriver>) {
  const { shardClient, documentBindings, turnOutcomes } = scope;
  const { requireActorId, submitTurn } = instances;
  const { acceptUiPatches } = ui;
  const { routeHostEffects } = jobs;
  /** 🔁️ Retains one completion's exact activation across evaluation, queueing and publication. */
  const captureExtensionCompletion = (instanceId: number, req: bigint): PluginExtensionCompletion => {
    if (typeof req !== "bigint" || req <= 0n || req > 0xffffffffffffffffn) throw new Error("extension.request-id-invalid");
    const actorId = requireActorId(instanceId);
    const activation = shardClient.captureActorActivation(actorId);
    const documentPort = documentBindings.get(instanceId)?.port;
    let submitted = false;
    const assertActive = (): void => { requireActorId(instanceId); activation.assertActive(); };
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
      noteGuestIngressV1(instanceId);
      return withTypedOperationCall(actorId, `extension-completion#${instanceId}`, (call) => serializeCommandIngressForActor(actorId, async () => {
        assertActive();
        const settled = await settlePluginTurn(
          actorId,
          await submitTurn(actorId, events, { activation }),
          "Interactive",
          new Set(),
          (turn) => { assertActive(); return acceptUiPatches(instanceId, turn); },
          true,
          activation,
          call,
        );
        assertActive();
        const frames: Uint8Array[] = [];
        const effects: WireVariant[] = [];
        for (const effect of routeHostEffects(instanceId, settled.effects, documentPort)) {
          const frame = shellFrameBytes(effect, instanceId);
          if (frame) frames.push(frame);
          else effects.push(effect);
        }
        turnOutcomes.push({ instanceId, frames });
        return invocationFromFrames(frames.map(decodeAppFrame), effects, "extension completion");
      }));
    };
    return Object.freeze({ instanceId, req, assertActive, complete });
  };
  return captureExtensionCompletion;
}

/** 📥️ Phase 11 of {@link loadPluginModule}: the inbound-call door of this handle's request actor. */
function createInboundInvoke(scope: PluginHandleScope, instances: ReturnType<typeof createInstanceTable>, requestActors: ReturnType<typeof createRequestActor>) {
  const { pluginId, shardClient, state } = scope;
  const { submitTurn } = instances;
  const { ensureRequestActor } = requestActors;
  /** 📥️ The host half of the ABI's ONE inbound-call seam. `📜️.wit`'s `request-event` "replaces
   * `extension.invoke`, `artifact-compose`, the guest `io-run`/`io-sniff` exports … every 'someone
   * else calls INTO this actor' seam", answered by the `respond` effect — so a capability call is a
   * SHARD EVENT on the callee's own actor, not a second export. Until this existed, every React-side
   * extension invocation died at `extension.invoke-unavailable` before the guest was ever entered
   * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END — eight boots of `meshes: 0`).
   *
   * 🚪️ The callee's actor is this handle's OWN request actor: an extension program declares no app,
   * so it never gets a `createApp` instance, and `Event::Request` needs none — the guest's turn loop
   * serves it out of the installed `ExtensionBundle`, which is actor-scoped. One actor per handle,
   * activated lazily on the first call and torn down with the handle.
   *
   * 📏️ `request` crosses as ONE contiguous `pack`, so it is refused past the guest's contiguous
   * ceiling rather than aborting its allocator (`🧮️memory/🟦️.ts`'s own header: a block the guest
   * refuses is a trap, not a fault). The ANSWER comes back inside a turn result and is bounded by the
   * same host-answer ceiling `captureExtensionCompletion` delivers against.
   *
   * @returns the guest's raw `respond` OK bytes; a `fault` arm is thrown as a {@link SemioFaultError}
   * carrying the guest's own decoded fault, so the caller's typed-refusal branch stays the one door. */
  const invoke = async (capability: string, request: Uint8Array | string, context?: PluginExtensionInvokeContext): Promise<Uint8Array> => {
    if (typeof capability !== "string" || capability.length === 0) throw new Error("extension.capability-required");
    const payload = typeof request === "string" ? new TextEncoder().encode(request) : request;
    const refuse = (code: string, message: string): SemioFaultError =>
      new SemioFaultError({ origin: "os", code, severity: "error", message, scope: { pluginId }, retryable: false });
    if (payload.byteLength > GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES) {
      throw refuse("extension.request-too-large", `${capability} request of ${payload.byteLength} B for ${pluginId} exceeds the ${GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES}-byte contiguous guest-request ceiling`);
    }
    const actorId = await ensureRequestActor();
    const activation = shardClient.captureActorActivation(actorId);
    state.extensionRequestSeq += 1n;
    const req = state.extensionRequestSeq;
    const answer = await serializeCommandIngressForActor(actorId, () =>
      driveInboundRequest({
        req,
        capability,
        payload,
        originInstanceId: context?.originInstanceId ?? 0,
        signal: context?.signal,
        onProgress: context?.onProgress,
        submit: (events) => {
          activation.assertActive();
          return submitTurn(actorId, events, { activation });
        },
      }));
    activation.assertActive();
    if (answer.status === "cancelled") throw refuse("extension.request-cancelled", `extension request ${req} to ${pluginId} was cancelled after ${answer.turns} turn(s)`);
    if (answer.status === "unanswered") throw refuse("extension.request-unanswered", `extension ${pluginId} did not answer ${capability} within ${answer.turns} of ${INBOUND_REQUEST_TURN_BUDGET} turns`);
    if ("fault" in answer.result) {
      const fault = decodeFaultFromWire(Array.from(answer.result.fault), decodePackValue);
      throw fault ? new SemioFaultError(fault) : refuse("extension.answer-not-a-fault", `extension ${pluginId} refused ${capability} with ${answer.result.fault.byteLength} undecodable bytes`);
    }
    if (answer.result.ok.byteLength > GUEST_HOST_ANSWER_CEILING_BYTES) throw refuse("extension.answer-too-large", `extension answer of ${answer.result.ok.byteLength} B exceeds the ${GUEST_HOST_ANSWER_CEILING_BYTES}-byte host-answer ceiling`);
    return answer.result.ok;
  };
  return invoke;
}

/** 🔗️ Phase 12 of {@link loadPluginModule}: binds an actor-owned document message port to one instance. */
function createDocumentPortBinder(scope: PluginHandleScope, instances: ReturnType<typeof createInstanceTable>, ui: ReturnType<typeof createRetainedUiIntake>, jobs: ReturnType<typeof createSpawnedJobDriver>) {
  const { shardClient, closingInstances, actorIdByInstance, documentBindings, documentBindingGenerations, turnOutcomes } = scope;
  const { requireActorId, submitTurn } = instances;
  const { acceptUiPatches } = ui;
  const { routeHostEffects } = jobs;
  const bindDocumentPort = async (instanceId: number, source: PluginDocumentBindingV1): Promise<ActorDocumentMessagePortV1> => {
    const identity = Object.freeze({ runtimeKey: source.runtimeKey, clientInstanceId: source.clientInstanceId, scope: source.scope === null ? null : Object.freeze({ ...source.scope }) });
    const { current: sourceCurrent, send, merge: receiveMerge, prepared } = source;
    const actorId = requireActorId(instanceId);
    const activation = shardClient.captureActorActivation(actorId);
    const previous = documentBindings.get(instanceId);
    if (previous && !previous.port.closing) throw new Error("actor-document-control.binding-live");
    if (previous) await releaseActorDocumentBindingV1(previous);
    if (documentBindings.get(instanceId) !== previous) throw new Error("actor-document-control.binding-collision");
    requireActorId(instanceId);
    activation.assertActive();
    const bindingGeneration = (documentBindingGenerations.get(instanceId) ?? 0n) + 1n;
    const current = () => !closingInstances.has(instanceId) && actorIdByInstance.get(instanceId) === actorId && sourceCurrent();
    const settle = (events: readonly ShardEventEnvelope[]) => withTypedOperationCall(actorId, `document-port#${instanceId}`, (call) => serializeCommandIngressForActor(actorId, async () => {
      activation.assertActive();
      return settlePluginTurn(actorId, await submitTurn(actorId, events, { activation }), "Interactive", new Set(), turn => acceptUiPatches(instanceId, turn), true, activation, call);
    }));
    const binding = new ActorDocumentBindingV1({ ...identity, actorId, activationGeneration: activation.activationGeneration, instanceId }, bindingGeneration, {
      current,
      assertActive: () => activation.assertActive(),
      limits: { messageBytes: BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, pendingBytes: DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes, pendingMessages: DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages },
      send,
      exchange: async command => {
        const result = await settle([{ kind: "message", payload: { source: { tag: "shell", val: String(instanceId) }, payload: Array.from(encodeDocumentBackboneControlV1(command)) } }]);
        const { receipts, unsolicited } = splitDocumentBackboneControlTurnV1(result.effects.flatMap(effect => {
          const bytes = shellFrameBytes(effect, instanceId);
          if (bytes) return [bytes];
          if (effect.tag === "send-message" && (effect.val as { target?: WireVariant } | undefined)?.target?.tag === "backbone") throw new Error("actor-document-control.data-before-receipt");
          return [];
        }));
        if (unsolicited.length > 0) turnOutcomes.push({ instanceId, frames: [...unsolicited] });
        return receipts;
      },
      deliver: async payload => {
        const result = await settle([{ kind: "message", payload: { source: { tag: "backbone", val: binding.port.uri }, payload: Array.from(documentPortHotBytes(payload)) } }]);
        if (!current()) return;
        const frames = routeHostEffects(instanceId, result.effects, binding.port).flatMap(effect => {
          const bytes = shellFrameBytes(effect, instanceId);
          return bytes ? [bytes] : [];
        });
        const values = frames.map(decodeAppFrame);
        const failed = values.find(value => "Error" in value);
        if (failed && "Error" in failed) throw new Error(faultDisplayMessage(failed.Error.fault, decodePackValue));
        const merge = values.find(value => "MergeReport" in value);
        const conflicts = values.find(value => "Conflicts" in value);
        if (merge || conflicts) receiveMerge?.(conflicts && "Conflicts" in conflicts ? decodeConflictsFromWire(conflicts.Conflicts.conflicts, decodePackValue) : null, merge && "MergeReport" in merge ? decodeMergeReportFromWire(merge.MergeReport.report, decodePackValue) : null);
        turnOutcomes.push({ instanceId, frames });
      },
    });
    documentBindings.set(instanceId, binding);
    documentBindingGenerations.set(instanceId, bindingGeneration);
    return bindActorDocumentV1(binding, prepared, (retirement) => {
      if (retirement !== null) console.error("[plugin-runtime] a document port that failed to bind could not be retired", binding.port.uri, retirement.error);
      else if (documentBindings.get(instanceId) === binding) documentBindings.delete(instanceId);
    });
  };
  return bindDocumentPort;
}

/** 🐚️ Acquires a real actor through `ActivationRegistry`/`ShardClient` (replacing the deleted
 * `acquirePluginModule`/`PluginModuleLease` per-plugin Worker lease — design-runtime.md §3) and
 * adapts it exactly like the old wasm-Worker handle: `dispose()` disposes this instance's worker-side
 * actor entry via `ShardClient.dispose` (not a `LeasePool` release — there is no shared module lease
 * to refcount anymore, one actor belongs to exactly one instance).
 *
 * 🪪️ `pluginId` is the PROGRAM id every host structure keys this module by; `manifestPluginId` is the plugin id its own
 * descriptor must name. They differ only for a hub document's catalog-resolved program (`🌎️hub-source/🔍️resolution`),
 * which runs beside the device's own program of the same plugin. */
export async function loadPluginModule(pluginId: string, moduleUrl: string, signal?: AbortSignal, manifestPluginId: string = pluginId): Promise<PluginWasmHandle> {
  const { manifest, runtime: registry } = await resolvePluginDescriptorAndRegistry(pluginId, moduleUrl, signal, manifestPluginId);
  const scope = createPluginHandleScope(pluginId, manifest, registry);
  const instances = createInstanceTable(scope);
  const ui = createRetainedUiIntake(scope, instances);
  const requestActors = createRequestActor(scope);
  const jobs = createSpawnedJobDriver(scope, instances, ui, { drainTypedOperations: (instanceId) => turns.drainTypedOperations(instanceId) });
  const turns = createTurnPump(scope, instances, ui, jobs);
  const handle = createKernelHandle(scope, instances, ui, requestActors, turns);

  const richHandle = await adaptPluginHandle(pluginId, { handle, release: handle.dispose });

  const refreshUi = createRefreshUi(scope, instances, ui, jobs);
  const captureExtensionCompletion = createExtensionCompletionCapture(scope, instances, ui, jobs);
  const invoke = createInboundInvoke(scope, instances, requestActors);
  const bindDocumentPort = createDocumentPortBinder(scope, instances, ui, jobs);

  return { ...richHandle, refreshUi, captureExtensionCompletion, invoke, bindDocumentPort, subscribeSpawnedJobProgress: jobs.subscribeSpawnedJobProgress,acquireOwnedUiSurface:ui.acquireOwnedUiSurface };
}
//#endregion 🔖️plugin-runtime

/** 📞️ The extension completion capture, the request actor and the extension invoke and dispatch paths.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | loadPluginModule}: each function builds one group of the handle's closure state and helpers and returns them for the later groups. */
import { type Effect, GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, GUEST_HOST_ANSWER_CEILING_BYTES, guestAnswerPages, type InvocationResponse, SemioFaultError } from "@semio-tech/framework";
import { decodeFaultFromWire, decodePackValue, encodePackValue } from "@semio-tech/framework-os";
import { WGPU_TYPED_OPERATION_SETTLE_LIMIT, type WgpuPluginExtensionCompletion, type WgpuPluginHandle, type WgpuPluginInvokeContext, WgpuTypedOperationDrive } from "../../🧾️typed-operations-and-handle/🟦️.ts";
import { admitSpawnedJob, loadedWgpuHandles, pendingTurnEffects, shellFrameAnswersACaller, wgpuHostEffect } from "../../🎛️contributions-and-effects/🟦️.ts";
import { actorTurnChains, extensionRequestActivationReason, serializeWgpuActorCall, submitTurn } from "../../🧵️shard-and-actor-calls/🟦️.ts";
import { yieldWgpuUi } from "../../🍽️ui-intake-and-route/🟦️.ts";
import { driveInboundRequest, INBOUND_REQUEST_TURN_BUDGET } from "../../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/📥️inbound-request/🟦️.ts";
import { shellFrameBytes, wireTurnStatusTag, type WireVariant } from "../../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts";
import { type ShardEventEnvelope } from "@semio-tech/framework-actor/shard-client/wire";
import type { acquirePluginDescriptor, createInstanceBookkeeping } from "../🪪️descriptor-and-instance-state/🟦️.ts";
import type { createTurnDrivers } from "../🖼️render-and-turn-drivers/🟦️.ts";

/** 🧩️ Builds the empty invocation and the extension completion capture. */
export function createExtensionCompletion({ requireActorId, requireUiRoute, executeFor, drainSpawnedJobs, turnOutcomes, drainTypedOperations }: Pick<ReturnType<typeof createInstanceBookkeeping>, "requireActorId" | "requireUiRoute" | "executeFor"> & Pick<ReturnType<typeof createTurnDrivers>, "drainSpawnedJobs" | "turnOutcomes" | "drainTypedOperations">) {
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
  return { captureExtensionCompletion, emptyInvocation };
}

/** 📮️ Builds the lazily activated request actor and the extension invoke and dispatch functions. */
export function createRequestActor({ state, pluginId, registry, captureExtensionCompletion }: Pick<ReturnType<typeof createInstanceBookkeeping>, "state"> & { readonly pluginId: string } & Pick<Awaited<ReturnType<typeof acquirePluginDescriptor>>, "registry"> & Pick<ReturnType<typeof createExtensionCompletion>, "captureExtensionCompletion">) {
  /** 📥️ See {@link WgpuPluginHandle.invoke}. One request actor per handle, activated lazily and
   * retired with it — an extension program declares no app, so it never gets a `createApp` instance
   * and `Event::Request` needs none. The actor activates `on-extension-request:<capability>` with the
   * capability whose request pulled it up — the declared extension point, not the host's `manual`. */
  const ensureRequestActor = (capability: string): Promise<string> => {
    if (state.disposing) return Promise.reject(new Error("wgpu-plugin-handle.closed"));
    state.requestActor ??= (async () => {
      const actorId = `${pluginId}#request`;
      await registry.activate(pluginId, actorId, extensionRequestActivationReason(capability));
      return actorId;
    })().catch((error: unknown) => {
      state.requestActor = null;
      throw error;
    });
    return state.requestActor;
  };
  const retireRequestActor = async (): Promise<void> => {
    const pending = state.requestActor;
    state.requestActor = null;
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
    state.inboundRequestSeq += 1n;
    const req = state.inboundRequestSeq;
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
  return { invoke, dispatchInvokeExtension, retireRequestActor };
}

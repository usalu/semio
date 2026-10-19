/** 🔌️ The plugin handle: app creation and destruction, actions, rendering, codec access and disposal.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | loadPluginModule}: each function builds one group of the handle's closure state and helpers and returns them for the later groups. */
import { activationReasonForAppId } from "@semio-tech/framework";
import { AppChannelClient, type AppFrameValue, decodeDocumentArchiveBytes, decodePackValue, decodePackWire, encodeDocumentArchiveBytes, encodePackValue, faultDisplayMessage } from "@semio-tech/framework-os";
import { decodeCausalEnvelopeBatch } from "@semio-tech/framework-replication";
import { operationCompletionPublication, performInvocation, stashOperationPublication, takeOperationPublications, wgpuEphemeralSnapshot, type WgpuPluginHandle } from "../../🧾️typed-operations-and-handle/🟦️.ts";
import { declareWgpuBootSubphase, jsonEffects } from "../../🎛️contributions-and-effects/🟦️.ts";
import { submitActorWork } from "../../🧵️shard-and-actor-calls/🟦️.ts";
import { retireWgpuOwnedUiInstanceLifecycle, settleInstanceLifecycle, WgpuOwnedUiInstanceRoute } from "../../🍽️ui-intake-and-route/🟦️.ts";
import { coerceTurnResult } from "../../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts";
import { DEFAULT_SHARD_BUDGET } from "@semio-tech/framework-actor/shard-runtime";
import { settleFailedInstanceOpen } from "@semio-tech/framework-actor/shard-client/heartbeat";
import { mediaTransportPort } from "@semio-tech/framework-os/modules/renderer/engine/media/channel";
import type { acquirePluginDescriptor, createInstanceBookkeeping } from "../🪪️descriptor-and-instance-state/🟦️.ts";
import type { createRenderSurface, createTurnDrivers } from "../🖼️render-and-turn-drivers/🟦️.ts";
import type { createExtensionCompletion, createRequestActor } from "../📞️extension-and-request-actors/🟦️.ts";
import type { createContributionsAndBackbone } from "../🧲️contributions-and-backbone/🟦️.ts";
import { globalInstanceIds } from "./🧰️module-instance-ids/🟦️.ts";

/** 🧷️ Assembles the plugin handle over the closure state of the earlier groups. */
export function buildPluginHandle({ pluginId, manifest, packageId, componentSha256, state, actorIdByInstance, closingInstances, registry, shardClient, lifecycleByInstance, executeFor, uiRouteByInstance, channelHandle, channelRequests, channelByInstance, openingInstances, retiringInstances, releaseInstance, requireChannel, renderSurface, requireActorId, captureExtensionCompletion, invoke, dispatchInvokeExtension, pushScopedContributions, documentBackbone, receiveDocumentBackbone, codecActorId, retireRequestActor, turnOutcomes }: { readonly pluginId: string } & Pick<Awaited<ReturnType<typeof acquirePluginDescriptor>>, "manifest" | "packageId" | "componentSha256" | "registry" | "shardClient"> & Pick<ReturnType<typeof createInstanceBookkeeping>, "state" | "actorIdByInstance" | "closingInstances" | "lifecycleByInstance" | "executeFor" | "uiRouteByInstance" | "channelRequests" | "channelByInstance" | "openingInstances" | "retiringInstances" | "releaseInstance" | "requireChannel" | "requireActorId" | "codecActorId"> & Pick<ReturnType<typeof createTurnDrivers>, "channelHandle" | "turnOutcomes"> & Pick<ReturnType<typeof createRenderSurface>, "renderSurface"> & Pick<ReturnType<typeof createExtensionCompletion>, "captureExtensionCompletion"> & Pick<ReturnType<typeof createRequestActor>, "invoke" | "dispatchInvokeExtension" | "retireRequestActor"> & Pick<ReturnType<typeof createContributionsAndBackbone>, "pushScopedContributions" | "documentBackbone" | "receiveDocumentBackbone">) {
  const handle: WgpuPluginHandle = {
    pluginId,
    manifest,
    packageId,
    componentSha256,
    createApp: (appId) => {
      if (state.disposing) return Promise.reject(new Error("wgpu-plugin-handle.closed"));
      const instanceId = globalInstanceIds.next;
      globalInstanceIds.next += 1;
      const actorId = `${pluginId}#${instanceId}`;
      actorIdByInstance.set(instanceId, actorId);
      const requireOpening = (): void => {
        if (state.disposing || closingInstances.has(instanceId) || actorIdByInstance.get(instanceId) !== actorId) throw new Error("wgpu-plugin-handle.closed");
      };
      const opening = Promise.resolve().then(async () => {
        requireOpening();
        const phase = declareWgpuBootSubphase(`shell-boot:create-app:${pluginId}`);
        try {
        await registry.activate(pluginId, actorId, activationReasonForAppId(appId));
        requireOpening();
        state.eventSeq += 1;
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
      if (state.disposal) return state.disposal;
      state.disposing = true;
      const retirements = [...actorIdByInstance.keys()].map((instanceId) => handle.destroyApp(instanceId));
      state.disposal = Promise.allSettled([...retirements, retireRequestActor()]).then((results) => {
        const failures = results.filter((result): result is PromiseRejectedResult => result.status === "rejected");
        if (failures.length) throw new AggregateError(failures.map((result) => result.reason), "wgpu-plugin-handle.retirement-failed");
        turnOutcomes.complete();
      });
      return state.disposal;
    },
  };
  return { handle };
}

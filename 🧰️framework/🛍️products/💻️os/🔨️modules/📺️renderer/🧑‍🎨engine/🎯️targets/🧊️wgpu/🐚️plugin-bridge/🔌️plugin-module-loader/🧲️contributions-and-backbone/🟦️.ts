/** 🧲️ The scoped contributions push and the document backbone control and payload turns.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | loadPluginModule}: each function builds one group of the handle's closure state and helpers and returns them for the later groups. */
import { appCommandIdsV1, type Effect, hostEffectInvocationV1, type InvocationResponse } from "@semio-tech/framework";
import { decodeAppFrame, decodePackValue, faultDisplayMessage } from "@semio-tech/framework-os";
import { performInvocation, WGPU_TYPED_OPERATION_SETTLE_LIMIT, WgpuTypedOperationDrive } from "../../🧾️typed-operations-and-handle/🟦️.ts";
import { admitSpawnedJob, wgpuBuildScopedContributionsPack, wgpuContributionsIngressSize, wgpuEffectDispatchScope, wgpuHostEffect, wgpuSetContributionsCommand, wgpuSlimContributionsView } from "../../🎛️contributions-and-effects/🟦️.ts";
import { serializeWgpuActorCall, submitTurn } from "../../🧵️shard-and-actor-calls/🟦️.ts";
import { yieldWgpuUi } from "../../🍽️ui-intake-and-route/🟦️.ts";
import { shellFrameBytes } from "../../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts";
import { SHARD_COMMAND_MAXIMUM_PAGES, type ShardEventEnvelope } from "@semio-tech/framework-actor/shard-client/wire";
import { DOCUMENT_BACKBONE_BINDING_SCHEMA_V1, encodeDocumentBackboneControlV1, requireDocumentBackboneReceiptV1, splitDocumentBackboneControlTurnV1 } from "@semio-tech/framework/backbone/binding";
import type { acquirePluginDescriptor, createInstanceBookkeeping } from "../🪪️descriptor-and-instance-state/🟦️.ts";
import type { createTurnDrivers } from "../🖼️render-and-turn-drivers/🟦️.ts";
import type { createExtensionCompletion } from "../📞️extension-and-request-actors/🟦️.ts";

/** 🛰️ Builds the scoped contributions push and the document backbone turns. */
export function createContributionsAndBackbone({ pluginId, consumedTopics, emptyInvocation, channelRequests, requireChannel, manifest, requireActorId, requireUiRoute, executeFor, drainSpawnedJobs, turnOutcomes }: { readonly pluginId: string; readonly consumedTopics: readonly string[] } & Pick<ReturnType<typeof createExtensionCompletion>, "emptyInvocation"> & Pick<ReturnType<typeof createInstanceBookkeeping>, "channelRequests" | "requireChannel" | "requireActorId" | "requireUiRoute" | "executeFor"> & Pick<Awaited<ReturnType<typeof acquirePluginDescriptor>>, "manifest"> & Pick<ReturnType<typeof createTurnDrivers>, "drainSpawnedJobs" | "turnOutcomes">) {
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
  return { pushScopedContributions, documentBackbone, receiveDocumentBackbone };
}

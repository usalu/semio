import { type WgpuPluginHandle } from "../🧾️typed-operations-and-handle/🟦️.ts";
import { loadedWgpuHandles } from "../🎛️contributions-and-effects/🟦️.ts";
import { acquirePluginDescriptor, createInstanceBookkeeping } from "./🪪️descriptor-and-instance-state/🟦️.ts";
import { createRenderSurface, createTurnDrivers } from "./🖼️render-and-turn-drivers/🟦️.ts";
import { createExtensionCompletion, createRequestActor } from "./📞️extension-and-request-actors/🟦️.ts";
import { createContributionsAndBackbone } from "./🧲️contributions-and-backbone/🟦️.ts";
import { buildPluginHandle } from "./🔌️plugin-handle/🟦️.ts";

/** 🐚️ Acquires a real actor through `ActivationRegistry`/`ShardClient` (replacing the deleted
 * `acquirePluginModule`/per-plugin `Worker` — design-runtime.md §3, copying `PluginRuntime`'s own
 * `loadPluginModule` shape). `dispose()` disposes every instance's worker-side actor entry via
 * `ShardClient.dispose` — no shared module lease to refcount any more, one actor belongs to exactly
 * one instance. */
export async function loadPluginModule(pluginId: string, moduleUrl: string, signal: AbortSignal | undefined, admittedDescriptor: import("@semio-tech/framework").PluginPackageDescriptor | undefined, consumedTopics: readonly string[]): Promise<WgpuPluginHandle> {
  const { registry, manifest, packageId, componentSha256, shardClient } = await acquirePluginDescriptor({ signal, admittedDescriptor, pluginId, moduleUrl });
  const { requireActorId, requireUiRoute, executeFor, uiRouteByInstance, state, closingInstances, actorIdByInstance, channelRequests, requireChannel, lifecycleByInstance, channelByInstance, openingInstances, retiringInstances, releaseInstance, codecActorId } = createInstanceBookkeeping({ pluginId });
  const { renderSurface } = createRenderSurface({ requireActorId, requireUiRoute, executeFor, uiRouteByInstance });
  const { drainSpawnedJobs, turnOutcomes, drainTypedOperations, channelHandle } = createTurnDrivers({ requireUiRoute, executeFor, state, closingInstances, actorIdByInstance, requireActorId, pluginId });
  const { captureExtensionCompletion, emptyInvocation } = createExtensionCompletion({ requireActorId, requireUiRoute, executeFor, drainSpawnedJobs, turnOutcomes, drainTypedOperations });
  const { invoke, dispatchInvokeExtension, retireRequestActor } = createRequestActor({ state, pluginId, registry, captureExtensionCompletion });
  const { pushScopedContributions, documentBackbone, receiveDocumentBackbone } = createContributionsAndBackbone({ pluginId, consumedTopics, emptyInvocation, channelRequests, requireChannel, manifest, requireActorId, requireUiRoute, executeFor, drainSpawnedJobs, turnOutcomes });
  const { handle } = buildPluginHandle({ pluginId, manifest, packageId, componentSha256, state, actorIdByInstance, closingInstances, registry, shardClient, lifecycleByInstance, executeFor, uiRouteByInstance, channelHandle, channelRequests, channelByInstance, openingInstances, retiringInstances, releaseInstance, requireChannel, renderSurface, requireActorId, captureExtensionCompletion, invoke, dispatchInvokeExtension, pushScopedContributions, documentBackbone, receiveDocumentBackbone, codecActorId, retireRequestActor, turnOutcomes });
  loadedWgpuHandles.set(pluginId, handle);
  return handle;
}

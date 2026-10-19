/** 🪪️ The admitted package descriptor, the activation registry and the per-instance bookkeeping maps and guards.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | loadPluginModule}: each function builds one group of the handle's closure state and helpers and returns them for the later groups. */
import { fetchPackageDescriptor } from "@semio-tech/framework";
import { AppChannelClient, AppChannelRequestSequence } from "@semio-tech/framework-os";
import { pendingOperationPublications } from "../../🧾️typed-operations-and-handle/🟦️.ts";
import { componentPackageReceiptSource } from "@semio-tech/framework-plugin/receipt";
import { contributionManifests, pendingTurnEffects } from "../../🎛️contributions-and-effects/🟦️.ts";
import { actorTurnChains, getActivationRegistry, getShardClient, submitActorWork } from "../../🧵️shard-and-actor-calls/🟦️.ts";
import { ShardInstanceLifecycleLease } from "@semio-tech/framework-actor/shard-client/activation";
import { type WgpuActorExecutor, WgpuOwnedUiInstanceRoute } from "../../🍽️ui-intake-and-route/🟦️.ts";

/** 🆔️ Admits the package descriptor and binds the activation registry and the shard client. */
export async function acquirePluginDescriptor({ signal, admittedDescriptor, pluginId, moduleUrl }: { readonly signal: AbortSignal | undefined; readonly admittedDescriptor: import("@semio-tech/framework").PluginPackageDescriptor | undefined; readonly pluginId: string; readonly moduleUrl: string }) {
  signal?.throwIfAborted();
  const { manifest, packageId, componentSha256 } = admittedDescriptor ?? await fetchPackageDescriptor(pluginId, moduleUrl, componentPackageReceiptSource, signal);
  signal?.throwIfAborted();
  if (manifest.pluginId !== pluginId) throw new Error(`plugin.descriptor-identity-mismatch: ${pluginId}`);
  contributionManifests.set(pluginId, manifest);
  const registry = getActivationRegistry();
  registry.registerManifest({ pluginId, moduleUrl, caps: [] });
  const shardClient = getShardClient();
  return { registry, manifest, packageId, componentSha256, shardClient };
}

/** 🗃️ Creates the per-instance maps, the lifecycle flags and the actor, channel and ui route guards. */
export function createInstanceBookkeeping({ pluginId }: { readonly pluginId: string }) {
  const actorIdByInstance = new Map<number, string>();
  const channelByInstance = new Map<number, AppChannelClient>();
  const lifecycleByInstance = new Map<number, ShardInstanceLifecycleLease>();
  const uiRouteByInstance = new Map<number, WgpuOwnedUiInstanceRoute>();
  const openingInstances = new Map<number, Promise<number>>();
  const retiringInstances = new Map<number, Promise<void>>();
  const closingInstances = new Set<number>();
  const channelRequests = new AppChannelRequestSequence();
  const state: { disposing: boolean; disposal: Promise<void> | null; eventSeq: number; requestActor: Promise<string> | null; inboundRequestSeq: bigint } = { disposing: false, disposal: null, eventSeq: 0, requestActor: null, inboundRequestSeq: 0n };
  const requireActorId = (instanceId: number): string => {
    const actorId = actorIdByInstance.get(instanceId);
    if (state.disposing) throw new Error("wgpu-plugin-handle.closed");
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
    if (state.disposing) throw new Error("wgpu-plugin-handle.closed");
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
  return { requireActorId, requireUiRoute, executeFor, uiRouteByInstance, state, closingInstances, actorIdByInstance, channelRequests, requireChannel, lifecycleByInstance, channelByInstance, openingInstances, retiringInstances, releaseInstance, codecActorId };
}

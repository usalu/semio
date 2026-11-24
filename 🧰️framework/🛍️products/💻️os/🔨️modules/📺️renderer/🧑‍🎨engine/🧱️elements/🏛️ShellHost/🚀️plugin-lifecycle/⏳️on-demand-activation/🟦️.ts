/** ⏳️ The React host's on-demand activation: the kernel activator over this host's own install path (`installPlugin`), so a node kind, tag or event that names a deferred plugin starts it (the deferred rows come from the full catalog, not from the boot plan, which leaves them out by design) with the install band's progress and cancel, and a missing optional plugin only tells the person so. The WGPU host runs the same activator over its frame Worker (`semioWgpuActivateOnDemand`). */
import { useCallback, useEffect, useMemo } from "../../../⚛️react-port/🟦️.ts";
import { admitOnDemandRequest, catalogPluginRegistryEntries, createOnDemandActivator, deferredPluginEntries, onDemandStatusLabel, type OnDemandActivator, type OnDemandOutcome, type OnDemandRequest } from "@semio-tech/framework";
import type { ShellCoreAfterDirectoryAndRoute } from "../../🛂️directory-route/🟦️.ts";
import type { PluginInstallOutcome } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";

type Installer = (pluginId: string, rebuiltAt?: number) => Promise<PluginInstallOutcome>;

/** 🧩️ Builds the host's activator; `activate` rejects for a plugin that did not load so the activator reports it `unavailable` instead of throwing at its caller; a plugin that did load owes the live session one full refresh, whose contributions pass pushes the new operators to the guest (the push is keyed by the loaded plugin ids, and a late install triggers no refresh of its own). */
export function useOnDemandActivation({ core, installPlugin }: { readonly core: ShellCoreAfterDirectoryAndRoute; readonly installPlugin: Installer }): { readonly activator: OnDemandActivator; readonly activateOnDemand: (request: OnDemandRequest, signal?: AbortSignal) => Promise<OnDemandOutcome>; readonly cancelOnDemand: (pluginId?: string) => number } {
  const { catalog, registry, loadedPluginsRef, pluginInstallAbortsRef, refreshUiRef, sessionRef, showTransientNoticeRef, uiLocaleRef } = core;
  const catalogRows = useMemo(() => catalogPluginRegistryEntries(catalog), [catalog]);
  const deferred = useMemo(() => deferredPluginEntries(catalog, registry), [catalog, registry]);
  const activator = useMemo(() => createOnDemandActivator({
    candidates: () => deferred,
    installed: () => catalogRows,
    active: () => new Set(loadedPluginsRef.current.map((entry) => entry.handle.pluginId)),
    activate: async (entry, context) => {
      context.signal.throwIfAborted();
      context.signal.addEventListener("abort", () => pluginInstallAbortsRef.current.get(entry.pluginId)?.abort(), { once: true });
      const outcome = await installPlugin(entry.pluginId);
      if (outcome === "failed" || outcome === "missing-registry") throw new Error(`on-demand.${outcome}: ${entry.pluginId}`);
      const live = sessionRef.current;
      if (live !== null) void refreshUiRef.current?.(live, { kind: "full" }).catch(() => undefined);
    },
    onStatus: (status) => {
      if (status.phase === "resolving") return;
      showTransientNoticeRef.current(onDemandStatusLabel(status, uiLocaleRef.current), status.phase === "unavailable" ? "error" : status.phase === "cancelled" ? "warning" : "info", `on-demand.${status.pluginId}`);
    },
  }), [catalogRows, deferred, installPlugin]);
  const activateOnDemand = useCallback((request: OnDemandRequest, signal?: AbortSignal) => activator.request(request, signal), [activator]);
  const cancelOnDemand = useCallback((pluginId?: string) => activator.cancel(pluginId), [activator]);
  useEffect(() => {
    const host = globalThis as { semioReactActivateOnDemand?: (requestJson: string) => Promise<string>; semioReactCancelOnDemand?: (pluginId?: string) => number };
    host.semioReactActivateOnDemand = async (requestJson) => JSON.stringify(await activator.request(admitOnDemandRequest(JSON.parse(requestJson) as unknown)));
    host.semioReactCancelOnDemand = (pluginId) => activator.cancel(pluginId);
    return () => { delete host.semioReactActivateOnDemand; delete host.semioReactCancelOnDemand; };
  }, [activator]);
  return { activator, activateOnDemand, cancelOnDemand };
}

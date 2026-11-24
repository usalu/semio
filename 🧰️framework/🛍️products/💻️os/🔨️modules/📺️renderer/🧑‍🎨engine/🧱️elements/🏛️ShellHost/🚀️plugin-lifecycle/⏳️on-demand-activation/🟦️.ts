/** ⏳️ The React host's on-demand activation: the kernel activator over this host's own install path (`installPlugin`), so a node kind, tag or event that names a deferred plugin starts it with the install band's progress and cancel, and a missing optional plugin only tells the person so. The WGPU host runs the same activator over its frame Worker (`semioWgpuActivateOnDemand`). */
import { useCallback, useEffect, useMemo } from "../../../⚛️react-port/🟦️.ts";
import { admitOnDemandRequest, createOnDemandActivator, onDemandPluginEntries, onDemandStatusLabel, type OnDemandActivator, type OnDemandOutcome, type OnDemandRequest } from "@semio-tech/framework";
import type { ShellCoreAfterDirectoryAndRoute } from "../../🛂️directory-route/🟦️.ts";
import type { PluginInstallOutcome } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";

type Installer = (pluginId: string, rebuiltAt?: number) => Promise<PluginInstallOutcome>;

/** 🧩️ Builds the host's activator; `activate` rejects for a plugin that did not load so the activator reports it `unavailable` instead of throwing at its caller. */
export function useOnDemandActivation({ core, installPlugin }: { readonly core: ShellCoreAfterDirectoryAndRoute; readonly installPlugin: Installer }): { readonly activator: OnDemandActivator; readonly activateOnDemand: (request: OnDemandRequest, signal?: AbortSignal) => Promise<OnDemandOutcome>; readonly cancelOnDemand: (pluginId?: string) => number } {
  const { plugins, registry, loadedPluginsRef, pluginInstallAbortsRef, showTransientNoticeRef, uiLocaleRef } = core;
  const activator = useMemo(() => createOnDemandActivator({
    candidates: () => onDemandPluginEntries(plugins, registry).filter((entry) => (entry.activationEvents ?? []).length > 0),
    installed: () => plugins,
    active: () => new Set(loadedPluginsRef.current.map((entry) => entry.handle.pluginId)),
    activate: async (entry, context) => {
      context.signal.addEventListener("abort", () => pluginInstallAbortsRef.current.get(entry.pluginId)?.abort(), { once: true });
      const outcome = await installPlugin(entry.pluginId);
      if (outcome === "failed" || outcome === "missing-registry") throw new Error(`on-demand.${outcome}: ${entry.pluginId}`);
    },
    onStatus: (status) => {
      if (status.phase === "resolving" || status.phase === "active") return;
      showTransientNoticeRef.current(onDemandStatusLabel(status, uiLocaleRef.current), status.phase === "unavailable" ? "error" : "info", `on-demand.${status.pluginId}`);
    },
  }), [plugins, registry, installPlugin]);
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

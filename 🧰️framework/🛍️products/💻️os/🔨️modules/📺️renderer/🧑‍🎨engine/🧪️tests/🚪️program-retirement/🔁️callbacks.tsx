/** 🔁️ Actual switch, reload and uninstall callbacks use captured retirement admission. */
import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { usePluginReloadCallbacks } from "../../🧱️elements/🏛️ShellHost/🚀️plugin-lifecycle/🔄️reload-and-extension-callbacks/🟦️.ts";
import { useSwitchPluginAppState } from "../../🧱️elements/🏛️ShellHost/🛫️spawned-session/🟠️switch-plugin-app/🟦️.ts";
import { createSessionAppSwitchGateV1, createSealedInstanceLedgerV1 } from "../../🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts";
import { panelJsonFromState } from "../../🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts";
const moduleState = vi.hoisted(() => ({ candidate: null as unknown }));
vi.mock("../../🧱️elements/🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx", async original => ({ ...await original<object>(), loadPluginModuleResilient: async () => moduleState.candidate }));
afterEach(cleanup);
function setup() {
  const events: string[] = [];
  const apps = ["before", "after"].map(id => ({ id, modes: [], windowKinds: [], breadcrumb: [] }));
  const handle = { pluginId: "stdio", createApp: vi.fn(async () => { events.push("create"); return 8; }), destroyApp: vi.fn(async () => { events.push("destroy"); }), dispose: vi.fn(async () => { events.push("dispose-old"); }) };
  const candidate = { ...handle, dispose: vi.fn(async () => { events.push("dispose-candidate"); }), manifest: { apps, version: "1" } };
  moduleState.candidate = candidate;
  const host = { pluginId: "stdio", instanceId: 7, app: apps[0]!, viewState: {} };
  const sessionRef = { current: host as typeof host | null }, loadedPluginsRef = { current: [{ handle, manifest: { apps, version: "1" } }] };
  const retire = vi.fn(async () => { events.push("retire"); return true; }), release = vi.fn();
  const admission = { current: () => sessionRef.current === host, retire, release };
  const admit = vi.fn(async (_owners: unknown, _current: () => boolean) => admission);
  const dispatch = vi.fn((event: { type: string; value?: unknown }) => { events.push(event.type); if (event.type === "SET_SESSION") sessionRef.current = (typeof event.value === "function" ? event.value(sessionRef.current) : event.value) as typeof host | null; });
  const options = {
    core: { sealedInstancesRef: { current: createSealedInstanceLedgerV1() }, admitProgramRetirementRef: { current: admit } }, pluginOpInFlightRef: { current: new Set<string>() }, loadedPluginsRef, sessionRef, dispatch,
    pluginSource: { acquireModule: vi.fn(async () => ({ moduleUrl: "module", rebuiltAt: 1 })) }, installPlugin: vi.fn(), directoryHomeOwnerRef: { current: null }, backboneWorkerRef: { current: null }, retireDirectoryHomeOwner: vi.fn(), contributorInstancesRef: { current: new Map() }, hostMode: false,
    pluginModuleUrlByIdRef: { current: new Map() }, pluginArtifactRebuiltAtRef: { current: new Map() }, recordPluginArtifactRebuiltAt: vi.fn(), establishPrimarySession: vi.fn(async () => {}), primaryPluginId: "host", catalog: { extensions: [] }, uiLocaleRef: { current: "en" }, uiTerminologyRef: { current: "default" }, extraWindowInstancesRef: { current: [] },
  };
  return { events, apps, handle, candidate, host, sessionRef, loadedPluginsRef, retire, release, admit, dispatch, options };
}
it("reload Keep disposes its candidate and preserves all old instances and the module", async () => {
  const view = setup(); view.admit.mockResolvedValue(null as never);
  const hook = renderHook(() => usePluginReloadCallbacks(view.options as unknown as Parameters<typeof usePluginReloadCallbacks>[0]));
  await act(async () => hook.result.current.reloadPlugin("stdio"));
  expect(view.admit).toHaveBeenCalledTimes(1); expect(view.handle.destroyApp).not.toHaveBeenCalled(); expect(view.handle.dispose).not.toHaveBeenCalled(); expect(view.candidate.dispose).toHaveBeenCalledTimes(1); expect(view.sessionRef.current).toBe(view.host);
  expect(view.options.pluginOpInFlightRef.current.size).toBe(0);
  expect(view.dispatch.mock.calls.some(([event]) => event.type === "SET_PLUGIN_SUPERVISOR" && event.value === "crashed")).toBe(false);
});
it("uninstall Keep preserves a background editor and its module", async () => {
  const view = setup(); const spawned = { id: "guest", pluginId: "stdio", instanceId: 7, appId: "before", label: "Draft", breadcrumb: [] };
  view.sessionRef.current = { ...view.host, pluginId: "host", instanceId: 1, app: { ...view.apps[0]!, id: "home" }, viewState: { panelJson: panelJsonFromState({ spawnedApps: [spawned], activePanelTab: "main" }) } } as typeof view.host;
  view.admit.mockResolvedValue(null as never);
  const hook = renderHook(() => usePluginReloadCallbacks(view.options as unknown as Parameters<typeof usePluginReloadCallbacks>[0]));
  await act(async () => hook.result.current.uninstallPlugin("stdio"));
  expect(view.admit).toHaveBeenCalledTimes(1); expect(view.handle.dispose).not.toHaveBeenCalled(); expect(view.handle.destroyApp).not.toHaveBeenCalled(); expect(view.events).not.toContain("REMOVE_LOADED_PLUGIN");
});
function switchHook(view: ReturnType<typeof setup>) {
  const options = { ...view.options, admitProgramRetirementRef: view.options.core.admitProgramRetirementRef, hostApp: {}, session: view.host, loadedPlugins: view.loadedPluginsRef.current, setSurfaceSwitchBusy: vi.fn(), sessionSwitchGateRef: { current: createSessionAppSwitchGateV1() }, sessionWorkRef: { current: { pending: () => 0, outstanding: () => "" } }, sealedInstancesRef: { current: createSealedInstanceLedgerV1() }, retireSessionInstance: vi.fn(), appLabelsOverlay: { modeLabels: {}, windowKindLabels: {} }, uiTerminology: "default", uiLocale: "en", extraWindowCounterRef: { current: 0 }, landingAppId: "home", openInstanceIdRef: { current: null }, rememberOpenSpaceId: vi.fn(), refreshUi: vi.fn(async () => {}), showTransientNoticeRef: { current: vi.fn() }, documentServices: [], documentServiceInventoryRef: { current: [] } };
  return renderHook(() => useSwitchPluginAppState(options as unknown as Parameters<typeof useSwitchPluginAppState>[0]));
}
it("ordinary app switch Keep creates and retires nothing", async () => {
  const view = setup(); view.admit.mockResolvedValue(null as never); const hook = switchHook(view);
  let answer: unknown; await act(async () => { answer = await hook.result.current.switchToPluginApp("stdio", "after"); });
  expect(answer).toBeNull(); expect(view.handle.createApp).not.toHaveBeenCalled(); expect(view.retire).not.toHaveBeenCalled(); expect(view.sessionRef.current).toBe(view.host);
});
it("ordinary app switch starts admitted retirement before publishing its successor", async () => {
  const view = setup(), hook = switchHook(view);
  await act(async () => { await hook.result.current.switchToPluginApp("stdio", "after"); });
  expect(view.retire).toHaveBeenCalledTimes(1); expect(view.events.indexOf("retire")).toBeLessThan(view.events.indexOf("SET_SESSION")); expect(view.sessionRef.current?.app.id).toBe("after"); expect(view.release).toHaveBeenCalledTimes(1);
});

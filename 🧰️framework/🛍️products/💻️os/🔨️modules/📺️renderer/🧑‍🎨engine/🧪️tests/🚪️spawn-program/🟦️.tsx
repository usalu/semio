import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { useSpawnProgram } from "../../🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🟢️spawn-program/🟦️.ts";
import { buildSpacePanelState, type SpacePanelState } from "../../🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

afterEach(cleanup);

describe("async program creation ownership", () => {
  for (const row of fixture.cases) it(row.name, async () => {
    let created!: (instance: number) => void;
    const instance = new Promise<number>((resolve) => { created = resolve; });
    const plugin = { handle: { pluginId: "stdio", createApp: vi.fn(() => instance), destroyApp: vi.fn(async () => undefined) } };
    let panel: SpacePanelState = buildSpacePanelState([], "home");
    const session = { pluginId: "host", instanceId: 1, app: { id: "home" }, viewState: { panelJson: JSON.stringify(panel) } };
    const sessionRef = { current: session };
    const loadedPluginsRef = { current: [plugin] };
    const options = {
      pluginOpInFlightRef: { current: new Set<string>() }, loadedPlugins: [plugin], loadedPluginsRef, session, sessionRef, hostApp: {},
      uiLocaleRef: { current: "en" }, showTransientNoticeRef: { current: vi.fn() },
      updateSpacePanel: (next: SpacePanelState) => { panel = next; },
      updateSpacePanelFrom: (compute: (latest: SpacePanelState) => SpacePanelState) => {
        if (row.change === "unavailable") return false;
        panel = compute(panel);
        return true;
      },
    } as unknown as Parameters<typeof useSpawnProgram>[0];
    const view = renderHook(() => useSpawnProgram(options));
    let opening!: Promise<void>;
    act(() => { opening = view.result.current.spawnProgram({ pluginId: "stdio", appId: "txt", label: "Text", breadcrumb: [] }); });
    if (row.change === "panel") panel = buildSpacePanelState([{ id: "sibling", pluginId: "stdio", appId: "json", instanceId: 3, label: "JSON", breadcrumb: [] }], "details");
    if (row.change === "host") sessionRef.current = { ...session, instanceId: 99 };
    if (row.change === "plugin") loadedPluginsRef.current = [];
    await act(async () => { created(2); await opening; });
    expect(panel.spawnedApps.map((entry) => entry.id)).toEqual(row.spawned);
    expect(plugin.handle.destroyApp).toHaveBeenCalledTimes(row.retired);
    if (row.change === "panel") expect(panel.activePanelTab).toBe("details");
  });
});

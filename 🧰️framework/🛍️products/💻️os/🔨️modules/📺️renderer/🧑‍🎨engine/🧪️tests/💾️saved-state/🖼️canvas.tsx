/** 🚪️ Drives the actual host canvas close callback without rendering the guest application. */
import * as React from "react";
import { cleanup, renderHook } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🖼️canvas.json";
import { Mode } from "@semio-tech/ui-react";
import { useCanvas } from "../../🧱️elements/🏛️ShellHost/🖼️canvas-footer/🔸️canvas/🟦️.tsx";
import { panelJsonFromState, parsePanelState } from "../../🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts";

afterEach(cleanup);
function callback(node: React.ReactNode): (id: string) => Promise<boolean> {
  if (!React.isValidElement(node)) throw new Error("Mode not found");
  if (node.type === Mode) return (node.props as { onWindowClose: (id: string) => Promise<boolean> }).onWindowClose;
  for (const child of React.Children.toArray((node.props as { children?: React.ReactNode }).children)) {
    try { return callback(child); } catch {}
  }
  throw new Error("Mode not found");
}
function setup(count = 1) {
  const trace: string[] = [];
  const spawned = { id: "guest", pluginId: "stdio", appId: "txt", instanceId: 7, label: "Text", breadcrumb: ["Text"] };
  const panel = { activePanelTab: "main", spawnedApps: [spawned], activeSpawnedId: "guest" };
  const kinds = Array.from({ length: count }, (_, i) => ({ id: "w" + i, bodyKey: "body", label: { en: "Window", de: "Fenster" } }));
  const layout = { kind: "stack", children: kinds.map(kind => ({ kind: "window", id: "guest::" + kind.id })) };
  const app = { id: "host", modes: [], breadcrumb: ["Host"], label: { en: "Host" }, windowKinds: [], defaultLayout: {} };
  const sessionRef = { current: { app, viewState: { panelJson: panelJsonFromState(panel) } } };
  const destroyApp = vi.fn(async () => { trace.push("destroy"); });
  const requestCloseDocument = vi.fn(async () => "close" as "close" | "stay");
  const release = vi.fn();
  const admit = vi.fn(async (_owners: unknown, current: () => boolean) => ({ current, release, retire: async () => { trace.push("attachment-close"); await destroyApp(); return true; } }));
  const updateSpacePanelFrom = vi.fn(compute => {
    trace.push("panel-remove");
    const next = compute(parsePanelState(sessionRef.current.viewState));
    sessionRef.current.viewState.panelJson = panelJsonFromState(next);
    return true;
  });
  const noteShellCommand = vi.fn(async () => { trace.push("note"); });
  const options = {
    hostMode: true, notFoundPath: null, primaryPluginId: "host", pluginSupervisorById: {}, error: null, sessionFault: null,
    session: sessionRef.current, sessionRef, initialExampleReady: true, uiTerminology: {}, hostAppId: "host", panel, uiLocale: "en",
    appLabelsOverlay: { modeLabels: {}, windowKindLabels: {} }, openSpaceIdRef: { current: null }, loadedPlugins: [{ handle: { pluginId: "stdio", destroyApp }, manifest: { apps: [{ id: "txt", windowKinds: kinds, defaultLayout: {} }] } }],
    effectiveModeLayout: layout, effectiveModeLayoutRef: { current: layout }, modeWindows: [], activeWindowId: "guest::w0",
    localCatalog: { documents: [] }, shellStateRef: { current: { sync: { syncStatusByDocumentId: {} } } }, currentDocumentRuntimeKey: null, openKeptFile: vi.fn(async () => {}),
    admitProgramRetirementRef: { current: admit }, requestCloseDocument, openDocumentSessionsRef: { current: new Map([["doc", { session: { pluginId: "stdio", app: { id: "txt" }, instanceId: 7 } }]]) },
    updateSpacePanelFrom, noteShellCommand, world3dWindowViewRegistry: { retireOwner: vi.fn(), retireWindow: vi.fn() },
    writeUtilityRegister: vi.fn(), dispatch: vi.fn(), extraWindowInstancesRef: { current: [] },
  } as unknown as Parameters<typeof useCanvas>[0];
  const view = renderHook(() => useCanvas(options));
  return { close: callback(view.result.current.canvas), trace, sessionRef, spawned, admit, release, requestCloseDocument, destroyApp, updateSpacePanelFrom, noteShellCommand, options };
}
describe("host canvas close admission", () => {
  it("Keep leaves the panel and plugin untouched", async () => {
    const view = setup();
    view.admit.mockResolvedValue(null as never);
    expect(await view.close("guest::w0")).toBe(false);
    expect(view.destroyApp).not.toHaveBeenCalled();
    expect(view.updateSpacePanelFrom).not.toHaveBeenCalled();
    expect(view.noteShellCommand).not.toHaveBeenCalled();
  });
  it("closes the attachment before destroying the app and removing the panel", async () => {
    const view = setup();
    expect(await view.close("guest::w0")).toBe(true);
    const oracle = new Ajv().compile(fixture.acceptedTraceSchema);
    expect(oracle(view.trace)).toBe(true);
    expect(view.trace).toEqual(fixture.acceptedTrace);
    expect(view.trace.indexOf("attachment-close")).toBeLessThan(view.trace.indexOf("destroy"));
    expect(view.trace.indexOf("destroy")).toBeLessThan(view.trace.indexOf("panel-remove"));
    expect(parsePanelState(view.sessionRef.current.viewState)?.spawnedApps).toEqual([]);
  });
  it("concurrent sibling closes guard the final window and Keep preserves the owner", async () => {
    const view = setup(2);
    view.admit.mockResolvedValue(null as never);
    const first = view.close("guest::w0"), second = view.close("guest::w1");
    expect(await first).toBe(true);
    expect(await second).toBe(false);
    expect(view.admit).toHaveBeenCalledTimes(1);
    expect(view.destroyApp).not.toHaveBeenCalled();
    expect(view.updateSpacePanelFrom).not.toHaveBeenCalled();
  });
  it("a replacement instance survives an older pending close", async () => {
    const view = setup();
    let settle!: (value: Awaited<ReturnType<typeof view.admit>>) => void;
    view.admit.mockImplementation(() => new Promise(resolve => { settle = resolve; }));
    const pending = view.close("guest::w0");
    await vi.waitFor(() => expect(view.admit).toHaveBeenCalledTimes(1));
    view.sessionRef.current.viewState.panelJson = panelJsonFromState({ activePanelTab: "main", spawnedApps: [{ ...view.spawned, instanceId: 8 }], activeSpawnedId: "guest" });
    settle({ current: () => false, release: view.release, retire: async () => { throw Error("stale retirement"); } });
    expect(await pending).toBe(false);
    expect(view.destroyApp).not.toHaveBeenCalled();
    expect(view.updateSpacePanelFrom).not.toHaveBeenCalled();
  });
});

it("admits an unbound draft before closing its final window", async () => {
  const view = setup(); view.options.openDocumentSessionsRef.current.clear();
  view.admit.mockResolvedValue(null as never);
  expect(await view.close("guest::w0")).toBe(false);
  expect(view.admit).toHaveBeenCalledTimes(1); expect(view.destroyApp).not.toHaveBeenCalled();
});

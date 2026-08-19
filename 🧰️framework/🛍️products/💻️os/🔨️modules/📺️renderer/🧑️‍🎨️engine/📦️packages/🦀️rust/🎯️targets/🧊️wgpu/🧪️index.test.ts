import { describe, expect, it } from "vitest";
import { pluginHandleForBridge, type WgpuPluginHandle } from "./🟦️typescript/🐚️plugin-bridge.ts";

function fakeHandle(overrides: Partial<WgpuPluginHandle> = {}): WgpuPluginHandle {
  return {
    pluginId: "draw",
    manifest: { pluginId: "draw", label: "Draw", version: "0.1.0", apps: [], workflows: [], examples: [] },
    createApp: async () => 1,
    destroyApp: async () => {},
    handleAction: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }),
    handleCommand: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }),
    render: async () => ({ type: "text", value: "hello" }),
    contextMenu: async () => [],
    dispose: () => {},
    ...overrides,
  };
}

describe("framework renderer wgpu plugin bridge", () => {
  it("builds a JS bridge whose manifest() is synchronous JSON, matching ProgramBridge.rs's Reflect::get(handle, \"manifest\") contract", () => {
    const bridge = pluginHandleForBridge(fakeHandle());
    expect(JSON.parse(bridge.manifest()).pluginId).toBe("draw");
  });

  it("forwards createApp/destroyApp by identity", async () => {
    const created: string[] = [];
    const bridge = pluginHandleForBridge(fakeHandle({ createApp: async (appId) => (created.push(appId), 7), destroyApp: async () => void created.push("destroyed") }));
    expect(await bridge.createApp("s")).toBe(7);
    await bridge.destroyApp(7);
    expect(created).toEqual(["s", "destroyed"]);
  });

  it("unwraps handleAction's contextJson third argument down to just its viewState before calling the typed handle — ProgramBridge.rs passes {viewState, actor} JSON, not the bare view state", async () => {
    let seenViewState: unknown;
    const bridge = pluginHandleForBridge(
      fakeHandle({
        handleAction: async (_instanceId, _actionJson, viewState) => {
          seenViewState = viewState;
          return { output: "ok", mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } };
        },
      }),
    );
    const result = await bridge.handleAction(1, "{}", JSON.stringify({ viewState: { zoom: 2 }, actor: "local" }));
    expect(seenViewState).toEqual({ zoom: 2 });
    expect(JSON.parse(result).output).toBe("ok");
  });

  it("bridges render() through JSON round-tripping", async () => {
    const bridge = pluginHandleForBridge(fakeHandle());
    const result = await bridge.render(1, "window", JSON.stringify({}));
    expect(JSON.parse(result)).toEqual({ type: "text", value: "hello" });
  });
});

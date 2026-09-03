import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { assertPinnedBunVersion, decodeAstralEscapes, renderBrowserEntry, renderFrameWorker } from "../📦️packages/🦀️rust/📜️script";
import { pluginHandleForBridge, type WgpuPluginHandle } from "../📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts";

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

describe("framework renderer wgpu generated worker", () => {
  it("fails closed unless the renderer uses the repository-pinned Bun runtime", () => {
    expect(assertPinnedBunVersion()).toBe(Bun.version);
    expect(() => assertPinnedBunVersion("0.0.0")).toThrow(/requires Bun/);
  });

  it("renders identical bytes twice with matching independent SHA-256 implementations", async () => {
    const bundleRoot = join(dirname(fileURLToPath(import.meta.url)), "../📦️packages/🦀️rust");
    const first = await renderFrameWorker(bundleRoot);
    const second = await renderFrameWorker(bundleRoot);
    const subtle = Buffer.from(await crypto.subtle.digest("SHA-256", Buffer.from(first.content))).toString("hex");
    expect(second).toEqual(first);
    expect(createHash("sha256").update(first.content).digest("hex")).toBe(subtle);
  });

  it("derives devcontainer and native Bun provisioning from the single packageManager pin", () => {
    let repoRoot = dirname(fileURLToPath(import.meta.url));
    while (!existsSync(join(repoRoot, "nx.json"))) repoRoot = dirname(repoRoot);
    const dockerfile = readFileSync(join(repoRoot, ".devcontainer/Dockerfile"), "utf8");
    const postCreatePath = join(repoRoot, ".devcontainer/post-create.sh");
    const nativeBootstrapPath = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh");
    const postCreate = readFileSync(postCreatePath, "utf8");
    const nativeBootstrap = readFileSync(nativeBootstrapPath, "utf8");
    expect(dockerfile).not.toContain("bun.sh/install");
    expect(postCreate).toContain(".packageManager");
    expect(postCreate).toContain('bash -s "bun-v$required_bun_version"');
    expect(nativeBootstrap).toContain('bash -s "bun-v$required_version"');
    expect(`${dockerfile}\n${postCreate}\n${nativeBootstrap}`).not.toContain(Bun.version);
    if (process.platform !== "win32") {
      execFileSync("bash", ["-n", postCreatePath]);
      execFileSync("bash", ["-n", nativeBootstrapPath]);
    }
  });

  it("decodeAstralEscapes matches JSON.parse (an independent, spec-compliant \\uXXXX decoder) on every well-formed surrogate pair, and is a no-op on plain ASCII/BMP text", () => {
    const cases = ["\\uD83E\\uDDF0️framework/\\uD83D\\uDD28️modules/\\uD83D\\uDDBC️assets", "no escapes here", "café — 日本語 — é", "\\uD83C\\uDF31️metabolism/\\uD83C\\uDFA8️representation"];
    for (const input of cases) {
      const viaOracle = JSON.parse(`"${input}"`) as string;
      expect(decodeAstralEscapes(input)).toBe(viaOracle);
    }
  });

  it("renders an astral-emoji-bearing browser entry (🟦️.ts, which references the \"🟨️frame-worker.js\" filename by URL) with the emoji as literal UTF-8, not Bun's astral \\uXXXX surrogate-pair escapes — otherwise the reference scanner cannot see or rewrite it", async () => {
    const bundleRoot = dirname(fileURLToPath(import.meta.url));
    const content = await renderBrowserEntry(join(bundleRoot, "../🧵️browser-boot/🟦️.ts"));
    expect(content).toContain("🟨️frame-worker.js");
    expect(content).not.toMatch(/\\u[Dd][89abAB][0-9a-fA-F]{2}/);
  });
});

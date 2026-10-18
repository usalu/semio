/** 🧪️ Runtime asset delivery to the wgpu renderer: the seven channel files, their routes, the installer and the host boot order. */
import { describe, expect, it } from "vitest";
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { RUNTIME_ASSET_CHANNELS, RUNTIME_ASSET_FILES, installRuntimeAssets, runtimeAssetDeliveries, type RuntimeAssetBindings } from "../../../../../../../🔨️modules/🖼️canvas/🔌️runtime-assets/🟦️.ts";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../..");
const canvas = join(repoRoot, "🧰️framework/🔨️modules/🖼️canvas/🔌️runtime-assets");
const fixture = JSON.parse(readFileSync(join(canvas, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8")) as { channels: { id: string; route: string; directory: string }[]; assets: { role: string; channel: string; file: string; bytes?: number }[] };
const frameWorker = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts"), "utf8");

const page = "https://play.invalid/os/index.html";
const payload = (file: string): Uint8Array => Uint8Array.from(Array.from(file, ch => ch.codePointAt(0)! & 255));
const urlOf = (route: string, file: string): string => new URL(`${route}/${encodeURIComponent(file)}`, page).href;

describe("runtime asset delivery to the wgpu renderer", () => {
  it("names exactly the Rust port's seven assets, in catalog order, and every file that is checked in exists with the fixture's size", () => {
    expect(RUNTIME_ASSET_CHANNELS.map(({ id, route, directory }) => ({ id, route, directory }))).toEqual(fixture.channels);
    expect(RUNTIME_ASSET_FILES.map(({ channel, file }) => ({ channel, file }))).toEqual(fixture.assets.map(({ channel, file }) => ({ channel, file })));
    for (const row of fixture.assets) {
      const directory = fixture.channels.find(channel => channel.id === row.channel)!.directory;
      if (row.bytes === undefined) continue;
      expect(statSync(join(repoRoot, directory, row.file)).size, row.role).toBe(row.bytes);
    }
    const rust = readFileSync(join(canvas, "🦀️.rs"), "utf8");
    for (const channel of fixture.channels) expect(rust).toContain(`"${channel.directory}"`);
    for (const row of fixture.assets.filter(asset => asset.channel !== "compiler-fonts")) expect(rust).toContain(`"${row.file}"`);
    const world = readFileSync(join(repoRoot, "🧰️framework/🔨️modules/📚️compiler/🌍️world/🦀️.rs"), "utf8");
    const declared = [...world.matchAll(/FontRole::\w+ => "([^"]+)",/gu)].map(match => match[1]);
    expect(declared).toEqual(fixture.assets.filter(asset => asset.channel === "compiler-fonts").map(asset => asset.file));
  });

  it("fetches the seven served files and installs them in one call, in catalog order", async () => {
    const requested: string[] = [];
    const calls: Uint8Array[][] = [];
    const bindings: RuntimeAssetBindings = { semioSetRuntimeAssets: (...assets) => void calls.push(assets) };
    const installed = await installRuntimeAssets(bindings, page, async url => {
      requested.push(url);
      const file = decodeURIComponent(url.slice(url.lastIndexOf("/") + 1));
      return new Response(new Uint8Array(payload(file)));
    });
    expect(installed).toBe(true);
    expect([...requested].sort()).toEqual(RUNTIME_ASSET_FILES.map(({ route, file }) => urlOf(route, file)).sort());
    expect(requested).toHaveLength(7);
    expect(calls).toHaveLength(1);
    expect(calls[0]).toEqual(RUNTIME_ASSET_FILES.map(({ file }) => payload(file)));
  });

  it("installs nothing and fetches nothing when the renderer module has no asset door", async () => {
    let fetched = 0;
    expect(await installRuntimeAssets({}, page, async () => (fetched++, new Response(new Uint8Array(1))))).toBe(false);
    expect(fetched).toBe(0);
  });

  it("rejects a missing asset and never installs a partial set", async () => {
    let installs = 0;
    const bindings: RuntimeAssetBindings = { semioSetRuntimeAssets: () => void installs++ };
    const last = RUNTIME_ASSET_FILES.at(-1)!;
    await expect(installRuntimeAssets(bindings, page, async url => (url === urlOf(last.route, last.file) ? new Response("no", { status: 404 }) : new Response(new Uint8Array(2))))).rejects.toThrow(/runtime-assets: 404/u);
    expect(installs).toBe(0);
  });

  it("is served by every host that serves the wgpu renderer module", () => {
    expect(runtimeAssetDeliveries()).toEqual(RUNTIME_ASSET_CHANNELS.map(({ route, directory }) => ({ kind: "static-dir", route, root: directory })));
    const devBuilder = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts"), "utf8");
    expect(devBuilder).toContain("...runtimeAssetDeliveries(),");
    const scope = readFileSync(join(repoRoot, ".storybook/📖️stories/🧭️coordination/🟦️.ts"), "utf8");
    expect(scope).toContain("      ...runtimeAssetDeliveries(),\n      { kind: \"static-dir\", route: \"/renderer-modules\"");
    const storybook = readFileSync(join(repoRoot, ".storybook/main.ts"), "utf8");
    expect(storybook).toContain("...RUNTIME_ASSET_CHANNELS.map(({ route, directory }) => ({ from: resolve(repoRootPath, directory), to: route }))");
    const server = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/🧊️wgpu/🌐️server/🟦️.ts"), "utf8");
    expect(server.replace(/\s+/gu, "")).toContain("...RUNTIME_ASSET_CHANNELS.map(({ route, directory }) => [route, resolve(options.workspace, directory)] as const)".replace(/\s+/gu, ""));
    for (const channel of RUNTIME_ASSET_CHANNELS) expect(existsSync(join(repoRoot, channel.directory)) || channel.id === "icon-pack", channel.id).toBe(true);
  });

  it("keeps booting, and says why, when the assets cannot be fetched", () => {
    expect(frameWorker).toMatch(/catch \(error\) \{\s+if \(closed \|\| closing\) throw error;\s+progress\(`runtime-assets-fault: /u);
  });

  it("boots the renderer wasm, then installs the assets, then calls the renderer bootstrap that first resolves an icon", () => {
    const instantiate = frameWorker.indexOf("await instantiateRendererWasm(loaded, message.bindingsWasmUrl)");
    const install = frameWorker.indexOf("installRuntimeAssets(loaded, scope.location.href");
    const bootstrap = frameWorker.indexOf("loaded.semioWgpuWorkerBootstrap!(");
    expect(instantiate).toBeGreaterThan(-1);
    expect(install).toBeGreaterThan(instantiate);
    expect(bootstrap).toBeGreaterThan(install);
    expect(frameWorker.indexOf("installRuntimeAssets(loaded")).toBe(frameWorker.lastIndexOf("installRuntimeAssets(loaded"));
  });
});

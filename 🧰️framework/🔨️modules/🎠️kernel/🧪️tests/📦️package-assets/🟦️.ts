import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { ActivationRegistry } from "../../🎛️activation-registry/🟦️.ts";
import {
  PACKAGE_ASSET_DIRECTORIES,
  PACKAGE_ASSET_ROUTE,
  fetchPackageAssets,
  packageAssetMounts,
  packageAssetUrl,
  readPackageAssetDeclarations,
  type PackageAssetDeclaration
} from "../../📦️package-assets/🟦️.ts";
import { descriptorPackageAssets, fetchDescriptorManifest } from "../../🛂️descriptor-admission/🟦️.ts";
import { OwnedResidentLedger } from "@semio-tech/value-ts/resident";
import { ShardClient, type ShardBudget } from "@semio-tech/framework-actor/shard-client";
import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";
import { renderTextFontsFixtureUrl } from "../../../🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts";
import { canvasRuntimeAssetsFixtureUrl } from "@semio-tech/framework-canvas-ts/testing/fixtures";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "../../../../..");
const BASE = "http://page.test/";
const BUDGET: ShardBudget = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16, maxFrames: 8 };
const FONT_FIXTURE = JSON.parse(readFileSync(renderTextFontsFixtureUrl, "utf8")) as { fonts: { assetPath: string; byteLength: number; sha256: string }[] };
const FONT_ROOT = PACKAGE_ASSET_DIRECTORIES[0]!;
const CANVAS_FIXTURE = JSON.parse(readFileSync(canvasRuntimeAssetsFixtureUrl, "utf8")) as { channels: { id: string; directory: string }[]; assets: { role: string; channel: string; file: string; bytes?: number; sha256?: string }[]; guestAssets: string[]; iconPackDigestFile: string };

const nodeDigest = async (bytes: ArrayBuffer): Promise<string> => createHash("sha256").update(new Uint8Array(bytes)).digest("hex");
const declared: PackageAssetDeclaration[] = FONT_FIXTURE.fonts.map(font => ({ name: `${FONT_ROOT}/${font.assetPath}`, sizeBytes: font.byteLength, sha256: font.sha256 }));

/** 🌐️ Serves the declared route from the repository files, exactly as the dev server's static mounts do. */
function servedFetch(requested: string[] = []): (url: string) => Promise<Response> {
  return async url => {
    requested.push(url);
    const path = decodeURIComponent(new URL(url).pathname);
    const prefix = `${PACKAGE_ASSET_ROUTE}/`;
    if (!path.startsWith(prefix)) return new Response("no", { status: 404 });
    try { return new Response(readFileSync(join(repoRoot, path.slice(prefix.length)))); } catch { return new Response("no", { status: 404 }); }
  };
}

function capturingShardClient(posted: unknown[]): ShardClient {
  const createWorker = () => {
    const worker: { postMessage: (message: unknown) => void; terminate: () => void; onmessage: ((event: { readonly data: unknown }) => void) | null; onerror: ((event: unknown) => void) | null } = {
      postMessage: message => {
        posted.push(message);
        const requestId = (message as { readonly requestId?: string }).requestId;
        if (requestId) queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId, ok: true, value: undefined } }));
      },
      terminate: () => {},
      onmessage: null,
      onerror: null,
    };
    return worker;
  };
  return new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(), residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker });
}

/** 🪶️ The worker's own splice, taken from its source text so the test runs the shipped code, not a copy. */
function workerSplice(): (entry: { pendingAssets: unknown[] }, events: unknown[]) => any[] {
  const source = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts"), "utf8");
  const start = source.indexOf("function spliceInstanceOpenAssets(");
  const end = source.indexOf("\n}\n", start) + 3;
  return new Function(`${source.slice(start, end).replaceAll("\\`", "`")}; return spliceInstanceOpenAssets;`)() as never;
}

describe("package assets in the browser kernel", () => {
  it("serves exactly the declared directories and maps a declared name to one URL below them", () => {
    expect(packageAssetMounts()).toEqual(PACKAGE_ASSET_DIRECTORIES.map(directory => [`${PACKAGE_ASSET_ROUTE}/${directory}`, directory]));
    expect(packageAssetUrl(declared[0]!.name, BASE)?.startsWith(`${BASE.slice(0, -1)}${PACKAGE_ASSET_ROUTE}/`)).toBe(true);
    expect(packageAssetUrl("📚️examples/s.puzzle.5d.1.-/body.json", BASE)).toBeUndefined();
    expect(packageAssetUrl(`${FONT_ROOT}/../../secret.bin`, BASE)).toBeUndefined();
    expect(packageAssetUrl("/etc/hosts", BASE)).toBeUndefined();
  });

  it("serves the canvas guest assets under the names the Rust port declares, with the sizes and digests of its fixture and of the builder's pack digest", async () => {
    const guest = CANVAS_FIXTURE.guestAssets.map(role => CANVAS_FIXTURE.assets.find(asset => asset.role === role)!);
    const directoryOf = (channel: string) => CANVAS_FIXTURE.channels.find(candidate => candidate.id === channel)!.directory;
    const pack = guest.find(asset => asset.role === "icon-pack")!;
    const digest = JSON.parse(readFileSync(join(repoRoot, directoryOf(pack.channel), CANVAS_FIXTURE.iconPackDigestFile), "utf8")) as { sizeBytes: number; sha256: string };
    const canvasDeclared: PackageAssetDeclaration[] = guest.map(asset => asset.role === "icon-pack" ? { name: `${directoryOf(asset.channel)}/${asset.file}`, sizeBytes: digest.sizeBytes, sha256: digest.sha256 } : { name: `${directoryOf(asset.channel)}/${asset.file}`, sizeBytes: asset.bytes!, sha256: asset.sha256! });
    for (const declaration of canvasDeclared) expect(packageAssetUrl(declaration.name, BASE), declaration.name).toBeDefined();
    const assets = await fetchPackageAssets(canvasDeclared, BASE, servedFetch(), nodeDigest);
    expect(assets.map(([name]) => name)).toEqual(canvasDeclared.map(declaration => declaration.name));
    for (const [name, bytes] of assets) expect(Buffer.from(bytes).equals(readFileSync(join(repoRoot, name))), name).toBe(true);
  });

  it("hands the assets a descriptor declares to a loader that registers only its manifest, like the React lane's module loader", async () => {
    const original = globalThis.fetch;
    const serve = (document: object) => { globalThis.fetch = Object.assign(async () => new Response(JSON.stringify(document), { headers: { "content-type": "application/json" } }), { preconnect: () => {} }); };
    try {
      serve({ manifest: { pluginId: "demo", apps: [] }, assets: declared });
      const manifest = await fetchDescriptorManifest("demo", "http://page.test/modules/demo/bridge.js");
      expect(descriptorPackageAssets(manifest)).toEqual(declared);
      serve({ manifest: { pluginId: "demo", apps: [] } });
      expect(descriptorPackageAssets(await fetchDescriptorManifest("demo", "http://page.test/modules/demo/bridge.js"))).toEqual([]);
      serve({ manifest: { pluginId: "demo", apps: [] }, assets: [{ name: "x", sizeBytes: -1, sha256: "0".repeat(64) }] });
      await expect(fetchDescriptorManifest("demo", "http://page.test/modules/demo/bridge.js")).rejects.toThrow("malformed package asset declaration");
    } finally { globalThis.fetch = original; }
  });

  it("reads the descriptor's declarations and refuses a malformed one", () => {
    expect(readPackageAssetDeclarations({ assets: declared })).toEqual(declared);
    expect(readPackageAssetDeclarations({})).toEqual([]);
    expect(() => readPackageAssetDeclarations({ assets: [{ name: "x", sizeBytes: -1, sha256: "0".repeat(64) }] })).toThrow("malformed");
    expect(() => readPackageAssetDeclarations({ assets: [{ name: "x", sizeBytes: 1, sha256: "nothex" }] })).toThrow("malformed");
  });

  it("fetches every declared font with its exact bytes, checked against size and SHA-256", async () => {
    const requested: string[] = [];
    const assets = await fetchPackageAssets(declared, BASE, servedFetch(requested), nodeDigest);
    expect(assets.map(([name]) => name)).toEqual(declared.map(declaration => declaration.name));
    for (const [index, [name, bytes]] of assets.entries()) {
      const onDisk = readFileSync(join(repoRoot, name));
      expect(Buffer.from(bytes).equals(onDisk), name).toBe(true);
      expect(bytes.byteLength).toBe(declared[index]!.sizeBytes);
    }
    expect(requested).toHaveLength(declared.length);
  });

  it("refuses bytes that differ from the declaration, naming the asset", async () => {
    const wrongSize = [{ ...declared[0]!, sizeBytes: declared[0]!.sizeBytes + 1 }];
    await expect(fetchPackageAssets(wrongSize, BASE, servedFetch(), nodeDigest)).rejects.toThrow(declared[0]!.name);
    const wrongHash = [{ ...declared[0]!, sha256: "0".repeat(64) }];
    await expect(fetchPackageAssets(wrongHash, BASE, servedFetch(), nodeDigest)).rejects.toThrow("hashes to");
    await expect(fetchPackageAssets(declared, BASE, async () => new Response("no", { status: 404 }), nodeDigest)).rejects.toThrow("404");
  });

  it("delivers a plugin's declared assets with the activation and the worker splices them, byte for byte, into the first instance-open", async () => {
    const posted: any[] = [];
    const registry = new ActivationRegistry({ shardClient: capturingShardClient(posted), defaultBudget: BUDGET, fetchAssets: async () => [], fetchPackageAssets: assets => fetchPackageAssets(assets, BASE, servedFetch(), nodeDigest) });
    registry.registerCatalog({
      plugins: [{ pluginId: "layout", wasmOut: "x", role: "plugin", contributes: [], consumes: [], assets: declared }],
      extensions: [],
      hosts: [],
      playgrounds: [],
      moduleUrl: () => "https://x/layout.js",
      extensionModuleUrl: () => "https://x/ext.js",
    });
    expect(registry.manifestFor("layout")?.assets).toEqual(declared);
    await registry.activate("layout", "actor-1", "manual");
    const activation = posted.find(message => message.kind === "activate");
    expect(activation.assets.map(([name]: [string]) => name)).toEqual(declared.map(declaration => declaration.name));
    const entry = { pendingAssets: activation.assets as unknown[] };
    const [opened] = workerSplice()(entry, [{ kind: "instance-open", payload: { assets: [] } }]);
    expect(opened.payload.assets).toHaveLength(declared.length);
    for (const [name, bytes] of opened.payload.assets as [string, ArrayBuffer][]) expect(Buffer.from(bytes).equals(readFileSync(join(repoRoot, name))), name).toBe(true);
    expect(entry.pendingAssets).toEqual([]);
  });

  it("activates a plugin without its assets, and reports it, when a delivery is refused", async () => {
    const posted: any[] = [];
    const registry = new ActivationRegistry({ shardClient: capturingShardClient(posted), defaultBudget: BUDGET, fetchAssets: async () => [], fetchPackageAssets: assets => fetchPackageAssets(assets, BASE, async () => new Response("no", { status: 404 }), nodeDigest) });
    registry.registerManifest({ pluginId: "layout", moduleUrl: "https://x/layout.js", caps: [], assets: declared });
    await registry.activate("layout", "actor-1", "manual");
    expect(posted.find(message => message.kind === "activate").assets).toEqual([]);
  });

  it("delivers nothing for a plugin that declares no assets", async () => {
    const posted: any[] = [];
    let fetched = 0;
    const registry = new ActivationRegistry({ shardClient: capturingShardClient(posted), defaultBudget: BUDGET, fetchAssets: async () => [], fetchPackageAssets: async () => { fetched += 1; return []; } });
    registry.registerManifest({ pluginId: "plain", moduleUrl: "https://x/plain.js", caps: [] });
    await registry.activate("plain", "actor-2", "manual");
    expect(fetched).toBe(0);
    expect(posted.find(message => message.kind === "activate").assets).toEqual([]);
  });
});

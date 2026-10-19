import { describe, it, expect } from "bun:test";
import { readFileSync, mkdtempSync, mkdirSync, writeFileSync, rmSync, existsSync } from "node:fs";
import { resolve, join, dirname } from "node:path";
import { tmpdir } from "node:os";
import { createServer } from "node:http";
import { meshCollectionVitePlugin } from "../../../🏗️builder/🌐️vite/🗺️mesh-collection/🟦️.ts";
import { parseMeshDeliveryCatalog, meshAssetTransportUrl, resolveMeshAsset } from "@semio-tech/assets/mesh";
import type { OwnedBuildMiddleware } from "@semio-tech/ui-react/build-tooling";
import type { MeshCollectionAssetSpecV1 } from "../../../🏗️builder/🌐️vite/🟦️.ts";
import { createHash } from "node:crypto";
import { MESH_DELIVERY_CATALOG } from "@semio-tech/assets/mesh/catalog";
import { ownedServer, repoRoot } from "../🧰️suite-build-server/🟦️.ts";

describe("nested mesh source identity", () => {
  it("preserves explicit neutral source ownership and nested output paths in HTTP and static output", async () => {
    const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧊️mesh-collection.json"), "utf8"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-mesh-collection-"));
    let server: ReturnType<typeof createServer> | undefined;
    try {
      for (const root of fixture.roots) {
        for (const file of root.files) {
          const path = resolve(sandbox, root.path, file.path);
          mkdirSync(dirname(path), { recursive: true });
          writeFileSync(path, file.content);
        }
      }
      writeFileSync(resolve(sandbox, fixture.placeholder.path), fixture.placeholder.content);
      writeFileSync(resolve(sandbox, "📇️catalog.json"), JSON.stringify(fixture.catalog));
      const { default: glob } = await import("fast-glob");
      const independent = new Map<string, string>();
      for (const root of fixture.roots) {
        for (const path of await glob("**/*.glb", { cwd: resolve(sandbox, root.path), onlyFiles: true })) {
          if (!independent.has(path)) independent.set(path, readFileSync(resolve(sandbox, root.path, path), "utf8"));
        }
      }
      independent.set(fixture.placeholder.path, fixture.placeholder.content);
      const expected = new Map<string, string>(fixture.expected.map((file: { path: string; content: string }) => [file.path, file.content]));
      expect(independent).toEqual(expected);
      const plugins = meshCollectionVitePlugin(sandbox, {
        kind: "mesh-collection", route: "/mesh", catalog: "📇️catalog.json",
      });
      const catalog = parseMeshDeliveryCatalog(fixture.catalog, () => { throw new Error("Unexpected source catalog"); });
      let handler: OwnedBuildMiddleware | undefined;
      plugins[0]!.configureServer!(ownedServer((value) => { handler = value; }));
      server = createServer((request, response) => handler!(request, response, () => { response.statusCode = 404; response.end(); }));
      await new Promise<void>(done => server!.listen(0, "127.0.0.1", done));
      const address = server.address();
      if (!address || typeof address === "string") throw new Error("Mesh test server has no TCP address");
      for (const entry of catalog) {
        const response = await fetch(`http://127.0.0.1:${address.port}${meshAssetTransportUrl(entry.url, catalog)}`);
        expect(response.status).toBe(200);
        expect(response.headers.get("content-type")).toBe("model/gltf-binary");
        const body = expected.get(entry.path);
        if (body === undefined) throw new Error(`no expected body for ${entry.path}`);
        expect(await response.text()).toBe(body);
      }
      expect((await fetch(`http://127.0.0.1:${address.port}/mesh/unregistered.glb`)).status).toBe(404);
      expect((await fetch(`http://127.0.0.1:${address.port}/mesh/💊️capsules/🪝️j/📐️source.3dm`)).status).toBe(404);
      plugins[1]!.configResolved!({ root: sandbox, build: { outDir: "output" } });
      plugins[1]!.closeBundle!();
      const output = resolve(sandbox, "output/mesh");
      expect((await glob("**/*", { cwd: output, onlyFiles: true })).sort()).toEqual([...expected.keys()].sort());
      for (const [path, content] of expected) expect(readFileSync(resolve(output, path), "utf8")).toBe(content);
    } finally {
      if (server) await new Promise<void>((done, reject) => server!.close(error => error ? reject(error) : done()));
      rmSync(sandbox, { recursive: true, force: true });
    }
  });
});

describe("puzzle3d mesh-collection asset spec", () => {
  const puzzle3dMeshSpec: MeshCollectionAssetSpecV1 = {
    kind: "mesh-collection",
    route: "/mesh",
    catalog: "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json",
  };

  it("delivers every current public mesh identity byte-identically through dev and nested static output", async () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-current-mesh-"));
    const plugins = meshCollectionVitePlugin(repoRoot, puzzle3dMeshSpec);
    let handler: OwnedBuildMiddleware | undefined;
    plugins[0]!.configureServer!(ownedServer((value) => { handler = value; }));
    const server = createServer((request, response) => handler!(request, response, () => { response.statusCode = 404; response.end(); }));
    try {
      await new Promise<void>(done => server.listen(0, "127.0.0.1", done));
      const address = server.address();
      if (!address || typeof address === "string") throw new Error("Mesh test server has no TCP address");
      plugins[1]!.configResolved!({ root: sandbox, build: { outDir: "output" } });
      plugins[1]!.closeBundle!();
      const hash = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
      for (const entry of MESH_DELIVERY_CATALOG) {
        const expected = hash(readFileSync(resolve(repoRoot, entry.source)));
        const response = await fetch(`http://127.0.0.1:${address.port}${meshAssetTransportUrl(entry.url, MESH_DELIVERY_CATALOG)}`);
        expect(response.status).toBe(200);
        expect(response.headers.get("content-type")).toBe("model/gltf-binary");
        expect(hash(new Uint8Array(await response.arrayBuffer()))).toBe(expected);
        expect(hash(readFileSync(resolve(sandbox, "output/mesh", entry.path)))).toBe(expected);
      }
      const { default: glob } = await import("fast-glob");
      expect((await glob("**/*", { cwd: resolve(sandbox, "output/mesh"), onlyFiles: true })).sort()).toEqual(MESH_DELIVERY_CATALOG.map(entry => entry.path).sort());
      expect(MESH_DELIVERY_CATALOG).toHaveLength(93);
      expect(() => meshAssetTransportUrl("/mesh/🧊️ellipsoid-🧊️capsule_J.glb", MESH_DELIVERY_CATALOG)).toThrow();
    } finally {
      await new Promise<void>((done, reject) => server.close(error => error ? reject(error) : done()));
      rmSync(sandbox, { recursive: true, force: true });
    }
  }, 30_000);

  it("resolves kit glb roots and shared placeholder", () => {
    expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️capsule_J.glb", MESH_DELIVERY_CATALOG).source))).toBe(true);
    expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️placeholder.glb", MESH_DELIVERY_CATALOG).source))).toBe(true);
  });

  it("registers a generic mesh-collection serve/build program pair", () => {
    const plugins = meshCollectionVitePlugin(repoRoot, puzzle3dMeshSpec);
    expect(plugins.map((plugin) => plugin.name)).toEqual(["mesh-collection-serve/mesh", "mesh-collection-build/mesh"]);
  });

  it("includes 🧊️base.glb for shooting's default fixture", () => {
    expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️base.glb", MESH_DELIVERY_CATALOG).source))).toBe(true);
  });
});

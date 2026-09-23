import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { createServer, type IncomingMessage, type ServerResponse } from "node:http";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { staticDirMountVitePlugins } from "../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import { MODULE_PLUGIN_ROUTE, MODULE_VENDOR_DIRECTORY } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { FONT_ASSET } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔤️fonts/🟦️.ts";
import type { playDevStaticDirMounts } from "../../🔨️modules/🧩️runtime/📦️assets/🟦️.ts";

type Middleware = (request: IncomingMessage, response: ServerResponse, next: () => void) => void;

/** 🗺️ Play's dev route table serves the guest typst font pack beside the staged Preview2 shim. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: { playDevStaticDirMounts: typeof playDevStaticDirMounts }): Promise<void> {
  const { describe, expect, it } = vitest;
  describe("playDevStaticDirMounts", () => {
    it("claims every route once and serves the font pack, the shim and the extensions through one table", async () => {
      const workspace = mkdtempSync(join(tmpdir(), "play-dev-mounts-"));
      const mounts = dependencies.playDevStaticDirMounts(workspace, name => join(workspace, "📥️installed", name));
      const put = (route: string, file: string, content: string) => { const mount = mounts.find(row => row.route === route)!; mkdirSync(join(workspace, mount.root), { recursive: true }); writeFileSync(join(workspace, mount.root, file), content); };
      const vendor = `${MODULE_PLUGIN_ROUTE}/${MODULE_VENDOR_DIRECTORY}`;
      expect(mounts.filter(row => row.route === vendor).map(row => row.root)).toEqual(["🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts"]);
      const shim = mounts.find(row => row.route.startsWith(vendor + "/"))!, extension = mounts.find(row => row.root.startsWith("📥️installed/"))!, plugin = mounts.find(row => row.route.startsWith(MODULE_PLUGIN_ROUTE + "/") && !row.route.startsWith(vendor) && row.root.includes("🔌️plugin-modules/"))!;
      put(vendor, FONT_ASSET, "font-pack");
      put(shim.route, "cli.js", "export const cli = 1;");
      put(extension.route, "🌉️bridge.js", "export const extension = 1;");
      put(plugin.route, "🌉️bridge.js", "export const plugin = 1;");
      const chain: Middleware[] = [];
      for (const item of staticDirMountVitePlugins(workspace, mounts)) (item.configureServer as ((server: unknown) => void) | undefined)?.({ middlewares: { use: (middleware: Middleware) => chain.push(middleware) } });
      const server = createServer((request, response) => { const step = (index: number): void => index === chain.length ? void (response.statusCode = 418, response.end()) : chain[index]!(request, response, () => step(index + 1)); step(0); });
      try {
        await new Promise<void>(done => server.listen(0, "127.0.0.1", done));
        const address = server.address();
        if (!address || typeof address === "string") throw new Error("Missing play mount server address");
        const get = async (path: string) => { const response = await fetch(new URL(path.split("/").map(encodeURIComponent).join("/"), `http://127.0.0.1:${address.port}`)); return [response.status, await response.text()]; };
        expect(await get(`${vendor}/${FONT_ASSET}`)).toEqual([200, "font-pack"]);
        expect(await get(`${shim.route}/cli.js`)).toEqual([200, "export const cli = 1;"]);
        expect(await get(`${extension.route}/🌉️bridge.js`)).toEqual([200, "export const extension = 1;"]);
        expect(await get(`${plugin.route}/🌉️bridge.js`)).toEqual([200, "export const plugin = 1;"]);
        expect((await get(`${vendor}/missing.bin`))[0]).toBe(404);
        console.log(`[DEBUG] play dev route table: ${mounts.length} unique mounts, font pack + shim + plugin + extension served`);
      } finally {
        await new Promise<void>(done => server.close(() => done()));
        rmSync(workspace, { recursive: true, force: true });
      }
    });
  });
}

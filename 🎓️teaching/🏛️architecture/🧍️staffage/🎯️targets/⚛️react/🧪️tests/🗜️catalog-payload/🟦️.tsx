/** 🗜️ A catalog asset file ships as compact JSON with a precomputed revision that equals the revision of its authored text.
 * @see ./🥒️.feature — the scenarios */
import { createHash } from "node:crypto";
import { inflateSync } from "node:zlib";
import { readFileSync, readdirSync } from "node:fs";
import { basename, dirname, resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { revisionOfText, validateAsset } from "@teaching/staffage";
import { staffageAssetPayload, staffageAssetVitePlugin, type StaffageServedPayload } from "../../🌐️vite/🟦️.ts";

const catalog = resolve(import.meta.dirname, "../../../../🗂️catalog");
const assetFiles = readdirSync(catalog, { withFileTypes: true }).flatMap((group) => (group.isDirectory() ? readdirSync(resolve(catalog, group.name), { withFileTypes: true }).filter((item) => item.isDirectory()).map((item) => resolve(catalog, group.name, item.name, "🔣️.json")) : []));
const file = assetFiles.find(path => basename(dirname(path)) === "toaster")!;
const slug = basename(dirname(file));
const served = import.meta.glob<StaffageServedPayload>("../../../../🗂️catalog/*/*/🔣️.json", { query: "?staffage-asset", import: "default" });

describe("catalog payload", () => {
  it("ships the revision of the authored text", () => {
    const text = readFileSync(file, "utf8");
    expect(staffageAssetPayload(file).revision).toBe(revisionOfText(text));
    expect(staffageAssetPayload(file).revision).toBe(`sha256:${createHash("sha256").update(text.replaceAll("\r\n", "\n")).digest("hex")}`);
  });

  it("ships the authored asset as compact json", () => {
    const text = readFileSync(file, "utf8");
    const { json } = staffageAssetPayload(file);
    expect(JSON.parse(json)).toEqual((JSON.parse(text) as { asset: unknown }).asset);
    expect(json).not.toMatch(/\n\s/u);
    expect(json.length).toBeLessThan(text.length);
  });

  it("indexes each exact volume within the compact document", () => {
    const payload = staffageAssetPayload(file);
    const reference = JSON.parse(readFileSync(file, "utf8")).asset;
    const { volumes, ...header } = reference;
    const index = payload.volumeIndex!;
    expect(JSON.parse(payload.json.slice(0, index.headerEnd) + "}")).toEqual(header);
    expect(Object.keys(index.ranges)).toEqual(Object.keys(volumes));
    for (const [scale, [start, end]] of Object.entries(index.ranges)) {
      expect(JSON.parse(payload.json.slice(start, end))).toEqual(volumes[scale]);
    }
  });

  it("serves every asset file through the plugin to the importer", async () => {
    const keys = Object.keys(served);
    expect(keys.length).toBeGreaterThan(0);
    const module = await served[keys.find((key) => key.includes(slug))!]!();
    const payload = staffageAssetPayload(file);
    expect(module.revision).toBe(payload.revision);
    expect("header" in module).toBe(true);
    expect(module.volumeIndex).toEqual(payload.volumeIndex);
    const header = JSON.parse(inflateSync(Buffer.from(module.header, "base64")).toString("utf8"));
    const volumes = Object.fromEntries(Object.entries(module.scales!).map(([scale, bytes]) => [
      scale, JSON.parse(inflateSync(Buffer.from(bytes, "base64")).toString("utf8")),
    ]));
    for (const [scale, packed] of Object.entries(module.drawings!)) {
      expect(header.representations[scale]).not.toHaveProperty("geometry");
      header.representations[scale].geometry = JSON.parse(inflateSync(Buffer.from(packed, "base64")).toString("utf8"));
    }
    expect({ ...header, volumes }).toEqual(JSON.parse(payload.json));
    expect(module.issues).toEqual(validateAsset(JSON.parse(payload.json)));
  });

  it("watches the asset file behind the served module", () => {
    const watched: string[] = [];
    const plugin = staffageAssetVitePlugin();
    const id = plugin.resolveId?.(`../${basename(dirname(file))}/🔣️.json?staffage-asset`, resolve(dirname(file), "x.ts")) as string;
    (plugin.load as (this: { addWatchFile: (file: string) => void }, id: string) => unknown).call({ addWatchFile: (path) => watched.push(path) }, id);
    expect(watched).toEqual([file]);
  });

  it("leaves other imports to Vite", () => {
    const plugin = staffageAssetVitePlugin();
    expect(plugin.resolveId?.("./x.json", "/a/b.ts")).toBeNull();
    expect(plugin.load?.("/a/x.json")).toBeNull();
  });
});

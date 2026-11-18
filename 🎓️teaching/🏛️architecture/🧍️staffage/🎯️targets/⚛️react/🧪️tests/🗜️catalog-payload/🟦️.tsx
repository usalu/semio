/** 🗜️ A catalog asset file ships as compact JSON with a precomputed revision that equals the revision of its authored text.
 * @see ./🥒️.feature — the scenarios */
import { createHash } from "node:crypto";
import { readFileSync, readdirSync } from "node:fs";
import { basename, dirname, resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { revisionOfText } from "@teaching/staffage";
import { staffageAssetPayload, staffageAssetVitePlugin } from "../../🌐️vite/🟦️.ts";

const catalog = resolve(import.meta.dirname, "../../../../🗂️catalog");
const assetFiles = readdirSync(catalog, { withFileTypes: true }).flatMap((group) => (group.isDirectory() ? readdirSync(resolve(catalog, group.name), { withFileTypes: true }).filter((item) => item.isDirectory()).map((item) => resolve(catalog, group.name, item.name, "🔣️.json")) : []));
const file = assetFiles[0]!;
const slug = basename(dirname(file));
const served = import.meta.glob<{ revision: string; json: string }>("../../../../🗂️catalog/*/*/🔣️.json", { query: "?staffage-asset", import: "default" });

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

  it("serves every asset file through the plugin to the importer", async () => {
    const keys = Object.keys(served);
    expect(keys.length).toBeGreaterThan(0);
    const module = await served[keys.find((key) => key.includes(slug))!]!();
    const payload = staffageAssetPayload(file);
    expect(module.revision).toBe(payload.revision);
    expect(module.json).toBe(payload.json);
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

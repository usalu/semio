/** 🧊️ The batch export of the 3D models: the complete set of the catalog as files, with a manifest, deterministic bytes, resumability, cancellation and bounded paths, every authored scale in one run, and never a deleted file of another scale.
 *
 * THE REFERENCES are `node:crypto` SHA-256 of the files read back, `fflate` for the ZIP, the single exports (`writeGlb`, `writeFbx`, `writeModel`, `writeModelBundle`) the batch must equal byte for byte, and the command line of the existing export CLI for the cancellation of its emit loops.
 * The catalog under test is a temporary one made of the handcrafted four-scale sample figure, so the test does not wait for the re-authoring of the 152 catalog assets.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { unzipSync } from "fflate";
import { afterAll, describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { BATCH_SCHEMA, type BatchManifest, MAX_PATH_CHARS, exportModels, keyOf, parseBatchArguments, plannedNames } from "../../🔨️modules/📤️export/📦️batch/🟦️.ts";
import { producerDigest } from "../../🔨️modules/📤️export/📦️batch/🔏️producer/🟦️.ts";
import { parseArguments, runExport } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { writeFbx } from "../../🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { writeGlb } from "../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { fbxName, glbName, modelName, writeModel, zipName } from "../../🔨️modules/📤️export/🗿️model/🟦️.ts";
import { writeModelBundle } from "../../🔨️modules/📤️export/🗜️zip/📦️models/🟦️.ts";
import { scaledFigure } from "../🧰️support/🧊️volumes/🟦️.ts";

const VIEWS = ["light-solid", "thermal"] as const;
const made: string[] = [];
const temporary = (): string => {
  const directory = mkdtempSync(join(tmpdir(), "staffage-batch-"));
  made.push(directory);
  return directory;
};
afterAll(() => made.forEach((d) => rmSync(d, { recursive: true, force: true })));

const figure = (id: string): Asset => {
  const base = scaledFigure();
  return { ...base, id, clips: base.clips.filter((c) => c.id === "walk") } as Asset;
};

const catalogOf = (assets: readonly Asset[]): string => {
  const root = temporary();
  writeFileSync(join(root, "🔣️.json"), JSON.stringify({ version: "semio.staffage/v2", categories: [{ id: "test", emoji: "🧪", title: { en: "Test", de: "Test" }, description: { en: "Test", de: "Test" }, folder: "test" }], collections: [] }));
  for (const asset of assets) {
    mkdirSync(join(root, "test", asset.id), { recursive: true });
    writeFileSync(join(root, "test", asset.id, "🔣️.json"), JSON.stringify({ version: "semio.staffage/v2", asset }));
  }
  return root;
};

const sha = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const manifestOf = (out: string): BatchManifest => JSON.parse(readFileSync(join(out, "manifest.json"), "utf8")) as BatchManifest;
const bytesOf = (path: string): Uint8Array => new Uint8Array(readFileSync(path));
const A = figure("figure-a");
const B = figure("figure-b");
const unauthored = { ...figure("figure-c"), volumes: Object.fromEntries([50, 100, 200, 500].map((s) => [`${s}`, { status: "unavailable", denominator: s }])) } as unknown as Asset;
const catalog = catalogOf([A, B, unauthored]);
const PRODUCER = "f".repeat(64);
const FBX = (out: string, asset: string, scale: number, view = "light-solid"): string => join(out, asset, fbxName(asset, "walk", scale, view as "light-solid"));

describe("🧊️ the batch export of the 3D models", () => {
  it("writes per asset a GLB per pose with every scale, the model JSON of each, an FBX per clip, authored scale and view, and the ZIP, with a manifest that matches the files", { timeout: 600_000 }, async () => {
    const out = temporary();
    const { manifest, summary } = await exportModels({ producer: PRODUCER, out, catalog, assets: ["figure-a"], views: VIEWS });
    expect(manifest.$schema).toBe(BATCH_SCHEMA);
    expect(manifestOf(out)).toEqual(manifest);
    const names = plannedNames(A, { views: VIEWS }).map((n) => n.name);
    expect(readdirSync(join(out, "figure-a")).sort()).toEqual([...names].sort());
    expect(names).toContain(glbName("figure-a", "stand"));
    expect(names).toContain(modelName("figure-a", "clip", "walk"));
    for (const scale of [50, 100, 200, 500]) for (const view of VIEWS) expect(names, `${scale} ${view}`).toContain(fbxName("figure-a", "walk", scale, view));
    expect(manifest.files.map((f) => f.file.slice("figure-a/".length)).sort()).toEqual([...names].sort());
    expect(summary.files).toBe(manifest.files.length);
    expect(summary.unauthored).toEqual([]);
    expect(manifest.files.map((f) => f.file)).toEqual([...manifest.files.map((f) => f.file)].sort());
    for (const f of manifest.files) {
      const bytes = bytesOf(join(out, f.file));
      expect(bytes.length, f.file).toBe(f.bytes);
      expect(sha(bytes), f.file).toBe(f.sha256);
      expect(join(out, f.file).length, f.file).toBeLessThanOrEqual(MAX_PATH_CHARS);
      if (f.kind === "glb" || f.kind === "fbx") expect(f.triangles, f.file).toBeGreaterThan(0);
      expect(f.scale === null, f.file).toBe(f.kind !== "fbx");
    }
    expect(JSON.stringify(manifest)).not.toMatch(/\d{4}-\d{2}-\d{2}T/);
    const members = Object.keys(unzipSync(bytesOf(join(out, "figure-a", zipName("figure-a")))));
    expect(members.length).toBe(names.length - 1);
    expect(new Set(members).size).toBe(members.length);
  });

  it("writes exactly what the single exports write", { timeout: 600_000 }, async () => {
    const out = temporary();
    await exportModels({ producer: PRODUCER, out, catalog, assets: ["figure-a"], scales: [100], views: ["thermal"] });
    const same = (path: string, bytes: Uint8Array | string): void => expect(sha(typeof bytes === "string" ? new TextEncoder().encode(bytes) : bytes), path).toBe(sha(bytesOf(path)));
    same(join(out, "figure-a", glbName("figure-a", "stand")), await writeGlb(A, { pose: "stand" }));
    same(FBX(out, "figure-a", 100, "thermal"), await writeFbx(A, { clip: "walk", scale: 100, view: "thermal" }));
    same(join(out, "figure-a", modelName("figure-a", "pose", "stand")), await writeModel(A, { pose: "stand" }));
    same(join(out, "figure-a", modelName("figure-a", "clip", "walk")), await writeModel(A, { clip: "walk" }));
    same(join(out, "figure-a", zipName("figure-a")), await writeModelBundle(A, { scales: [100], views: ["thermal"] }));
  });

  it("never deletes the files of other scales: scale 50 then scale 100 into one directory keeps both, byte for byte, and lists both", { timeout: 600_000 }, async () => {
    const out = temporary();
    await exportModels({ producer: PRODUCER, out, catalog, assets: ["figure-a"], scales: [50], views: VIEWS });
    const before = new Map(VIEWS.map((v) => [v, sha(bytesOf(FBX(out, "figure-a", 50, v)))] as const));
    const second = await exportModels({ producer: PRODUCER, out, catalog, assets: ["figure-a"], scales: [100], views: VIEWS });
    for (const view of VIEWS) {
      expect(existsSync(FBX(out, "figure-a", 50, view)), `s50 ${view} survives`).toBe(true);
      expect(sha(bytesOf(FBX(out, "figure-a", 50, view))), `s50 ${view} unchanged`).toBe(before.get(view));
      expect(existsSync(FBX(out, "figure-a", 100, view)), `s100 ${view}`).toBe(true);
    }
    const listed = second.manifest.files.filter((f) => f.kind === "fbx").map((f) => f.scale);
    expect(new Set(listed)).toEqual(new Set([50, 100]));
    expect(second.summary.skipped).toBeGreaterThanOrEqual(2 + 1);
    const third = await exportModels({ producer: PRODUCER, out, catalog, assets: ["figure-a"], views: VIEWS });
    expect(new Set(third.manifest.files.filter((f) => f.kind === "fbx").map((f) => f.scale))).toEqual(new Set([50, 100, 200, 500]));
    for (const view of VIEWS) expect(sha(bytesOf(FBX(out, "figure-a", 50, view))), `s50 ${view} after the full run`).toBe(before.get(view));
    expect(third.summary.written).toBe(2 * 2 * 1 + 1);
  });

  it("keeps the files of other scales even when their key is stale: a changed asset rewrites only the selected scales", { timeout: 600_000 }, async () => {
    const out = temporary();
    const changed = catalogOf([{ ...A, summary: { en: "changed", de: "geändert" } } as Asset]);
    await exportModels({ producer: PRODUCER, out, catalog, assets: ["figure-a"], scales: [50, 100], views: ["light-solid"] });
    const old50 = sha(bytesOf(FBX(out, "figure-a", 50)));
    const run = await exportModels({ producer: PRODUCER, out, catalog: changed, assets: ["figure-a"], scales: [100], views: ["light-solid"] });
    expect(existsSync(FBX(out, "figure-a", 50))).toBe(true);
    expect(sha(bytesOf(FBX(out, "figure-a", 50)))).toBe(old50);
    const files = run.manifest.files;
    const key50 = files.find((f) => f.kind === "fbx" && f.scale === 50)!.key;
    const key100 = files.find((f) => f.kind === "fbx" && f.scale === 100)!.key;
    expect(key50).not.toBe(key100);
  });

  it("produces identical bytes and an identical manifest in two runs", { timeout: 600_000 }, async () => {
    const [first, second] = [temporary(), temporary()];
    await exportModels({ producer: PRODUCER, out: first, catalog, assets: ["figure-a"], scales: [200], views: ["forces"] });
    await exportModels({ producer: PRODUCER, out: second, catalog, assets: ["figure-a"], scales: [200], views: ["forces"] });
    expect(readFileSync(join(first, "manifest.json"), "utf8")).toBe(readFileSync(join(second, "manifest.json"), "utf8"));
    for (const f of manifestOf(first).files) expect(sha(bytesOf(join(first, f.file))), f.file).toBe(sha(bytesOf(join(second, f.file))));
  });

  it("resumes: files whose key and hash match are kept, a damaged file or ZIP is rewritten, force rewrites all", { timeout: 600_000 }, async () => {
    const out = temporary();
    const options = { producer: PRODUCER, out, catalog, assets: ["figure-a"], scales: [50], views: ["light-solid"] } as const;
    const first = await exportModels(options);
    const again = await exportModels(options);
    expect(again.summary.written).toBe(0);
    expect(again.summary.skipped).toBe(first.summary.files);
    const victim = first.manifest.files.find((f) => f.kind === "glb")!;
    writeFileSync(join(out, victim.file), "damaged");
    const repaired = await exportModels(options);
    expect(repaired.summary.written).toBe(1);
    expect(sha(bytesOf(join(out, victim.file)))).toBe(victim.sha256);
    const zip = first.manifest.files.find((f) => f.kind === "zip")!;
    writeFileSync(join(out, zip.file), "damaged");
    expect((await exportModels(options)).summary.written).toBe(1);
    expect(sha(bytesOf(join(out, zip.file)))).toBe(zip.sha256);
    expect((await exportModels({ ...options, force: true })).summary.written).toBe(first.summary.files);
  });

  it("resumes only files of the same producing code: the key holds the producer, the asset, the presets and the thermal options, and the digest reacts to a change of any producing file", { timeout: 600_000 }, async () => {
    const out = temporary();
    const options = { producer: PRODUCER, out, catalog, assets: ["figure-a"], scales: [50], views: ["light-solid"] } as const;
    const first = await exportModels(options);
    expect(first.manifest.settings.producer).toMatch(/^[0-9a-f]{64}$/u);
    expect(new Set([keyOf("p", "s", "x", null, null), keyOf("q", "s", "x", null, null), keyOf("p", "t", "x", null, null), keyOf("p", "s", "y", null, null), keyOf("p", "s", "x", "e", null), keyOf("p", "s", "x", null, 5)]).size).toBe(6);
    const older = { ...manifestOf(out), files: manifestOf(out).files.map((f) => ({ ...f, key: "0".repeat(64) })) };
    writeFileSync(join(out, "manifest.json"), `${JSON.stringify(older, null, 1)}\n`);
    const again = await exportModels(options);
    expect(again.summary.skipped).toBe(0);
    expect(again.summary.written).toBe(first.summary.files);
    const tree = temporary();
    writeFileSync(join(tree, "a.ts"), "export const a = 1;\n");
    writeFileSync(join(tree, "b.json"), "{}\n");
    writeFileSync(join(tree, "ignored.md"), "x");
    const digest = await producerDigest([tree], tree);
    expect(await producerDigest([tree], tree)).toBe(digest);
    const independent = createHash("sha256").update("a.ts\0" + createHash("sha256").update("export const a = 1;\n").digest("hex") + "\n" + "b.json\0" + createHash("sha256").update("{}\n").digest("hex") + "\n").digest("hex");
    expect(digest).toBe(independent);
    writeFileSync(join(tree, "a.ts"), "export const a = 1; \n");
    expect(await producerDigest([tree], tree)).not.toBe(digest);
  });

  it("lists an asset without an authored scale instead of failing, and refuses it when it is named", { timeout: 600_000 }, async () => {
    const out = temporary();
    const run = await exportModels({ producer: PRODUCER, out, catalog, scales: [50], views: ["light-solid"] });
    expect(run.summary.unauthored).toEqual(["figure-c"]);
    expect(run.manifest.assets.map((a) => a.id)).toEqual(["figure-a", "figure-b"]);
    expect(existsSync(join(out, "figure-c"))).toBe(false);
    await expect(exportModels({ producer: PRODUCER, out, catalog, assets: ["figure-c"] })).rejects.toMatchObject({ code: "model-no-authored-scale" });
  });

  it("cancels between and inside files, leaves no temporary file and a manifest of the finished assets, and resumes from them", { timeout: 600_000 }, async () => {
    const out = temporary();
    const controller = new AbortController();
    const run = exportModels({ producer: PRODUCER, out, catalog, assets: ["figure-a", "figure-b"], scales: [200], views: ["light-solid"] }, { signal: controller.signal, progress: (done) => { if (done > 1000) controller.abort(); } });
    await expect(run).rejects.toThrow();
    for (const id of ["figure-a", "figure-b"]) if (existsSync(join(out, id))) expect(readdirSync(join(out, id)).some((n) => n.endsWith(".tmp")), id).toBe(false);
    const partial = manifestOf(out);
    expect(partial.assets.map((a) => a.id)).toEqual(["figure-a"]);
    expect(partial.files.every((f) => f.asset === "figure-a")).toBe(true);
    const finished = await exportModels({ producer: PRODUCER, out, catalog, assets: ["figure-a", "figure-b"], scales: [200], views: ["light-solid"] });
    expect(finished.summary.skipped).toBeGreaterThan(0);
  });

  it("refuses a malformed command line in both languages and names the offending flag", () => {
    expect(() => parseBatchArguments(["--out", "x"])).toThrow(/--lang[\s\S]*--lang/);
    expect(() => parseBatchArguments(["--lang", "en", "--bogus", "1"])).toThrow(/unknown --bogus[\s\S]*unbekanntes --bogus/);
    expect(() => parseBatchArguments(["--lang", "en", "--scale", "0"])).toThrow(/must be among[\s\S]*muss unter/);
    expect(() => parseBatchArguments(["--lang", "en", "--view", "sepia"])).toThrow(/must be among[\s\S]*muss unter/);
    expect(parseBatchArguments(["--lang", "en", "--scale", "100,50", "--view", "thermal,light-solid"])).toMatchObject({ scales: [100, 50], views: ["thermal", "light-solid"] });
    expect(parseBatchArguments(["--lang", "de", "--asset", "a,b", "--force"])).toMatchObject({ language: "de", assets: ["a", "b"], force: true });
  });

  it("stops the emit loops of the export command when the signal aborts", async () => {
    const out = temporary();
    const options = parseArguments(["--asset", "tealight", "--clip", "flicker", "--view", "light-solid", "--viewpoint", "elevation", "--scale", "50", "--fps", "2", "--format", "svg", "--out", out, "--lang", "en"]);
    const controller = new AbortController();
    await expect(runExport(options, undefined, { signal: controller.signal, progress: (done, total, label) => { if (label.en === "Evaluating frames" && done === total) controller.abort(); } })).rejects.toThrow();
    expect(existsSync(out) ? readdirSync(out) : []).toEqual([]);
  });
});

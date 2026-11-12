/** 🧊️ The batch export of the 3D models: the complete set of the catalog as files, with a manifest, deterministic bytes, resumability, cancellation and bounded paths.
 *
 * THE REFERENCES are `node:crypto` SHA-256 of the files read back, `fflate` for the ZIP, the single exports (`writeGlb`, `writeFbx`, `writeModel`, `writeModelBundle`) the batch must equal byte for byte, and the command line of the existing export CLI for the cancellation of its emit loops.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { createHash } from "node:crypto";
import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { unzipSync } from "fflate";
import { afterAll, describe, expect, it } from "vitest";
import { BATCH_SCHEMA, type BatchManifest, MAX_PATH_CHARS, exportModels, parseBatchArguments, plannedNames } from "../../🔨️modules/📤️export/📦️batch/🟦️.ts";
import { producerDigest } from "../../🔨️modules/📤️export/📦️batch/🔏️producer/🟦️.ts";
import { parseArguments, runExport } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { writeFbx } from "../../🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { writeGlb } from "../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { modelExtension, modelStem, writeModel } from "../../🔨️modules/📤️export/🗿️model/🟦️.ts";
import { writeModelBundle } from "../../🔨️modules/📤️export/🗜️zip/📦️models/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";

const IDS = ["tealight", "herring-gull"] as const;
const made: string[] = [];
const temporary = (): string => {
  const directory = mkdtempSync(join(tmpdir(), "staffage-batch-"));
  made.push(directory);
  return directory;
};
afterAll(() => made.forEach((d) => rmSync(d, { recursive: true, force: true })));

const sha = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const manifestOf = (out: string): BatchManifest => JSON.parse(readFileSync(join(out, "manifest.json"), "utf8")) as BatchManifest;
const asset = async (id: string) => (await readStreamedAsset((await assetPathsFromFiles()).find((e) => e.folder === id)!)).asset;

describe("🧊️ the batch export of the 3D models", () => {
  it("writes per asset a GLB per pose, an FBX per clip, the model JSON of each and the ZIP, with a manifest that matches the files", { timeout: 300_000 }, async () => {
    const out = temporary();
    const { manifest, summary } = await exportModels({ out, scale: 50, assets: [...IDS] });
    expect(manifest.$schema).toBe(BATCH_SCHEMA);
    expect(manifestOf(out)).toEqual(manifest);
    for (const id of IDS) {
      const a = await asset(id);
      const names = plannedNames(a, 50, "light-solid").map((n) => n.name);
      expect(readdirSync(join(out, id)).sort()).toEqual([...names].sort());
      expect(manifest.files.filter((f) => f.asset === id).map((f) => f.file.slice(id.length + 1)).sort()).toEqual([...names].sort());
    }
    expect(summary.files).toBe(manifest.files.length);
    expect(manifest.files.map((f) => f.file)).toEqual([...manifest.files.map((f) => f.file)].sort());
    for (const f of manifest.files) {
      const bytes = new Uint8Array(readFileSync(join(out, f.file)));
      expect(bytes.length, f.file).toBe(f.bytes);
      expect(sha(bytes), f.file).toBe(f.sha256);
      expect(join(out, f.file).length, f.file).toBeLessThanOrEqual(MAX_PATH_CHARS);
      if (f.kind === "glb" || f.kind === "fbx") expect(f.triangles, f.file).toBeGreaterThan(0);
    }
    expect(JSON.stringify(manifest)).not.toMatch(/\d{4}-\d{2}-\d{2}T/);
  });

  it("names a pose and a clip of one id apart (herring-gull landing is both) and writes the files the single exports write", { timeout: 300_000 }, async () => {
    const out = temporary();
    await exportModels({ out, scale: 50, assets: ["herring-gull"] });
    const a = await asset("herring-gull");
    const stem = (what: string, view: "all" | "light-solid") => join(out, "herring-gull", `${modelStem("herring-gull", what, view, 50)}`);
    expect(existsSync(`${stem("landing", "all")}.${modelExtension("pose")}`)).toBe(true);
    expect(existsSync(`${stem("landing", "all")}.${modelExtension("clip")}`)).toBe(true);
    const same = (path: string, bytes: Uint8Array | string): void => expect(sha(typeof bytes === "string" ? new TextEncoder().encode(bytes) : bytes), path).toBe(sha(new Uint8Array(readFileSync(path))));
    same(`${stem("landing", "all")}.glb`, await writeGlb(a, { pose: "landing", scale: 50 }));
    same(`${stem("landing", "light-solid")}.fbx`, await writeFbx(a, { clip: "landing", scale: 50 }));
    same(`${stem("landing", "all")}.pose.json`, await writeModel(a, { pose: "landing", scale: 50 }));
    same(`${stem("landing", "all")}.clip.json`, await writeModel(a, { clip: "landing", scale: 50 }));
    const zip = new Uint8Array(readFileSync(join(out, "herring-gull", "herring-gull-all-model-1to50.zip")));
    expect(sha(zip)).toBe(sha(await writeModelBundle(a, { scale: 50 })));
    const members = Object.keys(unzipSync(zip));
    expect(members.length).toBe(plannedNames(a, 50, "light-solid").length - 1);
    expect(new Set(members).size).toBe(members.length);
  });

  it("produces identical bytes and an identical manifest in two runs", { timeout: 300_000 }, async () => {
    const [first, second] = [temporary(), temporary()];
    await exportModels({ out: first, scale: 50, assets: ["tealight"] });
    await exportModels({ out: second, scale: 50, assets: ["tealight"] });
    expect(readFileSync(join(first, "manifest.json"), "utf8")).toBe(readFileSync(join(second, "manifest.json"), "utf8"));
    for (const f of manifestOf(first).files) expect(sha(new Uint8Array(readFileSync(join(first, f.file)))), f.file).toBe(sha(new Uint8Array(readFileSync(join(second, f.file)))));
  });

  it("resumes: files whose hash matches the manifest are kept, a damaged file and the ZIP are rewritten, a changed option rewrites everything", { timeout: 300_000 }, async () => {
    const out = temporary();
    const first = await exportModels({ out, scale: 50, assets: ["tealight"] });
    const again = await exportModels({ out, scale: 50, assets: ["tealight"] });
    expect(again.summary.written).toBe(0);
    expect(again.summary.skipped).toBe(first.summary.files);
    const victim = first.manifest.files.find((f) => f.kind === "glb")!;
    writeFileSync(join(out, victim.file), "damaged");
    const repaired = await exportModels({ out, scale: 50, assets: ["tealight"] });
    expect(repaired.summary.written).toBe(2);
    expect(sha(new Uint8Array(readFileSync(join(out, victim.file))))).toBe(victim.sha256);
    expect((await exportModels({ out, scale: 50, assets: ["tealight"], force: true })).summary.written).toBe(first.summary.files);
    expect((await exportModels({ out, scale: 100, assets: ["tealight"] })).summary.skipped).toBe(0);
  });

  it("resumes only files of the same producing code: an older producer digest rewrites everything, the digest is deterministic and reacts to a change of any producing file", { timeout: 300_000 }, async () => {
    const out = temporary();
    const first = await exportModels({ out, scale: 50, assets: ["tealight"] });
    expect(first.manifest.settings.producer).toMatch(/^[0-9a-f]{64}$/u);
    const older = { ...manifestOf(out), settings: { ...manifestOf(out).settings, producer: "0".repeat(64) } };
    writeFileSync(join(out, "manifest.json"), `${JSON.stringify(older, null, 1)}\n`);
    const again = await exportModels({ out, scale: 50, assets: ["tealight"] });
    expect(again.summary.skipped).toBe(0);
    expect(again.summary.written).toBe(first.summary.files);
    expect(again.manifest.settings.producer).toBe(first.manifest.settings.producer);
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

  it("cancels between and inside files, leaves no temporary file and a manifest of the finished assets", { timeout: 300_000 }, async () => {
    const out = temporary();
    const controller = new AbortController();
    const run = exportModels({ out, scale: 50, assets: [...IDS] }, { signal: controller.signal, progress: (done) => { if (done > 1000) controller.abort(); } });
    await expect(run).rejects.toThrow();
    for (const id of IDS) expect(readdirSync(join(out, id)).some((n) => n.endsWith(".tmp")), id).toBe(false);
    const partial = manifestOf(out);
    expect(partial.assets.map((a) => a.id)).toEqual(["herring-gull"]);
    expect(partial.files.every((f) => f.asset === "herring-gull")).toBe(true);
    const finished = await exportModels({ out, scale: 50, assets: [...IDS] });
    expect(finished.summary.skipped).toBeGreaterThan(0);
  });

  it("refuses a malformed command line in both languages and names the offending flag", () => {
    expect(() => parseBatchArguments(["--out", "x"])).toThrow(/--lang[\s\S]*--lang/);
    expect(() => parseBatchArguments(["--lang", "en", "--bogus", "1"])).toThrow(/unknown --bogus[\s\S]*unbekanntes --bogus/);
    expect(() => parseBatchArguments(["--lang", "en", "--scale", "0"])).toThrow(/positive[\s\S]*positive Zahl/);
    expect(parseBatchArguments(["--lang", "de", "--asset", "a,b", "--force"])).toMatchObject({ language: "de", assets: ["a", "b"], force: true, scale: 50 });
  });

  it("stops the emit loops of the export command when the signal aborts", async () => {
    const out = temporary();
    const options = parseArguments(["--asset", "tealight", "--clip", "flicker", "--view", "light-solid", "--viewpoint", "elevation", "--scale", "50", "--fps", "2", "--format", "svg", "--out", out, "--lang", "en"]);
    const controller = new AbortController();
    await expect(runExport(options, undefined, { signal: controller.signal, progress: (done, total, label) => { if (label.en === "Evaluating frames" && done === total) controller.abort(); } })).rejects.toThrow();
    expect(existsSync(out) ? readdirSync(out) : []).toEqual([]);
  });
});

/** 🧊️ The three exports of the 3D model: a GLB per static pose, an FBX per clip, the neutral JSON model, and the ZIP bundle of all of them; the command line that asks for them and the catalog `exports` that name them.
 *
 * THE REFERENCES are the Khronos `gltf-validator` and three's `GLTFLoader` for every GLB, three's `FBXLoader` for every FBX, ajv with the kernel's JSON Schema for the model document and fflate for the ZIP; none is a runtime dependency.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv from "ajv";
import { unzipSync } from "fflate";
import { describe, expect, it } from "vitest";
import { validateModel, type ModelDocument } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { exportIssues, modelFormats } from "../../🔨️modules/🗂️catalog/🧊️volume/🟦️.ts";
import { DEFAULT_CATALOG_ROOT } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { MODEL_FORMATS, parseArguments, runModelExport } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { writeFbx, writeFbxDetailed } from "../../🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { writeGlb } from "../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { writeModelBundle } from "../../🔨️modules/📤️export/🗜️zip/📦️models/🟦️.ts";
import { fbxName, glbName, modelName, plannedFiles, writeModel, zipName } from "../../🔨️modules/📤️export/🗿️model/🟦️.ts";
import { MODES, ModelRequestError, sceneIdOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { loadFbx, loadGlb, validateGlb } from "../🧰️support/🧊️.ts";
import { scaledFigure as sampleFigure } from "../🧰️support/🧊️volumes/🟦️.ts";

const SCALE = 50;
const asset = sampleFigure();
const poses = asset.poses.filter((p) => p.key !== true).map((p) => p.id);
const schema = JSON.parse(readFileSync(new URL("../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧬️schema/🔣️.json", import.meta.url), "utf8"));
const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
const decode = (bytes: Uint8Array): string => new TextDecoder().decode(bytes);
const BASE = ["--asset", asset.id, "--scale", "50", "--lang", "en", "--out", "out"];

const refused = (args: readonly string[]): ModelRequestError => {
  try {
    parseArguments(args);
  } catch (error) {
    expect(error).toBeInstanceOf(ModelRequestError);
    return error as ModelRequestError;
  }
  throw new Error(`${args.join(" ")} was accepted`);
};

describe("🧊️ GLB, FBX and JSON exports of the 3D model", () => {
  it.each(poses)("writes the GLB of pose %s: Khronos-valid, loadable by three, seven variants, feature edges, joints and tags in the extras", async (pose) => {
    const bytes = await writeGlb(asset, { pose });
    const report = await validateGlb(bytes, `${pose}.glb`);
    expect(report.messages, JSON.stringify(report.messages)).toEqual([]);
    expect(report.errors).toBe(0);
    expect(report.warnings).toBe(0);
    const { scene, parser } = await loadGlb(bytes);
    const json = parser.json;
    expect(json.extensions.KHR_materials_variants.variants.map((v: { name: string }) => v.name)).toEqual(["dark-line", "dark-solid", "light-line", "light-solid", "thermal", "forces", "moments"]);
    expect(json.extensionsUsed).toEqual(expect.arrayContaining(["KHR_materials_variants", "KHR_materials_unlit"]));
    expect(json.meshes.flatMap((m: { primitives: { mode?: number }[] }) => m.primitives).some((p: { mode?: number }) => p.mode === 1)).toBe(true);
    expect(json.scenes.map((s: { name: string }) => s.name)).toEqual([50, 100, 200, 500].flatMap((scale) => MODES.map((mode) => sceneIdOf(scale, mode))));
    expect(json.scene).toBe(0);
    expect(json.skins.length).toBeGreaterThanOrEqual(1);
    expect(json.animations ?? []).toHaveLength(0);
    for (const bone of asset.rig.bones) {
      const node = json.nodes.find((n: { name: string }) => n.name === bone.id);
      expect(node?.extras?.joint?.type, bone.id).toBeDefined();
      expect(scene.getObjectByName(bone.id), bone.id).toBeDefined();
    }
    expect(json.nodes.filter((n: { extras?: { tags?: string[] } }) => n.extras?.tags?.includes("glyph")).length).toBeGreaterThan(0);
  });

  it("writes identical bytes for identical input", async () => {
    const a = await writeGlb(asset, { pose: "stand" });
    const b = await writeGlb(asset, { pose: "stand" });
    expect(Buffer.from(a).equals(Buffer.from(b))).toBe(true);
    const fa = await writeFbx(asset, { clip: "walk", scale: SCALE });
    const fb = await writeFbx(asset, { clip: "walk", scale: SCALE });
    expect(Buffer.from(fa).equals(Buffer.from(fb))).toBe(true);
  });

  it.each(asset.clips.map((c) => c.id))("writes the FBX of clip %s in every one of the seven views and three's FBXLoader loads each", async (clip) => {
    for (const view of ["light-solid", "dark-line", "thermal", "forces", "moments"] as const) {
      const { bytes } = await writeFbxDetailed(asset, { clip, view, scale: SCALE });
      const scene = loadFbx(bytes);
      expect(scene.animations, `${clip} ${view}`).toHaveLength(1);
      expect(scene.animations[0]!.name).toBe(clip);
      expect(scene.getObjectByName("foot-l"), `${clip} ${view}`).toBeDefined();
    }
  });

  it("writes the model document of a pose and of a clip: valid against the kernel schema and the kernel rules, procedural or resolved", async () => {
    const pose = JSON.parse(await writeModel(asset, { pose: "stand" })) as ModelDocument;
    expect(validate(pose), JSON.stringify(validate.errors?.slice(0, 3))).toBe(true);
    expect(validateModel(pose)).toEqual([]);
    expect(pose.solids.length).toBeGreaterThan(17);
    expect(pose.scenes).toHaveLength(12);
    expect(pose.meshes).toEqual([]);
    expect(pose.poses.map((p) => p.id)).toEqual(["stand"]);
    expect(pose.animations).toEqual([]);
    const clip = JSON.parse(await writeModel(asset, { clip: "walk" })) as ModelDocument;
    expect(validate(clip)).toBe(true);
    expect(clip.animations.map((a) => a.id)).toEqual(["walk"]);
    const resolved = JSON.parse(await writeModel(asset, { pose: "stand", resolved: true })) as ModelDocument;
    expect(validate(resolved), JSON.stringify(validate.errors?.slice(0, 3))).toBe(true);
    expect(resolved.solids).toEqual([]);
    expect(resolved.meshes.flatMap((m) => m.primitives).some((p) => p.mode === "lines")).toBe(true);
    expect(await writeModel(asset, { pose: "stand" })).toBe(await writeModel(asset, { pose: "stand" }));
  });

  it("holds an individually authored model per scale: the nose and the buttons are solids of the 1:50 body scene only, and a restricted scale list restricts the scenes", async () => {
    const all = JSON.parse(await writeModel(asset, { pose: "stand" })) as ModelDocument;
    const own = (scene: string): string[] => all.solids.filter((s) => s.id.startsWith(`${scene}-`)).map((s) => s.id.slice(scene.length + 1));
    for (const id of ["nose", "buttons"]) {
      expect(own("s50-body"), id).toContain(id);
      expect(own("s100-body"), id).not.toContain(id);
    }
    const only = JSON.parse(await writeModel(asset, { pose: "stand", scales: [100] })) as ModelDocument;
    expect(only.scenes!.map((s) => s.id)).toEqual(["s100-body", "s100-thermal", "s100-structure"]);
  });

  it("reports monotonic progress from 0 to the end and stops on cancellation without producing bytes", async () => {
    const seen: number[] = [];
    await writeGlb(asset, { pose: "stand" }, { progress: (done, total) => seen.push(done / total) });
    expect(seen.length).toBeGreaterThan(4);
    seen.forEach((v, i) => i > 0 && expect(v).toBeGreaterThanOrEqual(seen[i - 1]! - 1e-9));
    expect(seen[seen.length - 1]).toBe(1);
    for (const run of [() => writeGlb(asset, { pose: "stand" }, control()), () => writeFbx(asset, { clip: "walk", scale: SCALE }, control()), () => writeModel(asset, { pose: "stand", resolved: true }, control()), () => writeModelBundle(asset, { scales: [SCALE], views: ["light-solid"] }, control())]) {
      await expect(run()).rejects.toMatchObject({ name: "AbortError" });
    }
    function control() {
      const controller = new AbortController();
      let calls = 0;
      return { signal: controller.signal, progress: () => void (++calls >= 3 && controller.abort()) };
    }
  });

  it("packs every pose and clip into the bundle: names, members that validate, load and parse", async () => {
    const zip = unzipSync(await writeModelBundle(asset, { scales: [SCALE, 500], views: ["light-solid", "thermal"] }));
    const names = Object.keys(zip).sort();
    expect(names).toEqual([
      ...asset.clips.flatMap((c) => [SCALE, 500].flatMap((scale) => (["light-solid", "thermal"] as const).map((view) => `fbx/${fbxName(asset.id, c.id, scale, view)}`))),
      ...poses.map((p) => `glb/${glbName(asset.id, p)}`),
      ...poses.map((p) => `model/${modelName(asset.id, "pose", p)}`),
      ...asset.clips.map((c) => `model/${modelName(asset.id, "clip", c.id)}`),
    ].sort());
    expect(names).toEqual(plannedFiles(asset, { scales: [SCALE, 500], views: ["light-solid", "thermal"] }).filter((f) => f.kind !== "zip").map((f) => `${f.kind}/${f.name}`).sort());
    for (const name of names) {
      if (name.endsWith(".glb")) expect((await validateGlb(zip[name]!, name)).errors, name).toBe(0);
      else if (name.endsWith(".fbx")) expect(loadFbx(zip[name]!).animations, name).toHaveLength(1);
      else expect(validate(JSON.parse(decode(zip[name]!))), name).toBe(true);
    }
  });

  it("refuses an asset without volume, an unknown pose and an unknown clip by name, in both languages", async () => {
    const bare = { ...asset, volumes: Object.fromEntries([50, 100, 200, 500].map((s) => [`${s}`, { status: "unavailable", denominator: s }])) } as unknown as typeof asset;
    await expect(writeGlb(bare, { pose: "stand" })).rejects.toMatchObject({ name: "ModelRequestError", code: "model-no-authored-scale" });
    await expect(writeGlb(asset, { pose: "no-such-pose" })).rejects.toMatchObject({ code: "model-unknown-pose" });
    await expect(writeFbx(asset, { clip: "no-such-clip", scale: SCALE })).rejects.toMatchObject({ code: "model-unknown-clip" });
    await expect(writeFbx(asset, { clip: "walk", scale: 77 })).rejects.toMatchObject({ code: "model-scale-unknown" });
    await expect(writeFbx(bare, { clip: "walk", scale: 100 })).rejects.toMatchObject({ code: "model-scale-unavailable" });
    await expect(writeGlb(bare, { pose: "stand", scales: [100] })).rejects.toMatchObject({ code: "model-scale-unavailable" });
    await expect(writeModel(asset, {})).rejects.toMatchObject({ code: "model-needs-one" });
    const error = await writeModelBundle(bare).catch((e: ModelRequestError) => e);
    expect((error as ModelRequestError).reason.en.length).toBeGreaterThan(10);
    expect((error as ModelRequestError).reason.de.length).toBeGreaterThan(10);
    expect((error as Error).message).toContain((error as ModelRequestError).reason.en);
    expect((error as Error).message).toContain((error as ModelRequestError).reason.de);
  });

  it("parses the command lines of the four 3D formats and names every nonsensical combination", () => {
    expect([...MODEL_FORMATS]).toEqual(["glb", "fbx", "model", "model-zip"]);
    expect(parseArguments([...BASE, "--format", "glb", "--pose", "stand"])).toMatchObject({ format: "glb", pose: "stand", view: "light-solid" });
    expect(parseArguments([...BASE, "--format", "fbx", "--clip", "walk", "--view", "thermal", "--environment", "temperate-shade", "--span-k", "12"])).toMatchObject({ format: "fbx", clip: "walk", view: "thermal", spanK: 12 });
    expect(parseArguments([...BASE, "--format", "model", "--clip", "walk"])).toMatchObject({ format: "model", clip: "walk" });
    expect(parseArguments([...BASE, "--format", "model-zip"])).toMatchObject({ format: "model-zip" });
    const cases: [string, readonly string[]][] = [
      ["glb-needs-pose", ["--format", "glb", "--clip", "walk"]],
      ["glb-needs-pose", ["--format", "glb"]],
      ["fbx-needs-clip", ["--format", "fbx", "--pose", "stand"]],
      ["model-needs-one", ["--format", "model"]],
      ["model-needs-one", ["--format", "model", "--pose", "stand", "--clip", "walk"]],
      ["bundle-takes-neither", ["--format", "model-zip", "--pose", "stand"]],
      ["model-no-viewpoint", ["--format", "glb", "--pose", "stand", "--viewpoint", "elevation"]],
      ["model-no-viewpoint", ["--format", "fbx", "--clip", "walk", "--projection", "third"]],
      ["model-no-time", ["--format", "fbx", "--clip", "walk", "--time", "0.5"]],
      ["model-no-time", ["--format", "fbx", "--clip", "walk", "--fps", "10"]],
      ["model-all-views", ["--format", "glb", "--pose", "stand", "--view", "thermal"]],
      ["model-all-views", ["--format", "model", "--pose", "stand", "--view", "dark-line"]],
      ["model-all-views", ["--format", "model-zip", "--view", "thermal"]],
      ["model-environment-view", ["--format", "fbx", "--clip", "walk", "--environment", "temperate-shade"]],
    ];
    for (const [code, args] of cases) {
      const error = refused([...BASE, ...args]);
      expect(error.code, args.join(" ")).toBe(code);
      expect(error.reason.en).not.toBe(error.reason.de);
    }
    expect(() => parseArguments([...BASE, "--format", "svg", "--pose", "stand", "--view", "dark-line", "--viewpoint", "elevation", "--span-k", "5"])).toThrow(/--span-k/);
  });

  it("writes the files of the commands with the names of the file set: asset, pose or clip, scale and view", async () => {
    const out = mkdtempSync(join(tmpdir(), "staffage-model-"));
    try {
      const run = async (args: readonly string[]): Promise<string[]> => {
        const options = parseArguments([...BASE.slice(0, -1), out, ...args, "--catalog", DEFAULT_CATALOG_ROOT]);
        return [...(await runModelExport(options, options.format as never, asset))];
      };
      const [glb] = await run(["--format", "glb", "--pose", "stand"]);
      const [fbx] = await run(["--format", "fbx", "--clip", "walk"]);
      const [json] = await run(["--format", "model", "--pose", "stand"]);
      const [zip] = await run(["--format", "model-zip", "--scale", "100"]);
      expect(glb).toBe(join(out, glbName(asset.id, "stand")));
      expect(fbx).toBe(join(out, fbxName(asset.id, "walk", 50, "light-solid")));
      expect(json).toBe(join(out, modelName(asset.id, "pose", "stand")));
      expect(zip).toBe(join(out, zipName(asset.id)));
      expect((await validateGlb(new Uint8Array(readFileSync(glb!)))).errors).toBe(0);
      expect(loadFbx(new Uint8Array(readFileSync(fbx!))).animations).toHaveLength(1);
      expect(validate(JSON.parse(readFileSync(json!, "utf8")))).toBe(true);
      expect(Object.keys(unzipSync(new Uint8Array(readFileSync(zip!)))).length).toBe(poses.length * 2 + asset.clips.length * (1 + 7));
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  });

  it("states in the catalog which 3D formats an asset can produce and judges the exports that name a format it cannot", () => {
    expect(modelFormats(asset)).toEqual(["glb", "fbx", "model", "model-zip"]);
    expect(modelFormats({ ...asset, clips: [] })).toEqual(["glb", "model", "model-zip"]);
    const bare = { ...asset, volumes: Object.fromEntries([50, 100, 200, 500].map((s) => [`${s}`, { status: "unavailable", denominator: s }])) } as unknown as typeof asset;
    expect(modelFormats(bare)).toEqual([]);
    expect(exportIssues({ ...asset, exports: ["svg", "glb", "fbx", "model", "model-zip"] })).toEqual([]);
    expect(exportIssues({ ...asset, clips: [], exports: ["fbx"] }).map((i) => i.code)).toEqual(["exports-fbx-clip"]);
    expect(exportIssues({ ...bare, exports: ["svg", "glb", "model"] }).map((i) => i.code)).toEqual(["exports-model-volume", "exports-model-volume"]);
    expect(exportIssues({ ...asset, poses: [], exports: ["glb"] }).map((i) => i.code)).toEqual(["exports-glb-pose"]);
  });
});

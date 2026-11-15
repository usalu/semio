/** 🧊️ One individually authored 3D model per scale and mode: `modelOf` reads `volumes[scale]`, tone views from `body`, `thermal` from the thermal solids, `forces` and `moments` from the structure solids; `modelOfAsset` builds one document with a kernel scene per authored scale and mode over one skeleton; an unavailable or unknown scale is a typed refusal, never a fallback.
 *
 * THE REFERENCES are the kernel validator and Ajv with the kernel JSON Schema for the documents, the Khronos `gltf-validator` and three's `GLTFLoader` for the GLB with its scenes, and the 2D depiction (`depict`) for the colours of the thermal and structural views.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { scenesOf, tessellate, validateModel, type ModelDocument } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { View } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { VIEWS } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { MODES, ModelRequestError, VIEWS_OF, authoredScales, modelOf, modelOfAsset, modeOf, sceneIdOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { qualityOf } from "../../🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { scaledFigure } from "../🧰️support/🧊️volumes/🟦️.ts";

const asset = scaledFigure();
const SCALES = [50, 100, 200, 500] as const;
const schema = JSON.parse(readFileSync(new URL("../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧬️schema/🔣️.json", import.meta.url), "utf8"));
const validateSchema = new Ajv({ strict: false, allErrors: true }).compile(schema);
const errorsOf = (document: ModelDocument): string[] => validateModel(document).filter((i) => i.severity === "error").map((i) => `${i.code} ${i.path}`);
const solidCount = (document: ModelDocument, scene: string): number => document.solids.filter((s) => s.id.startsWith(`${scene}-`) && s.tags?.includes("glyph") !== true).length;

describe("🧊️ one model per scale and mode", () => {
  it("authors every scale of the fixture", () => expect(authoredScales(asset.volumes)).toEqual([50, 100, 200, 500]));

  it.each(SCALES.flatMap((scale) => VIEWS.map((view) => [scale, view] as const)))("1:%i %s: modelOf is a valid kernel document of the mode's own list", async (scale, view) => {
    const document = await modelOf(asset, { scale, view });
    expect(errorsOf(document)).toEqual([]);
    expect(validateSchema(document), JSON.stringify(validateSchema.errors)).toBe(true);
    expect(document.scenes).toBeUndefined();
    const list = asset.volumes[`${scale}` as "50"];
    if (list.status !== "authored") throw new Error("authored");
    const own = document.solids.filter((s) => s.tags?.includes("glyph") !== true).map((s) => s.id).sort();
    expect(own).toEqual(list[modeOf(view)].map((s) => s.id).sort());
    expect(document.variants.map((v) => v.id)).toEqual([...VIEWS_OF[modeOf(view)]]);
  });

  it("the three lists differ in geometry at one scale and the scales differ in solid count", async () => {
    const counts = new Map<string, number>();
    for (const scale of SCALES) for (const mode of MODES) counts.set(`${scale}-${mode}`, (await modelOf(asset, { scale, view: VIEWS_OF[mode][0]! })).solids.filter((s) => s.tags?.includes("glyph") !== true).length);
    expect(counts.get("50-body")).toBeGreaterThan(counts.get("100-body")!);
    expect(counts.get("100-body")).toBeGreaterThan(counts.get("200-body")!);
    expect(counts.get("50-structure")).toBeGreaterThan(counts.get("500-structure")!);
    expect(counts.get("50-structure")).not.toBe(counts.get("50-body"));
  });

  it("a thermal solid that references a body solid shares its geometry by construction", async () => {
    const body = await modelOf(asset, { scale: 50, view: "light-solid" });
    const thermal = await modelOf(asset, { scale: 50, view: "thermal" });
    const bodyHead = body.solids.find((s) => s.id === "head")!;
    const thermalHead = thermal.solids.find((s) => s.id === "t-head")!;
    expect({ ...thermalHead, id: "", material: "", tags: [] }).toEqual({ ...bodyHead, id: "", material: "", tags: [] });
    expect(thermalHead.tags).toEqual(expect.arrayContaining(["region-head"]));
  });

  it("the thermal model has one material per region and layer, painted from the field", async () => {
    const document = await modelOf(asset, { scale: 50, view: "thermal" });
    const variant = document.variants.find((v) => v.id === "thermal")!;
    const used = new Set(Object.values(variant.bindings));
    const titles = new Set([...used].map((id) => document.materials.find((m) => m.id === id)!.title.en));
    for (const region of asset.thermal.regions) expect([...titles].some((title) => title.startsWith(region.title.en)), `a material for region ${region.id}`).toBe(true);
    const colours = new Set([...used].map((id) => document.materials.find((m) => m.id === id)!.baseColor.slice(0, 3).join(",")));
    expect(colours.size).toBeGreaterThan(2);
  });

  it("the structure model colours member solids by their member's value and the joint solids like their member", async () => {
    const document = await modelOf(asset, { scale: 50, view: "forces" });
    const bind = document.variants.find((v) => v.id === "forces")!.bindings;
    const colourOf = (solid: string): string => document.materials.find((m) => m.id === bind[document.solids.find((s) => s.id === solid)!.material])!.baseColor.join(",");
    expect(colourOf("j-hip-l")).toBe(colourOf("m-thigh-l"));
    expect(document.solids.some((s) => s.tags?.includes("glyph"))).toBe(true);
  });

  it("refuses an unknown scale and an unavailable scale by name, never answers with another scale", async () => {
    await expect(modelOf(asset, { scale: 77, view: "light-solid" })).rejects.toMatchObject({ code: "model-scale-unknown" });
    const unavailable = { ...asset, volumes: { ...asset.volumes, "200": { status: "unavailable", denominator: 200 } } } as typeof asset;
    await expect(modelOf(unavailable, { scale: 200, view: "thermal" })).rejects.toBeInstanceOf(ModelRequestError);
    await expect(modelOf(unavailable, { scale: 200, view: "thermal" })).rejects.toMatchObject({ code: "model-scale-unavailable" });
    await expect(modelOfAsset(unavailable, { scales: [200] })).rejects.toMatchObject({ code: "model-scale-unavailable" });
    const none = { ...asset, volumes: Object.fromEntries(SCALES.map((s) => [`${s}`, { status: "unavailable", denominator: s }])) } as unknown as typeof asset;
    await expect(modelOfAsset(none)).rejects.toMatchObject({ code: "model-no-authored-scale" });
  });

  it("modelOfAsset holds one scene per authored scale and mode over one skeleton and skips an unavailable scale", async () => {
    const full = await modelOfAsset(asset);
    expect(errorsOf(full)).toEqual([]);
    expect(validateSchema(full), JSON.stringify(validateSchema.errors)).toBe(true);
    expect(full.scenes!.map((s) => s.id)).toEqual(SCALES.flatMap((scale) => MODES.map((mode) => sceneIdOf(scale, mode))));
    expect(full.variants.map((v) => v.id)).toEqual([...VIEWS]);
    expect(full.nodes.filter((n) => n.tags?.includes("bone"))).toHaveLength(asset.rig.bones.length);
    for (const scale of SCALES) expect(solidCount(full, `s${scale}-body`)).toBe((await modelOf(asset, { scale, view: "light-solid" })).solids.filter((s) => s.tags?.includes("glyph") !== true).length);
    const partial = await modelOfAsset({ ...asset, volumes: { ...asset.volumes, "100": { status: "unavailable", denominator: 100 } } } as typeof asset);
    expect(partial.scenes!.map((s) => s.id)).not.toContain("s100-body");
    expect(partial.scenes).toHaveLength(9);
    expect(scenesOf(partial).map((s) => s.id)).toEqual(partial.scenes!.map((s) => s.id));
  });

  it("every scene tessellates on its own and its triangle count does not grow towards coarser scales", async () => {
    const full = await modelOfAsset(asset, { poses: [asset.presentation.defaultPose], clips: [] });
    const resolved = await tessellate(full, qualityOf(asset), { report: () => {}, cancelled: () => false });
    expect(errorsOf(resolved)).toEqual([]);
    const trianglesOf = async (scene: string): Promise<number> => {
      const only = await tessellate(full, qualityOf(asset), { report: () => {}, cancelled: () => false }, { scene });
      return only.meshes.reduce((n, m) => n + m.primitives.reduce((k, p) => k + (p.mode === "triangles" ? p.indices.length / 3 : 0), 0), 0);
    };
    const body = await Promise.all(SCALES.map((scale) => trianglesOf(sceneIdOf(scale, "body"))));
    for (let i = 1; i < body.length; i += 1) expect(body[i]!, `1:${SCALES[i]} body`).toBeLessThanOrEqual(body[i - 1]!);
    expect(body[3]!).toBeLessThan(body[0]!);
  });

  it("modelOfAsset is deterministic and cancellable", async () => {
    const a = JSON.stringify(await modelOfAsset(asset, { poses: [asset.presentation.defaultPose], clips: [] }));
    const b = JSON.stringify(await modelOfAsset(asset, { poses: [asset.presentation.defaultPose], clips: [] }));
    expect(a).toBe(b);
    let polled = 0;
    await expect(modelOfAsset(asset, {}, { report: () => {}, cancelled: () => ++polled > 3 })).rejects.toThrow();
    expect(polled).toBeGreaterThan(3);
  });

  it("view and mode agree: tone views live in body, thermal in thermal, forces and moments in structure", () => {
    const expected: Record<View, string> = { "dark-line": "body", "dark-solid": "body", "light-line": "body", "light-solid": "body", thermal: "thermal", forces: "structure", moments: "structure" };
    for (const view of VIEWS) expect(modeOf(view)).toBe(expected[view]);
  });
});

/** 🧊️ The seven views as looks of one geometry: the materials of the seven variants against the colours the 2D depiction paints for the same shapes, the animated colour channels against the 2D views at the same instants, and the glyph sizes against the 2D arrows and curls.
 *
 * THE REFERENCES are the 2D depiction itself (`depict`, items `shape:<id>`, `force:*`, `moment:*`) and three's colour management (`Color.setHex(hex, SRGBColorSpace)`) for the sRGB to linear conversion.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { Color, Quaternion, SRGBColorSpace, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import { poseModel, sampleMaterials, type Material, type ModelDocument } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Asset, AuthoredVolume } from "../../🧬️schema/🟦️.ts";
import type { DrawingItem, DrawingScene, View } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";
import { solveThermalAt } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import { modelOf, frameTimes, modeOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { frameDataOf, glyphOf, Slots, type FrameSetup } from "../../🔨️modules/🧊️volume/🧷️frames/🟦️.ts";
import { HIDDEN, paperSizing } from "../../🔨️modules/🧊️volume/🪧️glyphs/🟦️.ts";
import { scaledFigure } from "../🧰️support/🧊️volumes/🟦️.ts";

const SCALE = 50;
const MM = SCALE / 1000;
const selector = { kind: "scale", scale: { denominator: SCALE } } as const;
const near = (a: readonly number[], b: readonly number[], eps: number, what: string): void => void a.forEach((v, i) => expect(Math.abs(v - b[i]!), `${what}: ${a} vs ${b}`).toBeLessThanOrEqual(eps));

const linear = (hex: string): number[] => {
  const c = new Color().setHex(Number.parseInt(hex.slice(1), 16), SRGBColorSpace);
  return [c.r, c.g, c.b];
};

type Look = { readonly rgb: number[]; readonly alpha: number };
const lookOf = (document: ModelDocument, view: View, solid: string): Look => {
  const asset = document.solids.find((s) => s.id === solid)!;
  const bound = document.variants.find((v) => v.id === view)!.bindings[asset.material]!;
  const material = document.materials.find((m) => m.id === bound)! as Material;
  return { rgb: [...material.baseColor.slice(0, 3)], alpha: material.baseColor[3]! };
};

const itemsOf = (scene: DrawingScene): Map<string, Extract<DrawingItem, { kind: "path" }>> => new Map(scene.layers.flatMap((l) => l.items).flatMap((i) => (i.kind === "path" ? [[i.id, i] as const] : [])));

function scene2d(asset: Asset, view: View, pose: string): { scene: DrawingScene; shapes: Map<string, Extract<DrawingItem, { kind: "path" }>> } {
  const query = { kind: "pose", pose } as const;
  const state = evaluate(asset, query, selector);
  const options = defaultDepictionOptions(view, "en", SCALE, "elevation");
  const analysis = view === "thermal" ? { thermal: solveThermalAt(asset, query) } : view === "forces" || view === "moments" ? { structure: solveStructure(state) } : {};
  const scene = depict(state, options, analysis);
  return { scene, shapes: itemsOf(scene) };
}

type Shape2d = { readonly id: string; readonly bone: string; readonly tone: number; readonly region?: string; readonly regions?: readonly string[] };
const shapes2d = (asset: Asset): readonly Shape2d[] => ((asset.representations[`${SCALE}` as "50"] as unknown as { geometry: { elevation: Shape2d[] } }).geometry.elevation);
const tag = (tags: readonly string[] | undefined, prefix: string): string | undefined => tags?.find((t) => t.startsWith(prefix))?.slice(prefix.length);

/** 🔗️ Every solid of the model of a view with the 2D shape that paints the same thing in the 2D view of the same scale: the shape of the same bone and tone for the tone views, of the same thermal region for the thermal view, of the same member bone for the force and moment views (support and mass solids are neutral and have none). Each scale authors its own shapes and solids, so ids are not compared, what they paint is. */
function pairs(asset: Asset, document: ModelDocument, view: View, scene: DrawingScene): { readonly solid: { readonly id: string }; readonly item: Extract<DrawingItem, { kind: "path" }> }[] {
  const items = itemsOf(scene);
  const out: { solid: { id: string }; item: Extract<DrawingItem, { kind: "path" }> }[] = [];
  for (const solid of document.solids) {
    if (solid.tags?.includes("glyph")) continue;
    const region = tag(solid.tags, "region-");
    const member = tag(solid.tags, "member-");
    const part = tag(solid.tags, "part-");
    const tone = (asset.volumes[`${SCALE}` as "50"] as AuthoredVolume).body;
    const match = shapes2d(asset).find((s) => {
      if (view === "thermal") return region !== undefined && (s.region === region || s.regions?.includes(region) === true);
      if (view === "forces" || view === "moments") return member !== undefined && (part === "member" || part === "joint") && s.bone === member;
      const own = tone.find((b) => b.id === solid.id);
      return own !== undefined && s.tone === own.tone && s.bone === solid.node;
    });
    const item = match === undefined ? undefined : items.get(`shape:${match.id}`);
    if (item?.fill !== undefined) out.push({ solid, item });
  }
  return out;
}

/** 🎨️ The colours of every solid that a 2D shape paints the same thing as equal the model's, within 1e-6 in linear sRGB and opacity; returns how many solids were compared. */
function compare(asset: Asset, document: ModelDocument, view: View, pose: string): number {
  const { scene } = scene2d(asset, view, pose);
  let compared = 0;
  for (const { solid, item } of pairs(asset, document, view, scene)) {
    const look = lookOf(document, view, solid.id);
    near(look.rgb, linear(item.fill!.color), 1e-6, `${view} ${solid.id} ${item.fill!.color}`);
    if (view !== "thermal") expect(look.alpha, `${view} ${solid.id} opacity`).toBeCloseTo(item.fill!.opacity ?? 1, 9);
    compared += 1;
  }
  return compared;
}

const extentOf = (item: Extract<DrawingItem, { kind: "path" }>): { width: number; height: number } => {
  const xs = item.commands.flatMap((c) => ("x" in c ? [c.x] : []));
  const ys = item.commands.flatMap((c) => ("y" in c ? [c.y] : []));
  return { width: Math.max(...xs) - Math.min(...xs), height: Math.max(...ys) - Math.min(...ys) };
};

describe("🧊️ the seven views as looks of one geometry", () => {
  const asset = scaledFigure();

  it("has the views of each mode as variants over the geometry of that mode, every base material bound in each, and the base look equal to light-solid", async () => {
    const expected = { body: ["dark-line", "dark-solid", "light-line", "light-solid"], thermal: ["thermal"], structure: ["forces", "moments"] } as const;
    for (const [mode, views] of Object.entries(expected)) {
      const document = await modelOf(asset, { scale: SCALE, view: views[0], poses: [], clips: [] });
      expect(document.variants.map((v) => v.id), mode).toEqual(views);
      const surfaces = new Set(document.solids.filter((s) => !s.tags?.includes("glyph")).map((s) => s.material));
      for (const variant of document.variants) for (const id of surfaces) expect(variant.bindings[id], `${variant.id} binds ${id}`).toBeDefined();
    }
    const body = await modelOf(asset, { scale: SCALE, view: "light-solid", poses: [], clips: [] });
    for (const solid of body.solids) {
      const base = body.materials.find((m) => m.id === solid.material)!;
      near(base.baseColor.slice(0, 3), lookOf(body, "light-solid", solid.id).rgb, 1e-12, `base ${solid.id}`);
    }
  });

  it.each(["dark-solid", "light-solid", "dark-line", "light-line"] as const)("paints %s like the 2D view: tone ramp, paper, glazing", async (view) => {
    const document = await modelOf(asset, { scale: SCALE, view, poses: [], clips: [] });
    expect(compare(asset, document, view, "stand")).toBeGreaterThanOrEqual(8);
    expect(document.materials.filter((m) => m.id.startsWith(`${view}-`) && !m.id.endsWith("edge")).every((m) => m.unlit === view.endsWith("-line"))).toBe(true);
  });

  it("paints the thermal view with the 2D thermal colours of the same regions under the default range and the span of a scene", async () => {
    for (const pose of ["stand", "carry-bag"]) {
      const document = await modelOf(asset, { scale: SCALE, view: "thermal", at: { kind: "pose", pose }, poses: [pose], clips: [] });
      expect(compare(asset, document, "thermal", pose)).toBeGreaterThanOrEqual(8);
    }
    const spanned = await modelOf(asset, { scale: SCALE, view: "thermal", thermalSpanK: 12, poses: [], clips: [] });
    const query = { kind: "pose", pose: asset.poses[0]!.id } as const;
    const state = evaluate(asset, query, selector);
    const scene = depict(state, { ...defaultDepictionOptions("thermal", "en", SCALE, "elevation"), range: { mode: "span", spanK: 12 } }, { thermal: solveThermalAt(asset, query) });
    let paired = 0;
    for (const { solid, item } of pairs(asset, spanned, "thermal", scene)) {
      near(lookOf(spanned, "thermal", solid.id).rgb, linear(item.fill!.color), 1e-6, `span ${solid.id}`);
      paired += 1;
    }
    expect(paired).toBeGreaterThanOrEqual(5);
  });

  it("solves the thermal view in the environment the caller names, as the 2D view does", async () => {
    const environment = { ...asset.thermal.environment, airTempC: 31, meanRadiantTempC: 33, solarWM2: 650, windMS: 0.2 };
    const document = await modelOf(asset, { scale: SCALE, view: "thermal", environment, poses: [], clips: [] });
    const query = { kind: "pose", pose: asset.poses[0]!.id } as const;
    const state = evaluate(asset, query, selector);
    const scene = depict(state, { ...defaultDepictionOptions("thermal", "en", SCALE, "elevation"), environment }, { thermal: solveThermalAt(asset, query, environment) });
    const own = await modelOf(asset, { scale: SCALE, view: "thermal", poses: [], clips: [] });
    let differs = 0;
    for (const { solid, item } of pairs(asset, document, "thermal", scene)) {
      near(lookOf(document, "thermal", solid.id).rgb, linear(item.fill!.color), 1e-6, `environment ${solid.id}`);
      if (Math.abs(lookOf(own, "thermal", solid.id).rgb[0]! - lookOf(document, "thermal", solid.id).rgb[0]!) > 1e-3) differs += 1;
    }
    expect(differs, "the environment changes the colours").toBeGreaterThan(0);
  });

  it.each(["forces", "moments"] as const)("paints the %s view with the 2D member colours of the same bones", async (view) => {
    const document = await modelOf(asset, { scale: SCALE, view, at: { kind: "pose", pose: "carry-bag" }, poses: ["carry-bag"], clips: [] });
    expect(compare(asset, document, view, "carry-bag")).toBeGreaterThanOrEqual(8);
  });

  it("animates the analytic colours: at every sampled instant the material colours equal the 2D colours of the same instant", async () => {
    const clip = asset.clips.find((c) => c.id === "walk")!;
    for (const view of ["thermal", "forces", "moments"] as const) {
      const document = await modelOf(asset, { scale: SCALE, view, poses: [], clips: ["walk"] });
      expect(document.animations[0]!.channels.some((c) => "material" in c.target), view).toBe(true);
      for (const timeS of [0, 0.2, 0.5, 0.8]) {
        const sampled = sampleMaterials(document, "walk", timeS);
        const query = { kind: "clip", clip: "walk", timeS } as const;
        const state = evaluate(asset, query, selector);
        const scene = depict(state, defaultDepictionOptions(view, "en", SCALE, "elevation"), view === "thermal" ? { thermal: solveThermalAt(asset, query) } : { structure: solveStructure(state) });
        let paired = 0;
        for (const { solid, item } of pairs(asset, document, view, scene)) {
          const bound = document.variants.find((v) => v.id === view)!.bindings[document.solids.find((s) => s.id === solid.id)!.material]!;
          const colour = sampled[bound]?.baseColor ?? [...document.materials.find((m) => m.id === bound)!.baseColor];
          near(colour.slice(0, 3), linear(item.fill!.color), 2e-3, `${view} ${solid.id} @${timeS}`);
          paired += 1;
        }
        expect(paired, `${view} @${timeS}`).toBeGreaterThanOrEqual(5);
      }
    }
    expect(frameTimes(clip)).toHaveLength(31);
  });

  it("sizes the force arrows by the 2D rule capped by amendment D1: the vertical arrows measure the paper length of the drawn ones or 0.35 of the largest extent of the asset, whichever is smaller, tip on the application point, pointing along the force", async () => {
    const pose = "carry-bag";
    const document = await modelOf(asset, { scale: SCALE, view: "forces", at: { kind: "pose", pose }, poses: [pose], clips: [] });
    const { shapes } = scene2d(asset, "forces", pose);
    const replay = poseModel(document, pose);
    let compared = 0;
    for (const [id, item] of shapes) {
      if (!id.startsWith("force:") || id.endsWith(":out") || id.endsWith(":label")) continue;
      const root = document.nodes.find((n) => n.id === `arrow-${id.toLowerCase().replace(/[^a-z0-9]+/gu, "-").replace(/^-+|-+$/gu, "")}`);
      if (root === undefined) continue;
      const head = document.nodes.find((n) => n.id === `${root.id}-head`)!;
      const shaft = document.nodes.find((n) => n.id === `${root.id}-shaft`)!;
      const base = -(document.solids.find((s) => s.node === head.id)! as unknown as { profile: [number, number][] }).profile[0]![0];
      const lengthM = shaft.scale[1] + head.scale[0] * base;
      const drawn = extentOf(item);
      if (drawn.width > 1.4) continue;
      expect(lengthM, id).toBeCloseTo(Math.min(drawn.height * MM, 0.35 * Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM)), 6);
      const direction = new Vector3(0, 1, 0).applyQuaternion(new Quaternion(...(root.rotation as [number, number, number, number])));
      expect(Math.abs(direction.x)).toBeLessThan(1e-9);
      expect(Math.abs(direction.y)).toBeCloseTo(1, 9);
      expect(replay.world[root.id]).toBeDefined();
      compared += 1;
    }
    expect(compared).toBeGreaterThanOrEqual(2);
  });

  it("sizes the moment arcs by the 2D rule capped by amendment D1: a planar moment has the paper radius 1.6 + 3.2 sqrt(M / max) mm of the drawn curl or half of 0.35 of the largest extent, whichever is smaller", async () => {
    const pose = "carry-bag";
    const document = await modelOf(asset, { scale: SCALE, view: "moments", at: { kind: "pose", pose }, poses: [pose], clips: [] });
    const { shapes } = scene2d(asset, "moments", pose);
    let compared = 0;
    for (const [id, item] of shapes) {
      if (!id.startsWith("moment:") || id.endsWith(":head") || id.endsWith(":vector") || id.endsWith(":label")) continue;
      const root = document.nodes.find((n) => n.id === `arc-${id.toLowerCase().replace(/[^a-z0-9]+/gu, "-").replace(/^-+|-+$/gu, "")}`);
      if (root === undefined) continue;
      const drawn = extentOf(item);
      expect(root.scale[0], id).toBeCloseTo(Math.min((drawn.width / 2) * MM, 0.175 * Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM)), 6);
      compared += 1;
    }
    expect(compared).toBeGreaterThanOrEqual(2);
  });

  it("omits an unloaded joint arc while preserving activation and release over sampled frames", () => {
    const state = evaluate(asset, { kind: "pose", pose: "carry-bag" }, selector);
    const result = solveStructure(state);
    const baseline = Object.values(result.joints)[0]!;
    const slots = new Slots();
    const setup: FrameSetup = { asset, slots, views: ["moments"], theme: "light", mm: MM, longestMm: 12, looks: { fixed: {}, thermal: undefined, forces: undefined, moments: undefined } };
    const frames = [0, 5, 0].map((load) => {
      const joint = (id: string, z: number) => ({ ...baseline, bone: id, mxNm: 0, myNm: 0, mzNm: z });
      const structure = { ...result, joints: { unloaded: joint("unloaded", 0), loaded: joint("loaded", load) }, reactions: [], applied: [] };
      return frameDataOf(setup, state, { structure });
    });
    expect(slots.order).toEqual(["arc-moment-joint-loaded"]);
    const root = slots.order[0]!;
    const geometry = glyphOf(root, slots.map.get(root)!, paperSizing(MM), frames[0]!.glyphs);
    expect(geometry.nodes.find((n) => n.id === root)!.scale).toEqual(HIDDEN);
    const lengths = frames.map((f) => new Vector3().fromArray(f.glyphs.get(root)?.scale ?? HIDDEN).length());
    expect(lengths[1]).toBeGreaterThan(lengths[0]! * 100);
    expect(lengths[2]).toBe(lengths[0]);
  });

  it("tags every glyph and hides it by default: glyph base materials are fully transparent, only their own view shows them", async () => {
    const document = await modelOf(asset, { scale: SCALE, view: "forces", at: { kind: "pose", pose: "carry-bag" }, poses: [], clips: [] });
    const glyphs = document.solids.filter((s) => s.tags?.includes("glyph"));
    expect(glyphs.length).toBeGreaterThan(4);
    for (const solid of glyphs) {
      const base = document.materials.find((m) => m.id === solid.material)!;
      expect(base.baseColor[3]).toBe(0);
      const shown = document.variants.filter((v) => v.bindings[solid.material] !== undefined).map((v) => v.id);
      expect(shown.length, solid.id).toBe(1);
      expect(["forces", "moments", "thermal"]).toContain(shown[0]);
    }
    const glyphNodes = document.nodes.filter((n) => n.tags?.includes("glyph"));
    for (const node of glyphNodes) expect(node.parent === null || glyphNodes.some((p) => p.id === node.parent), node.id).toBe(true);
    expect(glyphNodes.some((n) => n.parent === null)).toBe(true);
  });
});

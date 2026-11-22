/** 🧊️ Amendment D1: the glyph solids of a 3D model (force and reaction arrows, moment arcs, the centre-of-mass mark) are the paper size at the scale but at most 0.35 (arrows) and 0.06 (centre-of-mass radius) of the largest extent of the asset, so that at 1:200 and 1:500 the body still fills the frame; drawings keep pure paper sizes. The stream tubes of the air flow follow the same idea: half the paper width of the stream at the scale, but at most 0.012 of the largest extent in radius, the arrow heads shrinking with them.
 * @see ./🥒️.feature — the scenarios */
import { describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import type { ModelDocument } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { longestArrowMm } from "../../🔨️modules/🎨️depiction/🏗️structural-view/🟦️.ts";
import { FLOW_STYLE } from "../../🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts";
import { FLOW_MODEL, flowUnitOf } from "../../🔨️modules/🧊️volume/🌊️flow/🟦️.ts";
import { ARROW_SHARE, MASS_SHARE, modelSizing, paperSizing } from "../../🔨️modules/🧊️volume/🪧️glyphs/🟦️.ts";
import { authoredScales, modelOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { sampleFigure, volumeAssets } from "../🧰️support/🧊️.ts";
import { scaledFigure } from "../🧰️support/🧊️volumes/🟦️.ts";

const extentOf = (asset: Asset): number => Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
const node = (document: ModelDocument, suffix: string) => document.nodes.filter((n) => n.id.endsWith(suffix));

function arrows(document: ModelDocument): number[] {
  return node(document, "-shaft").filter((n) => n.id.includes("arrow-")).map((shaft) => {
    const head = document.nodes.find((n) => n.id === shaft.id.replace(/-shaft$/u, "-head"))!;
    const solid = document.solids.find((s) => s.node === head.id)! as unknown as { profile: [number, number][] };
    const root = document.nodes.find((n) => n.id === shaft.id.replace(/-shaft$/u, ""))!;
    return root.scale[0] * (shaft.scale[1] + head.scale[0] * -solid.profile[0]![0]);
  }).filter((length) => length > 1e-3);
}
const radii = (document: ModelDocument, id: string): number[] => document.solids.filter((s) => s.id.endsWith(id)).map((s) => (s as unknown as { radii: number[] }).radii[0]!);

const withGlyphs = (): readonly Asset[] => [scaledFigure(), ...volumeAssets().filter((a) => ["cantilever-balcony", "tied-canopy", "electric-fan-heater"].includes(a.id))];

describe("🧊️ glyph sizes in the 3D model", () => {
  it("has the cap shares of the design", () => {
    expect([ARROW_SHARE, MASS_SHARE]).toEqual([0.35, 0.06]);
  });

  it("sizes a drawing by paper millimetres only and a model by the smaller of paper and share", () => {
    expect(paperSizing(0.5)).toEqual({ mm: 0.5, unit: 0.5, arrowCapM: Infinity, arcCapM: Infinity, massRadiusM: 0.65 });
    const sizing = modelSizing(0.5, 2, 10);
    expect(sizing.arrowCapM).toBeCloseTo(0.7, 12);
    expect(sizing.arcCapM).toBeCloseTo(0.35, 12);
    expect(sizing.massRadiusM).toBeCloseTo(0.12, 12);
    expect(sizing.unit).toBeCloseTo(0.07, 12);
    const fine = modelSizing(0.001, 2, 10);
    expect([fine.unit, fine.massRadiusM]).toEqual([0.001, expect.closeTo(0.0013, 12)]);
  });

  it.each(withGlyphs().map((a) => [a.id, a] as const))("%s: at every authored scale no arrow is longer than the cap, the longest is min(paper, cap), the sphere is at most 0.06 of the extent", async (_id, asset) => {
    const extent = extentOf(asset);
    let seen = 0;
    for (const scale of authoredScales(asset.volumes)) {
      const document = await modelOf(asset, { scale, view: "forces" });
      const lengths = arrows(document);
      seen += lengths.length;
      for (const length of lengths) expect(length, `1:${scale}`).toBeLessThanOrEqual(ARROW_SHARE * extent * (1 + 1e-6));
      if (lengths.length > 0) {
        const paper = longestArrowMm(asset.dimensions.heightM / (scale / 1000)) * (scale / 1000);
        expect(Math.max(...lengths), `longest arrow at 1:${scale}`).toBeCloseTo(Math.min(paper, ARROW_SHARE * extent), 6);
      }
      for (const radius of radii(document, "centre-of-mass")) expect(radius, `sphere at 1:${scale}`).toBeLessThanOrEqual(MASS_SHARE * extent * (1 + 1e-9));
    }
    if (asset.id !== sampleFigure().id) expect(seen, "the asset has force arrows").toBeGreaterThan(0);
  });

  it("sizes the stream tubes and arrow heads of the air flow by paper width, capped at 0.012 of the extent in radius", { timeout: 240_000 }, async () => {
    expect(FLOW_MODEL.tubeShare).toBe(0.012);
    expect(flowUnitOf(50, 1.73)).toBe(0.05);
    expect(flowUnitOf(500, 1.73)).toBeCloseTo((0.012 * 1.73) / 0.15, 12);
    const asset = scaledFigure();
    const extent = extentOf(asset);
    for (const scale of authoredScales(asset.volumes)) {
      const document = await modelOf(asset, { scale, view: "thermal", poses: ["stand"], clips: [] });
      const unit = flowUnitOf(scale, extent);
      const tubes = document.solids.filter((s) => s.kind === "loft" && s.tags?.includes("flow") === true) as unknown as { sections: { rx: number; ry: number }[] }[];
      const heads = document.solids.filter((s) => s.kind === "revolve" && s.tags?.includes("flow") === true) as unknown as { profile: [number, number][] }[];
      expect(tubes.length, `tubes at 1:${scale}`).toBeGreaterThan(0);
      expect(heads.length, `arrow heads at 1:${scale}`).toBeGreaterThan(0);
      const radii = tubes.flatMap((t) => t.sections.map((s) => s.rx));
      for (const t of tubes) for (const s of t.sections) expect(s.ry).toBe(s.rx);
      expect(Math.max(...radii), `widest tube at 1:${scale}`).toBeLessThanOrEqual(Math.min((FLOW_STYLE.streamMaxMm / 2) * (scale / 1000), FLOW_MODEL.tubeShare * extent) * (1 + 1e-12));
      expect(Math.min(...radii), `thinnest tube at 1:${scale}`).toBeGreaterThanOrEqual((FLOW_STYLE.streamMinMm / 2) * unit * (1 - 1e-12));
      for (const h of heads) expect(h.profile).toEqual([[-FLOW_STYLE.headLengthMm * unit, 0], [-FLOW_STYLE.headLengthMm * unit, (FLOW_STYLE.headWidthMm / 2) * unit], [0, 0]]);
    }
  });

  it("keeps the arrow of a coarse scale as large relative to the body as the cap says: 1:500 is no longer than 1:50", async () => {
    const asset = scaledFigure();
    const longest = async (scale: number) => Math.max(0, ...arrows(await modelOf(asset, { scale, view: "forces" })));
    expect(await longest(500)).toBeLessThanOrEqual(await longest(50) + 1e-9);
  });
});

/** 📏️ Measured print suitability: strokes, hatch spacing, feature widths, gaps and negative-space slivers; oracle: hand-computed paper millimetres of axis-aligned rectangles (distances are exact there) and the profile values of the catalog document.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🗂️catalog/🖨️print-profiles/🔣️.json — the profiles
 */
import { describe, expect, it } from "vitest";
import type { AuthoredRepresentation, Shape } from "../../🧬️schema/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../🔨️modules/🖨️representation/🟦️.ts";
import { defaultShapes, rolesAssetWith } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";

const rect = (id: string, x: number, w: number, h = 0.5, role: Shape["role"] = "body", extra: Partial<Shape> = {}): Shape => ({ kind: "rect", id, bone: "ground", role, z: 0, filled: true, tone: 0.5, x, y: 0, w, h, ...extra }) as Shape;
const measure = (denominator: number, shapes: readonly Shape[], change: (rep: AuthoredRepresentation) => AuthoredRepresentation = (rep) => rep) => {
  const asset = rolesAssetWith((d) => (d === denominator ? shapes : defaultShapes()), (rep) => (rep.denominator === denominator ? change(rep) : rep));
  return measureSuitability(asset, asset.representations[`${denominator}` as "100"] as AuthoredRepresentation, denominator, DEFAULT_PRINT_PROFILE);
};

describe("📏️ print suitability", () => {
  it("passes well separated shapes at every scale", () => {
    for (const denominator of [500, 200, 100, 50]) {
      const result = measure(denominator, [rect("a", 0, 0.25, 1.8), rect("b", 0.5, 0.25), rect("c", 1, 0.25, 0.5, "detail")]);
      expect(result.violations, `1:${denominator}`).toEqual([]);
      expect(result.suitable).toBe(true);
    }
  });

  it("treats touching and overlapping ink as a join, not a gap", () => {
    const touching = measure(50, [rect("a", 0, 0.25), rect("b", 0.25, 0.25)]);
    expect(touching.violations.filter((v) => v.check === "gap")).toEqual([]);
    const overlapping = measure(50, [rect("a", 0, 0.25), rect("b", 0.2575, 0.25)]);
    expect(overlapping.violations.filter((v) => v.check === "gap")).toEqual([]);
  });

  it("reports a white gap between separate shapes that is narrower than the minimum", () => {
    const result = measure(500, [rect("a", 0, 0.25), rect("b", 0.38, 0.25, 0.5, "detail")]);
    const gap = result.violations.find((v) => v.check === "gap" && v.shapes.includes("a") && v.shapes.includes("b"));
    expect(gap).toBeDefined();
    expect(gap!.measuredMm).toBeCloseTo(0.26 - (0.18 + 0.13) / 2, 3);
    expect(gap!.requiredMm).toBe(DEFAULT_PRINT_PROFILE.minGapMm);
    expect(gap!.at).toBeDefined();
  });

  it("reports a negative-space sliver inside a single outline", () => {
    const u: Shape = { kind: "poly", id: "u", bone: "ground", role: "body", z: 0, filled: true, tone: 0.5, closed: true, points: [[0, 0], [0.3, 0], [0.3, 0.5], [0.166, 0.5], [0.166, 0.05], [0.134, 0.05], [0.134, 0.5], [0, 0.5]] } as Shape;
    const result = measure(50, [u]);
    const sliver = result.violations.find((v) => v.shapes[0] === "negative-space");
    expect(sliver).toBeDefined();
    expect(sliver!.measuredMm).toBeLessThan(DEFAULT_PRINT_PROFILE.minGapMm);
    expect(sliver!.at!.xMm).toBeCloseTo(0.15 * 20, 0);
    const wide: Shape = { ...u, points: [[0, 0], [0.3, 0], [0.3, 0.5], [0.2, 0.5], [0.2, 0.05], [0.1, 0.05], [0.1, 0.5], [0, 0.5]] } as Shape;
    expect(measure(50, [wide]).violations.filter((v) => v.check === "gap")).toEqual([]);
  });

  it("reports thin features, thin pens and tight hatches", () => {
    const result = measure(500, [rect("sliver", 0, 0.05, 1), rect("hatched", 1, 0.5, 0.5, "body", { hatch: "h" })], (rep) => ({ ...rep, pens: { ...rep.pens, body: 0.05 }, hatches: [{ id: "h", title: { en: "h", de: "h" }, pattern: "lines", angleDeg: 45, spacingMm: 0.2, penMm: 0.1 }] }));
    const checks = new Set(result.violations.map((v) => v.check));
    expect([...checks].sort()).toEqual(["feature", "hatch", "stroke"]);
    expect(result.violations.find((v) => v.check === "feature")!.measuredMm).toBeLessThan(0.2);
    expect(result.violations.find((v) => v.check === "hatch")!.measuredMm).toBe(0.2);
    expect(result.minStrokeMm).toBe(0.05);
  });
});

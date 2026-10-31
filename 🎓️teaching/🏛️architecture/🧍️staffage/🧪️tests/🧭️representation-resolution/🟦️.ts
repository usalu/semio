/** 🧭️ Representation resolution: exact, derived, unavailable; paper size follows the requested scale; zoom is a pure magnification. Oracle: the fixture vectors and d3-scale for the linear metre → millimetre map.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/👤️sample-figure/🔣️.json — the figure
 */
import { readFileSync } from "node:fs";
import { scaleLinear } from "d3-scale";
import { describe, expect, it } from "vitest";
import type { AssetDocument, Shape } from "../../🧬️schema/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, RepresentationError, resolutionText, resolveRepresentation, resolveSymbol } from "../../🔨️modules/🖨️representation/🟦️.ts";
import { viewportMm } from "../../🔨️modules/📐️units/🟦️.ts";
import { UNCHECKED_PROFILE, defaultShapes, rolesAssetWith, unchecked } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";

const asset = (JSON.parse(readFileSync(new URL("../../🧫️fixtures/👤️sample-figure/🔣️.json", import.meta.url), "utf8")) as AssetDocument).asset;
const resolve = (denominator: number) => resolveRepresentation(asset, { denominator }, UNCHECKED_PROFILE);
const sceneAt = (denominator: number, zoom = 1) => {
  const state = evaluate(asset, { kind: "pose", pose: "stand" }, { kind: "scale", scale: { denominator } });
  const scene = depict(state, unchecked(defaultDepictionOptions("light-line", "en", denominator)));
  return { state, scene, screen: viewportMm(scene.widthMm, scene.heightMm, zoom) };
};

describe("🧭️ representation resolution", () => {
  it("resolves a preset scale to exactly its authored drawing", () => {
    for (const denominator of [200, 100, 50] as const) {
      const r = resolve(denominator);
      expect(r.kind).toBe("authored");
      if (r.kind === "authored") {
        expect(r.exact).toBe(true);
        expect(r.denominator).toBe(denominator);
        expect(r.representation).toBe(asset.representations[`${denominator}`]);
      }
    }
  });

  it("resolves a custom scale inside a valid range as a derived use that names its source and range", () => {
    const r = resolve(120);
    expect(r.kind).toBe("authored");
    if (r.kind !== "authored") return;
    expect(r.exact).toBe(false);
    expect(r.denominator).toBe(100);
    expect(r.requested).toBe(120);
    expect(r.representation.validRange).toEqual({ minDenominator: 75, maxDenominator: 150 });
    const { scene, state } = sceneAt(120);
    expect(scene.metadata.resolution).toBe("authored");
    expect(state.representation).toEqual({ kind: "scale", denominator: 100, requested: 120, exact: false });
  });

  it("keeps paper size on the requested scale, never on the source drawing", () => {
    for (const denominator of [200, 120, 100, 60, 50]) {
      const { state } = sceneAt(denominator);
      const height = state.bounds.maxY - state.bounds.minY;
      expect(height * (1000 / denominator)).toBeCloseTo(scaleLinear().domain([0, 1]).range([0, 1000 / denominator])(height), 9);
      expect(sceneAt(denominator).scene.metadata.mmPerMetre).toBeCloseTo(1000 / denominator, 12);
    }
    expect(sceneAt(100).scene.heightMm - 10).toBeGreaterThan(sceneAt(200).scene.heightMm - 10);
  });

  it("declares unavailable scales with reason, limiting feature, supported scales and alternatives, and never draws them", () => {
    for (const denominator of [500, 1000]) {
      const r = resolve(denominator);
      expect(r.kind).toBe("unavailable");
      if (r.kind !== "unavailable") return;
      expect(r.supported).toEqual([200, 100, 50]);
      expect(r.alternatives.length).toBeGreaterThan(0);
      expect(r.reason.en.length).toBeGreaterThan(0);
      expect(r.reason.de.length).toBeGreaterThan(0);
      expect(() => evaluate(asset, { kind: "pose", pose: "stand" }, { kind: "scale", scale: { denominator } })).toThrow(RepresentationError);
    }
    const missing = resolve(500);
    if (missing.kind !== "unavailable" || missing.limitingSizeM === null) throw new Error("1:500 is available");
    expect(missing.limitingSizeM).toBeGreaterThan(0);
  });

  it("labels derived and unavailable resolutions in both languages", () => {
    expect(resolutionText(resolve(120), "en")).toMatch(/Derived: 1:120 uses the drawing authored for 1:100 \(valid 75…150\)/u);
    expect(resolutionText(resolve(120), "de")).toMatch(/Abgeleitet: 1:120 nutzt .* 1:100 .* 75…150/u);
    expect(resolutionText(resolve(100), "en")).toBe("Drawing authored for 1:100");
    expect(resolutionText(resolve(500), "en")).toMatch(/Not available at 1:500.*1:200, 1:100, 1:50/u);
    expect(resolutionText(resolve(500), "de")).toMatch(/nicht verfügbar/u);
  });

  it("resolves an authored drawing that violates the print profile as unsuitable, with violations and alternatives, and refuses to draw it", () => {
    const near = (denominator: number) => [{ kind: "rect", id: "a", bone: "ground", role: "body", z: 0, filled: true, tone: 0.5, x: 0, y: 0, w: 0.25, h: 0.5 }, { kind: "rect", id: "b", bone: "ground", role: "detail", z: 0, filled: true, tone: 0.5, x: 0.38, y: 0, w: 0.25, h: 0.5 }].map((shape) => ({ ...shape, id: `${shape.id}${denominator}` })) as unknown as Shape[];
    const bad = rolesAssetWith((denominator) => (denominator === 500 ? near(500) : defaultShapes()));
    const r = resolveRepresentation(bad, { denominator: 500 }, DEFAULT_PRINT_PROFILE);
    expect(r.kind).toBe("unsuitable");
    if (r.kind !== "unsuitable") return;
    expect(r.suitability.violations.some((v) => v.check === "gap")).toBe(true);
    expect(r.supported).toEqual([500, 200, 100, 50]);
    expect(r.alternatives.some((a) => a.kind === "scale" && a.denominator !== 500)).toBe(true);
    expect(resolutionText(r, "en")).toMatch(/Not printable at 1:500/u);
    expect(() => depict(evaluate(bad, { kind: "pose", pose: "stand" }, { kind: "scale", scale: { denominator: 500 } }), defaultDepictionOptions("light-line", "en", 500))).toThrow(RepresentationError);
    expect(resolveRepresentation(bad, { denominator: 200 }, DEFAULT_PRINT_PROFILE).kind).toBe("authored");
  });

  it("resolves symbols separately from scales", () => {
    expect(resolveSymbol(asset, "position-dot", 500).kind).toBe("symbol");
    expect(() => resolveSymbol(asset, "missing", 500)).toThrow(/no symbol/u);
  });

  it("does not let the viewport zoom change the geometry selection", () => {
    const base = sceneAt(100);
    for (const zoom of [0.5, 1, 2, 4]) {
      const zoomed = sceneAt(100, zoom);
      expect(JSON.stringify(zoomed.scene)).toBe(JSON.stringify(base.scene));
      expect(zoomed.state.representation).toEqual(base.state.representation);
      expect(zoomed.screen.widthMm).toBeCloseTo(base.scene.widthMm * zoom, 9);
      expect(zoomed.screen.heightMm).toBeCloseTo(base.scene.heightMm * zoom, 9);
    }
  });
});

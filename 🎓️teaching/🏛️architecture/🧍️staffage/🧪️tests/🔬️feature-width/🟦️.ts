/** 🔬️ Feature width: the minimum feature width of a small filled shape is its true width, not a multiple of the raster pitch, and the suitability verdict follows the measured representation.
 *
 * THE REFERENCE is shapely (🐍️.py) and the hand-computed widths of the vectors.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🔬️feature-width/🔣️.json — the vectors
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, PrintProfile } from "../../🧬️schema/🟦️.ts";
import { PRINT_PROFILES, measureSuitability } from "../../🔨️modules/🖨️representation/🟦️.ts";
import { featureWidthMm, releaseMeasureScratch } from "../../🔨️modules/🖨️representation/📐️measure/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";
requireTool("uv");

type Case = { name: string; rings: { x: number; y: number }[][]; widthMm: number; oracle: "erosion" | "opening" };
type Vectors = { minFeatureMm: number; relativeTolerance: number; absoluteToleranceMm: number; cases: Case[]; monotone: string[] };

const vectors = fixture<Vectors>("🔬️feature-width");
const tolerance = (expected: number): number => Math.max(vectors.relativeTolerance * expected, vectors.absoluteToleranceMm);
const byName = (name: string): Case => vectors.cases.find((c) => c.name === name)!;
const measured = (c: Case): number => featureWidthMm(c.rings, vectors.minFeatureMm);

describe("🔬️ minimum feature width", () => {
  for (const c of vectors.cases)
    it(`measures ${c.name} as ${c.widthMm} mm`, () => {
      expect(Math.abs(measured(c) - c.widthMm)).toBeLessThanOrEqual(tolerance(c.widthMm));
    });

  it("grows with the shape instead of stopping at the minimum feature", () => {
    const widths = vectors.monotone.map((name) => measured(byName(name)));
    for (let i = 1; i < widths.length; i++) expect(widths[i]!, vectors.monotone[i]).toBeGreaterThan(widths[i - 1]! + 0.01);
  });

  it("measures every shape alike whatever raster the retained scratch held before", () => {
    const cases = vectors.cases.slice(0, 8);
    const first = cases.map((c) => featureWidthMm(c.rings, vectors.minFeatureMm, 1e-9));
    const reversed = [...cases].reverse().map((c) => featureWidthMm(c.rings, vectors.minFeatureMm, 1e-9)).reverse();
    releaseMeasureScratch();
    const fresh = cases.map((c) => featureWidthMm(c.rings, vectors.minFeatureMm, 1e-9));
    expect(reversed).toEqual(first);
    expect(fresh).toEqual(first);
  });

  it("equals the widths that shapely finds by erosion and opening",{ timeout: 300_000 }, () => {
    const directory = mkdtempSync(join(tmpdir(), "staffage-feature-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify(vectors));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🔬️feature-width/🐍️.py"), "geometry", [input])) as Record<string, number>;
      for (const c of vectors.cases) expect(Math.abs(measured(c) - reference[c.name]!), `${c.name}: ${measured(c)} against ${reference[c.name]}`).toBeLessThanOrEqual(tolerance(c.widthMm));
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🔬️ suitability follows the measured representation", () => {
  const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
  const profile = PRINT_PROFILES.profiles.find((candidate) => candidate.id === PRINT_PROFILES.defaultProfile) as PrintProfile;

  it("reports the feature violation of a shape added to a representation that was measured before", () => {
    const asset: Asset = structuredClone(sample);
    const representation = asset.representations["100"] as AuthoredRepresentation;
    const before = measureSuitability(asset, representation, 100, profile);
    expect(before.violations.filter((violation) => violation.check === "feature").map((violation) => violation.shapes.join())).not.toContain("sliver");
    const shapes = representation.geometry.elevation as unknown as unknown[];
    shapes.push({ id: "sliver", bone: "pelvis", role: "body", z: 9, filled: true, tone: 0.5, kind: "rect", x: 0, y: 0, w: 0.2, h: 0.0003, penMm: 0.1 });
    const after = measureSuitability(asset, representation, 100, profile);
    expect(after.violations.some((violation) => violation.check === "feature" && violation.shapes.includes("sliver"))).toBe(true);
  });
});

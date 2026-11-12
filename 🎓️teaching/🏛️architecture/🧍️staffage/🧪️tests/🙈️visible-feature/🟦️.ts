/** 🙈️ Visible feature width: the print check measures what is visible after depth ordering. A shape fully covered by nearer filled shapes contributes nothing, a partly covered one is measured on its visible part.
 *
 * THE REFERENCE is shapely (🐍️.py: `difference` of the shape and the union of its occluders, then the opening criterion) and the hand-computed widths of the vectors.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🙈️visible-feature/🔣️.json — the vectors
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, PrintProfile } from "../../🧬️schema/🟦️.ts";
import type { Ring } from "../../🔨️modules/🎨️depiction/📐️geometry/🟦️.ts";
import { PRINT_PROFILES, featureWidthMm, measureSuitability, visibleFeatureWidthMm } from "../../🔨️modules/🖨️representation/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";
requireTool("uv");

type Case = { name: string; rings: Ring[]; covers: { penMm: number; rings: Ring[] }[]; widthMm: number | null };
type Vectors = { minFeatureMm: number; relativeTolerance: number; absoluteToleranceMm: number; cases: Case[]; monotone: string[] };

const vectors = fixture<Vectors>("🙈️visible-feature");
const tolerance = (expected: number): number => Math.max(vectors.relativeTolerance * expected, vectors.absoluteToleranceMm);
const measured = (c: Case): number | null => visibleFeatureWidthMm(c.rings, c.covers, vectors.minFeatureMm)?.widthMm ?? null;
const byName = (name: string): Case => vectors.cases.find((c) => c.name === name)!;

function expectWidth(actual: number | null, expected: number | null, label: string): void {
  if (expected === null) expect(actual, label).toBeNull();
  else {
    expect(actual, label).not.toBeNull();
    expect(Math.abs(actual! - expected), `${label}: ${actual} against ${expected}`).toBeLessThanOrEqual(tolerance(expected));
  }
}

describe("🙈️ feature width of the visible part", () => {
  for (const c of vectors.cases) it(`measures ${c.name} as ${c.widthMm === null ? "invisible" : `${c.widthMm} mm`}`, () => expectWidth(measured(c), c.widthMm, c.name));

  it("measures the whole shape when nothing covers it, as the plain feature width does", () => {
    const c = byName("a thin bar with no occluder");
    expect(Math.abs(measured(c)! - featureWidthMm(c.rings, vectors.minFeatureMm))).toBeLessThanOrEqual(1e-9);
  });

  it("grows when the shape is widened", () => {
    const widths = vectors.monotone.map((name) => measured(byName(name))!);
    for (let i = 1; i < widths.length; i++) expect(widths[i]!, vectors.monotone[i]).toBeGreaterThan(widths[i - 1]! + 0.01);
  });

  it("would flag the hidden bar if it were measured whole", () => {
    const c = byName("a thin bar completely behind a body");
    expect(featureWidthMm(c.rings, vectors.minFeatureMm)).toBeLessThan(vectors.minFeatureMm);
  });

  it("equals the visible widths that shapely finds by difference and opening", { timeout: 300_000 }, () => {
    const directory = mkdtempSync(join(tmpdir(), "staffage-visible-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify(vectors));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🙈️visible-feature/🐍️.py"), "geometry", [input])) as Record<string, number | null>;
      for (const c of vectors.cases) {
        expectWidth(reference[c.name]!, c.widthMm, `oracle ${c.name}`);
        expectWidth(measured(c), reference[c.name]!, c.name);
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🙈️ suitability measures the drawn result", () => {
  const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
  const profile = PRINT_PROFILES.profiles.find((candidate) => candidate.id === PRINT_PROFILES.defaultProfile) as PrintProfile;
  const withBar = (bar: Record<string, unknown>): ReturnType<typeof measureSuitability> => {
    const asset: Asset = structuredClone(sample);
    const representation = asset.representations["100"] as AuthoredRepresentation;
    (representation.geometry.elevation as unknown as unknown[]).push({ id: "bar", bone: "torso", role: "body", z: 2.9, filled: true, tone: 0.5, kind: "rect", penMm: 0.1, region: "torso", material: "cotton-clothing", ...bar });
    return measureSuitability(asset, representation, 100, profile);
  };
  const flagged = (result: ReturnType<typeof measureSuitability>): boolean => result.violations.some((violation) => violation.check === "feature" && violation.shapes.includes("bar"));

  it("ignores a hairline bar that lies completely behind the torso", () => {
    expect(flagged(withBar({ x: 0.1, y: -0.02, w: 0.2, h: 0.0003 }))).toBe(false);
  });

  it("reports the visible hairline of a bar that sticks out of the torso", () => {
    expect(flagged(withBar({ x: 0.2, y: 0, w: 0.003, h: 0.3 }))).toBe(true);
  });

  it("reports the same hairline bar drawn in front of the torso", () => {
    expect(flagged(withBar({ z: 9, x: 0.1, y: -0.02, w: 0.2, h: 0.0003 }))).toBe(true);
  });
});

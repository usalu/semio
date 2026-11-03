/** 🔗️ Same-region merge: filled shapes of one region read as one silhouette, unfilled detail strokes survive.
 *
 * THE REFERENCE is shapely (🐍️.py) and the hand-computed lengths of the vectors.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🔗️region-merge/🔣️.json — the vectors
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, Shape } from "../../🧬️schema/🟦️.ts";
import { outlineRuns, prepare, sourceOf } from "../../🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool, python } from "../🧰️support/🧰️.ts";
requireTool("uv");

type Vectors = { scale: number; paperMmPerM: number; pose: string; toleranceMm: number; shapes: Shape[]; expectedMm: Record<string, number> };

const vectors = fixture<Vectors>("🔗️region-merge");
const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const key = String(vectors.scale) as keyof Asset["representations"];
const authored = sample.representations[key] as AuthoredRepresentation;
const patched: Asset = { ...sample, representations: { ...sample.representations, [key]: { ...authored, geometry: { ...authored.geometry, elevation: vectors.shapes } } } as Asset["representations"] };

function visibleMm(): Record<string, number> {
  const state = evaluate(patched, { kind: "pose", pose: vectors.pose }, { kind: "scale", scale: { denominator: vectors.scale } });
  const drawn = prepare(state, "elevation", sourceOf(state), vectors.paperMmPerM);
  const lengths: Record<string, number> = Object.fromEntries(vectors.shapes.map((shape) => [shape.id, 0]));
  for (const { p, run } of outlineRuns(drawn, true)) {
    const points = run.closed ? [...run.points, run.points[0]!] : run.points;
    for (let i = 1; i < points.length; i++) lengths[p.shape.id]! += Math.hypot(points[i]!.x - points[i - 1]!.x, points[i]!.y - points[i - 1]!.y);
  }
  return lengths;
}

describe("🔗️ same-region merge", () => {
  it("merges the filled shapes of one region and keeps every unfilled detail not covered by a higher filled shape", () => {
    const found = visibleMm();
    for (const [id, expected] of Object.entries(vectors.expectedMm)) expect(Math.abs(found[id]! - expected), `${id}: ${found[id]} mm`).toBeLessThanOrEqual(vectors.toleranceMm);
    expect(found["body-a"]! + found["body-b"]!).toBeCloseTo(17.2, 1);
  });

  it("measures the same visible lengths as shapely", { timeout: 300_000 }, () => {
    const directory = mkdtempSync(join(tmpdir(), "staffage-merge-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ shapes: vectors.shapes, paperMmPerM: vectors.paperMmPerM }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🔗️region-merge/🐍️.py"), ["shapely"], [input])) as Record<string, number>;
      const found = visibleMm();
      for (const [id, length] of Object.entries(reference)) if (id !== "cover") expect(Math.abs(found[id]! - length), `${id}`).toBeLessThanOrEqual(vectors.toleranceMm);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

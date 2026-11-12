/** 🧊️ Solid placements: front and plan shapes that are revolved (axisymmetric) or extruded (laterally prismatic) solids of their bone-local profile, placed with the full bone transform.
 *
 * THE REFERENCE is shapely (🐍️.py) and the hand-computed numbers of the vectors: a 0.45 m by 0.05 m profile on a thigh that hangs down at rest and is turned by 22 degrees in the stride.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️solid-placements/🔣️.json — the vectors
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, Shape } from "../../🧬️schema/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Extent = { minX: number; maxX: number; minY: number; maxY: number };
type Vectors = {
  asset: string;
  scale: number;
  toleranceM: number;
  ovalToleranceM: number;
  bones: unknown;
  shapes: { front: Shape[]; plan: Shape[] };
  cases: { name: string; pose: string; front: Record<string, Extent>; plan: Record<string, Extent> }[];
};

const vectors = fixture<Vectors>("🧊️solid-placements");
const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const key = String(vectors.scale) as keyof Asset["representations"];
const authored = sample.representations[key] as AuthoredRepresentation;
const patched: Asset = { ...sample, representations: { ...sample.representations, [key]: { ...authored, geometry: { ...authored.geometry, ...vectors.shapes } } } as Asset["representations"] };
const toleranceFor = (id: string): number => (id === "crown" ? vectors.ovalToleranceM : vectors.toleranceM);

function measured(pose: string): Record<"front" | "plan", Record<string, Extent>> {
  const state = evaluate(patched, { kind: "pose", pose }, { kind: "scale", scale: { denominator: vectors.scale } });
  const origin = state.bones.pelvis!.origin;
  const out = { front: {} as Record<string, Extent>, plan: {} as Record<string, Extent> };
  for (const view of ["front", "plan"] as const)
    for (const id of vectors.shapes[view].map((shape) => shape.id)) {
      const points = state.shapes[view].find((evaluated) => evaluated.shape.id === id)!.rings.flat();
      const xs = points.map((p) => p.x - (view === "plan" ? origin.x : 0));
      const ys = points.map((p) => p.y - (view === "front" ? origin.y : 0));
      out[view][id] = { minX: Math.min(...xs), maxX: Math.max(...xs), minY: Math.min(...ys), maxY: Math.max(...ys) };
    }
  return out;
}

const expectExtent = (label: string, actual: Extent, expected: Extent, tolerance: number): void => {
  for (const edge of ["minX", "maxX", "minY", "maxY"] as const) expect(Math.abs(actual[edge] - expected[edge]), `${label} ${edge}`).toBeLessThanOrEqual(tolerance);
};

describe("🧊️ revolved and extruded front and plan shapes", () => {
  for (const c of vectors.cases)
    it(c.name, () => {
      const found = measured(c.pose);
      for (const view of ["front", "plan"] as const) for (const [id, extent] of Object.entries(c[view])) expectExtent(`${view} ${id}`, found[view][id]!, extent, toleranceFor(id));
    });

  it("equals the solids that shapely builds from the rotated profile", { timeout: 300_000 }, () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-solids-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ shapes: vectors.shapes, bones: vectors.bones, cases: vectors.cases }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🧊️solid-placements/🐍️.py"), "geometry", [input])) as Record<string, Record<"front" | "plan", Record<string, Extent>>>;
      for (const c of vectors.cases) {
        const found = measured(c.pose);
        for (const view of ["front", "plan"] as const) for (const [id, extent] of Object.entries(found[view])) expectExtent(`${c.pose} ${view} ${id} (shapely)`, extent, reference[c.pose]![view][id]!, vectors.ovalToleranceM);
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

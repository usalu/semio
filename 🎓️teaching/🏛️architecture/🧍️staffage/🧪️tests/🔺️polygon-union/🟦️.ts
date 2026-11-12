/** 🔺️ Polygon union: the agave leaf profiles that returned no ring, duplicates and degenerate input.
 *
 * THE REFERENCE is shapely `unary_union` in 🐍️.py.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🔺️polygon-union/🔣️.json — the profiles
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import polygonClipping from "polygon-clipping";
import { describe, expect, it } from "vitest";
import { ringArea, unionPolygons } from "../../🔨️modules/🔺️polygon/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Vectors = { tolerance: { relativeArea: number }; cases: { name: string; polygons: [number, number][][] }[]; slivers: { name: string; rings: number; polygons: [number, number][][] }[] };
const vectors = fixture<Vectors>("🔺️polygon-union");
const rings = (polygons: [number, number][][]) => polygons.map((ring) => ring.map(([x, y]) => ({ x, y })));

describe("🔺️ the union of the agave leaf profiles", () => {
  it("equals the shapely union in area and in number of rings", () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-union-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify(vectors));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🔺️polygon-union/🐍️.py"), "geometry", [input])) as { area: number; rings: number }[];
      vectors.cases.forEach((c, i) => {
        const union = unionPolygons(rings(c.polygons));
        expect(union.length, c.name).toBeGreaterThan(0);
        const area = union.reduce((sum, ring) => sum + ringArea(ring), 0);
        expect(Math.abs(area - reference[i]!.area), c.name).toBeLessThanOrEqual(vectors.tolerance.relativeArea * reference[i]!.area);
        expect(union.length, `${c.name} rings`).toBe(reference[i]!.rings);
      });
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🔺️ the union of the vectors against polygon-clipping", () => {
  vectors.cases.forEach((c) =>
    it(`equals the polygon-clipping union in area: ${c.name}`, () => {
      const [first, ...rest] = c.polygons.map((ring) => [ring] as [number, number][][]);
      const reference = polygonClipping.union(first!, ...rest);
      const expected = reference.reduce((sum, polygon) => sum + polygon.reduce((inner, ring, k) => inner + (k === 0 ? 1 : -1) * Math.abs(ringArea(ring.slice(0, -1).map(([x, y]) => ({ x: x!, y: y! })))), 0), 0);
      const union = unionPolygons(rings(c.polygons));
      expect(union.length, c.name).toBeGreaterThan(0);
      const area = union.reduce((sum, ring) => sum + ringArea(ring), 0);
      expect(Math.abs(area - expected), c.name).toBeLessThanOrEqual(vectors.tolerance.relativeArea * expected);
    }),
  );
});

describe("🔺️ slivers thinner than the resolution of the union", () => {
  vectors.slivers.forEach((c) =>
    it(`builds the union and equals the polygon-clipping union in area: ${c.name}`, () => {
      const [first, ...rest] = c.polygons.map((ring) => [ring] as [number, number][][]);
      const reference = polygonClipping.union(first!, ...rest);
      const expected = reference.reduce((sum, polygon) => sum + polygon.reduce((inner, ring, k) => inner + (k === 0 ? 1 : -1) * Math.abs(ringArea(ring.slice(0, -1).map(([x, y]) => ({ x: x!, y: y! })))), 0), 0);
      const union = unionPolygons(rings(c.polygons));
      expect(union.length, `${c.name} rings`).toBe(c.rings);
      const area = union.reduce((sum, ring) => sum + ringArea(ring), 0);
      expect(Math.abs(area - expected), c.name).toBeLessThanOrEqual(Math.max(vectors.tolerance.relativeArea * expected, 1e-8));
    }),
  );
});

describe("🔺️ degenerate input", () => {
  it("removes duplicate and closing points", () => {
    const square = [{ x: 0, y: 0 }, { x: 1, y: 0 }, { x: 1, y: 0 }, { x: 1, y: 1 }, { x: 0, y: 1 }, { x: 0, y: 0 }];
    const union = unionPolygons([square]);
    expect(union).toHaveLength(1);
    expect(ringArea(union[0]!)).toBeCloseTo(1, 12);
  });

  it("gives an empty union for polygons without area", () => {
    expect(unionPolygons([[{ x: 0, y: 0 }, { x: 1, y: 0 }, { x: 2, y: 0 }]])).toEqual([]);
  });
});

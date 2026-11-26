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

/** 🫧️ A mass of overlapping ellipsoids as the loft of a foliage mass draws it: every ellipsoid is a bead of parallel elliptic sections `step` degrees apart across a tilted view, the body between two sections is the convex hull of their two rings, and the mass is the union of all bodies. Deterministic (a linear congruential generator), so the vectors are the same on every machine. */
function beadMass(step: number, seed: number, ellipsoids: number): [number, number][][] {
  let state = seed;
  const random = (): number => (state = (state * 1664525 + 1013904223) % 4294967296) / 4294967296;
  const [tilt, turn] = [random() * 90, random() * 180].map((degrees) => (degrees * Math.PI) / 180) as [number, number];
  const bodies: [number, number][][] = [];
  const hull = (points: [number, number][]): [number, number][] => {
    const sorted = [...points].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
    const cross = (o: [number, number], a: [number, number], b: [number, number]) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
    const build = (input: [number, number][]) => { const out: [number, number][] = []; for (const p of input) { while (out.length >= 2 && cross(out.at(-2)!, out.at(-1)!, p) <= 0) out.pop(); out.push(p); } out.pop(); return out; };
    return [...build(sorted), ...build([...sorted].reverse())];
  };
  for (let k = 0; k < ellipsoids; k++) {
    const [rx, ry, rz] = [0.1 + random() * 0.4, 0.1 + random() * 0.4, 0.1 + random() * 0.4];
    const centre = [(random() - 0.5) * 0.8, (random() - 0.5) * 0.8, (random() - 0.5) * 0.8] as const;
    const rings: [number, number][][] = [];
    for (let latitude = -90 + step; latitude < 90 - 1e-9; latitude += step) {
      const a = (latitude * Math.PI) / 180;
      rings.push(Array.from({ length: 48 }, (_, i): [number, number] => {
        const t = (2 * Math.PI * i) / 48;
        const [x, y, z] = [rz * Math.sin(a) + centre[0], ry * Math.cos(a) * Math.sin(t) + centre[1], rx * Math.cos(a) * Math.cos(t) + centre[2]];
        const [x2, z2] = [x * Math.cos(turn) - z * Math.sin(turn), x * Math.sin(turn) + z * Math.cos(turn)];
        return [x2, y * Math.cos(tilt) - z2 * Math.sin(tilt)];
      }));
    }
    for (let i = 0; i + 1 < rings.length; i++) bodies.push(hull([...rings[i]!, ...rings[i + 1]!]));
  }
  return bodies;
}

/** polygon-clipping itself fails on the 2 degree masses ("Unable to find segment in SweepLine tree"), so those are held against shapely alone. */
describe("🔺️ masses of overlapping beads at small spacing", () => {
  const masses = [4, 2, 13].flatMap((step) => [1, 2, 3, 4, 5, 6, 7, 8].map((seed) => ({ name: `${step} degrees, seed ${seed}`, polygons: beadMass(step, seed * 7919, 2 + (seed % 6)) })));

  it("equals the shapely union in area and in extent: no ellipsoid is dropped", () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-union-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ cases: masses, slivers: [] }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🔺️polygon-union/🐍️.py"), "geometry", [input])) as { area: number; rings: number; bounds: number[] }[];
      masses.forEach((c, i) => {
        const union = unionPolygons(rings(c.polygons));
        const area = union.reduce((sum, ring) => sum + ringArea(ring), 0);
        expect(Math.abs(area - reference[i]!.area), c.name).toBeLessThanOrEqual(1e-6 * reference[i]!.area);
        const box = [Math.min(...union.flat().map((p) => p.x)), Math.min(...union.flat().map((p) => p.y)), Math.max(...union.flat().map((p) => p.x)), Math.max(...union.flat().map((p) => p.y))];
        box.forEach((value, k) => expect(Math.abs(value - reference[i]!.bounds[k]!), `${c.name} box ${k}`).toBeLessThanOrEqual(1e-9));
      });
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });

  masses.filter((c) => !c.name.startsWith("2 degrees")).forEach((c) =>
    it(`equals the polygon-clipping union in area: ${c.name}`, () => {
      const [first, ...rest] = c.polygons.map((ring) => [ring] as [number, number][][]);
      const reference = polygonClipping.union(first!, ...rest);
      const expected = reference.reduce((sum, polygon) => sum + polygon.reduce((inner, ring, k) => inner + (k === 0 ? 1 : -1) * Math.abs(ringArea(ring.slice(0, -1).map(([x, y]) => ({ x: x!, y: y! })))), 0), 0);
      const area = unionPolygons(rings(c.polygons)).reduce((sum, ring) => sum + ringArea(ring), 0);
      expect(Math.abs(area - expected), c.name).toBeLessThanOrEqual(1e-6 * expected);
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

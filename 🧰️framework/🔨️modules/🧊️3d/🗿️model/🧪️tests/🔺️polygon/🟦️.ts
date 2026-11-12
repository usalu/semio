/** 🔺️ Cap triangulation: collinear and near-duplicate vertices stay vertices (no T-junctions), area equals the polygon's, triangle count is V + 2H - 2, every boundary edge is used once; area against three's ShapeUtils (earcut) and a seeded random stress run. */
import { ShapeUtils, Vector2 } from "three";
import { describe, expect, it } from "vitest";
import { cleanRing, contact, holesFault, ringFault, signedArea, triangulatePolygon } from "../../🔺️tessellation/🔺️polygon/🟦️.ts";
import { fixture } from "../🧰️support/🟦️.ts";

type P = [number, number];
type Case = { name: string; outer: P[]; holes: P[][]; expected: { area: number; triangles: number; vertices: number } };
const data = fixture<{ areaTolerance: number; cases: Case[] }>("🔺️polygon");

function audit(outer: P[], holes: P[][]) {
  const all = [...outer, ...holes.flat()];
  const triangles = triangulatePolygon(outer, holes);
  let area = 0, clockwise = 0;
  const used = new Set<number>(), directed = new Map<string, number>();
  for (let t = 0; t < triangles.length; t += 3) {
    const [a, b, c] = [triangles[t]!, triangles[t + 1]!, triangles[t + 2]!];
    const cross = (all[b]![0] - all[a]![0]) * (all[c]![1] - all[a]![1]) - (all[b]![1] - all[a]![1]) * (all[c]![0] - all[a]![0]);
    area += cross / 2;
    if (cross <= 0) clockwise++;
    for (const v of [a, b, c]) used.add(v);
    for (const [u, v] of [[a, b], [b, c], [c, a]] as const) directed.set(`${u}>${v}`, (directed.get(`${u}>${v}`) ?? 0) + 1);
  }
  const boundary = new Set<string>();
  let base = 0;
  for (const ring of [outer, ...holes]) {
    ring.forEach((_, i) => boundary.add(`${base + i}>${base + ((i + 1) % ring.length)}`));
    base += ring.length;
  }
  const boundaryOnce = [...boundary].every((key) => directed.get(key) === 1);
  const interiorPaired = [...directed].every(([key, count]) => {
    if (boundary.has(key)) return true;
    const [u, v] = key.split(">");
    return count === 1 && directed.get(`${v}>${u}`) === 1 && !boundary.has(`${v}>${u}`);
  });
  return { triangles: triangles.length / 3, area, clockwise, used: used.size, vertices: all.length, boundaryOnce, interiorPaired };
}

describe("triangulatePolygon", () => {
  for (const c of data.cases) {
    it(`${c.name}: complete, no T-junctions, area like three's ShapeUtils`, () => {
      const r = audit(c.outer, c.holes);
      expect(r.triangles).toBe(c.expected.triangles);
      expect(r.used).toBe(c.expected.vertices);
      expect(r.clockwise).toBe(0);
      expect(r.boundaryOnce).toBe(true);
      expect(r.interiorPaired).toBe(true);
      expect(Math.abs(r.area - c.expected.area)).toBeLessThan(data.areaTolerance);
      const vec = (ring: P[]) => ring.map((p) => new Vector2(p[0], p[1]));
      const all = [...c.outer, ...c.holes.flat()];
      const earcut = ShapeUtils.triangulateShape(vec(c.outer), c.holes.map(vec)).reduce((s, [a, b, d]) => s + Math.abs((all[b!]![0] - all[a!]![0]) * (all[d!]![1] - all[a!]![1]) - (all[b!]![1] - all[a!]![1]) * (all[d!]![0] - all[a!]![0])) / 2, 0);
      expect(Math.abs(earcut - r.area)).toBeLessThan(1e-6);
    });
  }

  it("triangulates every grid of aligned rectangular holes completely (holes sharing rows and columns, collinear bridges)", () => {
    for (let columns = 1; columns <= 4; columns++) {
      for (let rows = 1; rows <= 4; rows++) {
        const holes: P[][] = [];
        for (let i = 0; i < columns; i++) for (let j = 0; j < rows; j++) holes.push([[1 + 3 * i, 1 + 3 * j], [1 + 3 * i, 3 + 3 * j], [3 + 3 * i, 3 + 3 * j], [3 + 3 * i, 1 + 3 * j]]);
        const outer: P[] = [[0, 0], [3 * columns + 1, 0], [3 * columns + 1, 3 * rows + 1], [0, 3 * rows + 1]];
        const r = audit(outer, holes);
        const label = `${columns}x${rows}`;
        expect(r.triangles, label).toBe(outer.length + holes.length * 4 + 2 * holes.length - 2);
        expect(r.boundaryOnce, label).toBe(true);
        expect(r.interiorPaired, label).toBe(true);
        expect(r.clockwise, label).toBe(0);
        expect(Math.abs(r.area - ((3 * columns + 1) * (3 * rows + 1) - 4 * columns * rows)), label).toBeLessThan(1e-9);
      }
    }
  });

  it("survives 3000 seeded random star polygons with collinear points, near-duplicates and holes", () => {
    let seed = 12345;
    const rnd = () => ((seed = (seed * 1664525 + 1013904223) % 4294967296) / 4294967296);
    let failures = 0;
    for (let n = 0; n < 3000; n++) {
      const scale = [1, 0.01, 100][n % 3]!;
      const star = (sides: number, r0: number, r1: number): P[] => Array.from({ length: sides }, (_, i) => {
        const a = (2 * Math.PI * i) / sides + rnd() * 0.3 * ((2 * Math.PI) / sides), r = r0 + rnd() * (r1 - r0);
        return [r * Math.cos(a), r * Math.sin(a)];
      });
      const decorate = (ring: P[], collinear: number, near: number): P[] => ring.flatMap((a, i) => {
        const b = ring[(i + 1) % ring.length]!, out: P[] = [a];
        if (rnd() < collinear) for (let q = 1; q <= 2; q++) out.push([a[0] + ((b[0] - a[0]) * q) / 3, a[1] + ((b[1] - a[1]) * q) / 3]);
        if (rnd() < near) out.push([a[0] + (b[0] - a[0]) * 1e-3 + 10 ** (-9 + rnd() * 3) * scale, a[1] + (b[1] - a[1]) * 1e-3]);
        return out;
      });
      const collinear = n % 4 === 1 || n % 4 === 3 ? 0.5 : 0, near = n % 4 >= 2 ? 0.4 : 0;
      let outer = decorate(star(4 + Math.floor(rnd() * 10), 0.6 * scale, scale), collinear, near);
      if (signedArea(outer) < 0) outer = outer.reverse();
      const holes: P[][] = [];
      if (n % 5 === 0) {
        let h = decorate(star(4 + Math.floor(rnd() * 4), 0.1 * scale, 0.3 * scale), collinear, near);
        if (signedArea(h) > 0) h = h.reverse();
        holes.push(h);
      }
      const r = audit(outer, holes);
      const expected = Math.abs(signedArea(outer)) - holes.reduce((s, h) => s + Math.abs(signedArea(h)), 0);
      if (r.clockwise > 0 || r.used !== r.vertices || !r.boundaryOnce || !r.interiorPaired || Math.abs(r.area - expected) > 1e-9 * scale * scale) failures++;
    }
    expect(failures).toBe(0);
  });
});

describe("polygon predicates", () => {
  const square: P[] = [[0, 0], [1, 0], [1, 1], [0, 1]];
  it("cleanRing drops duplicates and a closing point", () => {
    expect(cleanRing([[0, 0], [0, 0], [1, 0], [1, 1], [0, 0]])).toEqual([[0, 0], [1, 0], [1, 1]]);
  });
  it("ringFault accepts simple rings (also with collinear points) and rejects bowties, touches, spikes and zero area", () => {
    expect(ringFault(square, true)).toBe(false);
    expect(ringFault([[0, 0], [0.5, 0], [1, 0], [1, 1], [0, 1]], true)).toBe(false);
    expect(ringFault([[0, 0], [1, 1], [1, 0], [0, 1]], true)).toBe(true);
    expect(ringFault([[0, 0], [2, 0], [1, 1], [2, 2], [0, 2], [1, 1]], true)).toBe(true);
    expect(ringFault([[0, 0], [2, 0], [1, 0], [1, 1], [0, 1]], true)).toBe(true);
    expect(ringFault([[0, 0], [1, 0], [2, 0]], true)).toBe(true);
  });
  it("ringFault exempts contacts on the axis for open polylines", () => {
    const bicone: P[] = [[0, 0], [1, 0.5], [2, 0], [3, 0.5], [4, 0]];
    expect(ringFault(bicone, false, (p) => p[1] <= 1e-9)).toBe(false);
    expect(ringFault([[0, 0.1], [2, 0.5], [2, 0.2], [0, 0.6]], false, (p) => p[1] <= 1e-9)).toBe(true);
  });
  it("holesFault rejects crossing, touching, outside and nested holes", () => {
    const inner: P[] = [[0.25, 0.25], [0.25, 0.75], [0.75, 0.75], [0.75, 0.25]];
    expect(holesFault(square, [inner])).toBe(false);
    expect(holesFault(square, [[[0.5, 0.5], [1.5, 0.5], [1.5, 0.9], [0.5, 0.9]]])).toBe(true);
    expect(holesFault(square, [[[2, 2], [3, 2], [3, 3], [2, 3]]])).toBe(true);
    expect(holesFault(square, [[[0, 0.5], [0.5, 0.3], [0.5, 0.7]]])).toBe(true);
    expect(holesFault(square, [[[0.1, 0.1], [0.9, 0.1], [0.9, 0.9], [0.1, 0.9]], [[0.4, 0.4], [0.6, 0.4], [0.6, 0.6], [0.4, 0.6]]])).toBe(true);
  });
  it("contact finds crossings, end touches and overlaps, and none for apart segments", () => {
    expect(contact([0, 0], [1, 1], [0, 1], [1, 0], 1e-12)?.length).toBe(1);
    expect(contact([0, 0], [1, 0], [1, 0], [2, 1], 1e-12)).not.toBeNull();
    expect(contact([0, 0], [2, 0], [1, 0], [3, 0], 1e-12)).not.toBeNull();
    expect(contact([0, 0], [1, 0], [0, 1], [1, 1], 1e-12)).toBeNull();
  });
});

/** 🩻️ Mesh integrity: closedness, manifoldness, winding, signed volume and outward normals of every solid kind and of hand-made meshes, against the analytic values of the fixture and against trimesh (watertight, winding consistent, volume, repaired normals) in the locked uv oracle group. Includes the signed-volume guard: no closed solid is ever inside-out, whatever the section order or rotation handedness of a loft. */
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { meshIntegrity, outwardNormals, outwardOrientation, tessellateSolid } from "../../🟦️.ts";
import type { Quality, Solid } from "../../🧬️schema/🟦️.ts";
import { fixture } from "../🧰️support/🟦️.ts";
import { pythonOracle } from "../🧰️support/🐍️oracle/🟦️.ts";

type Expected = { closed?: boolean; manifold?: boolean; oriented?: boolean; volume?: number; volumeTolerance?: number; openEdgesPositive?: boolean; signedVolume?: number; openEdges?: number; nonManifoldEdges?: number; pinchVertices?: number; outwardFrom?: number[][]; components?: number[][] };
type SolidCase = { name: string; solid: Solid; expected: Expected };
type MeshCase = { name: string; positions: number[]; indices: number[]; expected: Expected };
type Oracle = Record<string, { watertight: boolean; windingConsistent: boolean; isVolume: boolean; volume: number; area: number; outwardFaces?: number; testedFaces?: number }>;

const data = fixture<{ quality: Quality; solids: SolidCase[]; meshes: MeshCase[] }>("🩻️integrity");
const ORACLE = fileURLToPath(new URL("./🐍️.py", import.meta.url));

const geometries = new Map<string, { positions: number[]; indices: number[] }>();
for (const c of data.solids) geometries.set(c.name, tessellateSolid(c.solid, data.quality));
for (const c of data.meshes) geometries.set(c.name, { positions: c.positions, indices: c.indices });

const cases: (SolidCase | MeshCase)[] = [...data.solids, ...data.meshes];
const normalsOf = new Map<string, Float64Array>();
for (const c of cases) if (c.expected.closed) normalsOf.set(c.name, outwardNormals(geometries.get(c.name)!.positions, geometries.get(c.name)!.indices));
const oracle = pythonOracle<Oracle>(ORACLE, { cases: cases.map((c) => ({ name: c.name, ...geometries.get(c.name)!, ...(normalsOf.has(c.name) ? { normals: [...normalsOf.get(c.name)!], components: (c.expected as Expected).components } : {}) })) });

describe("🩻️ meshIntegrity", () => {
  for (const c of cases) {
    it(`${c.name}: the topology and the volume are the expected ones and trimesh agrees`, () => {
      const g = geometries.get(c.name)!;
      const m = meshIntegrity(g.positions, g.indices);
      const e = c.expected;
      const t = oracle[c.name]!;
      if (e.closed !== undefined) expect(m.closed).toBe(e.closed);
      if (e.manifold !== undefined) expect(m.manifold).toBe(e.manifold);
      if (e.oriented !== undefined) expect(m.oriented).toBe(e.oriented);
      if (e.openEdges !== undefined) expect(m.openEdges).toBe(e.openEdges);
      if (e.openEdgesPositive) expect(m.openEdges).toBeGreaterThan(0);
      if (e.nonManifoldEdges !== undefined) expect(m.nonManifoldEdges).toBe(e.nonManifoldEdges);
      if (e.pinchVertices !== undefined) expect(m.pinchVertices).toBe(e.pinchVertices);
      if (e.signedVolume !== undefined) expect(m.signedVolume).toBeCloseTo(e.signedVolume, 9);
      if (e.volume !== undefined) {
        expect(m.signedVolume, "the signed-volume guard: a closed solid is never inside-out").toBeGreaterThan(0);
        expect(Math.abs(m.signedVolume - e.volume) / e.volume).toBeLessThanOrEqual(e.volumeTolerance!);
      }
      if (m.closed) expect(Math.abs(m.signedVolume - t.volume)).toBeLessThanOrEqual(1e-9 * Math.max(1, Math.abs(t.volume)));
      expect(Math.abs(m.area - t.area)).toBeLessThanOrEqual(1e-9 * Math.max(1, t.area));
      if (e.closed !== undefined && e.nonManifoldEdges === undefined) expect(m.closed, "trimesh watertight").toBe(t.watertight);
      if (e.oriented !== undefined && e.closed) expect(m.oriented, "trimesh winding consistent").toBe(t.windingConsistent);
      if (e.manifold && e.oriented && m.signedVolume > 0) expect(t.isVolume).toBe(true);
    });
  }
});

describe("🧭️ outwardNormals", () => {
  for (const c of cases) {
    const e = c.expected;
    if (!e.closed) continue;
    it(`${c.name}: every face of a closed mesh points out, a probe along the normal leaves the body and one against it enters (trimesh contains)`, () => {
      const g = geometries.get(c.name)!;
      const normals = normalsOf.get(c.name)!;
      for (let t = 0; t < g.indices.length / 3; t++) expect(Math.hypot(normals[t * 3]!, normals[t * 3 + 1]!, normals[t * 3 + 2]!)).toBeCloseTo(1, 9);
      expect(oracle[c.name]!.testedFaces).toBeGreaterThan(0);
      expect(oracle[c.name]!.outwardFaces).toBe(oracle[c.name]!.testedFaces);
    });
  }
  for (const c of data.meshes.filter((m) => m.expected.outwardFrom)) {
    it(`${c.name}: each component's faces point away from its own centre, touching or not`, () => {
      const normals = outwardNormals(c.positions, c.indices);
      c.expected.components!.forEach(([first, count], k) => {
        const centre = c.expected.outwardFrom![k]!;
        for (let t = first; t < first + count; t++) {
          const corners = [0, 1, 2].map((j) => c.indices[t * 3 + j]! * 3);
          const centroid = [0, 1, 2].map((a) => corners.reduce((s, i) => s + c.positions[i + a]!, 0) / 3);
          const out = [0, 1, 2].reduce((s, a) => s + normals[t * 3 + a]! * (centroid[a]! - centre[a]!), 0);
          expect(out, `triangle ${t}`).toBeGreaterThan(0);
        }
      });
    });
  }
  for (const name of ["loft-open-tube-start-cap-off", "loft-open-tube-both-caps-off-outward"]) {
    it(`${name}: an open loft keeps its faces looking outward (away from the path)`, () => {
      const g = geometries.get(name)!;
      const normals = outwardNormals(g.positions, g.indices);
      let sides = 0;
      for (let t = 0; t < g.indices.length / 3; t++) {
        const corners = [0, 1, 2].map((j) => g.indices[t * 3 + j]! * 3);
        const centroid = [0, 1, 2].map((a) => corners.reduce((s, i) => s + g.positions[i + a]!, 0) / 3);
        if (Math.abs(normals[t * 3 + 2]!) > 0.5) continue;
        sides++;
        expect(normals[t * 3]! * centroid[0]! + normals[t * 3 + 1]! * centroid[1]!).toBeGreaterThan(0);
      }
      expect(sides).toBeGreaterThan(0);
    });
  }
});

describe("🧭️ outwardOrientation", () => {
  for (const c of cases) {
    it(`${c.name}: closedness and the oriented volume agree with meshIntegrity`, () => {
      const g = geometries.get(c.name)!;
      const o = outwardOrientation(g.positions, g.indices);
      const m = meshIntegrity(g.positions, g.indices);
      expect(o.closed).toBe(m.closed);
      if (m.closed && m.oriented && m.components === 1) expect(o.signedVolume).toBeCloseTo(Math.abs(m.signedVolume), 9);
      if (m.closed) expect(o.signedVolume).toBeGreaterThan(0);
    });
  }
  it("openWinding -1 turns the faces of an open piece (a mirrored frame) and leaves closed pieces alone", () => {
    const open = geometries.get("loft-open-tube-both-caps-off-outward")!;
    const [a, b] = [outwardNormals(open.positions, open.indices), outwardNormals(open.positions, open.indices, { openWinding: -1 })];
    for (let i = 0; i < a.length; i++) expect(b[i]).toBeCloseTo(-a[i]!, 12);
    const cube = geometries.get("cube-outward")!;
    expect([...outwardNormals(cube.positions, cube.indices, { openWinding: -1 })]).toEqual([...outwardNormals(cube.positions, cube.indices)]);
  });
});

/** 🧊️ One suite per solid fixture: tessellation checked against analytic expectations and three.js measurements (volume, bounds, area, watertightness, normals) plus determinism and schema validity. */
import type { BufferGeometry } from "three";
import { describe, expect, it } from "vitest";
import { tessellate, validateModel } from "../../../🟦️.ts";
import type { ModelDocument, Quality } from "../../../🧬️schema/🟦️.ts";
import { badNormals, bounds, fixture, isClosedOrientedManifold, mergedGeometry, signedVolume, surfaceArea, trianglesOf, validateSchema, weldedGeometry } from "../🟦️.ts";

export type SolidCase = { name: string; document: ModelDocument; expected: Record<string, any>; volumeRelativeTolerance: number; boundsTolerance: number };
export type Oracle = (c: SolidCase, ours: BufferGeometry, resolved: ModelDocument) => void;


export function solidSuite(name: string, oracle?: Oracle): void {
  const data = fixture<{ quality: Quality; cases: SolidCase[] }>(name);
  describe(`${name} tessellation`, () => {
    for (const c of data.cases) {
      describe(c.name, () => {
        const resolved = tessellate(c.document, data.quality, { report: () => {}, cancelled: () => false });
        it("validates against the JSON schema and the kernel, procedural and resolved", async () => {
          const out = await resolved;
          expect(validateSchema(c.document), JSON.stringify(validateSchema.errors)).toBe(true);
          expect(validateSchema(out), JSON.stringify(validateSchema.errors)).toBe(true);
          expect(validateModel(c.document).filter((i) => i.severity === "error")).toEqual([]);
          expect(validateModel(out).filter((i) => i.severity === "error")).toEqual([]);
          expect(out.solids).toEqual([]);
        });
        it("matches the expected volume and bounds measured with three.js", async () => {
          const out = await resolved;
          const g = mergedGeometry(out);
          if (c.expected.volume !== null) {
            const volume = signedVolume(g);
            expect(volume).toBeGreaterThan(0);
            expect(Math.abs(volume - c.expected.volume) / c.expected.volume).toBeLessThanOrEqual(c.volumeRelativeTolerance);
          }
          const b = bounds(g);
          for (let i = 0; i < 3; i++) {
            expect(Math.abs(b.min[i]! - c.expected.bounds.min[i])).toBeLessThanOrEqual(c.boundsTolerance);
            expect(Math.abs(b.max[i]! - c.expected.bounds.max[i])).toBeLessThanOrEqual(c.boundsTolerance);
          }
          if (c.expected.triangles !== undefined) expect(g.getIndex()!.count / 3).toBe(c.expected.triangles);
        });
        it("is watertight with consistent outward orientation when the kind is closed, and open otherwise", async () => {
          const out = await resolved;
          expect(isClosedOrientedManifold(weldedGeometry(mergedGeometry(out)))).toBe(c.expected.closed);
        });
        it("has unit vertex normals that agree with the geometric face normals", async () => {
          const out = await resolved;
          for (const p of trianglesOf(out)) expect(badNormals(p)).toBe(0);
        });
        it("is deterministic", async () => {
          expect(JSON.stringify(await tessellate(c.document, data.quality, { report: () => {}, cancelled: () => false }))).toBe(JSON.stringify(await resolved));
        });
        if (oracle) it("agrees with the independent three.js geometry", async () => {
          const out = await resolved;
          oracle(c, mergedGeometry(out), out);
        });
      });
    }
  });
}

export { surfaceArea };

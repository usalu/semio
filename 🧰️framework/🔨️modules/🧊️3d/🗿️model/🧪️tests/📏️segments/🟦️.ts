/** 📏️ Segments of a full circle: the chord tolerance, the bounds and the `round` floor of the quality, measured on a tessellated cylinder against three's CylinderGeometry of the expected segment count (the same sagitta and the same number of meridians), with smooth normals around the barrel and hard normals at the two rims.
 * @see ../../🧫️fixtures/📏️segments/🔣️.json — the language-agnostic vectors
 * @see ../../🔺️tessellation/📏️segments/🟦️.ts */
import { CylinderGeometry } from "three";
import { describe, expect, it } from "vitest";
import { tessellateSolid } from "../../🟦️.ts";
import type { Quality, Solid } from "../../🧬️schema/🟦️.ts";
import { fixture, schema } from "../🧰️support/🟦️.ts";
import Ajv from "ajv";

type Case = { name: string; radiusM: number; heightM: number; quality: Quality; expected: { segments: number } };
const { cases } = fixture<{ cases: Case[] }>("📏️segments");
const validateQuality = new Ajv({ strict: false }).addSchema(schema(), "model").compile({ $ref: "model#/$defs/Quality" });

/** 🧮️ The meridians of a barrel about the Y axis and the largest gap between the true circle and the mesh, measured at the middle of every edge between two barrel vertices of one ring. */
function barrel(positions: ArrayLike<number>, indices: ArrayLike<number>, radius: number): { meridians: number; sagitta: number } {
  const angles = new Set<number>();
  let sagitta = 0;
  const onBarrel = (i: number): boolean => Math.abs(Math.hypot(positions[i * 3]!, positions[i * 3 + 2]!) - radius) < radius * 1e-6;
  for (let t = 0; t < indices.length; t += 3) for (let e = 0; e < 3; e++) {
    const a = indices[t + e]!, b = indices[t + ((e + 1) % 3)]!;
    if (!onBarrel(a) || !onBarrel(b)) continue;
    angles.add(((Math.round((Math.atan2(positions[a * 3 + 2]!, positions[a * 3]!) * 1e6) / (2 * Math.PI)) % 1e6) + 1e6) % 1e6);
    if (Math.abs(positions[a * 3 + 1]! - positions[b * 3 + 1]!) > radius * 1e-9) continue;
    sagitta = Math.max(sagitta, radius - Math.hypot((positions[a * 3]! + positions[b * 3]!) / 2, (positions[a * 3 + 2]! + positions[b * 3 + 2]!) / 2));
  }
  return { meridians: angles.size, sagitta };
}

describe("📏️ segments of a full circle", () => {
  it("the schema knows the round floor of a quality", () => {
    expect(schema().$defs.Quality.properties.round.required).toEqual(["radiusM", "segments"]);
    for (const c of cases) expect(validateQuality(c.quality), JSON.stringify(validateQuality.errors)).toBe(true);
  });

  for (const c of cases)
    it(`${c.name}: ${c.expected.segments} segments, the sagitta of three's cylinder, smooth barrel and hard rims`, () => {
      const solid = { id: "c", kind: "revolve", node: "root", material: "m", profile: [[0, c.radiusM], [c.heightM, c.radiusM]], axis: [0, 1, 0], origin: [0, 0, 0] } as unknown as Solid;
      const ours = tessellateSolid(solid, c.quality);
      const measured = barrel(ours.positions, ours.indices, c.radiusM);
      const cylinder = new CylinderGeometry(c.radiusM, c.radiusM, c.heightM, c.expected.segments, 1);
      const reference = barrel(cylinder.getAttribute("position").array, cylinder.getIndex()!.array, c.radiusM);
      expect(measured.meridians).toBe(c.expected.segments);
      expect(reference.meridians).toBe(c.expected.segments);
      expect(measured.sagitta / reference.sagitta).toBeCloseTo(1, 3);
      expect(measured.sagitta).toBeCloseTo(c.radiusM * (1 - Math.cos(Math.PI / c.expected.segments)), 12);
      let radial = 0, axial = 0;
      for (let i = 0; i < ours.positions.length; i += 3) {
        const [x, z] = [ours.positions[i]!, ours.positions[i + 2]!];
        const [nx, ny, nz] = [ours.normals[i]!, ours.normals[i + 1]!, ours.normals[i + 2]!];
        const r = Math.hypot(x, z);
        if (Math.abs(ny) > 0.5) axial += 1;
        else {
          radial += 1;
          expect((nx * x + nz * z) / r).toBeGreaterThan(1 - 1e-9);
        }
      }
      expect(radial).toBe(2 * c.expected.segments);
      expect(axial).toBe(2 * c.expected.segments + 2);
    });
});

/** 🦴️ Forward kinematics of a two-link arm against hand-computed vectors and gl-matrix `mat2d`.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🦴️forward-kinematics/🔣️.json — the vectors
 */
import { readFileSync } from "node:fs";
import { glMatrix, mat2d } from "gl-matrix";
import { describe, expect, it } from "vitest";
import type { Rig } from "../../🧬️schema/🟦️.ts";
import { apply, compose, invert, parentFirstOrder, solveRig } from "../../🔨️modules/🦴️rig/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🦴️forward-kinematics/🔣️.json", import.meta.url), "utf8"));
const rig = fixture.rig as Rig;
glMatrix.setMatrixArrayType(Array);

describe("🦴️ forward kinematics", () => {
  for (const c of fixture.cases) {
    it(c.name, () => {
      const world = solveRig(rig, c.offsets, c.root);
      for (const [id, e] of Object.entries(c.expected) as [string, { origin: number[]; tip: number[]; angleDeg: number }][]) {
        const bone = rig.bones.find((b) => b.id === id)!;
        const origin = apply(world[id]!, 0, 0);
        const tip = apply(world[id]!, bone.lengthM, 0);
        expect(origin.x).toBeCloseTo(e.origin[0]!, 12);
        expect(origin.y).toBeCloseTo(e.origin[1]!, 12);
        expect(tip.x).toBeCloseTo(e.tip[0]!, 12);
        expect(tip.y).toBeCloseTo(e.tip[1]!, 12);
        const m = world[id]!;
        expect(((Math.atan2(m[1], m[0]) * 180) / Math.PI + 360) % 360).toBeCloseTo((e.angleDeg + 360) % 360, 9);
      }
    });
  }

  it("agrees with gl-matrix mat2d on a random chain", () => {
    const offsets = { shoulder: { angleDeg: 33 }, elbow: { angleDeg: 71 }, slider: { dx: 0.13 } };
    const root = { x: 0.7, y: -0.2, angleDeg: 17 };
    const world = solveRig(rig, offsets, root);
    const rad = (d: number) => (d * Math.PI) / 180;
    const oracle = mat2d.create();
    mat2d.translate(oracle, oracle, [root.x, root.y]);
    mat2d.rotate(oracle, oracle, rad(root.angleDeg));
    mat2d.rotate(oracle, oracle, rad(33));
    mat2d.translate(oracle, oracle, [1, 0]);
    mat2d.rotate(oracle, oracle, rad(71));
    mat2d.translate(oracle, oracle, [0.5 + 0.13, 0]);
    world.slider!.forEach((v, i) => expect(v).toBeCloseTo(oracle[i]!, 12));
  });

  it("composes, applies and inverts affine matrices", () => {
    const { a, b, composeAB, applyPoint } = fixture.affine;
    expect(compose(a, b)).toEqual(composeAB);
    const p = apply(applyPoint.matrix, applyPoint.point[0], applyPoint.point[1]);
    expect([p.x, p.y]).toEqual(applyPoint.result);
    const m = compose(a, [0.6, 0.8, -0.8, 0.6, 1, 2]);
    const identity = compose(m, invert(m));
    [1, 0, 0, 1, 0, 0].forEach((v, i) => expect(identity[i]).toBeCloseTo(v, 12));
    expect(() => invert([0, 0, 0, 0, 1, 1])).toThrow(/singular/);
  });

  it("orders bones parent-first and rejects broken rigs", () => {
    const shuffled: Rig = { ...rig, bones: [...rig.bones].reverse() };
    const order = parentFirstOrder(shuffled).map((b) => b.id);
    for (const bone of rig.bones) if (bone.parent !== null) expect(order.indexOf(bone.parent)).toBeLessThan(order.indexOf(bone.id));
    expect(() => parentFirstOrder({ ...rig, bones: rig.bones.map((b) => (b.id === "elbow" ? { ...b, parent: "nowhere" } : b)) })).toThrow(/missing parent/);
    expect(() => parentFirstOrder({ ...rig, bones: rig.bones.map((b) => (b.id === "shoulder" ? { ...b, parent: "plate" } : b)) })).toThrow(/must not have a parent/);
    expect(() => parentFirstOrder({ ...rig, bones: rig.bones.map((b) => (b.id === "elbow" ? { ...b, parent: "plate" } : b)) })).toThrow(/cycle/);
    expect(() => parentFirstOrder({ ...rig, bones: [...rig.bones, { ...rig.bones[1]!, id: "second-root", parent: null }] })).toThrow(/no parent but is not the rig root/);
  });
});

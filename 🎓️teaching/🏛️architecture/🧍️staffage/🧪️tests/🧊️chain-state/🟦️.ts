/** 🧊️ Chain state: one 3D chain feeds the elevation, front and plan views; absolute time, one-sided accelerations, respiration and scrubbing.
 *
 * THE REFERENCE is `solveChain` of the rig (three-dimensional rotation of the bone frames, itself checked against numpy in `../🧊️lateral-spread`) and, for the invariance of the parent solid, the rest extents of the same shapes in the same pose without the child turned.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { blendLoads, samplePose } from "../../🔨️modules/🎞️animation/🟦️.ts";
import { evaluate, restShapeDepth, sampleOf, shapeDepth } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { solveChain } from "../../🔨️modules/🦴️rig/🟦️.ts";
import { respirationOffsets, respirationState } from "../../🔨️modules/💨️transport/🫁️respiration/🟦️.ts";
import { depthOrder } from "../../🔨️modules/🎨️depiction/🧊️depth/🟦️.ts";
import { catalog, extentOf } from "../🧊️three-views/🧰️.ts";

const assets = catalog();
const find = (id: string): Asset => assets.find((asset) => asset.id === id)!;
const select = (denominator: number) => ({ kind: "scale", scale: { denominator } }) as const;

describe("🧊️ the state is the solved chain", () => {
  const gull = find("herring-gull");

  it("carries the frame, tip and lateral anchor position of every bone", () => {
    for (const pose of gull.poses) {
      const query = { kind: "pose", pose: pose.id } as const;
      const state = evaluate(gull, query);
      const sample = sampleOf(gull, query);
      const chain = solveChain(gull.rig, sample.offsets, sample.root);
      for (const bone of gull.rig.bones) {
        const f = chain[bone.id]!.frame;
        const b = state.bones[bone.id]!;
        expect(b.frame.o).toEqual({ x: f.o[0], y: f.o[1], z: f.o[2] });
        expect(b.frame.w).toEqual({ x: f.w[0], y: f.w[1], z: f.w[2] });
        expect(b.tip.z).toBeCloseTo(f.o[2] + f.u[2] * bone.lengthM, 12);
        expect(b.origin.x).toBeCloseTo(f.o[0], 9);
        expect(b.origin.y).toBeCloseTo(f.o[1], 9);
      }
      for (const { anchor, z } of Object.values(state.anchors)) {
        const f = chain[anchor.bone]!.frame;
        expect(z).toBeCloseTo(f.o[2] + anchor.x * f.u[2] + anchor.y * f.v[2] + (anchor.z ?? 0) * f.w[2], 12);
      }
      expect(state.root).toEqual({ x: sample.root.x, y: sample.root.y, z: 0, angleDeg: sample.root.angleDeg });
    }
  });

  it("spreads a wing laterally in the soaring pose", () => {
    const state = evaluate(gull, { kind: "pose", pose: "gliding" });
    expect(Math.max(...Object.values(state.bones).map((bone) => Math.abs(bone.tip.z)))).toBeGreaterThan(0.1);
  });
});

describe("🧊️ turning a child bone leaves the solid of its parent unchanged", () => {
  const grill = find("charcoal-grill");
  const lid = (pose: string, view: "elevation" | "front" | "plan") => extentOf(evaluate(grill, { kind: "pose", pose }, select(50)).shapes[view].filter((shape) => shape.shape.id === "lid"), true)!;

  it.each(["elevation", "front", "plan"] as const)("keeps the lid of the grill in the %s view when the vent turns by 70 degrees", (view) => {
    const closed = lid("grilling-closed", view);
    const vented = lid("embers-vents-low", view);
    for (const edge of ["minX", "maxX", "minY", "maxY"] as const) expect(Math.abs(closed[edge] - vented[edge]), `${view} ${edge}`).toBeLessThanOrEqual(1e-6);
  });
});

describe("🧊️ a revolved part on a laterally offset bone keeps its real radius", () => {
  const person = find("person-with-umbrella");

  it("never draws the canopy wider than the real diameter in any view and pose", () => {
    for (const pose of person.poses)
      for (const view of ["front", "plan"] as const) {
        const state = evaluate(person, { kind: "pose", pose: pose.id }, select(50));
        const canopy = state.shapes[view].filter((shape) => shape.shape.solid?.kind === "revolved" && shape.shape.solid.axisBone === "umbrella");
        const extent = extentOf(canopy, true);
        if (extent === undefined) continue;
        const lateral = view === "front" ? extent.maxX - extent.minX : extent.maxY - extent.minY;
        expect(lateral, `${view} ${pose.id}`).toBeLessThanOrEqual(person.dimensions.depthM + 0.01);
      }
  });
});

describe("🧊️ absolute time and clip phase", () => {
  const gull = find("herring-gull");

  it("reports the absolute time and the wrapped clip time", () => {
    const state = evaluate(gull, { kind: "clip", clip: "soar", timeS: 9.5 });
    expect(state.timeS).toBe(9.5);
    expect(state.clipTimeS).toBeCloseTo(1.5, 12);
    expect(state.cycle).toBe(2);
  });

  it("clamps the clip time of a non-looping clip but keeps the absolute time", () => {
    const state = evaluate(gull, { kind: "clip", clip: "landing", timeS: 99 });
    expect(state.timeS).toBe(99);
    expect(state.clipTimeS).toBe(3.2);
  });
});

describe("🧊️ accelerations are continuous at the ends of a clip", () => {
  const gull = find("herring-gull");
  const h = 1 / 240;
  const magnitude = (id: string, t: number): number => {
    const a = evaluate(gull, { kind: "clip", clip: "landing", timeS: t }).kinematics.segmentAccelerations[id]!;
    return Math.hypot(a.x, a.y, a.z);
  };
  const massed = gull.rig.bones.filter((bone) => (bone.mass?.kg ?? 0) > 0).map((bone) => bone.id);

  it("has no jump between the exact end and one step inside", () => {
    const scale = Math.max(...massed.flatMap((id) => [0, 0.4, 0.8, 1.6, 2.4, 3.2].map((t) => magnitude(id, t)))) || 1;
    for (const id of massed) {
      expect(Math.abs(magnitude(id, 0) - magnitude(id, 1.5 * h)), `${id} start`).toBeLessThanOrEqual(0.25 * scale);
      expect(Math.abs(magnitude(id, 3.2) - magnitude(id, 3.2 - 1.5 * h)), `${id} end`).toBeLessThanOrEqual(0.25 * scale);
    }
  });

  it("stays zero outside the clip, where the pose is held", () => {
    for (const id of massed) expect(magnitude(id, 3.2 + 1), id).toBe(0);
  });
});

describe("🧊️ scrubbing equals playing", () => {
  it("gives the same bones at every time on a fresh copy of the asset evaluated backwards", () => {
    const asset = find("herring-gull");
    const clone = JSON.parse(JSON.stringify(asset)) as Asset;
    const times = Array.from({ length: 20 }, (_, i) => i * 0.37);
    const play = times.map((t) => JSON.stringify(evaluate(asset, { kind: "clip", clip: "soar", timeS: t }, select(50)).bones));
    const scrub = [...times].reverse().map((t) => JSON.stringify(evaluate(clone, { kind: "clip", clip: "soar", timeS: t }, select(50)).bones)).reverse();
    expect(scrub).toEqual(play);
  });
});

describe("🧊️ breathing never vanishes", () => {
  it("adds the respiration offset to every driven bone, keyed in the pose or not", () => {
    const checked: string[] = [];
    for (const asset of assets.filter((candidate) => candidate.thermal.respiration !== undefined)) {
      const pose = asset.presentation.defaultPose;
      const base = samplePose(asset, asset.poses.find((p) => p.id === pose)!);
      for (const timeS of [0.3, 0.9, 1.7, 2.6]) {
        const query = { kind: "pose", pose, timeS } as const;
        const respiration = respirationState(asset, query)!;
        const extra = respirationOffsets(asset, respiration);
        const state = evaluate(asset, query);
        for (const [id, add] of Object.entries(extra)) {
          const bone = asset.rig.bones.find((b) => b.id === id)!;
          if (bone.joint.type !== "revolute") continue;
          const expected = (base.offsets[id]?.angleDeg ?? 0) + (add.angleDeg ?? 0);
          const [min, max] = [bone.joint.minDeg ?? -Infinity, bone.joint.maxDeg ?? Infinity];
          expect(state.bones[id]!.offset.angleDeg, `${asset.id} ${id} @${timeS}`).toBeCloseTo(Math.min(Math.max(expected, min), max), 9);
          if ((add.angleDeg ?? 0) !== 0) checked.push(`${asset.id}/${id}`);
        }
      }
    }
    expect(checked.length).toBeGreaterThan(0);
  });
});

describe("🧊️ blended loads keep every component", () => {
  const load = (k: number) => ({ id: "l", kind: "carried", title: { en: "l", de: "l" }, bone: "torso", x: 0.1 * k, y: 0.2 * k, z: 0.3 * k, fxN: 10 * k, fyN: 20 * k, fzN: 30 * k, momentXNm: 4 * k, momentYNm: 5 * k, momentNm: 6 * k, distributed: { bone: "torso", fromM: 0, toM: 0.4, wxNPerM: 1 * k, wyNPerM: 2 * k, wzNPerM: 3 * k }, source: "illustrative" }) as const;

  it("interpolates fz, the three moments, the lateral position and the lateral distributed load", () => {
    const [mid] = blendLoads([load(1)], [load(3)], 0.5);
    const flat = { ...mid!, ...mid!.distributed } as unknown as Record<string, number>;
    for (const [key, value] of Object.entries({ x: 0.2, y: 0.4, z: 0.6, fxN: 20, fyN: 40, fzN: 60, momentXNm: 8, momentYNm: 10, momentNm: 12, wxNPerM: 2, wyNPerM: 4, wzNPerM: 6 })) expect(flat[key], key).toBeCloseTo(value, 9);
  });

  it("fades a load that only one pose has in or out in every component", () => {
    const [faded] = blendLoads([load(2)], [], 0.25);
    expect(faded).toMatchObject({ fxN: 15, fyN: 30, fzN: 45, momentXNm: 6, momentYNm: 7.5, momentNm: 9, distributed: { wzNPerM: 4.5 } });
  });
});

describe("🧊️ depth is the extent of the shape in its own 3D carry", () => {
  it("equals the rest depth at rest for every shape in every view", () => {
    const grill = find("charcoal-grill");
    const state = evaluate(grill, { kind: "pose", pose: "cold-closed" }, select(50));
    for (const view of ["elevation", "front", "plan"] as const)
      for (const { shape } of state.shapes[view]) expect(shapeDepth(state, shape, view), `${view} ${shape.id}`).toBeCloseTo(restShapeDepth(grill, shape, view), 9);
  });

  it("ranks the right of the asset nearer in the elevation when its bone sits beside the symmetry plane", () => {
    const person = JSON.parse(JSON.stringify(find("person-with-umbrella"))) as { rig: { bones: { id: string; z?: number }[] } } & Asset;
    for (const bone of person.rig.bones) if (bone.id === "upper-arm-r") bone.z = -0.23;
    else if (bone.id === "upper-arm-l") bone.z = 0.23;
    const shapes = (person.representations["50"] as unknown as { geometry: { elevation: { id: string; bone: string }[] } }).geometry.elevation;
    const right = shapes.find((s) => s.id === "upper-arm-r")! as Parameters<typeof restShapeDepth>[1];
    const left = shapes.find((s) => s.id === "upper-arm-l")! as Parameters<typeof restShapeDepth>[1];
    expect(restShapeDepth(person, right, "elevation")).toBeGreaterThan(restShapeDepth(person, left, "elevation") + 0.4);
  });
});

describe("🧊️ occlusion under spread and twist", () => {
  const cases = [
    ["herring-gull", "gliding"],
    ["tower-crane", "parked-weathervane"],
  ] as const;

  it.each(cases)("paints %s in a deterministic order in the front and plan view of the spread or twisted pose %s, and is a permutation of its authored order", (id, pose) => {
    const asset = find(id);
    const authored = (view: "front" | "plan") => (evaluate(asset, { kind: "pose", pose }, select(50)).shapes[view] ?? []);
    for (const view of ["front", "plan"] as const) {
      const state = evaluate(asset, { kind: "pose", pose }, select(50));
      const items = state.shapes[view];
      const order = depthOrder(state, view, items);
      expect([...order].sort((a, b) => a - b)).toEqual(items.map((_, i) => i));
      expect(depthOrder(state, view, items)).toEqual(order);
      expect(authored(view).length).toBe(items.length);
    }
  });

  it("ranks a laterally spread wing by its own extent in the plan: the raised wing tip is nearer than the body", () => {
    const gull = find("herring-gull");
    const state = evaluate(gull, { kind: "pose", pose: "gliding" }, select(50));
    const heights = state.shapes.plan.filter((s) => s.shape.role !== "ground").map((s) => shapeDepth(state, s.shape, "plan"));
    expect(Math.max(...heights) - Math.min(...heights)).toBeGreaterThan(0.01);
  });
});

describe("🧊️ a rest z with a compensated shape offset changes no extent", () => {
  it("leaves every view of person-on-crutches unchanged in every pose when the limb roots gain their lateral z and their revolved axes lose it", () => {
    const original = find("person-on-crutches");
    const moved = JSON.parse(JSON.stringify(original)) as Asset;
    const worldZ = (asset: Asset, id: string): number => {
      const byId = new Map(asset.rig.bones.map((bone) => [bone.id, bone] as const));
      let z = 0;
      for (let bone = byId.get(id); bone !== undefined; bone = bone.parent === null ? undefined : byId.get(bone.parent)) z += bone.z ?? 0;
      return z;
    };
    for (const bone of moved.rig.bones as unknown as { id: string; z?: number }[]) if (/^(arm|upper-arm|thigh)-[lr]$/.test(bone.id)) bone.z = (bone.id.endsWith("-l") ? 1 : -1) * 0.2;
    for (const representation of Object.values(moved.representations))
      if (representation.status === "authored")
        for (const view of ["elevation", "front", "plan"] as const)
          for (const shape of representation.geometry[view] ?? []) {
            const solid = shape.solid as { kind: string; axisBone?: string; axisZ?: number } | undefined;
            if (solid?.kind === "revolved") solid.axisZ = (solid.axisZ ?? 0) + worldZ(original, solid.axisBone!) - worldZ(moved, solid.axisBone!);
          }
    for (const pose of original.poses)
      for (const view of ["elevation", "front", "plan"] as const) {
        const a = extentOf(evaluate(original, { kind: "pose", pose: pose.id }, select(50)).shapes[view], true)!;
        const b = extentOf(evaluate(moved, { kind: "pose", pose: pose.id }, select(50)).shapes[view], true)!;
        for (const edge of ["minX", "maxX", "minY", "maxY"] as const) expect(Math.abs(a[edge] - b[edge]), `${pose.id} ${view} ${edge}`).toBeLessThanOrEqual(1e-6);
      }
  });
});

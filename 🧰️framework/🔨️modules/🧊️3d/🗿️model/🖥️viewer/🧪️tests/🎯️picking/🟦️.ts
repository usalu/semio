/** 🎯️ Picking: hits equal three's Raycaster on the same posed meshes (node, distance), misses, back faces, blended zero alpha, pick through a camera ray. */
import { Raycaster, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import type { ModelDocument } from "../../../🧬️schema/🟦️.ts";
import { cameraOf, fitCamera, rayOf } from "../../📷️camera/🟦️.ts";
import { boundsOf, frameOf } from "../../🎞️frame/🟦️.ts";
import { pick, pickAt, solidOf } from "../../🎯️picking/🟦️.ts";
import { sceneOf } from "../../🧱️scene/🟦️.ts";
import { fixture, threeCamera, threeWorld } from "../🧰️support/🟦️.ts";

const doc = fixture("🧱️rig").document as ModelDocument;
const data = fixture("🎯️picking");
const scene = sceneOf(doc);
const camera = cameraOf("front");
const frame = frameOf(scene, { pose: data.pose, camera, aspect: 1 });

describe("pick", () => {
  for (const c of data.cases) {
    it(`${c.name} equals the recorded three hit`, () => {
      const hit = pick(scene, frame, { origin: c.origin, direction: c.direction });
      if (c.node === null) return expect(hit).toBeNull();
      expect(hit?.node).toBe(c.node);
      expect(hit?.solid).toBe(c.solid);
      expect(Math.abs(hit!.distance - c.distance)).toBeLessThan(data.tolerance);
      expect(Math.abs(Math.hypot(...hit!.normal) - 1)).toBeLessThan(1e-9);
    });
  }

  it("agrees with live three raycasts over a grid of rays, including the skinned limb", () => {
    const world = threeWorld(doc, data.pose);
    const rc = new Raycaster();
    let agree = 0, total = 0;
    for (let y = 0; y < 14; y++) for (let x = 0; x < 14; x++) {
      const origin = new Vector3(-1.4 + x * 0.2, -0.1 + y * 0.25, 6), dir = new Vector3(0, 0, -1);
      rc.set(origin, dir);
      const expected = rc.intersectObjects(world.meshes, false)[0];
      const hit = pick(scene, frame, { origin: origin.toArray() as [number, number, number], direction: [0, 0, -1] });
      total++;
      if (!expected && !hit) agree++;
      else if (expected && hit && hit.node === expected.object.userData.node && Math.abs(hit.distance - expected.distance) < 1e-4) agree++;
    }
    expect(agree / total).toBeGreaterThan(0.98);
  });

  it("returns the id of the solid that produced the triangle, also on the skinned and the multi-solid meshes", () => {
    const solids = new Set<string>();
    for (let y = 0; y < 14; y++) for (let x = 0; x < 14; x++) {
      const hit = pick(scene, frame, { origin: [-1.4 + x * 0.2, -0.1 + y * 0.25, 6], direction: [0, 0, -1] });
      if (hit) solids.add(`${hit.node}:${hit.solid}`);
    }
    expect([...solids].sort()).toEqual(expect.arrayContaining(["head:head", "root:pad", "halo-b:b", "skinned:limb"]));
    expect([...solids].every((entry) => !entry.endsWith(":null"))).toBe(true);
  });

  it("reports solid null for a document without provenance", () => {
    const stripped = { ...doc, meshes: doc.meshes.map((m) => ({ ...m, primitives: m.primitives.map(({ solids: _, ...rest }) => rest) })) } as ModelDocument;
    const bare = sceneOf(stripped);
    const hit = pick(bare, frameOf(bare, { pose: data.pose, camera, aspect: 1 }), { origin: [0.4, 4, 0.1], direction: [0, -1, 0] });
    expect(hit?.node).toBe("root");
    expect(hit?.solid).toBeNull();
  });

  it("solidOf finds the range of an index by binary search", () => {
    const ranges = [{ id: "a", first: 0, count: 6 }, { id: "b", first: 6, count: 3 }, { id: "c", first: 9, count: 12 }];
    expect([0, 5, 6, 8, 9, 20, 21, -1].map((i) => solidOf(ranges, i))).toEqual(["a", "a", "b", "b", "c", "c", null, null]);
    expect(solidOf([], 0)).toBeNull();
  });

  it("returns the point and the front-facing normal", () => {
    const hit = pick(scene, frame, { origin: [0.4, 4, 0.1], direction: [0, -1, 0] })!;
    expect(hit.node).toBe("root");
    expect(Math.abs(hit.point[1] - 0)).toBeLessThan(1e-6);
    expect(hit.normal[1]).toBeGreaterThan(0.99);
    expect(hit.tags).toEqual([]);
  });

  it("ignores back faces of single-sided materials and hits both sides of double-sided ones", () => {
    const rest = frameOf(scene, { camera, aspect: 1 });
    expect(pick(scene, rest, { origin: [0, 0.4, 0], direction: [1, 0, 0] })).toBeNull();
    expect(pick(scene, rest, { origin: [0.4, 0.4, 0], direction: [-1, 0, 0] })?.node).toBe("skinned");
    expect(pick(scene, rest, { origin: [0.4, -3, 0.1], direction: [0, 1, 0] })?.node).toBe("root");
  });

  it("reports the variant-resolved material", () => {
    const f = frameOf(scene, { pose: data.pose, variant: "dark", camera, aspect: 1 });
    expect(pick(scene, f, { origin: [5, 0.4, 0], direction: [-1, 0, 0] })!.material).toBe("skin-dark");
    expect(pick(scene, frame, { origin: [5, 0.4, 0], direction: [-1, 0, 0] })!.material).toBe("skin");
  });

  it("picks through the camera: the ray of the head's projected centre hits the head", () => {
    const f0 = frameOf(scene, { pose: data.pose, camera, aspect: 1 });
    const cam = fitCamera(cameraOf("perspective"), boundsOf(scene, f0), 1);
    const f = frameOf(scene, { pose: data.pose, camera: cam, aspect: 1 });
    const world = threeWorld(doc, data.pose);
    const head = new Vector3().setFromMatrixPosition(world.objects.get("head")!.matrixWorld);
    const ndc = head.clone().project(threeCamera(cam, 1));
    const hit = pickAt(scene, f, cam, 1, ndc.x, ndc.y);
    expect(hit?.node).toBe("head");
    const r = rayOf(cam, 1, ndc.x, ndc.y);
    expect(hit!.point.every((v, i) => Math.abs(v - (r.origin[i]! + r.direction[i]! * hit!.distance)) < 1e-9)).toBe(true);
  });
});

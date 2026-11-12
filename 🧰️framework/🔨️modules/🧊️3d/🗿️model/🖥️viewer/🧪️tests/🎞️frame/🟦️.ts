/** 🎞️ Frames: joint palettes and CPU-skinned vertices equal three's Skeleton/SkinnedMesh, variant and animated material colours, draw list order, camera-dependent blend sorting, frame reuse, rejections. */
import { Box3, Matrix4, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import type { ModelDocument } from "../../../🧬️schema/🟦️.ts";
import { MATERIAL_STRIDE } from "../../🧬️contract/🟦️.ts";
import { cameraOf } from "../../📷️camera/🟦️.ts";
import { boundsOf, createFrame, frameOf, posedPositions } from "../../🎞️frame/🟦️.ts";
import { sceneOf } from "../../🧱️scene/🟦️.ts";
import { fixture, threeWorld } from "../🧰️support/🟦️.ts";

const doc = fixture("🧱️rig").document as ModelDocument;
const data = fixture("🎞️frame");
const scene = sceneOf(doc);
const camera = cameraOf("front");

describe("pose and palette", () => {
  for (const c of data.poses) {
    it(`${c.name}: palette equals three's skeleton matrices`, () => {
      const f = frameOf(scene, { pose: c.pose, camera, aspect: 1 });
      expect(f.palette.length).toBe(c.palette.length);
      for (let i = 0; i < c.palette.length; i++) expect(Math.abs(f.palette[i]! - c.palette[i])).toBeLessThan(data.tolerance);
    });
  }

  it("skins vertices like three's SkinnedMesh for a pose and a clip time", () => {
    for (const [key, pose] of [["bent", "bent"], ["wave", { animation: "wave", timeS: 0.5 }]] as const) {
      const f = frameOf(scene, { pose, camera, aspect: 1 });
      const prim = scene.primitives.find((p) => p.skin >= 0 && p.mode === "triangles")!;
      const w = posedPositions(scene, f, prim.index);
      const world = threeWorld(doc, pose);
      for (const [v, x, y, z] of data.skinnedVertices[key]) expect(new Vector3(w[v * 3], w[v * 3 + 1], w[v * 3 + 2]).distanceTo(new Vector3(x, y, z))).toBeLessThan(2e-5);
      const m = world.skinned[0]!;
      for (let v = 0; v < prim.vertexCount; v += 5) {
        const p = new Vector3();
        m.getVertexPosition(v, p);
        expect(p.distanceTo(new Vector3(w[v * 3], w[v * 3 + 1], w[v * 3 + 2]))).toBeLessThan(2e-5);
      }
    }
  });

  it("places rigid primitives with their node world matrix (three)", () => {
    const f = frameOf(scene, { pose: "bent", camera, aspect: 1 });
    const world = threeWorld(doc, "bent");
    const prim = scene.primitives.find((p) => p.node === "head" && p.mode === "triangles")!;
    const w = posedPositions(scene, f, prim.index);
    const m = new Matrix4().fromArray(Array.from(f.matrices.subarray(prim.nodeIndex * 16, prim.nodeIndex * 16 + 16)));
    expect(m.elements.every((v, i) => Math.abs(v - world.objects.get("head")!.matrixWorld.elements[i]!) < 1e-6)).toBe(true);
    const p0 = new Vector3(scene.vertices[prim.vertexStart * 10], scene.vertices[prim.vertexStart * 10 + 1], scene.vertices[prim.vertexStart * 10 + 2]).applyMatrix4(world.objects.get("head")!.matrixWorld);
    expect(p0.distanceTo(new Vector3(w[0], w[1], w[2]))).toBeLessThan(1e-6);
  });

  it("surfaces joint-limit findings and rejects unknown poses, clips and variants", () => {
    const tight: ModelDocument = { ...doc, poses: [{ id: "over", title: { en: "o", de: "o" }, nodes: { upper: { rotation: [0, 0, Math.sin(1.4), Math.cos(1.4)] } } }] };
    expect(frameOf(sceneOf(tight), { pose: "over", camera, aspect: 1 }).issues.map((i) => i.code)).toContain("joint-limit");
    expect(() => frameOf(scene, { pose: "ghost", camera, aspect: 1 })).toThrow();
    expect(() => frameOf(scene, { pose: { animation: "ghost", timeS: 0 }, camera, aspect: 1 })).toThrow();
    expect(() => frameOf(scene, { variant: "ghost", camera, aspect: 1 })).toThrow(/unknown variant/);
  });

  it("bounds equal the union of posed vertices and move with the pose", () => {
    const rest = boundsOf(scene, frameOf(scene, { camera, aspect: 1 }));
    const bent = boundsOf(scene, frameOf(scene, { pose: "bent", camera, aspect: 1 }));
    expect(Math.abs(rest.max[1] - bent.max[1])).toBeGreaterThan(0.05);
    const world = threeWorld(doc, "bent");
    const box = new Box3();
    box.setFromObject(world.root, true);
    expect(Math.abs(bent.max[1] - box.max.y)).toBeLessThan(0.05);
  });
});

describe("materials", () => {
  for (const c of data.materials) {
    it(`${c.material} under ${c.variant ?? "base"} at ${JSON.stringify(c.pose)}`, () => {
      const f = frameOf(scene, { pose: c.pose ?? undefined, variant: c.variant, camera, aspect: 1 });
      const o = scene.materialIndex.get(c.material)! * MATERIAL_STRIDE;
      c.baseColor.forEach((v: number, k: number) => expect(Math.abs(f.materials[o + k]! - v)).toBeLessThan(1e-6));
      c.emissive.forEach((v: number, k: number) => expect(Math.abs(f.materials[o + 4 + k]! - v)).toBeLessThan(1e-6));
    });
  }

  it("maps primitives of bound materials to the variant's slot", () => {
    const f = frameOf(scene, { variant: "dark", camera, aspect: 1 });
    const limb = f.draws.slice(0, f.drawCount).find((d) => scene.primitives[d.primitive]!.node === "skinned" && d.pass === 0)!;
    expect(limb.material).toBe(scene.materialIndex.get("skin-dark"));
    const base = frameOf(scene, { variant: null, camera, aspect: 1 });
    expect(base.draws.slice(0, base.drawCount).find((d) => scene.primitives[d.primitive]!.node === "skinned" && d.pass === 0)!.material).toBe(scene.materialIndex.get("skin"));
  });
});

describe("draw list", () => {
  const names = (f: ReturnType<typeof frameOf>) => f.draws.slice(0, f.drawCount).map((d) => `${d.pass}:${scene.primitives[d.primitive]!.node}`);
  it("lists every primitive once: opaque triangles, blended triangles, lines", () => {
    const f = frameOf(scene, { camera, aspect: 1 });
    expect(f.drawCount).toBe(scene.primitives.length);
    expect(new Set(f.draws.slice(0, f.drawCount).map((d) => d.primitive)).size).toBe(scene.primitives.length);
    const passes = f.draws.slice(0, f.drawCount).map((d) => d.pass);
    expect([...passes].sort()).toEqual(passes);
    expect(passes.filter((p) => p === 1)).toHaveLength(2);
    expect(passes.filter((p) => p === 2)).toHaveLength(scene.primitives.filter((p) => p.mode === "lines").length);
  });
  it("sorts blended draws back to front per camera", () => {
    const front = frameOf(scene, { camera: { ...cameraOf("front"), target: [0, 1, 0], distance: 6 }, aspect: 1 });
    expect(names(front).filter((n) => n.startsWith("1:"))).toEqual(["1:halo-a", "1:halo-b"]);
    const back = frameOf(scene, { camera: { ...cameraOf("front"), yaw: Math.PI, target: [0, 1, 0], distance: 6 }, aspect: 1 });
    expect(names(back).filter((n) => n.startsWith("1:"))).toEqual(["1:halo-b", "1:halo-a"]);
  });
  it("draws skinned primitives with the identity matrix and their palette row, rigid ones with the node matrix", () => {
    const f = frameOf(scene, { pose: "bent", camera, aspect: 1 });
    for (const d of f.draws.slice(0, f.drawCount)) {
      const p = scene.primitives[d.primitive]!;
      if (p.skin >= 0) {
        expect(d.paletteOffset).toBe(scene.skins[p.skin]!.paletteOffset);
        expect(Array.from(f.matrices.subarray(d.matrixOffset, d.matrixOffset + 16))).toEqual([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
      } else {
        expect(d.paletteOffset).toBe(-1);
        expect(d.matrixOffset).toBe(p.nodeIndex * 16);
      }
    }
  });
  it("reuses the frame's arrays and draw objects without reallocating", () => {
    const f = createFrame(scene);
    const before = [f.view, f.matrices, f.palette, f.materials, f.draws, f.draws[0], f.light.direction];
    frameOf(scene, { pose: "bent", camera, aspect: 1 }, f);
    frameOf(scene, { pose: { animation: "wave", timeS: 1 }, variant: "dark", camera: cameraOf("plan"), aspect: 2 }, f);
    expect([f.view, f.matrices, f.palette, f.materials, f.draws, f.draws[0] ?? null, f.light.direction].length).toBe(before.length);
    before.slice(0, 5).forEach((x, i) => expect([f.view, f.matrices, f.palette, f.materials, f.draws][i]).toBe(x));
    expect(f.draws.length).toBe(scene.primitives.length);
  });
  it("lights from the camera by default and fixed in the world on request", () => {
    const a = frameOf(scene, { camera: cameraOf("front"), aspect: 1 });
    const b = frameOf(scene, { camera: cameraOf("elevation"), aspect: 1 });
    expect(Array.from(a.light.direction)).not.toEqual(Array.from(b.light.direction));
    const w1 = frameOf(scene, { camera: cameraOf("front"), aspect: 1, lighting: { mode: "world", direction: [0, 1, 0] } });
    const w2 = frameOf(scene, { camera: cameraOf("plan"), aspect: 1, lighting: { mode: "world", direction: [0, 1, 0] } });
    expect(Array.from(w1.light.direction)).toEqual([0, 1, 0]);
    expect(Array.from(w2.light.direction)).toEqual([0, 1, 0]);
  });
});

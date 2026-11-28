/** 🦴️ Skinning: weights, joint order, inverse bind matrices, rigid and skinned meshes; the posed skin is evaluated by three's Skeleton/SkinnedMesh from the kernel's local transforms and compared with the hand-computed linear blend. */
import { Bone, Box3, BoxGeometry, BufferAttribute, BufferGeometry, Matrix4, Mesh, Object3D, Skeleton, SkinnedMesh, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import { poseModel, tessellate, validateModel } from "../../🟦️.ts";
import { fixture } from "../🧰️support/🟦️.ts";

const data = fixture("🦴️skinning");
const e = data.expected;

describe("skinning", () => {
  it("binds weighted solids to one skin and keeps rigid solids on their nodes", async () => {
    const out = await tessellate(data.document, data.quality);
    expect(validateModel(out).filter((i) => i.severity === "error")).toEqual([]);
    expect(out.skins).toHaveLength(1);
    expect(out.skins[0]!.joints).toEqual(e.jointOrder);
    const lower = out.skins[0]!.inverseBind[1]!;
    expect([lower[12], lower[13], lower[14]]).toEqual(e.inverseBindLowerTranslation);
    const skinnedNode = out.nodes.find((n) => n.skin)!;
    expect(skinnedNode.parent).toBeNull();
    const skinned = out.meshes.find((m) => m.id === skinnedNode.mesh)!;
    expect(skinned.primitives).toHaveLength(e.skinnedMeshes);
    const p = skinned.primitives[0]!;
    for (let v = 0; v < p.positions.length / 3; v++) {
      expect(p.joints!.slice(v * 4, v * 4 + 2)).toEqual([0, 1]);
      expect(p.weights!.slice(v * 4, v * 4 + 4)).toEqual(e.weights);
    }
    expect(out.nodes.filter((n) => n.mesh && !n.skin).map((n) => n.id)).toEqual(e.rigidMeshes);
  });

  it("evaluates the pose with clamped joints and reports the clamp", async () => {
    const out = await tessellate(data.document, data.quality);
    const pose = poseModel(out, data.pose);
    const w = pose.world.lower!;
    for (let k = 0; k < 3; k++) expect(Math.abs(w[12 + k]! - e.lowerWorldPosition[k])).toBeLessThan(e.tolerance);
    expect(Math.abs(Math.atan2(w[1]!, w[0]!) - e.lowerWorldAngleZ)).toBeLessThan(e.tolerance);
    expect(pose.issues.map((i) => ({ code: i.code, path: i.path }))).toEqual(e.issues);
  });

  it("skins like three's SkinnedMesh", async () => {
    const out = await tessellate(data.document, data.quality);
    const pose = poseModel(out, data.pose);
    const objects = new Map<string, Object3D>();
    for (const n of out.nodes) {
      const o = n.skin || n.mesh ? new Object3D() : new Bone();
      objects.set(n.id, o);
    }
    for (const n of out.skins[0]!.joints) objects.set(n, new Bone());
    const apply = (rest: boolean) => {
      for (const n of out.nodes) {
        const o = objects.get(n.id)!;
        const t = rest ? n : pose.local[n.id]!;
        o.position.fromArray(t.translation as number[]);
        o.quaternion.fromArray([...(rest ? n.rotation : t.rotation)]);
        o.scale.fromArray(t.scale as number[]);
      }
    };
    const scene = new Object3D();
    for (const n of out.nodes) (n.parent ? objects.get(n.parent)! : scene).add(objects.get(n.id)!);
    apply(true);
    scene.updateMatrixWorld(true);
    const bones = out.skins[0]!.joints.map((j) => objects.get(j) as Bone);
    const skeleton = new Skeleton(bones);
    out.skins[0]!.inverseBind.forEach((m, i) => skeleton.boneInverses[i]!.toArray().forEach((v, k) => expect(Math.abs(v - m[k]!)).toBeLessThan(1e-9)));
    const p = out.meshes.find((m) => m.id === out.nodes.find((n) => n.skin)!.mesh)!.primitives[0]!;
    const g = new BufferGeometry();
    g.setAttribute("position", new BufferAttribute(Float32Array.from(p.positions), 3));
    g.setAttribute("skinIndex", new BufferAttribute(Uint16Array.from(p.joints!), 4));
    g.setAttribute("skinWeight", new BufferAttribute(Float32Array.from(p.weights!), 4));
    const mesh = new SkinnedMesh(g);
    scene.add(mesh);
    mesh.bind(skeleton, new Matrix4());
    apply(false);
    scene.updateMatrixWorld(true);
    skeleton.update();
    const bind = out.skins[0]!.inverseBind;
    let moved = 0;
    for (let v = 0; v < p.positions.length / 3; v += 7) {
      const target = new Vector3().fromArray(p.positions, v * 3);
      mesh.applyBoneTransform(v, target);
      const expected = new Vector3();
      for (let j = 0; j < 2; j++) {
        const world = new Matrix4().fromArray(pose.world[out.skins[0]!.joints[j]!]!);
        const skinMatrix = world.multiply(new Matrix4().fromArray([...bind[j]!]));
        expected.add(new Vector3().fromArray(p.positions, v * 3).applyMatrix4(skinMatrix).multiplyScalar(p.weights![v * 4 + j]!));
      }
      expect(target.distanceTo(expected)).toBeLessThan(1e-5);
      if (target.distanceTo(new Vector3().fromArray(p.positions, v * 3)) > 0.05) moved++;
    }
    expect(moved).toBeGreaterThan(0);
  });

  it("places rigid meshes with their node world matrix", async () => {
    const out = await tessellate(data.document, data.quality);
    const pose = poseModel(out, data.pose);
    const hand = out.meshes.find((m) => m.id === out.nodes.find((n) => n.id === "lower")!.mesh)!.primitives[0]!;
    const world = new Matrix4().fromArray(pose.world.lower!);
    const c = new Vector3();
    for (let v = 0; v < hand.positions.length / 3; v++) c.add(new Vector3().fromArray(hand.positions, v * 3).applyMatrix4(world));
    c.divideScalar(hand.positions.length / 3);
    expect(c.distanceTo(new Vector3(...(e.lowerWorldPosition as [number, number, number])))).toBeLessThan(0.01);
  });

  it("repeats a weighted solid in the frame of its node, where three places the same copies as children of that node", async () => {
    const out = await tessellate(data.repeated.document, data.quality);
    expect(validateModel(out).filter((i) => i.severity === "error")).toEqual([]);
    const p = out.meshes.find((m) => m.id === out.nodes.find((n) => n.skin)!.mesh)!.primitives[0]!;
    const solid = data.repeated.document.solids[0];
    const node = data.repeated.document.nodes[0];
    const carrier = new Object3D();
    carrier.position.fromArray(node.translation);
    carrier.quaternion.fromArray(node.rotation);
    const perCopy = p.positions.length / solid.repeat.count;
    expect(Number.isInteger(perCopy / 3)).toBe(true);
    for (let k = 0; k < solid.repeat.count; k++) {
      const child = new Mesh(new BoxGeometry(...(solid.size as [number, number, number])));
      child.position.fromArray(solid.transform.translation.map((v: number, at: number) => v + k * solid.repeat.translation[at]));
      carrier.add(child);
      carrier.updateMatrixWorld(true);
      const reference = new Box3().expandByObject(child, true);
      const ours = new Box3().setFromArray(p.positions.slice(k * perCopy, (k + 1) * perCopy));
      const want = data.repeated.expected.copies[k];
      for (let at = 0; at < 3; at++) {
        expect(Math.abs(ours.min.getComponent(at) - want.min[at])).toBeLessThan(data.repeated.expected.tolerance);
        expect(Math.abs(ours.max.getComponent(at) - want.max[at])).toBeLessThan(data.repeated.expected.tolerance);
        expect(Math.abs(ours.min.getComponent(at) - reference.min.getComponent(at))).toBeLessThan(1e-6);
        expect(Math.abs(ours.max.getComponent(at) - reference.max.getComponent(at))).toBeLessThan(1e-6);
      }
    }
  });
});

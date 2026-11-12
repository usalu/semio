import { readFileSync } from "node:fs";
import { Bone, Matrix4, Object3D, Quaternion, Skeleton, SkinnedMesh, Triangle, Vector3 } from "three";
import type { BufferAttribute, Material as ThreeMaterial, MeshPhongMaterial } from "three";
import { FBXLoader } from "three/examples/jsm/loaders/FBXLoader.js";
import { afterAll, beforeAll, describe, expect, it, vi } from "vitest";
import type { Animation, Channel, ModelDocument, Primitive } from "../../../../🧬️schema/🟦️.ts";
import { tessellate } from "../../../../🔺️tessellation/🟦️.ts";
import { encodeFbx } from "../../🟦️.ts";
import { DOCUMENT_FIXTURES, NO_OPERATION, fixture, localMatrix, worldMatricesOf } from "../🧰️support/🟦️.ts";
import type { FbxCase, FbxFixture } from "../🧰️support/🟦️.ts";

const TOLERANCE = 1e-4;

type Interpolating = { createInterpolant(): { evaluate(time: number): ArrayLike<number> } };

function load(bytes: Uint8Array): Object3D {
  const buffer = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer;
  return new FBXLoader().parse(buffer, "");
}

function close(actual: ArrayLike<number>, expected: ArrayLike<number>, tolerance = TOLERANCE): void {
  expect(actual.length).toBe(expected.length);
  for (let index = 0; index < expected.length; index++) expect(Math.abs(actual[index] - expected[index])).toBeLessThanOrEqual(tolerance);
}

type Corner = { position: number[]; normal: number[]; colour: number[]; weights: Map<number, number>; material: string };

function cornersOf(document: ModelDocument, meshId: string, variant: string | undefined): Corner[] {
  const mesh = document.meshes.find((item) => item.id === meshId)!;
  const bindings = document.variants.find((item) => item.id === variant)?.bindings ?? {};
  const out: Corner[] = [];
  for (const primitive of mesh.primitives.filter((item: Primitive) => item.mode === "triangles"))
    for (let triangle = 0; triangle < primitive.indices.length; triangle += 3) {
      const ids = [primitive.indices[triangle], primitive.indices[triangle + 1], primitive.indices[triangle + 2]];
      const at = (vertex: number): number[] => Array.from(primitive.positions.slice(vertex * 3, vertex * 3 + 3));
      const flat = new Triangle(new Vector3(...at(ids[0])), new Vector3(...at(ids[1])), new Vector3(...at(ids[2]))).getNormal(new Vector3()).toArray();
      for (const vertex of ids) {
        const weights = new Map<number, number>();
        if (primitive.joints && primitive.weights) for (let slot = 0; slot < 4; slot++) if (primitive.weights[vertex * 4 + slot] > 0) weights.set(primitive.joints[vertex * 4 + slot], (weights.get(primitive.joints[vertex * 4 + slot]) ?? 0) + primitive.weights[vertex * 4 + slot]);
        out.push({
          position: at(vertex),
          normal: primitive.normals ? Array.from(primitive.normals.slice(vertex * 3, vertex * 3 + 3)) : flat,
          colour: primitive.colors ? Array.from(primitive.colors.slice(vertex * 4, vertex * 4 + 3)) : [1, 1, 1],
          weights,
          material: bindings[primitive.material] ?? primitive.material,
        });
      }
    }
  return out;
}

function checkTracks(document: ModelDocument, animation: Animation, scene: Object3D): void {
  const clip = (scene.animations ?? [])[0];
  expect(clip.name).toBe(animation.id);
  const seen = new Set<string>();
  for (const channel of animation.channels as readonly Channel[]) {
    if (!("node" in channel.target) || channel.times.length < 2) continue;
    const key = `${channel.target.node}.${channel.target.path}`;
    if (seen.has(key)) continue;
    seen.add(key);
    const trackName = `${channel.target.node}.${channel.target.path === "translation" ? "position" : channel.target.path === "rotation" ? "quaternion" : "scale"}`;
    const track = clip.tracks.find((item) => item.name === trackName);
    expect(track, trackName).toBeDefined();
    const size = channel.target.path === "rotation" ? 4 : 3;
    const interpolant = (track as unknown as Interpolating).createInterpolant();
    channel.times.forEach((time, index) => {
      expect(track!.times.some((candidate) => Math.abs(candidate - time) < 1e-9)).toBe(true);
      const got = Array.from(interpolant.evaluate(time));
      const want = Array.from(channel.values.slice(index * size, index * size + size));
      if (size === 4) {
        const sign = got[0] * want[0] + got[1] * want[1] + got[2] * want[2] + got[3] * want[3] < 0 ? -1 : 1;
        close(got.map((value) => value * sign), want, 2e-4);
      } else close(got, want, 2e-5);
    });
    if (channel.target.path === "rotation" && channel.interpolation === "linear")
      for (let index = 0; index + 1 < channel.times.length; index++) {
        const mid = (channel.times[index] + channel.times[index + 1]) / 2;
        const a = new Quaternion(...channel.values.slice(index * 4, index * 4 + 4)).normalize();
        const b = new Quaternion(...channel.values.slice(index * 4 + 4, index * 4 + 8)).normalize();
        const want = a.clone().slerp(b, 0.5);
        const got = new Quaternion().fromArray(Array.from(interpolant.evaluate(mid)));
        expect(Math.abs(got.dot(want))).toBeGreaterThan(0.9999);
      }
  }
}

function check(document: ModelDocument, item: FbxCase, scene: Object3D): void {
  scene.updateMatrixWorld(true);
  const world = worldMatricesOf(document);
  const byName = (id: string): Object3D => scene.getObjectByName(id)!;
  const jointIds = new Set(document.skins.flatMap((skin) => skin.joints));
  for (const node of document.nodes) {
    const object = byName(node.id);
    expect(object, node.id).toBeDefined();
    const parent = node.parent === null ? undefined : byName(node.parent);
    if (parent) expect(object.parent).toBe(parent);
    close(object.matrixWorld.elements, world.get(node.id)!.elements, 2e-4);
    if (node.mesh === undefined && (jointIds.has(node.id) || node.joint)) expect(object instanceof Bone, node.id).toBe(true);
    if (node.parent !== null) {
      const local = new Matrix4().compose(object.position, object.quaternion, object.scale);
      close(local.elements, localMatrix(node).elements, 2e-4);
    }
  }
  for (const node of document.nodes.filter((entry) => entry.mesh !== undefined)) {
    const mesh = byName(node.id) as SkinnedMesh;
    const corners = cornersOf(document, node.mesh!, item.options.variant);
    const position = mesh.geometry.getAttribute("position") as BufferAttribute;
    expect(position.count).toBe(corners.length);
    corners.forEach((corner, index) => {
      close([position.getX(index), position.getY(index), position.getZ(index)], corner.position);
      const normal = mesh.geometry.getAttribute("normal") as BufferAttribute;
      close([normal.getX(index), normal.getY(index), normal.getZ(index)], corner.normal, 1e-4);
    });
    const colour = mesh.geometry.getAttribute("color") as BufferAttribute | undefined;
    const hasColour = document.meshes.find((entry) => entry.id === node.mesh)!.primitives.some((primitive) => primitive.colors);
    expect(colour !== undefined).toBe(hasColour);
    if (colour) corners.forEach((corner, index) => close([colour.getX(index), colour.getY(index), colour.getZ(index)], corner.colour, 1e-4));
    const materials = Array.isArray(mesh.material) ? mesh.material : [mesh.material];
    corners.forEach((corner, index) => {
      const group = mesh.geometry.groups.find((candidate) => index >= candidate.start && index < candidate.start + candidate.count)!;
      const loaded = materials[group.materialIndex!] as MeshPhongMaterial;
      const source = document.materials.find((entry) => entry.id === corner.material)!;
      close([loaded.color.r, loaded.color.g, loaded.color.b], source.baseColor.slice(0, 3), 1e-4);
      close([loaded.emissive.r, loaded.emissive.g, loaded.emissive.b], source.emissive, 1e-4);
      expect(loaded.opacity).toBeCloseTo(source.alphaMode === "blend" ? source.baseColor[3] : 1, 5);
      expect(loaded.transparent).toBe(source.alphaMode === "blend" && source.baseColor[3] < 1);
      expect((loaded as ThreeMaterial & { isMeshLambertMaterial?: boolean }).isMeshLambertMaterial === true).toBe(source.unlit);
    });
    if (node.skin !== undefined) {
      const skin = document.skins.find((entry) => entry.id === node.skin)!;
      expect(mesh.isSkinnedMesh).toBe(true);
      const skeleton: Skeleton = mesh.skeleton;
      expect(skeleton.bones.map((bone) => bone.name)).toEqual(skin.joints);
      close(mesh.bindMatrix.elements, world.get(node.id)!.elements, 2e-4);
      skin.joints.forEach((_, joint) => close(skeleton.boneInverses[joint].elements, new Matrix4().fromArray(skin.inverseBind[joint] as unknown as number[]).multiply(world.get(node.id)!.clone().invert()).elements, 2e-4));
      const indices = mesh.geometry.getAttribute("skinIndex") as BufferAttribute;
      const weights = mesh.geometry.getAttribute("skinWeight") as BufferAttribute;
      corners.forEach((corner, index) => {
        const got = new Map<number, number>();
        for (let slot = 0; slot < 4; slot++) if (weights.getComponent(index, slot) > 1e-9) got.set(indices.getComponent(index, slot), weights.getComponent(index, slot));
        expect([...got.keys()].sort()).toEqual([...corner.weights.keys()].sort());
        for (const [joint, weight] of corner.weights) expect(got.get(joint)!).toBeCloseTo(weight, 5);
      });
    }
  }
  if (item.options.animation !== undefined) checkTracks(document, document.animations.find((entry) => entry.id === item.options.animation)!, scene);
}

/** 🧍️ three's FBXLoader as the independent oracle: geometry, skeleton hierarchy, bind pose, skin weights, animation tracks and material colours read back against the source document. */
describe("FBX encoder against three FBXLoader", () => {
  const silence = vi.spyOn(console, "warn").mockImplementation(() => {});
  beforeAll(() => silence.mockClear());
  afterAll(() => silence.mockRestore());

  for (const name of DOCUMENT_FIXTURES) {
    const { document, cases } = fixture<FbxFixture>(name);
    for (const item of cases)
      it(`${name} / ${item.name}`, () => {
        const scene = load(encodeFbx(document, item.options, NO_OPERATION));
        check(document, item, scene);
      });
  }

  it("loads the thermal variant with the bound materials of the variant", () => {
    const { document } = fixture<FbxFixture>("rigged-arm");
    const scene = load(encodeFbx(document, { variant: "thermal", format: "binary" }, NO_OPERATION));
    const mesh = scene.getObjectByName("body") as SkinnedMesh;
    const materials = (Array.isArray(mesh.material) ? mesh.material : [mesh.material]) as MeshPhongMaterial[];
    expect(materials.map((material) => material.name).sort()).toEqual(["paper", "skin-hot"]);
  });

  it("warns only about what FBX colour curves inherently cause in three (a colour-curve link reads as a missing texture, its curve as unused)", () => {
    const { document, cases } = fixture<FbxFixture>("rigged-arm");
    silence.mockClear();
    load(encodeFbx(document, cases[1].options, NO_OPERATION));
    const messages = silence.mock.calls.map((call) => String(call[0]));
    expect(messages.filter((message) => !message.includes("unused curve") && !/parameter '(emissiveMap|map)' has value of undefined/.test(message))).toEqual([]);
  });

  it("keeps the geometry of a joint that also holds a mesh: a LimbNode with a child Mesh model, so three's loader still sees every triangle", async () => {
    const data = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/🧬️provenance/🔣️.json", import.meta.url), "utf8"));
    const resolved = await tessellate(data.document, data.quality);
    const expected = resolved.meshes.flatMap((m) => m.primitives).filter((p) => p.mode === "triangles").reduce((k: number, p) => k + p.indices.length / 3, 0);
    const scene = load(encodeFbx(resolved, { format: "binary" }, NO_OPERATION));
    let triangles = 0;
    scene.traverse((object) => {
      const geometry = (object as { geometry?: { index?: { count: number }; attributes: { position: { count: number } } } }).geometry;
      if (geometry !== undefined) triangles += (geometry.index?.count ?? geometry.attributes.position.count) / 3;
    });
    expect(triangles).toBe(expected);
    for (const joint of resolved.nodes.filter((n) => n.mesh !== undefined && n.joint !== undefined)) expect(scene.getObjectByName(joint.id), joint.id).toBeDefined();
  });
});

import { Matrix4, Quaternion } from "three";
import { beforeAll, describe, expect, it } from "vitest";
import type { ModelDocument } from "../../../../🧬️schema/🟦️.ts";
import { encodeFbx } from "../../🟦️.ts";
import { DOCUMENT_FIXTURES, NO_OPERATION, fixture, worldMatricesOf } from "../🧰️support/🟦️.ts";
import type { FbxCase, FbxFixture } from "../🧰️support/🟦️.ts";
import { loadAssimp } from "../🧰️support/🧩️assimpjs/🟦️.ts";
import type { Assimp } from "../🧰️support/🧩️assimpjs/🟦️.ts";

type AssimpNode = { name: string; transformation: number[]; children?: AssimpNode[]; meshes?: number[] };
type AssimpScene = {
  rootnode: AssimpNode;
  meshes?: { materialindex: number; vertices: number[]; faces: number[][]; bones?: { name: string; offsetmatrix: number[]; weights: [number, number][] }[] }[];
  materials?: { properties: { key: string; value: unknown }[] }[];
  animations?: { name: string; tickspersecond: number; duration: number; channels: { name: string; positionkeys: [number, number[]][]; rotationkeys: [number, number[]][]; scalingkeys: [number, number[]][] }[] }[];
};

let converter: Assimp;

function read(bytes: Uint8Array): AssimpScene {
  const list = new converter.FileList();
  list.AddFile("model.fbx", bytes);
  const result = converter.ConvertFileList(list, "assjson");
  expect(result.IsSuccess(), result.GetErrorCode()).toBe(true);
  return JSON.parse(new TextDecoder().decode(result.GetFile(0).GetContent())) as AssimpScene;
}

const rowMajor = (values: number[]): Matrix4 => new Matrix4().fromArray(values).transpose();

function close(actual: ArrayLike<number>, expected: ArrayLike<number>, tolerance: number): void {
  expect(actual.length).toBe(expected.length);
  for (let index = 0; index < expected.length; index++) expect(Math.abs(actual[index] - expected[index])).toBeLessThanOrEqual(tolerance);
}

function assimpWorlds(scene: AssimpScene): Map<string, { world: Matrix4; node: AssimpNode }> {
  const out = new Map<string, { world: Matrix4; node: AssimpNode }>();
  const walk = (node: AssimpNode, parent: Matrix4): void => {
    const world = parent.clone().multiply(rowMajor(node.transformation));
    out.set(node.name, { world, node });
    node.children?.forEach((child) => walk(child, world));
  };
  walk(scene.rootnode, new Matrix4());
  return out;
}

function canonical(corners: number[][]): string {
  const keys = corners.map((corner) => corner.map((value) => (Math.round(value * 1e4) / 1e4 + 0).toFixed(4)).join(","));
  const start = keys.indexOf([...keys].sort()[0]);
  return [0, 1, 2].map((offset) => keys[(start + offset) % 3]).join("|");
}

function check(document: ModelDocument, item: FbxCase, scene: AssimpScene): void {
  const worlds = worldMatricesOf(document);
  const tree = assimpWorlds(scene);
  for (const node of document.nodes) {
    expect(tree.has(node.id), node.id).toBe(true);
    close(tree.get(node.id)!.world.elements, worlds.get(node.id)!.elements, 2e-4);
  }
  const bindings = document.variants.find((variant) => variant.id === item.options.variant)?.bindings ?? {};
  const expectedTriangles: string[] = [];
  const jointWeights = new Map<string, number>();
  for (const node of document.nodes.filter((entry) => entry.mesh !== undefined)) {
    const mesh = document.meshes.find((entry) => entry.id === node.mesh)!;
    const skin = document.skins.find((entry) => entry.id === node.skin);
    for (const primitive of mesh.primitives.filter((entry) => entry.mode === "triangles")) {
      for (let triangle = 0; triangle < primitive.indices.length; triangle += 3) expectedTriangles.push(canonical([0, 1, 2].map((corner) => Array.from(primitive.positions.slice(primitive.indices[triangle + corner] * 3, primitive.indices[triangle + corner] * 3 + 3)))));
      if (skin && primitive.joints && primitive.weights) for (let vertex = 0; vertex < primitive.positions.length / 3; vertex++) for (let slot = 0; slot < 4; slot++) jointWeights.set(skin.joints[primitive.joints[vertex * 4 + slot]], (jointWeights.get(skin.joints[primitive.joints[vertex * 4 + slot]]) ?? 0) + primitive.weights[vertex * 4 + slot]);
    }
    if (skin) for (const joint of skin.joints) expect((scene.meshes ?? []).flatMap((mesh2) => mesh2.bones ?? []).some((bone) => bone.name === joint), joint).toBe(true);
  }
  const got = (scene.meshes ?? []).flatMap((mesh) => mesh.faces.map((face) => canonical(face.map((vertex) => mesh.vertices.slice(vertex * 3, vertex * 3 + 3)))));
  expect(got.sort()).toEqual(expectedTriangles.sort());
  for (const [joint, total] of jointWeights) {
    const sum = (scene.meshes ?? []).flatMap((mesh) => mesh.bones ?? []).filter((bone) => bone.name === joint).reduce((acc, bone) => acc + bone.weights.reduce((inner, [, weight]) => inner + weight, 0), 0);
    expect(sum).toBeCloseTo(total, 4);
  }
  const used = new Set<string>();
  for (const node of document.nodes.filter((entry) => entry.mesh !== undefined))
    for (const primitive of document.meshes.find((entry) => entry.id === node.mesh)!.primitives.filter((entry) => entry.mode === "triangles")) used.add(bindings[primitive.material] ?? primitive.material);
  const loaded = new Map((scene.materials ?? []).map((material) => [String(material.properties.find((property) => property.key === "?mat.name")?.value), material]));
  expect([...loaded.keys()].sort()).toEqual([...used].sort());
  const srgb = (value: number): number => (value <= 0.0031308 ? 12.92 * value : 1.055 * Math.pow(value, 1 / 2.4) - 0.055);
  for (const id of used) {
    const source = document.materials.find((entry) => entry.id === id)!;
    const property = (key: string): number[] | number => loaded.get(id)!.properties.find((entry) => entry.key === key)!.value as number[] | number;
    close(property("$clr.diffuse") as number[], source.baseColor.slice(0, 3).map(srgb), 1e-4);
    close(property("$clr.emissive") as number[], source.emissive.map(srgb), 1e-4);
    expect(property("$mat.opacity") as number).toBeCloseTo(source.alphaMode === "blend" ? source.baseColor[3] : 1, 5);
  }
  if (item.options.animation === undefined) return;
  const animation = document.animations.find((entry) => entry.id === item.options.animation)!;
  const clip = scene.animations![0];
  expect(clip.name).toBe(animation.id);
  const seen = new Set<string>();
  for (const channel of animation.channels) {
    if (!("node" in channel.target)) continue;
    const target = channel.target;
    const key = `${target.node}.${target.path}`;
    if (seen.has(key)) continue;
    seen.add(key);
    const track = clip.channels.find((entry) => entry.name === target.node);
    expect(track, key).toBeDefined();
    const keys = target.path === "translation" ? track!.positionkeys : target.path === "rotation" ? track!.rotationkeys : track!.scalingkeys;
    const size = target.path === "rotation" ? 4 : 3;
    channel.times.forEach((time, index) => {
      const hit = keys.find(([at]) => Math.abs(at / clip.tickspersecond - time) < 2e-3);
      expect(hit, `${key}@${time}`).toBeDefined();
      const want = Array.from(channel.values.slice(index * size, index * size + size));
      if (size === 4) {
        const [w, x, y, z] = hit![1];
        const q = new Quaternion(x, y, z, w);
        expect(Math.abs(q.dot(new Quaternion(...want).normalize()))).toBeGreaterThan(1 - 2e-4);
      } else close(hit![1], want, 2e-4);
    });
  }
}

/** 🔍️ Assimp (through the `assimpjs` WebAssembly build) as a second, independent reader besides three's FBXLoader: hierarchy and pre-rotations, triangles and winding, bone names and skin weights (its JSON export reports identity offset matrices for any input, so bind matrices are checked by three only), material colours, animation keys. */
describe("FBX encoder against assimp", () => {
  beforeAll(async () => {
    converter = await loadAssimp();
  });

  for (const name of DOCUMENT_FIXTURES) {
    const { document, cases } = fixture<FbxFixture>(name);
    for (const item of cases)
      it(`${name} / ${item.name}`, () => {
        check(document, item, read(encodeFbx(document, item.options, NO_OPERATION)));
      });
  }
});

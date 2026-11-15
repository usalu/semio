/** 🎞️ FBX holds one scene per file: option `scene` selects it (absent = the first, with a notice when there are several). Independent readers: assimp (node tree, world matrices, triangle multiset, skin bones) and three's `FBXLoader` (mesh nodes and triangle counts) must see exactly the nodes and triangles of the chosen scene; the digests of the vector pin identical bytes under bun and node. */
import { Matrix4 } from "three";
import { FBXLoader } from "three/examples/jsm/loaders/FBXLoader.js";
import { beforeAll, describe, expect, it } from "vitest";
import type { ModelDocument } from "../../../../🧬️schema/🟦️.ts";
import { selectScene } from "../../../../🎞️scenes/🟦️.ts";
import { tessellate } from "../../../../🔺️tessellation/🟦️.ts";
import { fixture as modelFixture } from "../../../../🧪️tests/🧰️support/🟦️.ts";
import { encodeFbx, encodeFbxDetailed, FbxEncodeError } from "../../🟦️.ts";
import { NO_OPERATION, sha256, worldMatricesOf } from "../🧰️support/🟦️.ts";
import { loadAssimp } from "../🧰️support/🧩️assimpjs/🟦️.ts";
import type { Assimp } from "../🧰️support/🧩️assimpjs/🟦️.ts";

type AssimpNode = { name: string; transformation: number[]; children?: AssimpNode[] };
type AssimpScene = { rootnode: AssimpNode; meshes?: { vertices: number[]; faces: number[][]; bones?: { name: string }[] }[]; animations?: { name: string }[] };

const data = modelFixture("🎞️scenes");
const ids = ["body", "structure", "thermal"] as const;
const resolved = (await tessellate(data.document, data.quality, NO_OPERATION)) as ModelDocument;
const bytesOf = (scene?: string) => encodeFbx(resolved, { format: "binary", animation: "wave", ...(scene ? { scene } : {}) }, NO_OPERATION);
const nodeNames: Record<(typeof ids)[number], string[]> = {
  body: ["arm", "hull-arm", "hull-body", "root", "skin-shared", "skinned-body-structure", "spine"],
  structure: ["arm", "frame-arm", "frame-body", "root", "skin-shared", "skinned-body-structure", "spine"],
  thermal: ["arm", "region-a", "root", "skin-thermal", "skinned-thermal", "spine"],
};
const rowMajor = (values: number[]): Matrix4 => new Matrix4().fromArray(values).transpose();
const canonical = (corners: number[][]): string => {
  const keys = corners.map((c) => c.map((v) => (Math.round(v * 1e4) / 1e4 + 0).toFixed(4)).join(","));
  const start = keys.indexOf([...keys].sort()[0]!);
  return [0, 1, 2].map((o) => keys[(start + o) % 3]).join("|");
};
const triangles = (doc: ModelDocument): string[] => doc.nodes.filter((n) => n.mesh).flatMap((n) => doc.meshes.find((m) => m.id === n.mesh)!.primitives.filter((p) => p.mode === "triangles").flatMap((p) => Array.from({ length: p.indices.length / 3 }, (_, t) => canonical([0, 1, 2].map((c) => Array.from(p.positions.slice(p.indices[t * 3 + c]! * 3, p.indices[t * 3 + c]! * 3 + 3))))))).sort();

let assimp: Assimp;
beforeAll(async () => { assimp = await loadAssimp(); });
const read = (bytes: Uint8Array): AssimpScene => {
  const list = new assimp.FileList();
  list.AddFile("model.fbx", bytes);
  const result = assimp.ConvertFileList(list, "assjson");
  expect(result.IsSuccess(), result.GetErrorCode()).toBe(true);
  return JSON.parse(new TextDecoder().decode(result.GetFile(0).GetContent())) as AssimpScene;
};

describe("🎞️ FBX scenes", () => {
  it.each(ids)("%s: assimp sees the nodes, world matrices, triangles and bones of that scene only", (id) => {
    const scene = read(bytesOf(id));
    const worlds = new Map<string, Matrix4>();
    const walk = (node: AssimpNode, parent: Matrix4): void => {
      const world = parent.clone().multiply(rowMajor(node.transformation));
      worlds.set(node.name, world);
      node.children?.forEach((child) => walk(child, world));
    };
    walk(scene.rootnode, new Matrix4());
    const cut = selectScene(resolved, id);
    const expected = worldMatricesOf(cut);
    expect([...worlds.keys()].filter((n) => n !== scene.rootnode.name).sort()).toEqual(nodeNames[id]);
    for (const node of cut.nodes) expect([...worlds.get(node.id)!.elements].map((v, i) => Math.abs(v - expected.get(node.id)!.elements[i]!)).every((d) => d < 2e-4), node.id).toBe(true);
    const got = (scene.meshes ?? []).flatMap((m) => m.faces.map((f) => canonical(f.map((v) => m.vertices.slice(v * 3, v * 3 + 3)))));
    expect(got.sort()).toEqual(triangles(cut));
    const bones = new Set((scene.meshes ?? []).flatMap((m) => (m.bones ?? []).map((b) => b.name)));
    expect(bones.size).toBeGreaterThan(0);
    expect(scene.animations![0]!.name).toBe("wave");
  });

  it.each(ids)("%s: three's FBXLoader loads only the meshes and triangles of that scene", (id) => {
    const bytes = bytesOf(id);
    const root = new FBXLoader().parse(bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer, "");
    const cut = selectScene(resolved, id);
    const meshes: string[] = [];
    let count = 0;
    root.traverse((o: any) => {
      if (!o.isMesh) return;
      meshes.push(o.name);
      count += (o.geometry.index ? o.geometry.index.count : o.geometry.getAttribute("position").count) / 3;
    });
    expect(meshes.sort()).toEqual(cut.nodes.filter((n) => n.mesh).map((n) => n.id).sort());
    expect(count).toBe(triangles(cut).length);
  });

  it("without the option the first scene is written and a notice says so; with it there is none", () => {
    const plain = encodeFbxDetailed(resolved, { format: "binary" }, NO_OPERATION);
    expect(plain.notices.filter((n) => n.code === "scene-defaulted").map((n) => n.subject)).toEqual(["body"]);
    expect(Buffer.from(plain.bytes).equals(Buffer.from(encodeFbx(resolved, { format: "binary", scene: "body" }, NO_OPERATION)))).toBe(true);
    expect(encodeFbxDetailed(resolved, { format: "binary", scene: "structure" }, NO_OPERATION).notices.some((n) => n.code === "scene-defaulted")).toBe(false);
  });

  it("rejects an unknown scene with a bilingual code and reports progress and cancellation", () => {
    try {
      encodeFbx(resolved, { format: "binary", scene: "nope" }, NO_OPERATION);
      throw new Error("expected a rejection");
    } catch (error) {
      expect(error).toBeInstanceOf(FbxEncodeError);
      expect((error as FbxEncodeError).code).toBe("unknown-scene");
      expect((error as FbxEncodeError).german).toContain("Szene");
    }
    const seen: number[] = [];
    encodeFbx(resolved, { format: "binary", scene: "thermal" }, { report: (f) => seen.push(f), cancelled: () => false });
    expect(seen.at(-1)).toBe(1);
    expect(() => encodeFbx(resolved, { format: "binary", scene: "thermal" }, { report: () => undefined, cancelled: () => true })).toThrow();
  });

  it("matches the recorded digests (identical bytes under bun and node)", () => {
    for (const id of ids) expect(sha256(bytesOf(id))).toBe(data.expected.fbx[id]);
  });
});

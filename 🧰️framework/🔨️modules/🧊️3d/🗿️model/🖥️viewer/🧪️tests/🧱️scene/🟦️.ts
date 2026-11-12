/** 🧱️ Scene preparation: buffer layout and counts from the source document, shared geometry, computed normals, skins, variant maps, rejections, progress and cancellation. */
import { describe, expect, it } from "vitest";
import { CancelledError } from "../../../🟦️.ts";
import type { ModelDocument } from "../../../🧬️schema/🟦️.ts";
import { SKIN_STRIDE, VERTEX_STRIDE } from "../../🧬️contract/🟦️.ts";
import { SceneError, sceneOf, sceneOfAsync } from "../../🧱️scene/🟦️.ts";
import { fixture } from "../🧰️support/🟦️.ts";

const doc = fixture("🧱️rig").document as ModelDocument;
const frameData = fixture("🎞️frame");

const triangle = (extra: object = {}): ModelDocument => ({
  version: "semio.model/v1", id: "t", title: { en: "t", de: "t" }, solids: [], skins: [], variants: [], poses: [], animations: [],
  nodes: [{ id: "a", parent: null, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], mesh: "m" }, { id: "b", parent: null, translation: [1, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], mesh: "m" }],
  meshes: [{ id: "m", primitives: [{ mode: "triangles", material: "x", positions: [0, 0, 0, 1, 0, 0, 0, 1, 0], indices: [0, 1, 2], ...extra }] }],
  materials: [{ id: "x", title: { en: "x", de: "x" }, baseColor: [1, 0, 0, 1], metallic: 0, roughness: 1, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false }],
});

describe("sceneOf", () => {
  it("lays out buffers exactly as the source document and fixture say", () => {
    const s = sceneOf(doc);
    const e = frameData.rig;
    expect(s.vertices.length).toBe(e.vertexCount * VERTEX_STRIDE);
    expect(s.indices.length).toBe(e.indexCount);
    expect(s.jointCount).toBe(e.jointCount);
    expect(s.nodeIds).toEqual(e.nodes);
    expect(s.primitives.map((p) => ({ node: p.node, mode: p.mode, material: p.material, skin: p.skin, vertexCount: p.vertexCount, indexCount: p.indexCount }))).toEqual(e.primitives);
    const sourceVertices = doc.meshes.flatMap((m) => m.primitives).reduce((n, p) => n + p.positions.length / 3, 0);
    expect(s.vertices.length / VERTEX_STRIDE).toBe(sourceVertices);
    expect(s.skinning!.length).toBe(sourceVertices * SKIN_STRIDE);
  });

  it("copies positions, normals and colours interleaved and makes indices global", () => {
    const s = sceneOf(doc);
    for (const p of s.primitives) {
      const src = doc.meshes.find((m) => m.id === p.mesh)!.primitives[p.primitive]!;
      for (let v = 0; v < p.vertexCount; v++) {
        const o = (p.vertexStart + v) * VERTEX_STRIDE;
        expect(Array.from(s.vertices.subarray(o, o + 3))).toEqual(Array.from(Float32Array.from(src.positions.slice(v * 3, v * 3 + 3))));
        if (src.normals) expect(Array.from(s.vertices.subarray(o + 3, o + 6))).toEqual(Array.from(Float32Array.from(src.normals.slice(v * 3, v * 3 + 3))));
        expect(Array.from(s.vertices.subarray(o + 6, o + 10))).toEqual(src.colors ? Array.from(Float32Array.from(src.colors.slice(v * 4, v * 4 + 4))) : [1, 1, 1, 1]);
      }
      for (let k = 0; k < p.indexCount; k++) expect(s.indices[p.indexStart + k]).toBe(src.indices[k]! + p.vertexStart);
      for (let k = 0; k < 3; k++) {
        expect(p.bounds.min[k]).toBeLessThanOrEqual(p.bounds.max[k]!);
      }
    }
  });

  it("carries skins with inverse bind matrices and renormalised weights", () => {
    const s = sceneOf(doc);
    expect(s.skins).toHaveLength(1);
    const sk = s.skins[0]!;
    expect(Array.from(sk.jointNodes)).toEqual(doc.skins[0]!.joints.map((j) => s.nodeIds.indexOf(j)));
    expect(Array.from(sk.inverseBind.subarray(16, 32))).toEqual(Array.from(Float32Array.from(doc.skins[0]!.inverseBind[1] as unknown as number[])));
    const p = s.primitives.find((p) => p.skin === 0 && p.mode === "triangles")!;
    for (let v = 0; v < p.vertexCount; v++) {
      const o = (p.vertexStart + v) * SKIN_STRIDE;
      expect(Math.abs(s.skinning![o + 4]! + s.skinning![o + 5]! + s.skinning![o + 6]! + s.skinning![o + 7]! - 1)).toBeLessThan(1e-6);
    }
  });

  it("builds variant maps from bindings", () => {
    const s = sceneOf(doc);
    const dark = s.variants.get("dark")!;
    expect(dark[s.materialIndex.get("skin")!]).toBe(s.materialIndex.get("skin-dark"));
    expect(dark[s.materialIndex.get("glass")!]).toBe(s.materialIndex.get("glass"));
    expect(Array.from(s.variants.get("light")!)).toEqual(s.materials.map((_, i) => i));
  });

  it("stores geometry shared by several nodes once and instances it per node", () => {
    const s = sceneOf(triangle());
    expect(s.primitives).toHaveLength(2);
    expect(s.vertices.length / VERTEX_STRIDE).toBe(3);
    expect(s.primitives[0]!.vertexStart).toBe(s.primitives[1]!.vertexStart);
    expect(s.primitives.map((p) => p.node)).toEqual(["a", "b"]);
  });

  it("computes smooth area-weighted normals when missing and white colours", () => {
    const s = sceneOf(triangle());
    expect(Array.from(s.vertices.subarray(3, 6))).toEqual([0, 0, 1]);
    expect(Array.from(s.vertices.subarray(6, 10))).toEqual([1, 1, 1, 1]);
  });

  it("rejects procedural, dangling and malformed documents", () => {
    expect(() => sceneOf({ ...doc, solids: [{ id: "s", kind: "box", node: "root", material: "skin", size: [1, 1, 1] }] } as ModelDocument)).toThrow(SceneError);
    expect(() => sceneOf(triangle({ indices: [0, 1, 7] }))).toThrow(/out of range/);
    expect(() => sceneOf(triangle({ positions: [0, 0] }))).toThrow(SceneError);
    expect(() => sceneOf(triangle({ material: "ghost" }))).toThrow(/unknown material/);
    expect(() => sceneOf(triangle({ colors: [1, 1, 1] }))).toThrow(/colors/);
  });

  it("reports monotonic progress and cancels with the kernel's CancelledError", () => {
    const reports: number[] = [];
    sceneOf(doc, { report: (f) => reports.push(f), cancelled: () => false });
    expect(reports[reports.length - 1]).toBe(1);
    expect(reports.every((v, i) => i === 0 || v >= reports[i - 1]!)).toBe(true);
    expect(() => sceneOf(doc, { report: () => undefined, cancelled: () => true })).toThrow(CancelledError);
  });

  it("prepares asynchronously with the same result and cancellation", async () => {
    const a = sceneOf(doc), b = await sceneOfAsync(doc);
    expect(Array.from(b.vertices)).toEqual(Array.from(a.vertices));
    expect(Array.from(b.indices)).toEqual(Array.from(a.indices));
    let n = 0;
    await expect(sceneOfAsync(doc, { report: () => undefined, cancelled: () => ++n > 2 })).rejects.toThrow(CancelledError);
  });
});

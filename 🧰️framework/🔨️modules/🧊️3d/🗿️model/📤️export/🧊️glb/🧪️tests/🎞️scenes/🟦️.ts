/** 🎞️ GLB scenes: one glTF scene per model scene (default = first) sharing meshes, accessors, materials, variants, skin inverse-bind matrices and animation samplers. Independent oracles: the Khronos `gltf-validator` (zero errors and warnings), glTF-Transform (scene count, per-scene node sets, sharing) and three's `GLTFLoader` (per-scene node and mesh sets, `Box3` bounds with skinning against the hand-derived vector). The digests of the vector pin identical bytes under bun and node.
 */
import { createHash } from "node:crypto";
import { describe, expect, it } from "vitest";

import { WebIO } from "@gltf-transform/core";
import { KHRMaterialsUnlit, KHRMaterialsVariants } from "@gltf-transform/extensions";
import validator from "gltf-validator";
import { Box3 } from "three";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";

import type { ModelDocument } from "../../../../🧬️schema/🟦️.ts";
import { tessellate } from "../../../../🔺️tessellation/🟦️.ts";
import { selectScene } from "../../../../🎞️scenes/🟦️.ts";
import { encodeGlb } from "../../🟦️.ts";
import { fixture } from "../../../../🧪️tests/🧰️support/🟦️.ts";
import { quiet } from "../🧰️support/🟦️.ts";

const data = fixture("🎞️scenes");
const options = { animations: true, variants: true };
const ids = ["body", "structure", "thermal"] as const;
const resolved = (await tessellate(data.document, data.quality, quiet)) as ModelDocument;
const sha = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const meshNodes: Record<(typeof ids)[number], string[]> = { body: ["hull-body", "hull-arm", "skinned-body-structure"], structure: ["frame-body", "frame-arm", "skinned-body-structure"], thermal: ["region-a", "skinned-thermal"] };
const nodeNames: Record<(typeof ids)[number], string[]> = {
  body: ["arm", "hull-arm", "hull-body", "root", "skin-shared", "skinned-body-structure", "spine"],
  structure: ["arm", "frame-arm", "frame-body", "root", "skin-shared", "skinned-body-structure", "spine"],
  thermal: ["arm", "region-a", "root", "skin-thermal", "skinned-thermal", "spine"],
};
const load = (bytes: Uint8Array) => new Promise<any>((resolve, reject) => new GLTFLoader().parse(bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer, "", resolve, reject));
const names = (scene: any, filter: (o: any) => boolean = () => true): string[] => {
  const out: string[] = [];
  scene.traverse((o: any) => { if (o !== scene && filter(o)) out.push(o.name.replace(/_\d+$/u, "")); });
  return out.sort();
};

describe("🎞️ GLB scenes", () => {
  it("validates with zero errors and zero warnings (Khronos gltf-validator): all scenes, each scene alone, baked pose", async () => {
    for (const bytes of [encodeGlb(resolved, options, quiet), ...ids.map((scene) => encodeGlb(resolved, { ...options, scene }, quiet)), encodeGlb(resolved, { ...options, pose: "bent" }, quiet), encodeGlb(resolved, { ...options, pose: { animation: "wave", timeS: 0.25 } }, quiet)]) {
      const report = await validator.validateBytes(bytes, { uri: "scenes.glb", maxIssues: 0 });
      expect(report.issues.messages.filter((m: { severity: number }) => m.severity <= 1)).toEqual([]);
      expect(report.info.version).toBe("2.0");
    }
  });

  it("writes one glTF scene per model scene, the first as default, with the title and node ids in extras", async () => {
    const io = new WebIO().registerExtensions([KHRMaterialsUnlit, KHRMaterialsVariants]);
    const root = (await io.readBinary(encodeGlb(resolved, options, quiet))).getRoot();
    expect(root.listScenes().map((s) => s.getName())).toEqual([...ids]);
    expect(root.getDefaultScene()!.getName()).toBe("body");
    root.listScenes().forEach((scene, i) => {
      const source = data.document.scenes[i];
      const cut = resolved.scenes![i]!;
      expect(scene.getExtras().title).toEqual(source.title);
      expect(scene.getExtras().nodes).toEqual(cut.nodes);
      expect(cut.nodes).toEqual(expect.arrayContaining(source.nodes));
      expect(scene.getExtras().tags).toEqual(source.tags);
      const seen: string[] = [];
      scene.traverse((node) => { seen.push(node.getName()); });
      expect(seen.sort(), scene.getName()).toEqual(nodeNames[ids[i]!]);
    });
  });

  it("shares meshes, materials, variants, inverse-bind matrices and sampler accessors across the scenes", async () => {
    const io = new WebIO().registerExtensions([KHRMaterialsUnlit, KHRMaterialsVariants]);
    const root = (await io.readBinary(encodeGlb(resolved, options, quiet))).getRoot();
    expect(root.listMeshes().map((m) => m.getName())).toEqual(resolved.meshes.map((m) => m.id));
    expect(root.listMaterials().length).toBe(resolved.materials.length);
    const skinned = root.listNodes().filter((n) => n.getName() === "skinned-body-structure");
    expect(skinned.length).toBe(2);
    expect(skinned[0]!.getMesh()).toBe(skinned[1]!.getMesh());
    expect(skinned[0]!.getSkin()).not.toBe(skinned[1]!.getSkin());
    expect(skinned[0]!.getSkin()!.getInverseBindMatrices()).toBe(skinned[1]!.getSkin()!.getInverseBindMatrices());
    expect(root.listSkins().length).toBe(3);
    const wave = root.listAnimations()[0]!;
    const arm = wave.listChannels().filter((c) => c.getTargetNode()?.getName() === "arm");
    expect(arm.length).toBe(3);
    expect(new Set(arm.map((c) => c.getSampler())).size).toBe(1);
    expect(new Set(arm.map((c) => c.getTargetNode())).size).toBe(3);
    expect(wave.listSamplers().length).toBe(2);
    const variants = root.listExtensionsUsed().find((e) => e.extensionName === "KHR_materials_variants") as KHRMaterialsVariants;
    expect(variants.listVariants().map((v) => v.getName())).toEqual(["dark"]);
  });

  it("loads through three's GLTFLoader with the per-scene node and mesh sets", async () => {
    const gltf = await load(encodeGlb(resolved, options, quiet));
    expect(gltf.scenes.length).toBe(3);
    expect(gltf.scene).toBe(gltf.scenes[0]);
    gltf.scenes.forEach((scene: any, i: number) => {
      const id = ids[i]!;
      expect(scene.name).toBe(id);
      expect(names(scene, (o) => o.isBone || o.isMesh || o.isSkinnedMesh || o.isGroup || o.isObject3D).filter((n, k, all) => all.indexOf(n) === k)).toEqual(nodeNames[id]);
      expect(names(scene, (o) => o.isMesh).sort()).toEqual([...meshNodes[id]].sort());
    });
    const shared = new Set<unknown>();
    gltf.scenes.forEach((scene: any) => scene.traverse((o: any) => { expect(shared.has(o), o.name).toBe(false); shared.add(o); }));
  });

  it.each(ids)("%s: a scene alone holds exactly the nodes of that scene and the bounds of three's Box3 match the vector (rest and bent)", async (id) => {
    for (const [pose, key] of [[undefined, "rest"], ["bent", "bent"]] as const) {
      const gltf = await load(encodeGlb(resolved, { ...options, scene: id, ...(pose ? { pose } : {}) }, quiet));
      expect(gltf.scenes.length).toBe(1);
      expect(names(gltf.scene, (o) => o.isMesh).sort()).toEqual([...meshNodes[id]].sort());
      gltf.scene.updateMatrixWorld(true);
      gltf.scene.traverse((o: any) => o.isSkinnedMesh && o.skeleton.update());
      const box = new Box3().setFromObject(gltf.scene, true);
      const want = data.expected.bounds[id][key];
      for (let k = 0; k < 3; k++) {
        expect(box.min.getComponent(k)).toBeCloseTo(want.min[k], 5);
        expect(box.max.getComponent(k)).toBeCloseTo(want.max[k], 5);
      }
    }
  });

  it("selecting a scene encodes the same as encoding the cut document", () => {
    for (const id of ids) expect(sha(encodeGlb(resolved, { ...options, scene: id }, quiet))).toBe(sha(encodeGlb(selectScene(resolved, id), options, quiet)));
  });

  it("matches the recorded digests (identical bytes under bun and node) and is deterministic", () => {
    const all = encodeGlb(resolved, options, quiet);
    expect(sha(all)).toBe(data.expected.glb.all);
    expect(Buffer.from(all).equals(Buffer.from(encodeGlb(structuredClone(resolved), { ...options }, quiet)))).toBe(true);
    for (const id of ids) expect(sha(encodeGlb(resolved, { ...options, scene: id }, quiet))).toBe(data.expected.glb.perScene[id]);
    expect(sha(encodeGlb(resolved, { ...options, pose: "bent" }, quiet))).toBe(data.expected.glb.posed);
  });

  it("rejects an unknown scene and reports progress and cancellation across all scenes", () => {
    expect(() => encodeGlb(resolved, { ...options, scene: "nope" }, quiet)).toThrow(/unknown scene/u);
    const seen: number[] = [];
    encodeGlb(resolved, options, { report: (f) => seen.push(f), cancelled: () => false });
    expect(seen.at(-1)).toBe(1);
    expect(() => encodeGlb(resolved, options, { report: () => undefined, cancelled: () => true })).toThrow("cancelled");
  });
});

/** 🧪️ Third-party oracles of the owned GLB encoder (`../../🟦️.ts`), tests only: the Khronos `gltf-validator` must report zero errors for every vector,
 * and two independent loaders (glTF-Transform and three's `GLTFLoader`) read the bytes back and must agree with the source document on geometry, skin, animation, material and variant content.
 */
import { describe, expect, it } from "vitest";

import { WebIO } from "@gltf-transform/core";
import { KHRMaterialsUnlit, KHRMaterialsVariants } from "@gltf-transform/extensions";
import validator from "gltf-validator";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";

import type { ModelDocument } from "../../../../🧬️schema/🟦️.ts";
import { featureEdges, tessellate } from "../../../../🟦️.ts";
import { encodeGlb } from "../../🟦️.ts";
import type { GlbOptions } from "../../🟦️.ts";
import { directoryNear, readFixture } from "../../../../🧪️tests/🧰️support/🧫️reader/🟦️.ts";
import { quiet, vectors as loadVectors } from "../🧰️support/🟦️.ts";

type Mapper = { listMappings(): { getMaterial(): { getName(): string }; listVariants(): { getName(): string }[] }[] };
type Vector = { name: string; document: ModelDocument; options: GlbOptions };

const vectors = loadVectors<Vector>();
const encode = (vector: Vector): Uint8Array => encodeGlb(vector.document, vector.options, quiet);
const unbaked = (vector: Vector): boolean => vector.options.pose === undefined;
const near = (actual: ArrayLike<number>, expected: ArrayLike<number>, digits = 6): void => {
  expect(actual.length).toBe(expected.length);
  for (let i = 0; i < expected.length; i += 1) expect(actual[i]!).toBeCloseTo(expected[i]!, digits);
};

describe("🧪️ GLB oracles", () => {
  it.each(vectors.map((vector) => [vector.name, vector] as const))("%s validates with zero errors and zero warnings (Khronos gltf-validator)", async (_name, vector) => {
    const report = await validator.validateBytes(encode(vector), { uri: `${vector.name}.glb`, maxIssues: 0 });
    expect(report.issues.messages.filter((message: { severity: number }) => message.severity === 0)).toEqual([]);
    expect(report.issues.numErrors).toBe(0);
    expect(report.issues.numWarnings).toBe(0);
    expect(report.info.version).toBe("2.0");
    expect(report.info.totalTriangleCount).toBe(vector.document.meshes.flatMap((m) => m.primitives).filter((p) => p.mode === "triangles").reduce((n, p) => n + p.indices.length / 3, 0));
  });

  it.each(vectors.map((vector) => [vector.name, vector] as const))("%s reads back through glTF-Transform with the source geometry, skin, animation and variants", async (_name, vector) => {
    const io = new WebIO().registerExtensions([KHRMaterialsUnlit, KHRMaterialsVariants]);
    const document = await io.readBinary(encode(vector));
    const root = document.getRoot();
    const source = vector.document;
    expect(root.listNodes().map((node) => node.getName())).toEqual(source.nodes.map((node) => node.id));
    for (const node of root.listNodes()) {
      const origin = source.nodes.find((n) => n.id === node.getName())!;
      const hoisted = origin.skin !== undefined && origin.mesh !== undefined;
      expect(node.getParentNode()?.getName() ?? null).toBe(hoisted ? null : origin.parent);
      if (hoisted) expect(node.getExtras().parent).toBe(origin.parent);
    }
    if (unbaked(vector))
      for (const node of source.nodes) {
        const back = root.listNodes().find((n) => n.getName() === node.id)!;
        near(back.getTranslation(), node.translation);
        near(back.getRotation(), node.rotation);
        near(back.getScale(), node.scale);
      }
    expect(root.listMeshes().map((mesh) => mesh.getName())).toEqual(source.meshes.map((mesh) => mesh.id));
    source.meshes.forEach((mesh, m) =>
      mesh.primitives.forEach((primitive, p) => {
        const back = root.listMeshes()[m]!.listPrimitives()[p]!;
        expect(back.getMode()).toBe(primitive.mode === "triangles" ? 4 : 1);
        near(back.getAttribute("POSITION")!.getArray()!, primitive.positions, 6);
        expect(Array.from(back.getIndices()!.getArray()!)).toEqual(primitive.indices);
        expect(back.getMaterial()!.getName()).toBe(primitive.material);
        expect(back.getExtras().solids).toEqual(primitive.solids);
        if (primitive.normals) near(back.getAttribute("NORMAL")!.getArray()!, primitive.normals);
        if (primitive.colors) near(back.getAttribute("COLOR_0")!.getArray()!, primitive.colors);
        if (primitive.joints) expect(Array.from(back.getAttribute("JOINTS_0")!.getArray()!)).toEqual(primitive.joints);
        if (primitive.weights) near(back.getAttribute("WEIGHTS_0")!.getArray()!, primitive.weights);
        expect(back.getAttribute("POSITION")!.getMin([])).toEqual(Array.from({ length: 3 }, (_, c) => Math.fround(Math.min(...primitive.positions.filter((_, i) => i % 3 === c)))));
      }),
    );
    expect(root.listSkins().map((skin) => skin.listJoints().map((joint) => joint.getName()))).toEqual(source.skins.map((skin) => skin.joints));
    source.skins.forEach((skin, s) => near(root.listSkins()[s]!.getInverseBindMatrices()!.getArray()!, skin.inverseBind.flat()));
    source.materials.forEach((material, i) => {
      const back = root.listMaterials()[i]!;
      expect(back.getName()).toBe(material.id);
      if (unbaked(vector)) {
        near(back.getBaseColorFactor(), material.baseColor);
        near(back.getEmissiveFactor(), material.emissive);
      }
      expect(back.getAlphaMode()).toBe(material.alphaMode === "blend" ? "BLEND" : "OPAQUE");
      expect(back.getDoubleSided()).toBe(material.doubleSided);
      expect(back.getExtension("KHR_materials_unlit") !== null).toBe(material.unlit);
    });
    const clips = vector.options.animations ? source.animations : [];
    expect(root.listAnimations().map((animation) => animation.getName())).toEqual(clips.map((clip) => clip.id));
    clips.forEach((clip, a) => {
      const back = root.listAnimations()[a]!;
      clip.channels.forEach((channel, c) => {
        const sampler = back.listChannels()[c]!.getSampler()!;
        near(sampler.getInput()!.getArray()!, channel.times);
        near(sampler.getOutput()!.getArray()!, channel.values);
        expect(sampler.getInterpolation()).toBe(channel.interpolation === "step" ? "STEP" : "LINEAR");
        if ("node" in channel.target) {
          const target = back.listChannels()[c]!;
          expect(target.getTargetNode()!.getName()).toBe(channel.target.node);
          expect(target.getTargetPath()).toBe(channel.target.path);
        }
      });
    });
    const variants = root.listExtensionsUsed().find((extension) => extension.extensionName === "KHR_materials_variants") as KHRMaterialsVariants | undefined;
    if (vector.options.variants && source.variants.length > 0) {
      expect(variants!.listVariants().map((variant) => variant.getName())).toEqual(source.variants.map((variant) => variant.id));
      source.meshes.forEach((mesh, m) =>
        mesh.primitives.forEach((primitive, p) => {
          const mappings = (root.listMeshes()[m]!.listPrimitives()[p]!.getExtension("KHR_materials_variants") as unknown as Mapper | null)?.listMappings() ?? [];
          const expected = source.variants.flatMap((variant) => (variant.bindings[primitive.material] && variant.bindings[primitive.material] !== primitive.material ? [`${variant.id}=${variant.bindings[primitive.material]}`] : [])).sort();
          const actual = mappings.flatMap((mapping) => mapping.listVariants().map((variant) => `${variant.getName()}=${mapping.getMaterial().getName()}`)).sort();
          expect(actual).toEqual(expected);
        }),
      );
    } else expect(variants).toBeUndefined();
  });

  it.each(vectors.map((vector) => [vector.name, vector] as const))("%s loads through three's GLTFLoader with the source geometry, skeleton, clips and variants", async (_name, vector) => {
    const bytes = encode(vector);
    const buffer = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer;
    const gltf = await new Promise<any>((resolve, reject) => new GLTFLoader().parse(buffer, "", resolve, reject));
    const source = vector.document;
    source.nodes.forEach((node) => expect(gltf.scene.getObjectByName(node.id), node.id).toBeDefined());
    for (const mesh of source.meshes) {
      const node = source.nodes.find((n) => n.mesh === mesh.id)!;
      const object = gltf.scene.getObjectByName(node.id);
      const primitive = mesh.primitives[0]!;
      const target = object.isGroup ? object.children[0] : object;
      const geometry = target.geometry;
      near(geometry.getAttribute("position").array, primitive.positions);
      expect(Array.from(geometry.index.array)).toEqual(primitive.indices);
      expect(Boolean(target.isLineSegments)).toBe(primitive.mode === "lines");
      if (primitive.mode === "triangles" && node.skin) {
        expect(target.isSkinnedMesh).toBe(true);
        const skin = source.skins.find((s) => s.id === node.skin)!;
        expect(target.skeleton.bones.map((bone: { name: string }) => bone.name)).toEqual(skin.joints);
        skin.inverseBind.forEach((matrix, i) => near(target.skeleton.boneInverses[i].elements, matrix));
        near(geometry.getAttribute("skinWeight").array, primitive.weights!);
        expect(Array.from(geometry.getAttribute("skinIndex").array)).toEqual(primitive.joints);
      }
      if (primitive.normals) near(geometry.getAttribute("normal").array, primitive.normals);
    }
    if (unbaked(vector))
      for (const node of source.nodes) {
        const object = gltf.scene.getObjectByName(node.id);
        near(object.position.toArray(), node.translation);
        near(object.quaternion.toArray(), node.rotation);
        near(object.scale.toArray(), node.scale);
      }
    const clips = vector.options.animations ? source.animations : [];
    expect(gltf.animations.map((clip: { name: string }) => clip.name)).toEqual(clips.map((clip) => clip.id));
    for (const clip of clips) {
      const back = gltf.animations.find((c: { name: string }) => c.name === clip.id);
      const nodeChannels = clip.channels.filter((channel) => "node" in channel.target);
      const names = nodeChannels.map((channel) => `${(channel.target as { node: string }).node}.${{ translation: "position", rotation: "quaternion", scale: "scale" }[channel.target.path as "translation"]}`).sort();
      expect(back.tracks.map((track: { name: string }) => track.name).sort()).toEqual(names);
      for (const channel of nodeChannels) {
        const track = back.tracks.find((t: { name: string }) => t.name === `${(channel.target as { node: string }).node}.${{ translation: "position", rotation: "quaternion", scale: "scale" }[channel.target.path as "translation"]}`);
        near(track.times, channel.times);
        near(track.values, channel.values);
      }
    }
    source.meshes.forEach((mesh, m) => mesh.primitives.forEach((primitive, p) => expect(gltf.parser.json.meshes[m].primitives[p].extras?.solids).toEqual(primitive.solids)));
    const declared = gltf.parser.json.extensions?.KHR_materials_variants?.variants?.map((variant: { name: string }) => variant.name);
    if (vector.options.variants && source.variants.length > 0) {
      expect(declared).toEqual(source.variants.map((variant) => variant.id));
    } else expect(declared).toBeUndefined();
  });

  it("a tessellated and edged procedural document keeps its solid provenance through Khronos and glTF-Transform", async () => {
    const data = readFixture(directoryNear(import.meta.url, "../../../../🧫️fixtures"), "🧬️provenance");
    const resolved = await featureEdges(await tessellate(data.document, data.quality), (data.angleDeg * Math.PI) / 180);
    const bytes = encodeGlb(resolved, { animations: false, variants: false }, quiet);
    const report = await validator.validateBytes(bytes, { uri: "provenance.glb", maxIssues: 0 });
    expect(report.issues.numErrors).toBe(0);
    expect(report.issues.numWarnings).toBe(0);
    const root = (await new WebIO().registerExtensions([KHRMaterialsUnlit, KHRMaterialsVariants]).readBinary(bytes)).getRoot();
    resolved.meshes.forEach((mesh, m) => mesh.primitives.forEach((primitive, p) => {
      expect(primitive.solids!.length).toBeGreaterThan(0);
      expect(root.listMeshes()[m]!.listPrimitives()[p]!.getExtras().solids).toEqual(primitive.solids);
    }));
  });
});

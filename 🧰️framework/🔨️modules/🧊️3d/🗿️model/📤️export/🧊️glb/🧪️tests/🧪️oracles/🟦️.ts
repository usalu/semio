/** 🧪️ Third-party oracles of the owned GLB encoder (`../../🟦️.ts`), tests only: the Khronos `gltf-validator` must report zero errors for every vector,
 * and two independent loaders (glTF-Transform and three's `GLTFLoader`) read the bytes back and must agree with the source document on geometry, skin, animation, material and variant content.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import { WebIO } from "@gltf-transform/core";
import { KHRMaterialsUnlit, KHRMaterialsVariants, KHRMeshQuantization } from "@gltf-transform/extensions";
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
  it("reuses small typed accessors in a large model", async () => {
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/♊️shared-positions/🔣️.json", import.meta.url), "utf8"));
    const template = vectors.find(v => v.name === "minimal-triangle")!;
    const first = template.document.meshes[0]!, part = first.primitives[0]!;
    const positions = Array.from({length:fixture.largeVertexCount},(_,i) => fixture.positions[i%3] as number[]).flat();
    const prefix = {...part, positions, indices:Array.from({length:fixture.largeVertexCount},(_,i) => i)};
    const distinct = {...part, positions:part.positions.map((x,i) => i===0 ? x+1 : x)};
    const model = {...template.document, meshes:[{...first, primitives:[prefix,part,{...part},distinct]}]};
    const bytes = encodeGlb(model,template.options,quiet);
    const loaded = await new WebIO().readBinary(bytes);
    const [,a,b,c] = loaded.getRoot().listMeshes()[0]!.listPrimitives();
    expect(a!.getAttribute("POSITION")).toBe(b!.getAttribute("POSITION"));
    expect(c!.getAttribute("POSITION")).not.toBe(b!.getAttribute("POSITION"));
    expect(a!.getIndices()).toBe(c!.getIndices());
    near(c!.getAttribute("POSITION")!.getArray()!,distinct.positions);
  });

  it("shares identical positions while retaining different geometry across meshes", async () => {
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/♊️shared-positions/🔣️.json", import.meta.url), "utf8"));
    const template = vectors.find((vector) => vector.name === "minimal-triangle")!;
    const xyz = Array.from({ length: fixture.vertexCount }, (_, i) => fixture.positions[i % 3] as number[]).flat();
    const variant = xyz.slice();
    variant.splice(-2, 2, ...fixture.collidingFloat32Suffix);
    const inputs = [xyz, xyz.slice(), variant];
    const primitive = template.document.meshes[0]!.primitives[0]!;
    const primitives = inputs.map((positions) => ({ ...primitive, positions, indices: Array.from({ length: fixture.vertexCount }, (_, i) => i) }));
    const document = { ...template.document, meshes: [{ ...template.document.meshes[0]!, primitives }] };
    const bytes = encodeGlb(document, template.options, quiet);
    const io = new WebIO();
    const json = await io.binaryToJSON(bytes);
    const ids = json.json.meshes![0]!.primitives.map((p) => p.attributes.POSITION);
    expect(new Set(ids).size).toBe(fixture.distinctPositionAccessors);
    expect(ids[0]).toBe(ids[1]);
    expect(ids[2]).not.toBe(ids[1]);
    const decoded = await io.readBinary(bytes);
    const actual = decoded.getRoot().listMeshes()[0]!.listPrimitives();
    for (const [i, item] of actual.entries()) expect(Array.from(item.getAttribute("POSITION")!.getArray()!)).toEqual(inputs[i]);
    expect((await validator.validateBytes(bytes, { uri: "shared.glb", maxIssues: 0 })).issues.numErrors).toBe(0);
  });

  it("shares rigid part geometry across material batches and retains every corner, line, binding and animation", async () => {
    const f = JSON.parse(readFileSync(new URL("./🧫️fixtures/🧱️shared-rigid-parts/🔣️.json", import.meta.url), "utf8")) as {
      trianglesPerPart: number; triangle: number[][]; partOrders: string[][]; partOffsetsM: number[][]; materials: string[]; sharedParts: string[]; weightedMesh: string; rotatingJoint: string;
    };
    const template = vectors.find((v) => v.name === "rig-full")!;
    const count = f.trianglesPerPart * 3;
    const batches = f.partOrders.map((ids, batch) => {
      const positions = f.partOffsetsM[batch]!.flatMap((offset) => Array.from({ length: f.trianglesPerPart }, (_, i) => f.triangle.flatMap(([x, y, z]) => [x! + 2 * i, y!, z! + offset])).flat());
      const indices = Array.from({ length: 2 * count }, (_, i) => i);
      const normals = indices.flatMap(() => [0, 0, 1]);
      const colors = indices.flatMap((i) => [1, (i % 3) / 2, 0, 1]);
      const solids = ids.map((id, i) => ({ id, first: i * count, count }));
      const triangles = { mode: "triangles" as const, positions, indices, normals, colors, material: f.materials[batch]!, solids };
      const lines = { mode: "lines" as const, positions, indices: [0, 1, count, count + 1], material: f.materials[batch]!, solids: ids.map((id, i) => ({ id, first: 2 * i, count: 2 })) };
      return [triangles, lines];
    });
    const document = { ...template.document, meshes: template.document.meshes.map((m) => m.id === "outline" ? { ...m, primitives: batches.flat() } : m) };
    const bytes = encodeGlb(document, template.options, quiet);
    const io = new WebIO().registerExtensions([KHRMaterialsUnlit, KHRMaterialsVariants]);
    const out = (await io.readBinary(bytes)).getRoot();
    const json = (await io.binaryToJSON(bytes)).json;
    const mesh = out.listMeshes().find((m) => m.getName() === "outline")!;
    const decoded = mesh.listPrimitives();
    const one = (id: string, mode: number) => decoded.find((p) => p.getMode() === mode && (p.getExtras().solids as { id: string }[])[0]?.id === id)!;
    const [left, right] = f.sharedParts.map((id) => one(id, 4));
    expect(left!.getAttribute("POSITION")).toBe(right!.getAttribute("POSITION"));
    expect(left!.getAttribute("POSITION")).toBe(one(f.sharedParts[0]!, 1).getAttribute("POSITION"));
    for (const [batch, slices] of batches.entries()) for (const original of slices) {
      const mode = original.mode === "triangles" ? 4 : 1;
      for (const range of original.solids) {
        const primitive = one(range.id, mode);
        expect(primitive.getMaterial()!.getName()).toBe(f.materials[batch]);
        const slots = primitive.getIndices()!.getArray()!;
        const actual = Array.from(slots).flatMap((i) => primitive.getAttribute("POSITION")!.getElement(i, []));
        const wanted = original.indices.slice(range.first, range.first + range.count).flatMap((i) => original.positions.slice(3 * i, 3 * i + 3));
        expect(actual).toEqual(wanted);
        if (original.mode === "triangles") for (const [attribute, values, width] of [["NORMAL", original.normals, 3], ["COLOR_0", original.colors, 4]] as const) {
          const read = Array.from(slots).flatMap((i) => primitive.getAttribute(attribute)!.getElement(i, []));
          expect(read).toEqual(original.indices.slice(range.first, range.first + range.count).flatMap((i) => values.slice(width * i, width * i + width)));
        }
      }
    }
    const weighted = out.listMeshes().find((m) => m.getName() === f.weightedMesh)!.listPrimitives()[0]!;
    const referenceBytes = encode(template);
    const referenceRoot = (await io.readBinary(referenceBytes)).getRoot();
    const originalSkin = referenceRoot.listMeshes().find((m) => m.getName() === f.weightedMesh)!.listPrimitives()[0]!;
    for (const name of ["POSITION", "NORMAL", "JOINTS_0", "WEIGHTS_0"]) expect(weighted.getAttribute(name)!.getArray()).toEqual(originalSkin.getAttribute(name)!.getArray());
    expect(weighted.getIndices()!.getArray()).toEqual(originalSkin.getIndices()!.getArray());
    expect(out.listSkins()[0]!.getInverseBindMatrices()!.getArray()).toEqual(referenceRoot.listSkins()[0]!.getInverseBindMatrices()!.getArray());
    for (const [index, animation] of out.listAnimations().entries()) {
      const original = referenceRoot.listAnimations()[index]!;
      for (const [i, sampler] of animation.listSamplers().entries()) {
        const before = original.listSamplers()[i]!;
        expect(sampler.getInput()!.getArray()).toEqual(before.getInput()!.getArray());
        expect(sampler.getOutput()!.getArray()).toEqual(before.getOutput()!.getArray());
        expect(sampler.getInterpolation()).toBe(before.getInterpolation());
      }
    }
    const reference = (await io.binaryToJSON(referenceBytes)).json;
    const channels = (j: typeof json) => j.animations!.flatMap((a) => a.channels.map((c) => [j.nodes![c.target.node ?? 0]!.name, c.target.path]));
    expect(channels(json)).toEqual(channels(reference));
    expect(channels(json)).toContainEqual([f.rotatingJoint, "rotation"]);
    expect((await validator.validateBytes(bytes, { uri: "rigid-parts.glb", maxIssues: 0 })).issues.numErrors).toBe(0);
  });

  it("exports feature lines with the same vertex accessor as their source triangles", async () => {
    const template = vectors.find((v) => v.name === "minimal-triangle")!;
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/♊️shared-positions/🔣️.json", import.meta.url), "utf8"));
    const positions = Array.from({ length: fixture.vertexCount }, (_, i) => {
      const [x, y, z] = fixture.positions[i % 3];
      return [x + 2 * Math.floor(i / 3), y, z];
    }).flat();
    const indices = Array.from({ length: fixture.vertexCount }, (_, i) => i);
    const mesh = template.document.meshes[0]!;
    const triangles = { ...mesh.primitives[0]!, indices, positions };
    const source = { ...template.document, meshes: [{ ...mesh, primitives: [triangles] }] };
    const compact = await featureEdges(source, 0.5, {}, quiet);
    const shared = await featureEdges(source, 0.5, { shareVertices: true }, quiet);
    const io = new WebIO();
    const decoded = await io.readBinary(encodeGlb(shared, template.options, quiet));
    const [faces, edges] = decoded.getRoot().listMeshes()[0]!.listPrimitives();
    expect(edges!.getAttribute("POSITION")).toBe(faces!.getAttribute("POSITION"));
    const original = compact.meshes[0]!.primitives[1]!;
    const actual = edges!.getIndices()!.getArray()!;
    expect(actual.length).toBe(original.indices.length);
    for (let i = 0; i < actual.length; i++) {
      const v = original.indices[i]! * 3;
      expect(edges!.getAttribute("POSITION")!.getElement(actual[i]!, [])).toEqual(original.positions.slice(v, v + 3));
    }
  });

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

  it("compact encoding writes quantized normals only where a lit material can read them and shares identical ones", async () => {
    const f = JSON.parse(readFileSync(new URL("./🧫️fixtures/📉️compact-normals/🔣️.json", import.meta.url), "utf8")) as { positions: number[][]; normals: number[][]; vertexCount: number; toleranceRadians: number; normalByteStride: number; normalComponentType: number; extension: string };
    const template = vectors.find((v) => v.name === "minimal-triangle")!;
    const [grey] = template.document.materials;
    const materials = [grey!, { ...grey!, id: "ink", unlit: true }, { ...grey!, id: "paper" }];
    const cycle = (rows: number[][]): number[] => Array.from({ length: f.vertexCount }, (_, i) => rows[i % rows.length]!).flat();
    const base = { mode: "triangles" as const, positions: cycle(f.positions), indices: Array.from({ length: f.vertexCount }, (_, i) => i), normals: cycle(f.normals) };
    const primitives = [{ ...base, material: "grey" }, { ...base, material: "ink" }, { ...base, material: "grey" }];
    const document = {
      ...template.document, materials,
      meshes: [{ ...template.document.meshes[0]!, primitives }],
      variants: [{ id: "paper", title: { en: "Paper", de: "Papier" }, bindings: { "ink": "paper" } }],
    };
    const only = { ...document, variants: [] as typeof document.variants };
    const compact = encodeGlb(only, { animations: false, variants: true, compact: true }, quiet);
    const plain = encodeGlb(only, { animations: false, variants: true }, quiet);
    const io = new WebIO().registerExtensions([KHRMaterialsUnlit, KHRMaterialsVariants, KHRMeshQuantization]);
    const json = (await io.binaryToJSON(compact)).json;
    const [lit, ink, twin] = json.meshes![0]!.primitives;
    expect(json.extensionsUsed).toContain(f.extension);
    expect(json.extensionsRequired).toEqual([f.extension]);
    expect(lit!.attributes.NORMAL).toBeDefined();
    expect(ink!.attributes.NORMAL).toBeUndefined();
    expect(twin!.attributes.NORMAL).toBe(lit!.attributes.NORMAL);
    expect(twin!.attributes.POSITION).toBe(lit!.attributes.POSITION);
    const accessor = json.accessors![lit!.attributes.NORMAL!]!;
    expect([accessor.componentType, accessor.type, accessor.normalized]).toEqual([f.normalComponentType, "VEC3", true]);
    expect(json.bufferViews![accessor.bufferView!]!.byteStride).toBe(f.normalByteStride);
    expect((await validator.validateBytes(compact, { uri: "compact.glb", maxIssues: 0 })).issues).toMatchObject({ numErrors: 0, numWarnings: 0 });
    const angle = (a: ArrayLike<number>, b: number[]): number => Math.acos(Math.min(1, (a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!) / Math.hypot(a[0]!, a[1]!, a[2]!)));
    const read = (await io.readBinary(compact)).getRoot().listMeshes()[0]!.listPrimitives()[0]!.getAttribute("NORMAL")!;
    const gltf = await new Promise<any>((resolve, reject) => new GLTFLoader().parse(compact.buffer.slice(compact.byteOffset, compact.byteOffset + compact.byteLength) as ArrayBuffer, "", resolve, reject));
    let geometry: any;
    gltf.scene.traverse((node: any) => { if (geometry === undefined && node.geometry?.attributes.normal !== undefined) { geometry = node.geometry; } });
    f.normals.forEach((normal, i) => {
      expect(angle(read.getElement(i, []), normal)).toBeLessThan(f.toleranceRadians);
      expect(angle([geometry.attributes.normal.getX(i), geometry.attributes.normal.getY(i), geometry.attributes.normal.getZ(i)], normal)).toBeLessThan(f.toleranceRadians);
    });
    const kept = (await io.binaryToJSON(encodeGlb(document, { animations: false, variants: true, compact: true }, quiet))).json.meshes![0]!.primitives;
    expect(kept[1]!.attributes.NORMAL).toBeDefined();
    const flat = (await io.binaryToJSON(plain)).json;
    expect(flat.extensionsRequired).toBeUndefined();
    const original = flat.accessors![flat.meshes![0]!.primitives[1]!.attributes.NORMAL!]!;
    expect([original.componentType, original.normalized]).toEqual([5126, undefined]);
    expect(compact.length).toBeLessThan(plain.length);
  });

  it("compact encoding welds the vertices of a primitive no lit material draws inside each solid and keeps its triangles, lines and solid ranges", async () => {
    const f = JSON.parse(readFileSync(new URL("./🧫️fixtures/🧷️welded-unread/🔣️.json", import.meta.url), "utf8")) as { positions: number[][]; indices: number[]; solids: { id: string; first: number; count: number }[]; lines: { indices: number[]; solids: { id: string; first: number; count: number }[] }; weldedVertexCount: number; triangleCorners: number[][][] };
    const template = vectors.find((v) => v.name === "minimal-triangle")!;
    const [grey] = template.document.materials;
    const positions = f.positions.flat();
    const normals = f.positions.flatMap((_, i) => (i === 3 ? [0, 0, -1] : [0, 0, 1]));
    const documentOf = (unlit: boolean) => ({
      ...template.document, variants: [] as never[],
      materials: [{ ...grey!, id: "ink", unlit }, { ...grey!, id: "edge", unlit: true }],
      meshes: [{ ...template.document.meshes[0]!, primitives: [
        { mode: "triangles" as const, material: "ink", positions, normals, indices: f.indices, solids: f.solids },
        { mode: "lines" as const, material: "edge", positions, indices: f.lines.indices, solids: f.lines.solids },
      ] }],
    });
    const io = new WebIO().registerExtensions([KHRMaterialsUnlit, KHRMaterialsVariants, KHRMeshQuantization]);
    const bytes = encodeGlb(documentOf(true), { animations: false, variants: true, compact: true }, quiet);
    const [triangles, lines] = (await io.binaryToJSON(bytes)).json.meshes![0]!.primitives;
    const json = (await io.binaryToJSON(bytes)).json;
    expect(json.accessors![triangles!.attributes.POSITION!]!.count).toBe(f.weldedVertexCount);
    expect(json.accessors![lines!.attributes.POSITION!]!.count).toBe(f.weldedVertexCount);
    expect(triangles!.extras).toEqual({ solids: f.solids });
    expect(lines!.extras).toEqual({ solids: f.lines.solids });
    const read = (await io.readBinary(bytes)).getRoot().listMeshes()[0]!.listPrimitives();
    const place = read[0]!.getAttribute("POSITION")!, index = read[0]!.getIndices()!;
    const corners = Array.from({ length: index.getCount() / 3 }, (_, t) => [0, 1, 2].map((k) => Array.from(place.getElement(index.getScalar(3 * t + k), [])))); 
    expect(corners).toEqual(f.triangleCorners);
    const segments = read[1]!.getIndices()!;
    const ends = Array.from({ length: segments.getCount() }, (_, i) => Array.from(read[1]!.getAttribute("POSITION")!.getElement(segments.getScalar(i), [])));
    expect(ends).toEqual(f.lines.indices.map((i) => f.positions[i]));
    expect((await validator.validateBytes(bytes, { uri: "welded.glb", maxIssues: 0 })).issues).toMatchObject({ numErrors: 0, numWarnings: 0 });
    const lit = (await io.binaryToJSON(encodeGlb(documentOf(false), { animations: false, variants: true, compact: true }, quiet))).json;
    expect(lit.accessors![lit.meshes![0]!.primitives[0]!.attributes.POSITION!]!.count).toBe(f.positions.length);
    const plain = (await io.binaryToJSON(encodeGlb(documentOf(true), { animations: false, variants: true }, quiet))).json;
    expect(plain.accessors![plain.meshes![0]!.primitives[0]!.attributes.POSITION!]!.count).toBe(f.positions.length);
  });

  it("compact encoding writes colour factors and node transforms as the shortest decimals of the same 32-bit floats", async () => {
    const f = JSON.parse(readFileSync(new URL("./🧫️fixtures/📉️compact-normals/🔣️.json", import.meta.url), "utf8")) as { shortFloats: { baseColor: number[]; translation: number[]; scale: number[]; maxCharacters: number } };
    const template = vectors.find((v) => v.name === "minimal-triangle")!;
    const material = { ...template.document.materials[0]!, baseColor: f.shortFloats.baseColor as never };
    const nodes = template.document.nodes.map((node, i) => (i === 0 ? { ...node, translation: f.shortFloats.translation as never, scale: f.shortFloats.scale as never } : node));
    const mesh0 = template.document.meshes[0]!;
    const meshes = [{ ...mesh0, primitives: mesh0.primitives.map((p, i) => (i === 0 ? { ...p, positions: p.positions.map((x) => x + 0.1) } : p)) }, ...template.document.meshes.slice(1)];
    const document = { ...template.document, meshes, materials: [material, ...template.document.materials.slice(1)], nodes };
    const compact = encodeGlb(document, { animations: false, variants: true, compact: true }, quiet);
    const plain = encodeGlb(document, { animations: false, variants: true }, quiet);
    const io = new WebIO().registerExtensions([KHRMaterialsUnlit, KHRMaterialsVariants, KHRMeshQuantization]);
    const json = (await io.binaryToJSON(compact)).json;
    const written = [json.materials![0]!.pbrMetallicRoughness!.baseColorFactor!, json.nodes![0]!.translation!, json.nodes![0]!.scale!];
    const wanted = [f.shortFloats.baseColor, f.shortFloats.translation, f.shortFloats.scale];
    written.forEach((values, k) => values.forEach((value, i) => {
      expect(Math.fround(value), `float ${k}/${i}`).toBe(Math.fround(wanted[k]![i]!));
      expect(String(value).length, `digits of ${k}/${i}`).toBeLessThanOrEqual(f.shortFloats.maxCharacters);
    }));
    const bounds = json.accessors!.find((accessor) => accessor.type === "VEC3" && accessor.min !== undefined)!;
    for (const value of [...bounds.min!, ...bounds.max!]) expect(String(value).length, "digits of an accessor bound").toBeLessThanOrEqual(f.shortFloats.maxCharacters);
    expect(bounds.min!.some((value) => Math.fround(value) === Math.fround(0.1))).toBe(true);
    const long = (await io.binaryToJSON(plain)).json;
    expect(long.materials![0]!.pbrMetallicRoughness!.baseColorFactor).toEqual(f.shortFloats.baseColor);
    expect((await validator.validateBytes(compact, { uri: "short.glb", maxIssues: 0 })).issues).toMatchObject({ numErrors: 0, numWarnings: 0 });
    const read = (await io.readBinary(compact)).getRoot();
    expect(Array.from(read.listMaterials()[0]!.getBaseColorFactor()).map(Math.fround)).toEqual(wanted[0]!.map(Math.fround));
    expect(compact.length).toBeLessThan(plain.length);
  });
  it("compact encoding leaves out glTF defaults, packs accessors of one kind into shared views and shares the indices of a pattern drawn at many places", async () => {
    const f = JSON.parse(readFileSync(new URL("./🧫️fixtures/📦️compact-structure/🔣️.json", import.meta.url), "utf8")) as {
      patternTriangles: number; copyOffsets: number[][]; copyNormals: number[][]; copyIds: string[]; plainMaterial: string; glassMaterial: string; edgeMaterial: string;
      expect: { trianglePrimitives: number; indexComponentTypes: { pattern: number; triangle: number; plainTriangle: number }; triangleCount: number; sharedIndexAccessors: number; maxBufferViews: number; attributeKinds: string[]; defaults: { alphaMode: string; doubleSided: boolean; trianglesMode: number; linesMode: number }; kept: { alphaMode: string; doubleSided: boolean } };
    };
    const template = vectors.find((v) => v.name === "minimal-triangle")!;
    const [grey] = template.document.materials;
    const copy = (offset: number[]): number[] => Array.from({ length: f.patternTriangles }, (_, t) => [offset[0]! + t, offset[1]!, offset[2]!, offset[0]! + t + 0.5, offset[1]! + 1, offset[2]!, offset[0]! + t + 1, offset[1]!, offset[2]!]).flat();
    const positions = f.copyOffsets.flatMap(copy);
    const per = f.patternTriangles * 3;
    const indices = Array.from({ length: per * f.copyOffsets.length }, (_, i) => i);
    const solids = f.copyIds.map((id, i) => ({ id, first: i * per, count: per }));
    const lineIndices = f.copyOffsets.flatMap((_, c) => [3 * 0, 3 * 0 + 1, 3 * 0 + 1, 3 * 0 + 2].map((v) => v + c * per));
    const document = {
      ...template.document, variants: [] as never[],
      materials: [{ ...grey!, id: f.plainMaterial }, { ...grey!, id: f.glassMaterial, alphaMode: "blend" as const, doubleSided: true }, { ...grey!, id: f.edgeMaterial, unlit: true }],
      meshes: [{ ...template.document.meshes[0]!, primitives: [
        { mode: "triangles" as const, material: f.plainMaterial, positions, normals: f.copyNormals.flatMap((normal) => Array.from({ length: per }, () => normal).flat()), indices, solids },
        { mode: "triangles" as const, material: f.glassMaterial, positions: positions.slice(0, 9), normals: [0, 0, 1, 0, 0, 1, 0, 0, 1], indices: [0, 1, 2] },
        { mode: "lines" as const, material: f.edgeMaterial, positions, indices: lineIndices, solids: f.copyIds.map((id, i) => ({ id, first: 4 * i, count: 4 })) },
      ] }],
    };
    const io = new WebIO().registerExtensions([KHRMaterialsUnlit, KHRMaterialsVariants, KHRMeshQuantization]);
    const compact = encodeGlb(document, { animations: false, variants: true, compact: true }, quiet);
    const plain = encodeGlb(document, { animations: false, variants: true }, quiet);
    const json = (await io.binaryToJSON(compact)).json;
    const plainJson = (await io.binaryToJSON(plain)).json;
    const primitives = json.meshes![0]!.primitives;
    const triangles = primitives.filter((p) => p.mode === undefined && p.attributes.NORMAL !== undefined && p.extras?.solids !== undefined);
    expect(triangles.length).toBe(f.expect.trianglePrimitives);
    expect(new Set(triangles.map((p) => p.indices)).size).toBe(f.expect.sharedIndexAccessors);
    expect(plainJson.meshes![0]!.primitives.length).toBe(3);
    expect(json.accessors![triangles[0]!.indices!]!.componentType).toBe(f.expect.indexComponentTypes.pattern);
    const single = primitives.find((p) => p.mode === undefined && p.extras === undefined)!;
    expect(json.accessors![single.indices!]!.componentType).toBe(f.expect.indexComponentTypes.triangle);
    expect(plainJson.accessors![plainJson.meshes![0]!.primitives[1]!.indices!]!.componentType).toBe(f.expect.indexComponentTypes.plainTriangle);
    expect(primitives.filter((p) => p.mode === f.expect.defaults.linesMode).length).toBeGreaterThan(0);
    expect(primitives.every((p) => p.mode === undefined || p.mode === f.expect.defaults.linesMode)).toBe(true);
    expect(plainJson.meshes![0]!.primitives.map((p) => p.mode)).toEqual([f.expect.defaults.trianglesMode, f.expect.defaults.trianglesMode, f.expect.defaults.linesMode]);
    const [leaf, glass] = json.materials!;
    expect(leaf!.alphaMode).toBeUndefined();
    expect(leaf!.doubleSided).toBeUndefined();
    expect([glass!.alphaMode, glass!.doubleSided]).toEqual([f.expect.kept.alphaMode, f.expect.kept.doubleSided]);
    expect([plainJson.materials![0]!.alphaMode, plainJson.materials![0]!.doubleSided]).toEqual([f.expect.defaults.alphaMode, f.expect.defaults.doubleSided]);
    expect(json.bufferViews!.length).toBeLessThanOrEqual(f.expect.maxBufferViews);
    expect(json.bufferViews!.length).toBeLessThan(json.accessors!.length);
    expect(plainJson.bufferViews!.length).toBe(plainJson.accessors!.length);
    for (const accessor of json.accessors!) {
      const view = json.bufferViews![accessor.bufferView!]!;
      if (view.target === 34962) expect(view.byteStride, "vertex attribute views carry their stride").toBeGreaterThanOrEqual(4);
      expect(((view.byteOffset ?? 0) + (accessor.byteOffset ?? 0)) % 4).toBe(0);
    }
    expect((await validator.validateBytes(compact, { uri: "structure.glb", maxIssues: 0 })).issues).toMatchObject({ numErrors: 0, numWarnings: 0 });
    const corners = (position: { getElement(i: number, target: number[]): number[] }, index: ArrayLike<number>, first: number, count: number): number[][] => Array.from({ length: count }, (_, i) => position.getElement(index[first + i]!, []));
    const wanted = f.copyOffsets.map((_, c) => Array.from({ length: per }, (_, i) => [positions[3 * (c * per + i)]!, positions[3 * (c * per + i) + 1]!, positions[3 * (c * per + i) + 2]!]));
    const read = (await io.readBinary(compact)).getRoot().listMeshes()[0]!.listPrimitives();
    for (const [c, id] of f.copyIds.entries()) {
      const primitive = read.find((p) => p.getMode() === 4 && p.getExtras().solids !== undefined && (p.getExtras().solids as { id: string }[])[0]!.id === id)!;
      expect(primitive.getExtras().solids).toEqual([{ id, first: 0, count: per }]);
      expect(corners(primitive.getAttribute("POSITION")!, primitive.getIndices()!.getArray()!, 0, per)).toEqual(wanted[c]);
      expect(primitive.getMaterial()!.getName()).toBe(f.plainMaterial);
    }
    const lone = read.find((p) => p.getMode() === 4 && p.getExtras().solids === undefined)!;
    expect(Array.from(lone.getIndices()!.getArray()!)).toEqual([0, 1, 2]);
    const segments = read.filter((p) => p.getMode() === 1).flatMap((p) => corners(p.getAttribute("POSITION")!, p.getIndices()!.getArray()!, 0, p.getIndices()!.getCount()));
    expect(segments).toEqual(f.copyOffsets.flatMap((_, c) => [0, 1, 1, 2].map((v) => wanted[c]![v]!)));
    const glb = await new Promise<any>((resolve, reject) => new GLTFLoader().parse(compact.buffer.slice(compact.byteOffset, compact.byteOffset + compact.byteLength) as ArrayBuffer, "", resolve, reject));
    let count = 0, doubled = 0, blended = 0;
    glb.scene.traverse((node: any) => {
      if (node.geometry === undefined || node.isLine || node.isLineSegments) return;
      const material = Array.isArray(node.material) ? node.material[0] : node.material;
      if (material.transparent) blended += 1;
      if (material.side === 2) doubled += 1;
      count += (node.geometry.index?.count ?? node.geometry.attributes.position.count) / 3;
    });
    expect(count).toBe(f.expect.triangleCount + 1);
    expect([blended, doubled]).toEqual([1, 1]);
    expect(compact.length).toBeLessThan(plain.length);
  });
});

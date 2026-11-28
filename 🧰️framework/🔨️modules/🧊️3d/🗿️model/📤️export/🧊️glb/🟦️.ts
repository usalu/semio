/** 🧊️ Owned GLB (binary glTF 2.0) encoder of the neutral 3D model kernel `semio.model/v1`; zero runtime dependencies, deterministic bytes.
 *
 * The neutral frame (right-handed, +Y up, +Z ahead, metres, radians, quaternions `[x, y, z, w]`, column-major matrices) is the glTF frame, so geometry and transforms are written unchanged.
 * Nodes carry TRS, meshes `triangles`/`lines` primitives with POSITION, NORMAL, COLOR_0, JOINTS_0, WEIGHTS_0, skins their inverse bind matrices, materials PBR metallic-roughness with `KHR_materials_unlit`,
 * Large repeated rigid parts share encoded attribute accessors across material batches and scenes. Each primitive retains the material and contiguous solid ranges of its source triangles or lines.
 * A skinned mesh node is a scene root (glTF ignores its parents' transforms for skinning) and keeps its source parent in `extras.parent`.
 * A primitive's solid provenance (`solids`: id, first index, index count) is written to the glTF primitive's `extras.solids` unchanged.
 * `variants` become `KHR_materials_variants`, animations node TRS channels plus material colour channels through `KHR_animation_pointer`; mechanism joints, tags, titles in both languages and clip metadata live in `extras`.
 *
 * @see https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html
 * @see https://github.com/KhronosGroup/glTF/tree/main/extensions/2.0/Khronos/KHR_materials_variants
 * @see https://github.com/KhronosGroup/glTF/tree/main/extensions/2.0/Khronos/KHR_animation_pointer
 */
import type { Animation, Channel, Material, ModelDocument, Node, Operation, Primitive, Slug, Text } from "../../🧬️schema/🟦️.ts";
import { poseModel, sampleMaterials } from "../../🦴️mechanism/🟦️.ts";
import type { PoseSelector } from "../../🦴️mechanism/🟦️.ts";
import { qNormalize } from "../../🧮️math/🟦️.ts";
import { selectScene } from "../../🎞️scenes/🟦️.ts";
import { Pace } from "../../⏱️operation/🟦️.ts";
import { BufferBuilder, BufferTarget, ComponentType, packGlb, shortFloat } from "./🧱️buffer/🟦️.ts";
import type { ElementType } from "./🧱️buffer/🟦️.ts";
import { partitionRigidParts } from "./🧱️parts/🟦️.ts";
import { weldUnread } from "./🧱️weld/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

export type { PoseSelector } from "../../🦴️mechanism/🟦️.ts";

/** ⚙️ What one encoding includes: an optional static pose baked into node TRS, the clips, the variants, and optionally one model scene (default: every scene as one glTF scene, the first is the default scene). `compact` (`KHR_mesh_quantization`, listed as used and required) leaves out the glTF defaults (`alphaMode` OPAQUE, `doubleSided` false, primitive `mode` 4), writes the indices of a primitive below 255 vertices as bytes, packs the accessors of one kind into shared buffer views, writes the normals of the primitives that a lit material can draw as normalized 8-bit integers and leaves them out where every material a primitive can show is unlit, welds the vertices of those unread-normal primitives that differ in nothing else inside a solid, and writes the colour factors and node transforms as the shortest decimals of the same 32-bit floats. */
export type GlbOptions = { readonly pose?: PoseSelector; readonly animations: boolean; readonly variants: boolean; readonly scene?: Slug; readonly compact?: boolean };

const GENERATOR = "semio.model glb/v1";
const MODES = { triangles: 4, lines: 1 } as const;
const IDENTITY_T = [0, 0, 0];
const IDENTITY_R = [0, 0, 0, 1];
const IDENTITY_S = [1, 1, 1];

type Json = Record<string, unknown>;

const unit = (q: readonly number[]): readonly number[] => (Math.abs(portable.hypot(q[0]!, q[1]!, q[2]!, q[3]!) - 1) < 1e-12 ? q : qNormalize(q));
const same = (a: ArrayLike<number>, b: readonly number[]): boolean => b.every((v, i) => a[i] === v);
const title = (text: Text): Text => ({ en: text.en, de: text.de });

function indexOf<T extends { readonly id: string }>(items: readonly T[], what: string): Map<string, number> {
  const map = new Map<string, number>();
  items.forEach((item, i) => {
    if (map.has(item.id)) throw new Error(`duplicate ${what} id ${item.id}`);
    map.set(item.id, i);
  });
  return map;
}

function lookup(map: Map<string, number>, id: string, what: string): number {
  const i = map.get(id);
  if (i === undefined) throw new Error(`unknown ${what} ${id}`);
  return i;
}

const floats = (values: readonly number[], short: boolean): number[] => (short ? values.map(shortFloat) : [...values]);

function materialJson(material: Material, short: boolean): Json {
  const out: Json = {
    name: material.id,
    pbrMetallicRoughness: { baseColorFactor: floats(material.baseColor, short), metallicFactor: short ? shortFloat(material.metallic) : material.metallic, roughnessFactor: short ? shortFloat(material.roughness) : material.roughness },
    ...(same(material.emissive, [0, 0, 0]) ? {} : { emissiveFactor: floats(material.emissive, short) }),
    ...(short && material.alphaMode !== "blend" ? {} : { alphaMode: material.alphaMode === "blend" ? "BLEND" : "OPAQUE" }),
    ...(short && !material.doubleSided ? {} : { doubleSided: material.doubleSided }),
    extras: { title: title(material.title) },
  };
  if (material.unlit) out.extensions = { KHR_materials_unlit: {} };
  return out;
}

function nodeJson(node: Node, local: Readonly<Record<"translation" | "rotation" | "scale", readonly number[]>>, meshes: Map<string, number>, skins: Map<string, number>, children: number[] | undefined, short: boolean): Json {
  const out: Json = { name: node.id };
  if (children !== undefined && children.length > 0) out.children = children;
  if (!same(local.translation, IDENTITY_T)) out.translation = floats(local.translation, short);
  const rotation = unit(local.rotation);
  if (!same(rotation, IDENTITY_R)) out.rotation = floats(rotation, short);
  if (!same(local.scale, IDENTITY_S)) out.scale = floats(local.scale, short);
  if (node.mesh !== undefined) out.mesh = lookup(meshes, node.mesh, "mesh");
  if (node.skin !== undefined) {
    if (node.mesh === undefined) throw new Error(`node ${node.id} has a skin but no mesh`);
    out.skin = lookup(skins, node.skin, "skin");
  }
  const extras: Json = {};
  if (node.joint !== undefined) extras.joint = JSON.parse(JSON.stringify(node.joint));
  if (node.tags !== undefined && node.tags.length > 0) extras.tags = [...node.tags];
  if (node.parent !== null && node.skin !== undefined) extras.parent = node.parent;
  if (Object.keys(extras).length > 0) out.extras = extras;
  return out;
}

function influences(joints: readonly number[], weights: readonly number[], pace: Pace): { joints: number[]; weights: number[] } {
  const j = Array.from(joints);
  const w = Array.from(weights);
  for (let v = 0; v < w.length; v += 4) {
    pace.tick(4);
    const sum = w[v]! + w[v + 1]! + w[v + 2]! + w[v + 3]!;
    if (!(sum > 0)) {
      j[v] = j[v + 1] = j[v + 2] = j[v + 3] = 0;
      w[v] = 1;
      w[v + 1] = w[v + 2] = w[v + 3] = 0;
      continue;
    }
    for (let k = 0; k < 4; k += 1) {
      w[v + k] = w[v + k]! / sum;
      if (w[v + k] === 0) j[v + k] = 0;
    }
  }
  return { joints: j, weights: w };
}

const NORMAL_BITS = 8;
const NORMAL_TYPE = NORMAL_BITS === 8 ? ComponentType.Byte : ComponentType.Short;

function quantizedNormals(normals: readonly number[], pace: Pace): number[] {
  const top = 2 ** (NORMAL_BITS - 1) - 1;
  const out = new Array<number>(normals.length);
  for (let i = 0; i < normals.length; i += 1) {
    pace.tick();
    out[i] = Math.round(Math.max(-1, Math.min(1, normals[i]!)) * top) + 0;
  }
  return out;
}

function primitiveJson(primitive: Primitive, jointCount: number | undefined, materials: Map<string, number>, buffer: BufferBuilder, pace: Pace, where: string, readsNormals: boolean, quantized: { used: boolean } | undefined): Json {
  const { positions, indices } = primitive;
  const vertices = positions.length / 3;
  if (!Number.isInteger(vertices)) throw new Error(`${where}: positions are not a multiple of 3`);
  const arity = primitive.mode === "triangles" ? 3 : 2;
  if (indices.length % arity !== 0) throw new Error(`${where}: ${primitive.mode} indices are not a multiple of ${arity}`);
  let max = -1;
  for (const index of indices) {
    pace.tick();
    if (index > max) max = index;
  }
  if (max >= vertices) throw new Error(`${where}: index ${max} exceeds ${vertices} vertices`);
  const attributes: Json = { POSITION: buffer.add(positions, ComponentType.Float, "VEC3", { target: BufferTarget.Attribute, bounds: true }) };
  const vec = (name: string, values: readonly number[] | undefined, type: ElementType, expected: number): void => {
    if (values === undefined) return;
    if (values.length !== vertices * expected) throw new Error(`${where}: ${name} needs ${expected} values per vertex`);
    attributes[name] = buffer.add(values, ComponentType.Float, type, { target: BufferTarget.Attribute });
  };
  if (primitive.mode === "triangles" && readsNormals && quantized === undefined) vec("NORMAL", primitive.normals, "VEC3", 3);
  else if (primitive.mode === "triangles" && readsNormals && primitive.normals !== undefined) {
    if (primitive.normals.length !== vertices * 3) throw new Error(`${where}: NORMAL needs 3 values per vertex`);
    attributes.NORMAL = buffer.add(quantizedNormals(primitive.normals, pace), NORMAL_TYPE, "VEC3", { target: BufferTarget.Attribute, normalized: true, aligned: true });
    quantized.used = true;
  }
  vec("COLOR_0", primitive.colors, "VEC4", 4);
  if (primitive.joints !== undefined || primitive.weights !== undefined) {
    if (primitive.joints === undefined || primitive.weights === undefined) throw new Error(`${where}: joints and weights come together`);
    if (primitive.joints.length !== vertices * 4 || primitive.weights.length !== vertices * 4) throw new Error(`${where}: joints and weights need 4 values per vertex`);
    if (jointCount !== undefined) {
      let top = 0;
      for (const joint of primitive.joints) {
        pace.tick();
        if (joint > top) top = joint;
      }
      if (top >= jointCount) throw new Error(`${where}: joint index ${top} exceeds the ${jointCount} joints of its skin`);
      const { joints, weights } = influences(primitive.joints, primitive.weights, pace);
      if (top > 65535) throw new Error(`${where}: joint index ${top} exceeds 65535`);
      attributes.JOINTS_0 = buffer.add(joints, top > 255 ? ComponentType.UnsignedShort : ComponentType.UnsignedByte, "VEC4", { target: BufferTarget.Attribute });
      attributes.WEIGHTS_0 = buffer.add(weights, ComponentType.Float, "VEC4", { target: BufferTarget.Attribute });
    }
  }
  const solids = primitive.solids;
  if (solids !== undefined) {
    let cursor = 0;
    for (const range of solids) {
      pace.tick();
      if (range.first !== cursor || !(range.count >= arity) || range.count % arity !== 0) throw new Error(`${where}: solid ranges must be contiguous and in whole ${primitive.mode}`);
      cursor += range.count;
    }
    if (cursor !== indices.length) throw new Error(`${where}: solid ranges must cover all ${indices.length} indices`);
  }
  return {
    attributes,
    indices: buffer.add(indices, quantized !== undefined && max < 255 ? ComponentType.UnsignedByte : max < 65535 ? ComponentType.UnsignedShort : ComponentType.UnsignedInt, "SCALAR", { target: BufferTarget.Index }),
    material: lookup(materials, primitive.material, "material"),
    ...(quantized !== undefined && primitive.mode === "triangles" ? {} : { mode: MODES[primitive.mode] }),
    ...(solids !== undefined && solids.length > 0 ? { extras: { solids: solids.map((range) => ({ id: range.id, first: range.first, count: range.count })) } } : {}),
  };
}

const POINTERS = { baseColor: "pbrMetallicRoughness/baseColorFactor", emissive: "emissiveFactor" } as const;
const PATH_TYPE: Record<string, [string, ElementType]> = { translation: ["translation", "VEC3"], rotation: ["rotation", "VEC4"], scale: ["scale", "VEC3"], baseColor: ["baseColor", "VEC4"], emissive: ["emissive", "VEC3"] };

function animationJson(animation: Animation, known: ReadonlySet<string>, instances: readonly Map<string, number>[], materials: Map<string, number>, buffer: BufferBuilder, pace: Pace): Json {
  const samplers: Json[] = [];
  const channels: Json[] = [];
  animation.channels.forEach((channel: Channel, c) => {
    const where = `animation ${animation.id} channel ${c}`;
    const target = channel.target;
    const [, type] = PATH_TYPE[target.path]!;
    const width = type === "VEC4" ? 4 : 3;
    if (channel.values.length !== channel.times.length * width) throw new Error(`${where}: ${channel.times.length} keys of ${type} need ${channel.times.length * width} values`);
    for (let i = 0; i < channel.times.length; i += 1) if (pace.tick(), !(channel.times[i]! >= 0) || (i > 0 && !(channel.times[i]! > channel.times[i - 1]!))) throw new Error(`${where}: times must be non-negative and strictly increasing`);
    if ("node" in target && !known.has(target.node)) throw new Error(`unknown node ${target.node}`);
    const targets = "node" in target ? instances.flatMap((nodes) => (nodes.has(target.node) ? [nodes.get(target.node)!] : [])) : [];
    if ("node" in target && targets.length === 0) return;
    let values: readonly number[] = channel.values;
    if (target.path === "rotation") values = Array.from({ length: channel.times.length }, (_, k) => unit(channel.values.slice(k * 4, k * 4 + 4))).flat();
    const sampler = samplers.length;
    samplers.push({
      input: buffer.add(channel.times, ComponentType.Float, "SCALAR", { bounds: true }),
      output: buffer.add(values, ComponentType.Float, type),
      interpolation: channel.interpolation === "step" ? "STEP" : "LINEAR",
    });
    if ("node" in target) for (const node of targets) channels.push({ sampler, target: { node, path: target.path } });
    else channels.push({ sampler, target: { path: "pointer", extensions: { KHR_animation_pointer: { pointer: `/materials/${lookup(materials, target.material, "material")}/${POINTERS[target.path]}` } } } });
  });
  return { name: animation.id, samplers, channels, extras: { title: title(animation.title), durationS: animation.durationS, loop: animation.loop, events: animation.events.map((event) => ({ t: event.t, id: event.id })) } };
}

/** 🏺️ Encodes a resolved {@link ModelDocument} as GLB bytes; throws on procedural documents, dangling references and cancellation (`CancelledError`, polled between phases and every few thousand values inside any accessor, no bytes are produced).
 *
 * A document without scenes becomes one glTF scene of all its nodes. With model scenes every scene becomes one glTF scene (`scene` = the first); with `options.scene` only that scene and only the meshes, skins and clip channels it needs are written (unknown scene: `UnknownSceneError`): glTF has one parent per node, so the nodes a scene needs (its mesh nodes, their ancestors, the joints of their skins) are written once per scene they serve (same name), while meshes, accessors, materials, variants, skin inverse-bind matrices and animation samplers are shared; an animation channel is repeated for every copy of its node. */
export function encodeGlb(whole: ModelDocument, options: GlbOptions, operation: Operation): Uint8Array {
  const document = options.scene === undefined ? whole : selectScene(whole, options.scene);
  if (document.solids.length > 0) throw new Error(`document ${document.id} is procedural (${document.solids.length} solids): tessellate it before encoding`);
  const pace = new Pace(operation);
  const step = (fraction: number): void => pace.step(fraction);
  step(0);
  const explicit = document.scenes !== undefined && document.scenes.length > 0;
  const chosen = explicit ? document.scenes! : [];
  const instances: { readonly name: string; readonly extras: Json; readonly document: ModelDocument }[] = explicit
    ? chosen.map((scene) => ({ name: scene.id, extras: { title: title(scene.title), nodes: [...scene.nodes], ...(scene.tags === undefined ? {} : { tags: [...scene.tags] }) }, document: selectScene(document, scene.id) }))
    : [{ name: document.id, extras: { title: title(document.title) }, document }];
  const posed = options.pose === undefined || options.pose === null ? undefined : poseModel(document, options.pose);
  const colors = typeof options.pose === "object" && options.pose !== null ? sampleMaterials(document, options.pose.animation, options.pose.timeS) : {};
  indexOf(document.nodes, "node");
  const meshIndex = indexOf(document.meshes, "mesh");
  const skinIndex = indexOf(document.skins, "skin");
  const materials = document.materials.map((material): Material => {
    const baked = colors[material.id];
    return baked === undefined ? material : { ...material, baseColor: (baked.baseColor as unknown as Material["baseColor"] | undefined) ?? material.baseColor, emissive: (baked.emissive as unknown as Material["emissive"] | undefined) ?? material.emissive };
  });
  const materialIndex = indexOf(materials, "material");
  const buffer = new BufferBuilder(pace, options.compact === true);

  const skinnedMeshes = new Map<string, number>();
  for (const node of document.nodes) {
    if (node.mesh === undefined || node.skin === undefined) continue;
    const count = document.skins[lookup(skinIndex, node.skin, "skin")]!.joints.length;
    skinnedMeshes.set(node.mesh, Math.min(count, skinnedMeshes.get(node.mesh) ?? count));
  }
  step(0.02);

  const unlit = new Set(materials.filter((material) => material.unlit).map((material) => material.id));
  const quantized = { used: false };
  const readsNormals = (primitive: Primitive): boolean => options.compact !== true || !unlit.has(primitive.material) || (options.variants && document.variants.some((variant) => {
    const bound = variant.bindings[primitive.material];
    return bound !== undefined && !unlit.has(bound);
  }));
  const meshes = partitionRigidParts(options.compact === true ? weldUnread(document.meshes, readsNormals, pace) : document.meshes, pace, options.compact === true);
  const totalPrimitives = meshes.reduce((n, mesh) => n + mesh.primitives.length, 0);
  let done = 0;
  const gltfMeshes = meshes.map((mesh) => ({
    name: mesh.id,
    primitives: mesh.primitives.map((primitive, p) => {
      step(0.02 + (0.68 * done++) / Math.max(1, totalPrimitives));
      return primitiveJson(primitive, skinnedMeshes.get(mesh.id), materialIndex, buffer, pace, `mesh ${mesh.id} primitive ${p}`, readsNormals(primitive), options.compact === true ? quantized : undefined);
    }),
  }));

  const inverseBind = new Map<string, number>();
  const gltfSkins: Json[] = [];
  const gltfNodes: Json[] = [];
  const instanceNodes: Map<string, number>[] = [];
  const sceneRoots: number[][] = [];
  for (const instance of instances) {
    const nodes = instance.document.nodes;
    const first = gltfNodes.length;
    const ids = new Map<string, number>(nodes.map((node, i) => [node.id, first + i]));
    const local = new Map(nodes.map((node, i) => [node.id, i]));
    const children = new Map<number, number[]>();
    const roots: number[] = [];
    nodes.forEach((node, i) => {
      pace.tick();
      if (node.parent === null || (node.skin !== undefined && node.mesh !== undefined)) roots.push(first + i);
      else children.set(lookup(local, node.parent, "parent node"), [...(children.get(lookup(local, node.parent, "parent node")) ?? []), first + i]);
    });
    const skinsHere = new Map<string, number>();
    for (const skin of instance.document.skins) {
      step(0.72);
      if (skin.inverseBind.length !== skin.joints.length) throw new Error(`skin ${skin.id}: ${skin.joints.length} joints need as many inverse bind matrices, got ${skin.inverseBind.length}`);
      if (!inverseBind.has(skin.id)) inverseBind.set(skin.id, buffer.add(skin.inverseBind.flat(), ComponentType.Float, "MAT4"));
      skinsHere.set(skin.id, gltfSkins.length);
      gltfSkins.push({ name: skin.id, joints: skin.joints.map((joint) => lookup(ids, joint, "joint node")), inverseBindMatrices: inverseBind.get(skin.id)! });
    }
    nodes.forEach((node, i) => gltfNodes.push(nodeJson(node, posed?.local[node.id] ?? node, meshIndex, skinsHere, children.get(i), options.compact === true)));
    instanceNodes.push(ids);
    sceneRoots.push(roots);
  }
  step(0.78);

  const used = new Set<string>();
  const gltfMaterials = materials.map((material) => materialJson(material, options.compact === true));
  if (materials.some((material) => material.unlit)) used.add("KHR_materials_unlit");
  if (quantized.used) used.add("KHR_mesh_quantization");

  const extensions: Json = {};
  if (options.variants && document.variants.length > 0) {
    used.add("KHR_materials_variants");
    extensions.KHR_materials_variants = { variants: document.variants.map((variant) => ({ name: variant.id, extras: { title: title(variant.title) } })) };
    meshes.forEach((mesh, m) =>
      mesh.primitives.forEach((primitive, p) => {
        pace.tick(document.variants.length);
        const byTarget = new Map<number, number[]>();
        document.variants.forEach((variant, v) => {
          const bound = variant.bindings[primitive.material];
          if (bound === undefined || bound === primitive.material) return;
          const target = lookup(materialIndex, bound, "variant material");
          byTarget.set(target, [...(byTarget.get(target) ?? []), v]);
        });
        if (byTarget.size > 0) (gltfMeshes[m]!.primitives[p] as Json).extensions = { KHR_materials_variants: { mappings: [...byTarget].map(([material, variants]) => ({ material, variants })) } };
      }),
    );
  }

  const known = new Set(document.nodes.map((node) => node.id));
  const gltfAnimations: Json[] = [];
  if (options.animations)
    for (const animation of document.animations) {
      step(0.8 + (0.18 * gltfAnimations.length) / Math.max(1, document.animations.length));
      gltfAnimations.push(animationJson(animation, known, instanceNodes, materialIndex, buffer, pace));
      if (animation.channels.some((channel) => "material" in channel.target)) used.add("KHR_animation_pointer");
    }

  const asset: Json = { version: "2.0", generator: GENERATOR, extras: { schema: document.version, id: document.id, title: title(document.title), ...(document.provenance === undefined ? {} : { provenance: JSON.parse(JSON.stringify(document.provenance)) }) } };
  const json: Json = { asset, scene: 0, scenes: instances.map((instance, i) => ({ name: instance.name, nodes: sceneRoots[i], extras: instance.extras })) };
  if (used.size > 0) json.extensionsUsed = [...used].sort();
  if (quantized.used) json.extensionsRequired = ["KHR_mesh_quantization"];
  if (Object.keys(extensions).length > 0) json.extensions = extensions;
  if (gltfNodes.length > 0) json.nodes = gltfNodes;
  if (gltfMeshes.length > 0) json.meshes = gltfMeshes;
  if (gltfSkins.length > 0) json.skins = gltfSkins;
  if (gltfMaterials.length > 0) json.materials = gltfMaterials;
  if (gltfAnimations.length > 0) json.animations = gltfAnimations;
  if (buffer.byteLength > 0) {
    json.buffers = [{ byteLength: buffer.byteLength }];
    json.bufferViews = buffer.views;
    json.accessors = buffer.accessors;
  }
  step(0.99);
  const bytes = packGlb(JSON.stringify(json), buffer.bytes());
  step(1);
  return bytes;
}

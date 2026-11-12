/** 🧊️ Owned GLB (binary glTF 2.0) encoder of the neutral 3D model kernel `semio.model/v1`; zero runtime dependencies, deterministic bytes.
 *
 * The neutral frame (right-handed, +Y up, +Z ahead, metres, radians, quaternions `[x, y, z, w]`, column-major matrices) is the glTF frame, so geometry and transforms are written unchanged.
 * Nodes carry TRS, meshes `triangles`/`lines` primitives with POSITION, NORMAL, COLOR_0, JOINTS_0, WEIGHTS_0, skins their inverse bind matrices, materials PBR metallic-roughness with `KHR_materials_unlit`,
 * A skinned mesh node is a scene root (glTF ignores its parents' transforms for skinning) and keeps its source parent in `extras.parent`.
 * A primitive's solid provenance (`solids`: id, first index, index count) is written to the glTF primitive's `extras.solids` unchanged.
 * `variants` become `KHR_materials_variants`, animations node TRS channels plus material colour channels through `KHR_animation_pointer`; mechanism joints, tags, titles in both languages and clip metadata live in `extras`.
 *
 * @see https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html
 * @see https://github.com/KhronosGroup/glTF/tree/main/extensions/2.0/Khronos/KHR_materials_variants
 * @see https://github.com/KhronosGroup/glTF/tree/main/extensions/2.0/Khronos/KHR_animation_pointer
 */
import type { Animation, Channel, Material, ModelDocument, Node, Operation, Primitive, Text } from "../../🧬️schema/🟦️.ts";
import { poseModel, sampleMaterials } from "../../🦴️mechanism/🟦️.ts";
import type { PoseSelector } from "../../🦴️mechanism/🟦️.ts";
import { qNormalize } from "../../🧮️math/🟦️.ts";
import { Pace } from "../../⏱️operation/🟦️.ts";
import { BufferBuilder, BufferTarget, ComponentType, packGlb } from "./🧱️buffer/🟦️.ts";
import type { ElementType } from "./🧱️buffer/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

export type { PoseSelector } from "../../🦴️mechanism/🟦️.ts";

/** ⚙️ What one encoding includes: an optional static pose baked into node TRS, the clips, the variants. */
export type GlbOptions = { readonly pose?: PoseSelector; readonly animations: boolean; readonly variants: boolean };

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

function materialJson(material: Material): Json {
  const out: Json = {
    name: material.id,
    pbrMetallicRoughness: { baseColorFactor: [...material.baseColor], metallicFactor: material.metallic, roughnessFactor: material.roughness },
    ...(same(material.emissive, [0, 0, 0]) ? {} : { emissiveFactor: [...material.emissive] }),
    alphaMode: material.alphaMode === "blend" ? "BLEND" : "OPAQUE",
    doubleSided: material.doubleSided,
    extras: { title: title(material.title) },
  };
  if (material.unlit) out.extensions = { KHR_materials_unlit: {} };
  return out;
}

function nodeJson(node: Node, local: Readonly<Record<"translation" | "rotation" | "scale", readonly number[]>>, meshes: Map<string, number>, skins: Map<string, number>, children: number[] | undefined): Json {
  const out: Json = { name: node.id };
  if (children !== undefined && children.length > 0) out.children = children;
  if (!same(local.translation, IDENTITY_T)) out.translation = [...local.translation];
  const rotation = unit(local.rotation);
  if (!same(rotation, IDENTITY_R)) out.rotation = [...rotation];
  if (!same(local.scale, IDENTITY_S)) out.scale = [...local.scale];
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

function primitiveJson(primitive: Primitive, jointCount: number | undefined, materials: Map<string, number>, buffer: BufferBuilder, pace: Pace, where: string): Json {
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
  if (primitive.mode === "triangles") vec("NORMAL", primitive.normals, "VEC3", 3);
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
    indices: buffer.add(indices, max < 65535 ? ComponentType.UnsignedShort : ComponentType.UnsignedInt, "SCALAR", { target: BufferTarget.Index }),
    material: lookup(materials, primitive.material, "material"),
    mode: MODES[primitive.mode],
    ...(solids !== undefined && solids.length > 0 ? { extras: { solids: solids.map((range) => ({ id: range.id, first: range.first, count: range.count })) } } : {}),
  };
}

const POINTERS = { baseColor: "pbrMetallicRoughness/baseColorFactor", emissive: "emissiveFactor" } as const;
const PATH_TYPE: Record<string, [string, ElementType]> = { translation: ["translation", "VEC3"], rotation: ["rotation", "VEC4"], scale: ["scale", "VEC3"], baseColor: ["baseColor", "VEC4"], emissive: ["emissive", "VEC3"] };

function animationJson(animation: Animation, nodes: Map<string, number>, materials: Map<string, number>, buffer: BufferBuilder, pace: Pace): Json {
  const samplers: Json[] = [];
  const channels: Json[] = [];
  animation.channels.forEach((channel: Channel, c) => {
    const where = `animation ${animation.id} channel ${c}`;
    const target = channel.target;
    const [, type] = PATH_TYPE[target.path]!;
    const width = type === "VEC4" ? 4 : 3;
    if (channel.values.length !== channel.times.length * width) throw new Error(`${where}: ${channel.times.length} keys of ${type} need ${channel.times.length * width} values`);
    for (let i = 0; i < channel.times.length; i += 1) if (pace.tick(), !(channel.times[i]! >= 0) || (i > 0 && !(channel.times[i]! > channel.times[i - 1]!))) throw new Error(`${where}: times must be non-negative and strictly increasing`);
    let values: readonly number[] = channel.values;
    if (target.path === "rotation") values = Array.from({ length: channel.times.length }, (_, k) => unit(channel.values.slice(k * 4, k * 4 + 4))).flat();
    samplers.push({
      input: buffer.add(channel.times, ComponentType.Float, "SCALAR", { bounds: true }),
      output: buffer.add(values, ComponentType.Float, type),
      interpolation: channel.interpolation === "step" ? "STEP" : "LINEAR",
    });
    if ("node" in target) channels.push({ sampler: c, target: { node: lookup(nodes, target.node, "node"), path: target.path } });
    else channels.push({ sampler: c, target: { path: "pointer", extensions: { KHR_animation_pointer: { pointer: `/materials/${lookup(materials, target.material, "material")}/${POINTERS[target.path]}` } } } });
  });
  return { name: animation.id, samplers, channels, extras: { title: title(animation.title), durationS: animation.durationS, loop: animation.loop, events: animation.events.map((event) => ({ t: event.t, id: event.id })) } };
}

/** 🏺️ Encodes a resolved {@link ModelDocument} as GLB bytes; throws on procedural documents, dangling references and cancellation (`CancelledError`, polled between phases and every few thousand values inside any accessor, no bytes are produced). */
export function encodeGlb(document: ModelDocument, options: GlbOptions, operation: Operation): Uint8Array {
  if (document.solids.length > 0) throw new Error(`document ${document.id} is procedural (${document.solids.length} solids): tessellate it before encoding`);
  const pace = new Pace(operation);
  const step = (fraction: number): void => pace.step(fraction);
  step(0);
  const posed = options.pose === undefined || options.pose === null ? undefined : poseModel(document, options.pose);
  const colors = typeof options.pose === "object" && options.pose !== null ? sampleMaterials(document, options.pose.animation, options.pose.timeS) : {};
  const nodeIndex = indexOf(document.nodes, "node");
  const meshIndex = indexOf(document.meshes, "mesh");
  const skinIndex = indexOf(document.skins, "skin");
  const materials = document.materials.map((material): Material => {
    const baked = colors[material.id];
    return baked === undefined ? material : { ...material, baseColor: (baked.baseColor as unknown as Material["baseColor"] | undefined) ?? material.baseColor, emissive: (baked.emissive as unknown as Material["emissive"] | undefined) ?? material.emissive };
  });
  const materialIndex = indexOf(materials, "material");
  const buffer = new BufferBuilder(pace);

  const children = new Map<number, number[]>();
  const roots: number[] = [];
  document.nodes.forEach((node, i) => {
    pace.tick();
    if (node.parent === null || (node.skin !== undefined && node.mesh !== undefined)) roots.push(i);
    else children.set(lookup(nodeIndex, node.parent, "parent node"), [...(children.get(lookup(nodeIndex, node.parent, "parent node")) ?? []), i]);
  });
  step(0.02);

  const skinnedMeshes = new Map<string, number>();
  for (const node of document.nodes) {
    if (node.mesh === undefined || node.skin === undefined) continue;
    const count = document.skins[lookup(skinIndex, node.skin, "skin")]!.joints.length;
    skinnedMeshes.set(node.mesh, Math.min(count, skinnedMeshes.get(node.mesh) ?? count));
  }

  const totalPrimitives = document.meshes.reduce((n, mesh) => n + mesh.primitives.length, 0);
  let done = 0;
  const gltfMeshes = document.meshes.map((mesh) => ({
    name: mesh.id,
    primitives: mesh.primitives.map((primitive, p) => {
      step(0.02 + (0.68 * done++) / Math.max(1, totalPrimitives));
      return primitiveJson(primitive, skinnedMeshes.get(mesh.id), materialIndex, buffer, pace, `mesh ${mesh.id} primitive ${p}`);
    }),
  }));

  const gltfSkins = document.skins.map((skin) => {
    step(0.72);
    if (skin.inverseBind.length !== skin.joints.length) throw new Error(`skin ${skin.id}: ${skin.joints.length} joints need as many inverse bind matrices, got ${skin.inverseBind.length}`);
    return { name: skin.id, joints: skin.joints.map((joint) => lookup(nodeIndex, joint, "joint node")), inverseBindMatrices: buffer.add(skin.inverseBind.flat(), ComponentType.Float, "MAT4") };
  });

  const gltfNodes = document.nodes.map((node, i) => nodeJson(node, posed?.local[node.id] ?? node, meshIndex, skinIndex, children.get(i)));
  step(0.78);

  const used = new Set<string>();
  const gltfMaterials = materials.map(materialJson);
  if (materials.some((material) => material.unlit)) used.add("KHR_materials_unlit");

  const extensions: Json = {};
  if (options.variants && document.variants.length > 0) {
    used.add("KHR_materials_variants");
    extensions.KHR_materials_variants = { variants: document.variants.map((variant) => ({ name: variant.id, extras: { title: title(variant.title) } })) };
    document.meshes.forEach((mesh, m) =>
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

  const gltfAnimations: Json[] = [];
  if (options.animations)
    for (const animation of document.animations) {
      step(0.8 + (0.18 * gltfAnimations.length) / Math.max(1, document.animations.length));
      gltfAnimations.push(animationJson(animation, nodeIndex, materialIndex, buffer, pace));
      if (animation.channels.some((channel) => "material" in channel.target)) used.add("KHR_animation_pointer");
    }

  const asset: Json = { version: "2.0", generator: GENERATOR, extras: { schema: document.version, id: document.id, title: title(document.title), ...(document.provenance === undefined ? {} : { provenance: JSON.parse(JSON.stringify(document.provenance)) }) } };
  const json: Json = { asset, scene: 0, scenes: [{ name: document.id, nodes: roots, extras: { title: title(document.title) } }] };
  if (used.size > 0) json.extensionsUsed = [...used].sort();
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

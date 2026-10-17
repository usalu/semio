/** ✂️ glTF fixture generator: the generic create, delete, move and reorder edits on the raw exported document and the index remapping every reference family needs. */
import type { Doc } from "../🏗️base-document/🟦️.ts";

//#region ✂️Generic structural edit
/** 🔧 Every `create/delete/move/reorder` mutation in this subset's manifest is one of these four
 *  ARRAY shapes — read directly off each mutation's own `🧬️schema/🔣️.json` `x-semio.touchedPaths`,
 *  never guessed. Applying them generically is what lets one engine cover the ~90 kinds that share this
 *  shape instead of writing bespoke code per kind. */
function arrayAt(doc: Doc, path: readonly (string | number)[]): unknown[] {
  let node: Doc = doc;
  for (const segment of path) node = node[segment];
  if (!Array.isArray(node)) throw new Error(`not an array at ${path.join("/")}`);
  return node;
}

export function create(doc: Doc, path: readonly (string | number)[], position: number, value: unknown): void {
  arrayAt(doc, path).splice(position, 0, value);
}
export function del(doc: Doc, path: readonly (string | number)[], index: number): void {
  arrayAt(doc, path).splice(index, 1);
}
export function move(doc: Doc, path: readonly (string | number)[], index: number, position: number): void {
  const array = arrayAt(doc, path);
  const [item] = array.splice(index, 1);
  array.splice(position, 0, item);
}
export function reorder(doc: Doc, path: readonly (string | number)[], order: readonly number[]): void {
  const array = arrayAt(doc, path);
  const original = [...array];
  array.length = 0;
  for (const index of order) array.push(original[index]);
}
/** 🗺️ Attribute maps (`primitive.attributes`, `target.attributes`) are JS objects, not arrays — the
 *  manifest still treats key order as semantic (`reorder-primitive-attributes` etc.), so the same four
 *  shapes apply to `Object.entries` order instead of array splice. */
function mapAt(doc: Doc, path: readonly (string | number)[]): Record<string, unknown> {
  let node: Doc = doc;
  for (const segment of path) node = node[segment];
  return node as Record<string, unknown>;
}
export function reorderKeys(doc: Doc, path: readonly (string | number)[], order: readonly string[]): void {
  const map = mapAt(doc, path);
  const entries = Object.fromEntries(order.map((key) => [key, map[key]]));
  for (const key of Object.keys(map)) delete map[key];
  Object.assign(map, entries);
}
export function moveKey(doc: Doc, path: readonly (string | number)[], key: string, position: number): void {
  const map = mapAt(doc, path);
  const keys = Object.keys(map);
  const from = keys.indexOf(key);
  keys.splice(from, 1);
  keys.splice(position, 0, key);
  reorderKeys(doc, path, keys);
}

/** 🗺️ Reindexing. Every one of `nodes`/`meshes`/`materials`/`cameras`/`skins`/`scenes` is referenced by
 *  INTEGER INDEX from elsewhere in the document, so create/delete/move/reorder on the array itself must
 *  walk every reference site and remap it — otherwise a shifted array silently detaches every reference
 *  that used to point at the right element. `fn` maps an OLD index to its NEW one, or `undefined` to
 *  drop the reference entirely (used only for `delete`). Each `remapXRefs` below is the full, honest
 *  list of every place this subset's schema lets that entity be referenced from. */
export type Remap = (oldIndex: number) => number | undefined;
export function remapNodeRefs(doc: Doc, fn: Remap): void {
  for (const node of doc.nodes ?? []) if (Array.isArray(node.children)) node.children = (node.children as number[]).map(fn).filter((i): i is number => i !== undefined);
  for (const scene of doc.scenes ?? []) if (Array.isArray(scene.nodes)) scene.nodes = (scene.nodes as number[]).map(fn).filter((i): i is number => i !== undefined);
  for (const skin of doc.skins ?? []) {
    if (Array.isArray(skin.joints)) skin.joints = (skin.joints as number[]).map(fn).filter((i): i is number => i !== undefined);
    if (typeof skin.skeleton === "number") {
      const mapped = fn(skin.skeleton);
      if (mapped === undefined) delete skin.skeleton;
      else skin.skeleton = mapped;
    }
  }
  for (const animation of doc.animations ?? []) for (const channel of animation.channels ?? []) {
    const mapped = fn(channel.target.node);
    if (mapped !== undefined) channel.target.node = mapped;
  }
}
export function remapMeshRefs(doc: Doc, fn: Remap): void {
  for (const node of doc.nodes ?? []) if (typeof node.mesh === "number") {
    const mapped = fn(node.mesh);
    if (mapped === undefined) delete node.mesh;
    else node.mesh = mapped;
  }
}
export function remapMaterialRefs(doc: Doc, fn: Remap): void {
  for (const mesh of doc.meshes ?? []) for (const primitive of mesh.primitives ?? []) if (typeof primitive.material === "number") {
    const mapped = fn(primitive.material);
    if (mapped === undefined) delete primitive.material;
    else primitive.material = mapped;
  }
}
export function remapCameraRefs(doc: Doc, fn: Remap): void {
  for (const node of doc.nodes ?? []) if (typeof node.camera === "number") {
    const mapped = fn(node.camera);
    if (mapped === undefined) delete node.camera;
    else node.camera = mapped;
  }
}
export function remapSkinRefs(doc: Doc, fn: Remap): void {
  for (const node of doc.nodes ?? []) if (typeof node.skin === "number") {
    const mapped = fn(node.skin);
    if (mapped === undefined) delete node.skin;
    else node.skin = mapped;
  }
}
export function remapSceneRefs(doc: Doc, fn: Remap): void {
  if (typeof doc.scene === "number") {
    const mapped = fn(doc.scene);
    if (mapped === undefined) delete doc.scene;
    else doc.scene = mapped;
  }
}

/** 🧱️ The six RESOURCE families. Referenced by integer index exactly like the six above, but from a
 *  different set of sites — and unlike nodes/meshes/materials, a resource may legitimately be
 *  referenced by NOTHING at all, which is precisely the case `three` cannot witness and the reason
 *  these twelve-plus-twelve kinds needed a document-level reader. */
export function remapAccessorRefs(doc: Doc, fn: Remap): void {
  const mapField = (holder: Doc, key: string): void => {
    if (typeof holder[key] !== "number") return;
    const mapped = fn(holder[key] as number);
    if (mapped === undefined) delete holder[key];
    else holder[key] = mapped;
  };
  for (const mesh of doc.meshes ?? []) for (const primitive of mesh.primitives ?? []) {
    mapField(primitive, "indices");
    for (const attributes of [primitive.attributes as Doc, ...((primitive.targets as Doc[]) ?? [])]) {
      if (!attributes) continue;
      for (const key of Object.keys(attributes)) mapField(attributes, key);
    }
  }
  for (const skin of doc.skins ?? []) mapField(skin, "inverseBindMatrices");
  for (const animation of doc.animations ?? []) for (const sampler of animation.samplers ?? []) {
    mapField(sampler, "input");
    mapField(sampler, "output");
  }
}
export function remapBufferViewRefs(doc: Doc, fn: Remap): void {
  const mapField = (holder: Doc, key: string): void => {
    if (typeof holder[key] !== "number") return;
    const mapped = fn(holder[key] as number);
    if (mapped === undefined) delete holder[key];
    else holder[key] = mapped;
  };
  for (const accessor of doc.accessors ?? []) {
    mapField(accessor, "bufferView");
    const sparse = accessor.sparse as Doc | undefined;
    if (sparse) {
      if (sparse.indices) mapField(sparse.indices as Doc, "bufferView");
      if (sparse.values) mapField(sparse.values as Doc, "bufferView");
    }
  }
  for (const image of doc.images ?? []) mapField(image, "bufferView");
}
export function remapBufferRefs(doc: Doc, fn: Remap): void {
  for (const view of doc.bufferViews ?? []) if (typeof view.buffer === "number") {
    const mapped = fn(view.buffer);
    if (mapped === undefined) delete view.buffer;
    else view.buffer = mapped;
  }
}
export function remapImageRefs(doc: Doc, fn: Remap): void {
  for (const texture of doc.textures ?? []) if (typeof texture.source === "number") {
    const mapped = fn(texture.source);
    if (mapped === undefined) delete texture.source;
    else texture.source = mapped;
  }
}
export function remapSamplerRefs(doc: Doc, fn: Remap): void {
  for (const texture of doc.textures ?? []) if (typeof texture.sampler === "number") {
    const mapped = fn(texture.sampler);
    if (mapped === undefined) delete texture.sampler;
    else texture.sampler = mapped;
  }
}
export function remapTextureRefs(doc: Doc, fn: Remap): void {
  const mapInfo = (info: Doc | undefined): void => {
    if (!info || typeof info.index !== "number") return;
    const mapped = fn(info.index as number);
    if (mapped === undefined) delete info.index;
    else info.index = mapped;
  };
  for (const material of doc.materials ?? []) {
    const pbr = material.pbrMetallicRoughness as Doc | undefined;
    mapInfo(pbr?.baseColorTexture as Doc | undefined);
    mapInfo(pbr?.metallicRoughnessTexture as Doc | undefined);
    mapInfo(material.normalTexture as Doc | undefined);
    mapInfo(material.occlusionTexture as Doc | undefined);
    mapInfo(material.emissiveTexture as Doc | undefined);
  }
}

/** ➕➖➡️🔀 The four array operations, each paired with the reference-family it must keep coherent. */
export function createReindexed(doc: Doc, path: readonly (string | number)[], position: number, value: unknown, remap: (fn: Remap) => void): void {
  create(doc, path, position, value);
  remap((i) => (i >= position ? i + 1 : i));
}
export function deleteReindexed(doc: Doc, path: readonly (string | number)[], index: number, remap: (fn: Remap) => void): void {
  del(doc, path, index);
  remap((i) => (i === index ? undefined : i > index ? i - 1 : i));
}
export function moveReindexed(doc: Doc, path: readonly (string | number)[], from: number, to: number, remap: (fn: Remap) => void): void {
  move(doc, path, from, to);
  remap((i) => {
    if (i === from) return to;
    if (from < to && i > from && i <= to) return i - 1;
    if (to < from && i >= to && i < from) return i + 1;
    return i;
  });
}
export function reorderReindexed(doc: Doc, path: readonly (string | number)[], order: readonly number[], remap: (fn: Remap) => void): void {
  const inverse = new Array<number>(order.length);
  order.forEach((oldIndex, newPosition) => {
    inverse[oldIndex] = newPosition;
  });
  reorder(doc, path, order);
  remap((i) => inverse[i]);
}
//#endregion ✂️Generic structural edit

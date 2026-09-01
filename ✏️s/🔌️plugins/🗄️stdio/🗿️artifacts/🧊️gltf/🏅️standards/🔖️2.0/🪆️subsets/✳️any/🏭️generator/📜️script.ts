#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Fixture generator for `s.stdio.gltf@2.0/✳️any`, reader-pattern retrofit.
//
// ONE real glTF document is built by `three` 0.182.0's own `GLTFExporter` (real scene graph:
// materials, a skinned+morphed mesh, a camera, two scenes, an animation clip) — that call does the
// actual byte-level work this generator never re-implements: accessor/bufferView layout, binary
// packing, hierarchy flattening, material serialization. Every recipe's `before`/`after` pair is then
// derived from that ONE base document by a small, GENERIC, mechanically-uniform structural edit —
// `applyOp` below — never a per-recipe hand-authored document. The edits are uniform because the
// mutation surface itself is: every `🔣️payload.schema.json`'s own `x-semio.touchedPaths` describes the
// same handful of shapes repeated across 13 entity families (`create-X{position}`, `delete-X{index}`,
// `move-X{index,position}`, `reorder-X{order[]}`, `bind/unbind-X-Y{parent,child,position}`), read
// directly out of those schemas, never guessed.
//
// Usage:
//   bun 📜️script.ts generate --only <fixture-id>   # writes SEMIO_FIXTURE_OUT/<id>/{before,after}.gltf
//   bun 📜️script.ts list                           # prints every recipe id this generator knows
//
// @see ../🔬️probes/📜️script.ts — the reader half; this file only WRITES, it never reads back semantics
// @see ../🧪️oracle/🔣️.json — mutationManifests / fixtureManifests this generator's output is registered under
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️pilot-playbook.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import * as THREE from "three";
import { GLTFExporter } from "three/examples/jsm/exporters/GLTFExporter.js";
//#endregion 🔌️Adapters

//#region 🩹️Shims
class ShimFileReader {
  result: unknown = null;
  onload: (() => void) | null = null;
  onloadend: (() => void) | null = null;
  onerror: ((e: unknown) => void) | null = null;
  private done(): void {
    this.onloadend?.();
    this.onload?.();
  }
  readAsArrayBuffer(blob: Blob): void {
    blob.arrayBuffer().then((buffer) => {
      this.result = buffer;
      this.done();
    }, (error) => this.onerror?.(error));
  }
  readAsDataURL(blob: Blob): void {
    blob.arrayBuffer().then((buffer) => {
      this.result = `data:${blob.type || "application/octet-stream"};base64,${Buffer.from(buffer).toString("base64")}`;
      this.done();
    }, (error) => this.onerror?.(error));
  }
}
(globalThis as { FileReader?: unknown }).FileReader ??= ShimFileReader as unknown as typeof FileReader;
//#endregion 🩹️Shims

//#region 📦️Document model (untyped — this generator edits the raw exported glTF JSON structurally)
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Doc = any;
//#endregion 📦️Document model

//#region 🏗️Base scene
/** 🌳 The ONE real glTF document every recipe derives from, built by three's own scene graph + GLTFExporter:
 *  two scenes, a skinned+morphed textured-free mesh, two materials, a camera, an animation clip. */
async function buildBaseDoc(): Promise<Doc> {
  const matA = new THREE.MeshStandardMaterial({ name: "matA", roughness: 0.4, metalness: 0.1, color: 0x3366cc });
  const matB = new THREE.MeshStandardMaterial({ name: "matB", roughness: 0.9, metalness: 0.0, color: 0xcc6633 });

  const geometry = new THREE.BoxGeometry(1, 1, 1);
  geometry.morphAttributes.position = [new THREE.Float32BufferAttribute(new Float32Array(geometry.attributes.position!.array.length).map((_, i) => (geometry.attributes.position!.array[i] as number) * 0.1), 3)];
  const mesh = new THREE.Mesh(geometry, matA);
  mesh.name = "meshNode";
  mesh.updateMorphTargets();
  mesh.morphTargetInfluences![0] = 0.5;

  const secondGeometry = new THREE.PlaneGeometry(1, 1);
  const secondMesh = new THREE.Mesh(secondGeometry, matB);
  secondMesh.name = "planeNode";

  const camera = new THREE.PerspectiveCamera(50, 1.5, 0.1, 100);
  camera.name = "cameraNode";

  const bone = new THREE.Bone();
  bone.name = "jointNode";
  bone.position.set(0, 1, 0);
  const skinnedGeometry = new THREE.CylinderGeometry(0.5, 0.5, 1, 8);
  const count = skinnedGeometry.attributes.position!.count;
  const skinIndex = new Float32Array(count * 4);
  const skinWeight = new Float32Array(count * 4);
  for (let i = 0; i < count; i += 1) skinWeight[i * 4] = 1;
  skinnedGeometry.setAttribute("skinIndex", new THREE.Uint16BufferAttribute(skinIndex, 4));
  skinnedGeometry.setAttribute("skinWeight", new THREE.Float32BufferAttribute(skinWeight, 4));
  const skinnedMesh = new THREE.SkinnedMesh(skinnedGeometry, matB);
  skinnedMesh.name = "skinnedNode";
  const skeleton = new THREE.Skeleton([bone]);
  skinnedMesh.add(bone);
  skinnedMesh.bind(skeleton);

  const root = new THREE.Group();
  root.name = "root";
  root.add(mesh, secondMesh, camera, skinnedMesh);
  const emptyNode = new THREE.Object3D();
  emptyNode.name = "emptyNode";
  root.add(emptyNode);

  const sceneA = new THREE.Scene();
  sceneA.name = "sceneA";
  sceneA.add(root);

  const sceneB = new THREE.Scene();
  sceneB.name = "sceneB";
  const soloNode = new THREE.Object3D();
  soloNode.name = "soloNode";
  sceneB.add(soloNode);

  const track = new THREE.VectorKeyframeTrack(`${mesh.uuid}.position`, [0, 1], [0, 0, 0, 0, 1, 0]);
  const clip = new THREE.AnimationClip("clip0", 1, [track]);

  const exporter = new GLTFExporter();
  exporter.register((writer: { json: Doc }) => ({
    afterParse(): void {
      const json = writer.json;
      json.asset.generator = "three.js GLTFExporter (semio oracle fixture base)";
      json.asset.copyright = "2026 Ueli Saluz — CC0 fixture data";
      json.asset.extras = { fixtureBase: "gltf-2-0-any-reader-oracle", revision: 1 };
      json.extras = { documentPurpose: "semio reader-oracle base document" };
      json.extensionsUsed = ["KHR_materials_unlit"];
      for (const scene of json.scenes ?? []) scene.extras = { sceneKind: "fixture" };
      for (const node of json.nodes ?? []) node.extras = { nodeKind: "fixture" };
      for (const m of json.meshes ?? []) m.extras = { meshKind: "fixture" };
    },
  }));

  const output = (await new Promise((resolve, reject) => {
    exporter.parse([sceneA, sceneB], resolve, reject, { binary: false, animations: [clip], includeCustomExtensions: true });
  })) as Doc;
  return output;
}
//#endregion 🏗️Base scene

//#region ✂️Generic structural edit
/** 🔧 Every `create/delete/move/reorder` mutation in this subset's manifest is one of these four
 *  ARRAY shapes — read directly off each mutation's own `🔣️payload.schema.json` `x-semio.touchedPaths`,
 *  never guessed. Applying them generically is what lets one engine cover the ~90 kinds that share this
 *  shape instead of writing bespoke code per kind. */
function arrayAt(doc: Doc, path: readonly (string | number)[]): unknown[] {
  let node: Doc = doc;
  for (const segment of path) node = node[segment];
  if (!Array.isArray(node)) throw new Error(`not an array at ${path.join("/")}`);
  return node;
}

function create(doc: Doc, path: readonly (string | number)[], position: number, value: unknown): void {
  arrayAt(doc, path).splice(position, 0, value);
}
function del(doc: Doc, path: readonly (string | number)[], index: number): void {
  arrayAt(doc, path).splice(index, 1);
}
function move(doc: Doc, path: readonly (string | number)[], index: number, position: number): void {
  const array = arrayAt(doc, path);
  const [item] = array.splice(index, 1);
  array.splice(position, 0, item);
}
function reorder(doc: Doc, path: readonly (string | number)[], order: readonly number[]): void {
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
function reorderKeys(doc: Doc, path: readonly (string | number)[], order: readonly string[]): void {
  const map = mapAt(doc, path);
  const entries = Object.fromEntries(order.map((key) => [key, map[key]]));
  for (const key of Object.keys(map)) delete map[key];
  Object.assign(map, entries);
}
function moveKey(doc: Doc, path: readonly (string | number)[], key: string, position: number): void {
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
type Remap = (oldIndex: number) => number | undefined;
function remapNodeRefs(doc: Doc, fn: Remap): void {
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
function remapMeshRefs(doc: Doc, fn: Remap): void {
  for (const node of doc.nodes ?? []) if (typeof node.mesh === "number") {
    const mapped = fn(node.mesh);
    if (mapped === undefined) delete node.mesh;
    else node.mesh = mapped;
  }
}
function remapMaterialRefs(doc: Doc, fn: Remap): void {
  for (const mesh of doc.meshes ?? []) for (const primitive of mesh.primitives ?? []) if (typeof primitive.material === "number") {
    const mapped = fn(primitive.material);
    if (mapped === undefined) delete primitive.material;
    else primitive.material = mapped;
  }
}
function remapCameraRefs(doc: Doc, fn: Remap): void {
  for (const node of doc.nodes ?? []) if (typeof node.camera === "number") {
    const mapped = fn(node.camera);
    if (mapped === undefined) delete node.camera;
    else node.camera = mapped;
  }
}
function remapSkinRefs(doc: Doc, fn: Remap): void {
  for (const node of doc.nodes ?? []) if (typeof node.skin === "number") {
    const mapped = fn(node.skin);
    if (mapped === undefined) delete node.skin;
    else node.skin = mapped;
  }
}
function remapSceneRefs(doc: Doc, fn: Remap): void {
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
function remapAccessorRefs(doc: Doc, fn: Remap): void {
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
function remapBufferViewRefs(doc: Doc, fn: Remap): void {
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
function remapBufferRefs(doc: Doc, fn: Remap): void {
  for (const view of doc.bufferViews ?? []) if (typeof view.buffer === "number") {
    const mapped = fn(view.buffer);
    if (mapped === undefined) delete view.buffer;
    else view.buffer = mapped;
  }
}
function remapImageRefs(doc: Doc, fn: Remap): void {
  for (const texture of doc.textures ?? []) if (typeof texture.source === "number") {
    const mapped = fn(texture.source);
    if (mapped === undefined) delete texture.source;
    else texture.source = mapped;
  }
}
function remapSamplerRefs(doc: Doc, fn: Remap): void {
  for (const texture of doc.textures ?? []) if (typeof texture.sampler === "number") {
    const mapped = fn(texture.sampler);
    if (mapped === undefined) delete texture.sampler;
    else texture.sampler = mapped;
  }
}
function remapTextureRefs(doc: Doc, fn: Remap): void {
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
function createReindexed(doc: Doc, path: readonly (string | number)[], position: number, value: unknown, remap: (fn: Remap) => void): void {
  create(doc, path, position, value);
  remap((i) => (i >= position ? i + 1 : i));
}
function deleteReindexed(doc: Doc, path: readonly (string | number)[], index: number, remap: (fn: Remap) => void): void {
  del(doc, path, index);
  remap((i) => (i === index ? undefined : i > index ? i - 1 : i));
}
function moveReindexed(doc: Doc, path: readonly (string | number)[], from: number, to: number, remap: (fn: Remap) => void): void {
  move(doc, path, from, to);
  remap((i) => {
    if (i === from) return to;
    if (from < to && i > from && i <= to) return i - 1;
    if (to < from && i >= to && i < from) return i + 1;
    return i;
  });
}
function reorderReindexed(doc: Doc, path: readonly (string | number)[], order: readonly number[], remap: (fn: Remap) => void): void {
  const inverse = new Array<number>(order.length);
  order.forEach((oldIndex, newPosition) => {
    inverse[oldIndex] = newPosition;
  });
  reorder(doc, path, order);
  remap((i) => inverse[i]);
}
//#endregion ✂️Generic structural edit

//#region 🧬️Recipes
type Recipe = { id: string; mutationId: string; outcome: "applied"; family: string; notes: string; build: (base: Doc) => { before: Doc; after: Doc } };

function clone<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

function bufferView(doc: Doc, byteLength: number): number {
  const buffer0 = 0;
  const view = { buffer: buffer0, byteOffset: 0, byteLength };
  doc.bufferViews ??= [];
  doc.bufferViews.push(view);
  return doc.bufferViews.length - 1;
}
function minimalAccessor(doc: Doc): number {
  const view = bufferView(doc, 12);
  doc.accessors ??= [];
  doc.accessors.push({ bufferView: view, componentType: 5126, count: 1, type: "SCALAR" });
  return doc.accessors.length - 1;
}

const RECIPES: Recipe[] = [];
function recipe(id: string, mutationId: string, family: string, notes: string, build: Recipe["build"]): void {
  RECIPES.push({ id, mutationId, outcome: "applied", family, notes, build });
}

//#region 👪️Node hierarchy
recipe("create-node", "create-node", "structural", "Inserts a new minimal node at index 1 of the flat nodes array, reindexing every node-reference site (children, scene roots, skin joints/skeleton, animation channel targets) so the document stays coherent — create-node{position}.", (base) => {
  const before = clone(base);
  const after = clone(base);
  createReindexed(after, ["nodes"], 1, { name: "createdNode" }, (fn) => remapNodeRefs(after, fn));
  return { before, after };
});
recipe("delete-node", "delete-node", "structural", "Removes the unreferenced solo node from scene B's node list, reindexing every node-reference site — delete-node{index}.", (base) => {
  const before = clone(base);
  const soloIndex = before.nodes.findIndex((n: Doc) => n.name === "soloNode");
  const after = clone(before);
  deleteReindexed(after, ["nodes"], soloIndex, (fn) => remapNodeRefs(after, fn));
  return { before, after };
});
recipe("move-node", "move-node", "structural", "Relocates the flat nodes[] array entry for emptyNode from its original index to the front, reindexing every node-reference site that shifted as a result — move-node{index,position}.", (base) => {
  const before = clone(base);
  const from = before.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  const after = clone(before);
  moveReindexed(after, ["nodes"], from, 0, (fn) => remapNodeRefs(after, fn));
  return { before, after };
});
recipe("reorder-nodes", "reorder-nodes", "structural", "A full permutation of the flat nodes[] array (reversed), with every node-reference site reindexed to match — reorder-nodes{order}. Node identity is index-based, so parser.json.nodes order plus the reindexed reference sites are the observable signature.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const order = before.nodes.map((_: Doc, i: number) => i).reverse();
  reorderReindexed(after, ["nodes"], order, (fn) => remapNodeRefs(after, fn));
  return { before, after };
});
recipe("bind-node-child", "bind-node-child", "structural", "Adopts emptyNode as a child of the plane's sibling meshNode at position 0 — bind-node-child{parent,child,position}.", (base) => {
  const before = clone(base);
  const root = before.nodes.find((n: Doc) => n.name === "root");
  const meshNodeIndex = before.nodes.findIndex((n: Doc) => n.name === "meshNode");
  const emptyIndex = before.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  root.children = (root.children as number[]).filter((c: number) => c !== emptyIndex);
  const after = clone(before);
  const afterMeshNode = after.nodes[meshNodeIndex];
  afterMeshNode.children ??= [];
  create(after, ["nodes", meshNodeIndex, "children"], 0, emptyIndex);
  return { before, after };
});
recipe("unbind-node-child", "unbind-node-child", "structural", "Removes emptyNode from root's children — unbind-node-child{parent,child}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const rootIndex = after.nodes.findIndex((n: Doc) => n.name === "root");
  const emptyIndex = after.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  after.nodes[rootIndex].children = (after.nodes[rootIndex].children as number[]).filter((c: number) => c !== emptyIndex);
  return { before, after };
});
recipe("move-node-child", "move-node-child", "structural", "Moves root's last child to the front of root.children — move-node-child{parent,child,position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const rootIndex = after.nodes.findIndex((n: Doc) => n.name === "root");
  const children = after.nodes[rootIndex].children as number[];
  const [last] = children.splice(children.length - 1, 1);
  children.splice(0, 0, last);
  return { before, after };
});
recipe("reorder-node-children", "reorder-node-children", "structural", "Reverses root.children — a complete permutation, reorder-node-children{parent,order}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const rootIndex = after.nodes.findIndex((n: Doc) => n.name === "root");
  after.nodes[rootIndex].children = [...(after.nodes[rootIndex].children as number[])].reverse();
  return { before, after };
});
recipe("move-node-parent", "move-node-parent", "structural", "Reparents emptyNode from root to sceneB's soloNode, clearing the old parent link and the old scene-root membership it never had — move-node-parent{parent,child,position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const rootIndex = after.nodes.findIndex((n: Doc) => n.name === "root");
  const emptyIndex = after.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  const soloIndex = after.nodes.findIndex((n: Doc) => n.name === "soloNode");
  after.nodes[rootIndex].children = (after.nodes[rootIndex].children as number[]).filter((c: number) => c !== emptyIndex);
  after.nodes[soloIndex].children = [emptyIndex];
  return { before, after };
});
recipe("change-node-name", "change-node-name", "metadata", "Renames emptyNode — change-node-name{node,value}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  after.nodes[i].name = "renamedEmptyNode";
  return { before, after };
});
recipe("change-node-extra-data", "change-node-extra-data", "metadata", "Sets emptyNode.extras.state — change-node-extra-data{node,data:{state:present,value}}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  after.nodes[i].extras = { nodeKind: "fixture", state: "mutated" };
  return { before, after };
});
recipe("change-node-extension-data", "change-node-extension-data", "metadata", "Sets emptyNode.extensions.KHR_materials_unlit — change-node-extension-data{node,data}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  after.nodes[i].extensions = { ACME_marker: { on: true } };
  return { before, after };
});
recipe("change-node-transform", "change-node-transform", "geometry", "Sets emptyNode's TRS translation to (2,3,4) — change-node-transform{node,transform:{kind:trs,...}}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  after.nodes[i].translation = [2, 3, 4];
  return { before, after };
});
recipe("change-node-morph-weights", "change-node-morph-weights", "geometry", "Overrides meshNode's morph weight from 0.5 to 0.9 at the node level — change-node-morph-weights{node,weights}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.nodes.findIndex((n: Doc) => n.name === "meshNode");
  after.nodes[i].weights = [0.9];
  return { before, after };
});
//#endregion 👪️Node hierarchy

//#region 🔘️Node <-> camera/mesh/skin bindings
recipe("bind-node-camera", "bind-node-camera", "structural", "Points emptyNode at the camera — bind-node-camera{node,camera}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  after.nodes[i].camera = 0;
  return { before, after };
});
recipe("unbind-node-camera", "unbind-node-camera", "structural", "Clears cameraNode's camera reference — unbind-node-camera{node}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.nodes.findIndex((n: Doc) => n.name === "cameraNode");
  delete after.nodes[i].camera;
  return { before, after };
});
recipe("bind-node-mesh", "bind-node-mesh", "structural", "Points emptyNode at mesh 0 — bind-node-mesh{node,mesh}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  after.nodes[i].mesh = 0;
  return { before, after };
});
recipe("unbind-node-mesh", "unbind-node-mesh", "structural", "Clears meshNode's mesh reference — unbind-node-mesh{node}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.nodes.findIndex((n: Doc) => n.name === "meshNode");
  delete after.nodes[i].mesh;
  return { before, after };
});
recipe("bind-node-skin", "bind-node-skin", "structural", "Points emptyNode at skin 0 (leaves its mesh unset — a skin reference is checkable independent of a bound mesh) — bind-node-skin{node,skin}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  after.nodes[i].skin = 0;
  return { before, after };
});
recipe("unbind-node-skin", "unbind-node-skin", "structural", "Clears skinnedNode's skin reference — unbind-node-skin{node}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.nodes.findIndex((n: Doc) => n.name === "skinnedNode");
  delete after.nodes[i].skin;
  return { before, after };
});
//#endregion 🔘️Node <-> camera/mesh/skin bindings

//#region 🎬️Scenes
recipe("create-scene", "create-scene", "structural", "Inserts a new empty scene at position 1, reindexing the document's default `scene` pointer — create-scene{position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  createReindexed(after, ["scenes"], 1, { nodes: [], extras: { sceneKind: "fixture" } }, (fn) => remapSceneRefs(after, fn));
  return { before, after };
});
recipe("delete-scene", "delete-scene", "structural", "Removes sceneB, reindexing the document's default `scene` pointer (which stays at 0, unaffected) — delete-scene{index}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.scenes.findIndex((s: Doc) => s.name === "sceneB");
  deleteReindexed(after, ["scenes"], i, (fn) => remapSceneRefs(after, fn));
  return { before, after };
});
recipe("move-scene", "move-scene", "structural", "Swaps sceneA and sceneB's positions, reindexing the default `scene` pointer with sceneA — move-scene{index,position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  moveReindexed(after, ["scenes"], 0, 1, (fn) => remapSceneRefs(after, fn));
  return { before, after };
});
recipe("reorder-scenes", "reorder-scenes", "structural", "Reverses the scenes[] array, reindexing the default `scene` pointer with it — reorder-scenes{order}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  reorderReindexed(after, ["scenes"], [1, 0], (fn) => remapSceneRefs(after, fn));
  return { before, after };
});
recipe("change-scene-name", "change-scene-name", "metadata", "Renames sceneB — change-scene-name{scene,value}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.scenes.findIndex((s: Doc) => s.name === "sceneB");
  after.scenes[i].name = "renamedSceneB";
  return { before, after };
});
recipe("change-scene-extra-data", "change-scene-extra-data", "metadata", "Sets sceneB.extras.state — change-scene-extra-data{scene,data}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.scenes.findIndex((s: Doc) => s.name === "sceneB");
  after.scenes[i].extras = { sceneKind: "fixture", state: "mutated" };
  return { before, after };
});
recipe("change-scene-extension-data", "change-scene-extension-data", "metadata", "Sets sceneB.extensions.ACME_marker — change-scene-extension-data{scene,data}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const i = after.scenes.findIndex((s: Doc) => s.name === "sceneB");
  after.scenes[i].extensions = { ACME_marker: { on: true } };
  return { before, after };
});
recipe("bind-default-scene", "bind-default-scene", "structural", "Moves the document's default scene pointer from sceneA (0) to sceneB (1) — bind-default-scene.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.scene = 1;
  return { before, after };
});
recipe("unbind-default-scene", "unbind-default-scene", "structural", "Clears the document's default scene pointer entirely — unbind-default-scene.", (base) => {
  const before = clone(base);
  const after = clone(before);
  delete after.scene;
  return { before, after };
});
recipe("bind-scene-root-node", "bind-scene-root-node", "structural", "Adds emptyNode as an ADDITIONAL root of sceneB (which starts with only soloNode) at position 0 — bind-scene-root-node{scene,node,position}. emptyNode is first detached from root so it is not a root twice over.", (base) => {
  const before = clone(base);
  const rootIndex = before.nodes.findIndex((n: Doc) => n.name === "root");
  const emptyIndex = before.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  before.nodes[rootIndex].children = (before.nodes[rootIndex].children as number[]).filter((c: number) => c !== emptyIndex);
  const after = clone(before);
  const sceneBIndex = after.scenes.findIndex((s: Doc) => s.name === "sceneB");
  create(after, ["scenes", sceneBIndex, "nodes"], 0, emptyIndex);
  return { before, after };
});
recipe("unbind-scene-root-node", "unbind-scene-root-node", "structural", "Removes soloNode from sceneB's root list, leaving sceneB empty — unbind-scene-root-node{scene,node}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const sceneBIndex = after.scenes.findIndex((s: Doc) => s.name === "sceneB");
  const soloIndex = after.nodes.findIndex((n: Doc) => n.name === "soloNode");
  after.scenes[sceneBIndex].nodes = (after.scenes[sceneBIndex].nodes as number[]).filter((n: number) => n !== soloIndex);
  return { before, after };
});
recipe("move-scene-root-node", "move-scene-root-node", "structural", "sceneA gets a second root (a detached copy-free extra: emptyNode) so a root-position move is observable, then moves it to the front — move-scene-root-node{scene,node,position}.", (base) => {
  const before = clone(base);
  const rootIndex = before.nodes.findIndex((n: Doc) => n.name === "root");
  const emptyIndex = before.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  before.nodes[rootIndex].children = (before.nodes[rootIndex].children as number[]).filter((c: number) => c !== emptyIndex);
  const sceneAIndex = before.scenes.findIndex((s: Doc) => s.name === "sceneA");
  before.scenes[sceneAIndex].nodes.push(emptyIndex);
  const after = clone(before);
  const roots = after.scenes[sceneAIndex].nodes as number[];
  const last = roots.pop()!;
  roots.unshift(last);
  return { before, after };
});
recipe("reorder-scene-root-nodes", "reorder-scene-root-nodes", "structural", "sceneA gets a second root (emptyNode), then the two-element root list is reversed — reorder-scene-root-nodes{scene,order}.", (base) => {
  const before = clone(base);
  const rootIndex = before.nodes.findIndex((n: Doc) => n.name === "root");
  const emptyIndex = before.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  before.nodes[rootIndex].children = (before.nodes[rootIndex].children as number[]).filter((c: number) => c !== emptyIndex);
  const sceneAIndex = before.scenes.findIndex((s: Doc) => s.name === "sceneA");
  before.scenes[sceneAIndex].nodes.push(emptyIndex);
  const after = clone(before);
  after.scenes[sceneAIndex].nodes = [...after.scenes[sceneAIndex].nodes].reverse();
  return { before, after };
});
//#endregion 🎬️Scenes

//#region 🕸️Mesh / 🔺️Primitive
recipe("create-mesh", "create-mesh", "structural", "Inserts a minimal empty-primitive-list mesh at position 1, reindexing every node.mesh reference so nothing shifts onto the new entry — create-mesh{position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  createReindexed(after, ["meshes"], 1, { name: "createdMesh", primitives: [] }, (fn) => remapMeshRefs(after, fn));
  return { before, after };
});
recipe("delete-mesh", "delete-mesh", "structural", "Removes the plane mesh, reindexing every node.mesh reference (planeNode loses its mesh; any mesh index after it shifts down) — delete-mesh{index}.", (base) => {
  const before = clone(base);
  const planeMeshIndex = before.nodes.find((n: Doc) => n.name === "planeNode").mesh;
  const after = clone(before);
  deleteReindexed(after, ["meshes"], planeMeshIndex, (fn) => remapMeshRefs(after, fn));
  return { before, after };
});
recipe("move-mesh", "move-mesh", "structural", "Swaps the box and plane meshes' array positions, reindexing every node.mesh reference to match — move-mesh{index,position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  moveReindexed(after, ["meshes"], 0, 1, (fn) => remapMeshRefs(after, fn));
  return { before, after };
});
recipe("reorder-meshs", "reorder-meshs", "structural", "Reverses the meshes[] array, reindexing every node.mesh reference to match — reorder-meshs{order}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const order = [...after.meshes.keys()].reverse();
  reorderReindexed(after, ["meshes"], order, (fn) => remapMeshRefs(after, fn));
  return { before, after };
});
recipe("change-mesh-name", "change-mesh-name", "metadata", "Renames mesh 0 — change-mesh-name{mesh,value}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.meshes[0].name = "renamedMesh0";
  return { before, after };
});
recipe("change-mesh-extra-data", "change-mesh-extra-data", "metadata", "Sets mesh 0's extras.state — change-mesh-extra-data{mesh,data}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.meshes[0].extras = { meshKind: "fixture", state: "mutated" };
  return { before, after };
});
recipe("change-mesh-extension-data", "change-mesh-extension-data", "metadata", "Sets mesh 0's extensions.ACME_marker — change-mesh-extension-data{mesh,data}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.meshes[0].extensions = { ACME_marker: { on: true } };
  return { before, after };
});
recipe("change-mesh-morph-weights", "change-mesh-morph-weights", "geometry", "Sets mesh 0's default weights to [0.75] (mesh-level, distinct from the node-level override) — change-mesh-morph-weights{mesh,weights}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.meshes[0].weights = [0.75];
  return { before, after };
});
recipe("create-primitive", "create-primitive", "structural", "Appends a minimal empty primitive to mesh 0 — create-primitive{mesh,position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  create(after, ["meshes", 0, "primitives"], after.meshes[0].primitives.length, { attributes: {} });
  return { before, after };
});
recipe("delete-primitive", "delete-primitive", "structural", "Removes mesh 0's only primitive — delete-primitive{mesh,primitive}. Mesh weights stay coherent (unchanged) exactly as the manifest's referencePolicy states.", (base) => {
  const before = clone(base);
  const after = clone(before);
  del(after, ["meshes", 0, "primitives"], 0);
  return { before, after };
});
recipe("move-primitive", "move-primitive", "structural", "Mesh 0 gets a second (empty) primitive appended, then the two are swapped — move-primitive{mesh,primitive,position}.", (base) => {
  const before = clone(base);
  before.meshes[0].primitives.push({ attributes: {} });
  const after = clone(before);
  move(after, ["meshes", 0, "primitives"], 0, 1);
  return { before, after };
});
recipe("reorder-primitives", "reorder-primitives", "structural", "Mesh 0 gets a second (empty) primitive appended, then the pair is reversed — reorder-primitives{mesh,order}.", (base) => {
  const before = clone(base);
  before.meshes[0].primitives.push({ attributes: {} });
  const after = clone(before);
  reorder(after, ["meshes", 0, "primitives"], [1, 0]);
  return { before, after };
});
recipe("change-primitive-topology-mode", "change-primitive-topology-mode", "geometry", "Changes mesh 0's primitive mode from 4 (TRIANGLES, three's default) to 0 (POINTS) — change-primitive-topology-mode{mesh,primitive,mode}. three converts drawing mode at parse time, so geometry shape itself is the witnessed evidence.", (base) => {
  const before = clone(base);
  before.meshes[0].primitives[0].mode = 4;
  const after = clone(before);
  after.meshes[0].primitives[0].mode = 0;
  return { before, after };
});
recipe("change-primitive-extra-data", "change-primitive-extra-data", "metadata", "Sets mesh 0 primitive 0's extras.state — change-primitive-extra-data{mesh,primitive,data}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.meshes[0].primitives[0].extras = { state: "mutated" };
  return { before, after };
});
recipe("change-primitive-extension-data", "change-primitive-extension-data", "metadata", "Sets mesh 0 primitive 0's extensions.ACME_marker — change-primitive-extension-data{mesh,primitive,data}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.meshes[0].primitives[0].extensions = { ACME_marker: { on: true } };
  return { before, after };
});
recipe("bind-primitive-material", "bind-primitive-material", "structural", "Rebinds mesh 0 primitive 0 from material 0 to material 1 — bind-primitive-material{mesh,primitive,material}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.meshes[0].primitives[0].material = 1;
  return { before, after };
});
recipe("unbind-primitive-material", "unbind-primitive-material", "structural", "Clears mesh 0 primitive 0's material reference — unbind-primitive-material{mesh,primitive}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  delete after.meshes[0].primitives[0].material;
  return { before, after };
});
recipe("bind-primitive-indices", "bind-primitive-indices", "structural", "Mesh 0 primitive 0 starts non-indexed (indices removed), then a fresh scalar accessor is created and bound — bind-primitive-indices{mesh,primitive,accessor}. three exposes this as geometry.index becoming non-null.", (base) => {
  const before = clone(base);
  delete before.meshes[0].primitives[0].indices;
  const after = clone(before);
  const accessorIndex = minimalAccessor(after);
  after.accessors[accessorIndex] = { bufferView: after.accessors[accessorIndex].bufferView, componentType: 5123, count: 3, type: "SCALAR" };
  after.meshes[0].primitives[0].indices = accessorIndex;
  return { before, after };
});
recipe("unbind-primitive-indices", "unbind-primitive-indices", "structural", "Clears mesh 0 primitive 0's indices reference — unbind-primitive-indices{mesh,primitive}. three exposes this as geometry.index becoming null (a non-indexed draw).", (base) => {
  const before = clone(base);
  const after = clone(before);
  delete after.meshes[0].primitives[0].indices;
  return { before, after };
});
recipe("bind-primitive-attribute", "bind-primitive-attribute", "structural", "Adds a COLOR_0 attribute to mesh 0 primitive 0, bound to a fresh accessor — bind-primitive-attribute{mesh,primitive,semantic,accessor}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const view = bufferView(after, 12);
  after.accessors.push({ bufferView: view, componentType: 5126, count: 1, type: "VEC3" });
  after.meshes[0].primitives[0].attributes.COLOR_0 = after.accessors.length - 1;
  return { before, after };
});
recipe("unbind-primitive-attribute", "unbind-primitive-attribute", "structural", "Removes mesh 0 primitive 0's NORMAL attribute — unbind-primitive-attribute{mesh,primitive,semantic}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  delete after.meshes[0].primitives[0].attributes.NORMAL;
  return { before, after };
});
recipe("move-primitive-attribute", "move-primitive-attribute", "structural", "Moves the POSITION key to the end of mesh 0 primitive 0's attribute map — move-primitive-attribute{mesh,primitive,semantic,position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const attrs = after.meshes[0].primitives[0].attributes as Record<string, number>;
  moveKey(after, ["meshes", 0, "primitives", 0, "attributes"], "POSITION", Object.keys(attrs).length - 1);
  return { before, after };
});
recipe("reorder-primitive-attributes", "reorder-primitive-attributes", "structural", "Reverses mesh 0 primitive 0's attribute key order — reorder-primitive-attributes{mesh,primitive,order}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const attrs = after.meshes[0].primitives[0].attributes as Record<string, number>;
  reorderKeys(after, ["meshes", 0, "primitives", 0, "attributes"], [...Object.keys(attrs)].reverse());
  return { before, after };
});
//#endregion 🕸️Mesh / 🔺️Primitive

//#region 🧬️Morph targets
recipe("create-morph-target", "create-morph-target", "structural", "Appends a second (empty) morph target to mesh 0 primitive 0 — create-morph-target{mesh,primitive,position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  create(after, ["meshes", 0, "primitives", 0, "targets"], after.meshes[0].primitives[0].targets.length, {});
  return { before, after };
});
recipe("delete-morph-target", "delete-morph-target", "structural", "Removes mesh 0 primitive 0's only morph target and its node/mesh-level weight entries so target-count coherence survives — delete-morph-target{mesh,primitive,target}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  del(after, ["meshes", 0, "primitives", 0, "targets"], 0);
  for (const node of after.nodes) if (Array.isArray(node.weights)) node.weights = [];
  return { before, after };
});
recipe("move-morph-target", "move-morph-target", "structural", "Mesh 0 primitive 0 gets a second, DISTINCT morph target appended (its own fresh accessor, so the two targets are not content-identical), then the two are swapped along with their parallel node.weights entries — move-morph-target{mesh,primitive,target,position}.", (base) => {
  const before = clone(base);
  const view = bufferView(before, 12);
  before.accessors.push({ bufferView: view, componentType: 5126, count: 1, type: "VEC3" });
  before.meshes[0].primitives[0].targets.push({ POSITION: before.accessors.length - 1 });
  before.nodes.find((n: Doc) => n.name === "meshNode").weights = [0.5, 0.1];
  const after = clone(before);
  move(after, ["meshes", 0, "primitives", 0, "targets"], 0, 1);
  const meshNode = after.nodes.find((n: Doc) => n.name === "meshNode");
  meshNode.weights = [meshNode.weights[1], meshNode.weights[0]];
  return { before, after };
});
recipe("reorder-morph-targets", "reorder-morph-targets", "structural", "Mesh 0 primitive 0 gets a second, DISTINCT morph target appended (its own fresh accessor), then the pair is reversed along with the parallel node.weights entries — reorder-morph-targets{mesh,primitive,order}.", (base) => {
  const before = clone(base);
  const view = bufferView(before, 12);
  before.accessors.push({ bufferView: view, componentType: 5126, count: 1, type: "VEC3" });
  before.meshes[0].primitives[0].targets.push({ POSITION: before.accessors.length - 1 });
  before.nodes.find((n: Doc) => n.name === "meshNode").weights = [0.5, 0.1];
  const after = clone(before);
  reorder(after, ["meshes", 0, "primitives", 0, "targets"], [1, 0]);
  const meshNode = after.nodes.find((n: Doc) => n.name === "meshNode");
  meshNode.weights = [...meshNode.weights].reverse();
  return { before, after };
});
recipe("bind-morph-target-attribute", "bind-morph-target-attribute", "structural", "Adds a NORMAL attribute to mesh 0 primitive 0's morph target 0 — bind-morph-target-attribute{mesh,primitive,target,semantic,accessor}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const view = bufferView(after, 12);
  after.accessors.push({ bufferView: view, componentType: 5126, count: 1, type: "VEC3" });
  after.meshes[0].primitives[0].targets[0].NORMAL = after.accessors.length - 1;
  return { before, after };
});
recipe("unbind-morph-target-attribute", "unbind-morph-target-attribute", "structural", "Removes mesh 0 primitive 0's morph target 0 POSITION attribute — unbind-morph-target-attribute{mesh,primitive,target,semantic}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  delete after.meshes[0].primitives[0].targets[0].POSITION;
  return { before, after };
});
recipe("move-morph-target-attribute", "move-morph-target-attribute", "structural", "Target 0 gets a second attribute (NORMAL) appended, then POSITION is moved after it — move-morph-target-attribute{mesh,primitive,target,semantic,position}.", (base) => {
  const before = clone(base);
  const view = bufferView(before, 12);
  before.accessors.push({ bufferView: view, componentType: 5126, count: 1, type: "VEC3" });
  before.meshes[0].primitives[0].targets[0].NORMAL = before.accessors.length - 1;
  const after = clone(before);
  moveKey(after, ["meshes", 0, "primitives", 0, "targets", 0], "POSITION", 1);
  return { before, after };
});
recipe("reorder-morph-target-attributes", "reorder-morph-target-attributes", "structural", "Target 0 gets a second attribute (NORMAL) appended, then the two-key map is reversed — reorder-morph-target-attributes{mesh,primitive,target,order}.", (base) => {
  const before = clone(base);
  const view = bufferView(before, 12);
  before.accessors.push({ bufferView: view, componentType: 5126, count: 1, type: "VEC3" });
  before.meshes[0].primitives[0].targets[0].NORMAL = before.accessors.length - 1;
  const after = clone(before);
  reorderKeys(after, ["meshes", 0, "primitives", 0, "targets", 0], ["NORMAL", "POSITION"]);
  return { before, after };
});
//#endregion 🧬️Morph targets

//#region 💎️Material
recipe("create-material", "create-material", "structural", "Inserts a minimal default material at position 1 — create-material{position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  create(after, ["materials"], 1, { name: "createdMaterial" });
  return { before, after };
});
recipe("delete-material", "delete-material", "structural", "Removes material 1 (matB), reindexing every primitive.material reference — delete-material{index}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  deleteReindexed(after, ["materials"], 1, (fn) => remapMaterialRefs(after, fn));
  return { before, after };
});
recipe("move-material", "move-material", "structural", "Swaps the two materials' positions, reindexing every primitive.material reference — move-material{index,position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  moveReindexed(after, ["materials"], 0, 1, (fn) => remapMaterialRefs(after, fn));
  return { before, after };
});
recipe("reorder-materials", "reorder-materials", "structural", "Reverses materials[], reindexing every primitive.material reference — reorder-materials{order}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const order = [...after.materials.keys()].reverse();
  reorderReindexed(after, ["materials"], order, (fn) => remapMaterialRefs(after, fn));
  return { before, after };
});
recipe("change-material-alpha-mode", "change-material-alpha-mode", "material", "matB's alphaMode: OPAQUE -> BLEND — change-material-alpha-mode{material,alphaMode}.", (base) => {
  const before = clone(base);
  before.materials[1].alphaMode = "OPAQUE";
  const after = clone(before);
  after.materials[1].alphaMode = "BLEND";
  return { before, after };
});
recipe("change-material-double-sided", "change-material-double-sided", "material", "matB's doubleSided: false -> true — change-material-double-sided{material,doubleSided}.", (base) => {
  const before = clone(base);
  before.materials[1].doubleSided = false;
  const after = clone(before);
  after.materials[1].doubleSided = true;
  return { before, after };
});
//#endregion 💎️Material

//#region 🎥️Camera
recipe("create-camera", "create-camera", "structural", "Inserts a minimal perspective camera at position 1, reindexing cameraNode's reference — create-camera{position,projection}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  createReindexed(after, ["cameras"], 1, { type: "perspective", perspective: { yfov: 1, znear: 0.1 } }, (fn) => remapCameraRefs(after, fn));
  return { before, after };
});
recipe("delete-camera", "delete-camera", "structural", "Removes camera 0, reindexing cameraNode's reference to it — delete-camera{index}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  deleteReindexed(after, ["cameras"], 0, (fn) => remapCameraRefs(after, fn));
  return { before, after };
});
recipe("move-camera", "move-camera", "structural", "A second camera is appended (referenced by cameraNode), then the two are swapped, reindexing the reference — move-camera{index,position}.", (base) => {
  const before = clone(base);
  before.cameras.push({ type: "perspective", perspective: { yfov: 0.5, znear: 0.05 } });
  before.nodes.find((n: Doc) => n.name === "cameraNode").camera = 1;
  const after = clone(before);
  moveReindexed(after, ["cameras"], 0, 1, (fn) => remapCameraRefs(after, fn));
  return { before, after };
});
recipe("reorder-cameras", "reorder-cameras", "structural", "A second camera is appended (referenced by cameraNode), then the pair is reversed, reindexing the reference — reorder-cameras{order}.", (base) => {
  const before = clone(base);
  before.cameras.push({ type: "perspective", perspective: { yfov: 0.5, znear: 0.05 } });
  before.nodes.find((n: Doc) => n.name === "cameraNode").camera = 1;
  const after = clone(before);
  reorderReindexed(after, ["cameras"], [1, 0], (fn) => remapCameraRefs(after, fn));
  return { before, after };
});
//#endregion 🎥️Camera

//#region 🧥️Skin
recipe("create-skin", "create-skin", "structural", "Inserts a minimal skin (one joint, the existing jointNode) at position 1, reindexing skinnedNode's reference — create-skin{position}.", (base) => {
  const before = clone(base);
  const jointIndex = before.nodes.findIndex((n: Doc) => n.name === "jointNode");
  const after = clone(before);
  createReindexed(after, ["skins"], 1, { joints: [jointIndex] }, (fn) => remapSkinRefs(after, fn));
  return { before, after };
});
recipe("delete-skin", "delete-skin", "structural", "Removes skin 0, reindexing skinnedNode's reference to it — delete-skin{index}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  deleteReindexed(after, ["skins"], 0, (fn) => remapSkinRefs(after, fn));
  return { before, after };
});
recipe("move-skin", "move-skin", "structural", "A second skin (same joint, referenced by skinnedNode) is appended, then the two are swapped, reindexing the reference — move-skin{index,position}.", (base) => {
  const before = clone(base);
  before.skins.push({ joints: [...before.skins[0].joints] });
  before.nodes.find((n: Doc) => n.name === "skinnedNode").skin = 1;
  const after = clone(before);
  moveReindexed(after, ["skins"], 0, 1, (fn) => remapSkinRefs(after, fn));
  return { before, after };
});
recipe("reorder-skins", "reorder-skins", "structural", "A second skin (same joint, referenced by skinnedNode) is appended, then the pair is reversed, reindexing the reference — reorder-skins{order}.", (base) => {
  const before = clone(base);
  before.skins.push({ joints: [...before.skins[0].joints] });
  before.nodes.find((n: Doc) => n.name === "skinnedNode").skin = 1;
  const after = clone(before);
  reorderReindexed(after, ["skins"], [1, 0], (fn) => remapSkinRefs(after, fn));
  return { before, after };
});
//#endregion 🧥️Skin

//#region 🎞️Animation
recipe("create-animation", "create-animation", "structural", "Inserts a second, minimal-but-valid animation (one channel targeting emptyNode's translation) at position 1 — create-animation{position}. Animations are eagerly resolved by three regardless of scene reachability, so this one is given real valid samplers rather than left empty.", (base) => {
  const before = clone(base);
  const after = clone(before);
  const timeView = bufferView(after, 8);
  after.accessors.push({ bufferView: timeView, componentType: 5126, count: 2, type: "SCALAR" });
  const timeAccessor = after.accessors.length - 1;
  const valueView = bufferView(after, 24);
  after.accessors.push({ bufferView: valueView, componentType: 5126, count: 2, type: "VEC3" });
  const valueAccessor = after.accessors.length - 1;
  const emptyIndex = after.nodes.findIndex((n: Doc) => n.name === "emptyNode");
  create(after, ["animations"], 1, { name: "clip1", channels: [{ sampler: 0, target: { node: emptyIndex, path: "translation" } }], samplers: [{ input: timeAccessor, output: valueAccessor }] });
  return { before, after };
});
recipe("delete-animation", "delete-animation", "structural", "Removes animation 0 — delete-animation{index}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  del(after, ["animations"], 0);
  return { before, after };
});
recipe("move-animation", "move-animation", "structural", "A second animation (identical shape) is appended, then the two are swapped — move-animation{index,position}.", (base) => {
  const before = clone(base);
  before.animations.push(clone(before.animations[0]));
  before.animations[1].name = "clip1";
  const after = clone(before);
  move(after, ["animations"], 0, 1);
  return { before, after };
});
recipe("reorder-animations", "reorder-animations", "structural", "A second animation (identical shape) is appended, then the pair is reversed — reorder-animations{order}.", (base) => {
  const before = clone(base);
  before.animations.push(clone(before.animations[0]));
  before.animations[1].name = "clip1";
  const after = clone(before);
  reorder(after, ["animations"], [1, 0]);
  return { before, after };
});
//#endregion 🎞️Animation

//#region 🧩️Extensions (used, required)
recipe("add-used-extension", "add-used-extension", "structural", "Declares ACME_marker as an additional used extension — add-used-extension.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.extensionsUsed = [...(after.extensionsUsed ?? []), "ACME_marker"];
  return { before, after };
});
recipe("remove-used-extension", "remove-used-extension", "structural", "Withdraws KHR_materials_unlit from extensionsUsed (nothing depends on it as required) — remove-used-extension.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.extensionsUsed = (after.extensionsUsed as string[]).filter((e: string) => e !== "KHR_materials_unlit");
  return { before, after };
});
recipe("move-used-extension", "move-used-extension", "structural", "extensionsUsed gets a second entry appended, then the two are swapped — move-used-extension.", (base) => {
  const before = clone(base);
  before.extensionsUsed = [...(before.extensionsUsed ?? []), "ACME_marker"];
  const after = clone(before);
  after.extensionsUsed = [after.extensionsUsed[1], after.extensionsUsed[0]];
  return { before, after };
});
recipe("reorder-used-extensions", "reorder-used-extensions", "structural", "extensionsUsed gets a second entry appended, then the pair is reversed — reorder-used-extensions.", (base) => {
  const before = clone(base);
  before.extensionsUsed = [...(before.extensionsUsed ?? []), "ACME_marker"];
  const after = clone(before);
  after.extensionsUsed = [...after.extensionsUsed].reverse();
  return { before, after };
});
recipe("add-required-extension", "add-required-extension", "structural", "Promotes the already-used KHR_materials_unlit into extensionsRequired too, matching the manifest's own referencePolicy ('requires prior extensionsUsed membership') — add-required-extension.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.extensionsRequired = [...(after.extensionsRequired ?? []), "KHR_materials_unlit"];
  return { before, after };
});
recipe("remove-required-extension", "remove-required-extension", "structural", "Withdraws a required-extension declaration — remove-required-extension. Starts from a document where KHR_materials_unlit is required (both used and required).", (base) => {
  const before = clone(base);
  before.extensionsRequired = ["KHR_materials_unlit"];
  const after = clone(before);
  after.extensionsRequired = [];
  return { before, after };
});
recipe("move-required-extension", "move-required-extension", "structural", "extensionsRequired gets a second entry, then the two are swapped — move-required-extension.", (base) => {
  const before = clone(base);
  before.extensionsUsed = [...(before.extensionsUsed ?? []), "ACME_marker"];
  before.extensionsRequired = ["KHR_materials_unlit", "ACME_marker"];
  const after = clone(before);
  after.extensionsRequired = [after.extensionsRequired[1], after.extensionsRequired[0]];
  return { before, after };
});
recipe("reorder-required-extensions", "reorder-required-extensions", "structural", "extensionsRequired gets a second entry, then the pair is reversed — reorder-required-extensions.", (base) => {
  const before = clone(base);
  before.extensionsUsed = [...(before.extensionsUsed ?? []), "ACME_marker"];
  before.extensionsRequired = ["KHR_materials_unlit", "ACME_marker"];
  const after = clone(before);
  after.extensionsRequired = [...after.extensionsRequired].reverse();
  return { before, after };
});
//#endregion 🧩️Extensions

//#region 📦️Asset / 📄️Document
recipe("change-asset-version", "change-asset-version", "metadata", "asset.version itself changes — change-asset-version{touches document/asset/version}. GLTFLoader accepts any version string here (empirically verified: 2.0, 2.0.1, 2.1 all parse), so the reader's evidence is the exact string parser.json.asset.version reports, not a derived field.", (base) => {
  const before = clone(base);
  before.asset.version = "2.0";
  const after = clone(before);
  after.asset.version = "2.0.1";
  return { before, after };
});
recipe("change-asset-descriptive-metadata", "change-asset-descriptive-metadata", "metadata", "asset.generator/copyright/minVersion change — change-asset-descriptive-metadata.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.asset.generator = "three.js GLTFExporter (mutated by semio fixture)";
  after.asset.copyright = "2026 Ueli Saluz — mutated";
  after.asset.minVersion = "2.0";
  return { before, after };
});
recipe("change-asset-extra-data", "change-asset-extra-data", "metadata", "asset.extras.revision 1 -> 2 — change-asset-extra-data.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.asset.extras = { fixtureBase: "gltf-2-0-any-reader-oracle", revision: 2 };
  return { before, after };
});
recipe("change-asset-extension-data", "change-asset-extension-data", "metadata", "Sets asset.extensions.ACME_marker — change-asset-extension-data.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.asset.extensions = { ACME_marker: { on: true } };
  return { before, after };
});
recipe("change-document-extra-data", "change-document-extra-data", "metadata", "Root document.extras.documentPurpose changes — change-document-extra-data.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.extras = { documentPurpose: "mutated by semio fixture" };
  return { before, after };
});
recipe("change-document-extension-data", "change-document-extension-data", "metadata", "Sets root document.extensions.ACME_marker — change-document-extension-data.", (base) => {
  const before = clone(base);
  const after = clone(before);
  after.extensions = { ACME_marker: { on: true } };
  return { before, after };
});
//#endregion 📦️Asset / 📄️Document

//#region 🧱️Resources (accessor, buffer, bufferView, image, sampler, texture)
/** 🖼️ Gives a document the texture-family arrays the shared base deliberately does not carry.
 *
 *  The base has 18 accessors, 1 buffer and 18 bufferViews but ZERO images, samplers and textures, and it
 *  must stay that way: it is the `before` of all 96 committed fixtures and changing it would invalidate
 *  every one of their recorded hashes. A recipe's `before` is written per fixture and need not equal the
 *  shared base, so the texture-family recipes grow their own — two of each, which is the minimum that
 *  lets delete, move and reorder all be observable. */
function withTextureFamily(doc: Doc): Doc {
  const png = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
  doc.images = [
    { name: "resourceImageA", uri: png, mimeType: "image/png" },
    { name: "resourceImageB", uri: png, mimeType: "image/png" },
  ];
  doc.samplers = [
    { name: "resourceSamplerA", magFilter: 9729, minFilter: 9987, wrapS: 10497, wrapT: 10497 },
    { name: "resourceSamplerB", magFilter: 9728, minFilter: 9728, wrapS: 33071, wrapT: 33071 },
  ];
  doc.textures = [
    { name: "resourceTextureA", source: 0, sampler: 0 },
    { name: "resourceTextureB", source: 1, sampler: 1 },
  ];
  return doc;
}

/** 🧱️ One `create`/`delete`/`move`/`reorder` quartet per resource array. Each names the reference family
 *  it must keep coherent — an unreindexed resource array silently detaches every index pointing into it. */
function resourceQuartet(
  family: string,
  singular: string,
  pluralKind: string,
  arrayKey: string,
  remap: (doc: Doc, fn: Remap) => void,
  seed: (doc: Doc) => Doc,
  created: unknown,
  deletableIndex: (doc: Doc) => number,
): void {
  const plural = pluralKind;
  recipe(`create-${singular}`, `create-${singular}`, family, `Inserts a new ${singular} at index 1 of ${plural}[], reindexing every site that references a ${singular} by index — create-${singular}{position}.`, (base) => {
    const before = seed(clone(base));
    const after = clone(before);
    createReindexed(after, [arrayKey], 1, created, (fn) => remap(after, fn));
    return { before, after };
  });
  recipe(`delete-${singular}`, `delete-${singular}`, family, `Removes one ${singular} from ${plural}[], reindexing every site that references a ${singular} by index — delete-${singular}{index}.`, (base) => {
    const before = seed(clone(base));
    const after = clone(before);
    deleteReindexed(after, [arrayKey], deletableIndex(before), (fn) => remap(after, fn));
    return { before, after };
  });
  recipe(`move-${singular}`, `move-${singular}`, family, `Relocates a ${plural}[] entry to the front, reindexing every reference site that shifted — move-${singular}{index,position}.`, (base) => {
    const before = seed(clone(base));
    const after = clone(before);
    moveReindexed(after, [arrayKey], deletableIndex(before), 0, (fn) => remap(after, fn));
    return { before, after };
  });
  recipe(`reorder-${pluralKind}`, `reorder-${pluralKind}`, family, `Reverses ${arrayKey}[], reindexing every reference site — reorder-${pluralKind}{order}.`, (base) => {
    const before = seed(clone(base));
    const after = clone(before);
    const order = ((after[arrayKey] as unknown[]) ?? []).map((_, index, all) => all.length - 1 - index);
    reorderReindexed(after, [arrayKey], order, (fn) => remap(after, fn));
    return { before, after };
  });
}

const PNG_1X1 = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

/** 🎯️ The index of the spare this seed appended — always the last, and always referenced by NOTHING.
 *  `delete` and `move` must target it: removing a resource something still points at does not exercise
 *  deletion, it produces an invalid document, and a third-party reader will (correctly) refuse the whole
 *  fixture rather than judge the mutation. Both failures were caught that way before this was fixed. */
const lastIndex = (key: string) => (doc: Doc): number => (doc[key] as unknown[]).length - 1;

resourceQuartet("resource", "accessor", "accessors", "accessors", remapAccessorRefs, (doc) => {
  (doc.accessors as Doc[]).push({ name: "spareAccessor", bufferView: 0, componentType: 5126, count: 1, type: "SCALAR" });
  return doc;
}, { name: "createdAccessor", bufferView: 0, componentType: 5126, count: 1, type: "SCALAR" }, lastIndex("accessors"));
resourceQuartet("resource", "buffer", "buffers", "buffers", remapBufferRefs, (doc) => {
  (doc.buffers as Doc[]).push({ name: "spareBuffer", byteLength: 4, uri: "data:application/octet-stream;base64,AAAAAA==" });
  return doc;
}, { name: "createdBuffer", byteLength: 4, uri: "data:application/octet-stream;base64,AAAAAA==" }, lastIndex("buffers"));
resourceQuartet("resource", "buffer-view", "buffer-views", "bufferViews", remapBufferViewRefs, (doc) => {
  (doc.bufferViews as Doc[]).push({ name: "spareBufferView", buffer: 0, byteOffset: 0, byteLength: 4 });
  return doc;
}, { name: "createdBufferView", buffer: 0, byteOffset: 0, byteLength: 4 }, lastIndex("bufferViews"));
resourceQuartet("resource", "image", "images", "images", remapImageRefs, (doc) => {
  withTextureFamily(doc);
  (doc.images as Doc[]).push({ name: "spareImage", uri: PNG_1X1, mimeType: "image/png" });
  return doc;
}, { name: "createdImage", uri: PNG_1X1, mimeType: "image/png" }, lastIndex("images"));
resourceQuartet("resource", "sampler", "samplers", "samplers", remapSamplerRefs, (doc) => {
  withTextureFamily(doc);
  (doc.samplers as Doc[]).push({ name: "spareSampler", magFilter: 9729, minFilter: 9729, wrapS: 10497, wrapT: 10497 });
  return doc;
}, { name: "createdSampler", magFilter: 9729, minFilter: 9729, wrapS: 10497, wrapT: 10497 }, lastIndex("samplers"));
resourceQuartet("resource", "texture", "textures", "textures", remapTextureRefs, (doc) => {
  withTextureFamily(doc);
  (doc.textures as Doc[]).push({ name: "spareTexture", source: 0, sampler: 0 });
  return doc;
}, { name: "createdTexture", source: 0, sampler: 0 }, lastIndex("textures"));
//#endregion 🧱️Resources

//#endregion 🧬️Recipes

//#region 🚀️Entry
function fileNameFor(id: string): string {
  return `${id}`;
}

async function main(argv: readonly string[]): Promise<number> {
  const [command = "generate", ...rest] = argv;
  if (command === "list") {
    if (rest.includes("--json")) {
      console.log(JSON.stringify(RECIPES.map((r) => ({ id: `${r.mutationId}-applied`, mutationId: r.mutationId, outcome: r.outcome, family: r.family, notes: r.notes })), null, 2));
      return 0;
    }
    for (const r of RECIPES) console.log(r.id);
    return 0;
  }
  if (command !== "generate") {
    console.error(`[generator] unknown command ${JSON.stringify(command)} — expected generate | list`);
    return 1;
  }
  const onlyIndex = rest.indexOf("--only");
  const only = onlyIndex >= 0 ? rest[onlyIndex + 1] : undefined;
  const outRoot = process.env.SEMIO_FIXTURE_OUT ?? join(import.meta.dir, "..", "🧫️fixtures");
  const base = await buildBaseDoc();
  const selected = only ? RECIPES.filter((r) => `${r.mutationId}-applied` === only) : RECIPES;
  if (selected.length === 0) {
    console.error(`[generator] no recipe matches --only ${JSON.stringify(only)} — known: ${RECIPES.map((r) => `${r.mutationId}-applied`).join(", ")}`);
    return 1;
  }
  let count = 0;
  for (const r of selected) {
    const id = `${r.mutationId}-applied`;
    const { before, after } = r.build(base);
    const dir = join(outRoot, fileNameFor(id));
    mkdirSync(dir, { recursive: true });
    writeFileSync(join(dir, "before.gltf"), `${JSON.stringify(before, null, 2)}\n`);
    writeFileSync(join(dir, "after.gltf"), `${JSON.stringify(after, null, 2)}\n`);
    count += 1;
  }
  console.log(`[generator] wrote ${count} fixture bundle(s) into ${outRoot}`);
  return 0;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚀️Entry

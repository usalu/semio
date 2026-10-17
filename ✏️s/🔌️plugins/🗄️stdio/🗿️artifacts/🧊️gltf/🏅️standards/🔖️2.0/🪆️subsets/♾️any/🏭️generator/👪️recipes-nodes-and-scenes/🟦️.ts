/** 👪️ glTF fixture generator: the recipes of the node hierarchy, the node bindings and the scenes. */
import type { Doc } from "../🏗️base-document/🟦️.ts";
import { create, createReindexed, deleteReindexed, moveReindexed, remapNodeRefs, remapSceneRefs, reorderReindexed } from "../✂️structural-edit/🟦️.ts";
import { clone, recipe } from "../🧬️recipe-registry/🟦️.ts";

//#region 🧬️Recipes
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
//#endregion 🧬️Recipes

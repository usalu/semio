/** 🕸️ glTF fixture generator: the recipes of meshes, primitives, morph targets, materials and cameras. */
import type { Doc } from "../🏗️base-document/🟦️.ts";
import { create, createReindexed, del, deleteReindexed, move, moveKey, moveReindexed, remapCameraRefs, remapMaterialRefs, remapMeshRefs, reorder, reorderKeys, reorderReindexed } from "../✂️structural-edit/🟦️.ts";
import { bufferView, clone, minimalAccessor, recipe } from "../🧬️recipe-registry/🟦️.ts";

//#region 🧬️Recipes
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
recipe("delete-morph-target", "delete-morph-target", "structural", "Mesh 0 starts without mesh-level default weights (a target may only be deleted while no weight list pins the target count), then its only morph target is removed and every node-level weight entry cleared so target-count coherence survives — delete-morph-target{mesh,primitive,target}.", (base) => {
  const before = clone(base);
  delete before.meshes[0].weights;
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
recipe("create-material", "create-material", "structural", "Inserts a default material at position 1, reindexing every primitive.material reference at or past it — create-material{position}.", (base) => {
  const before = clone(base);
  const after = clone(before);
  createReindexed(after, ["materials"], 1, {}, (fn) => remapMaterialRefs(after, fn));
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
//#endregion 🧬️Recipes

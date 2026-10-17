/** 🧥️ glTF fixture generator: the recipes of skins, animations, extensions, the asset block and the accessor, buffer, image, sampler and texture resources. */
import type { Doc } from "../🏗️base-document/🟦️.ts";
import { create, createReindexed, del, deleteReindexed, move, moveReindexed, type Remap, remapAccessorRefs, remapBufferRefs, remapBufferViewRefs, remapImageRefs, remapSamplerRefs, remapSkinRefs, remapTextureRefs, reorder, reorderReindexed } from "../✂️structural-edit/🟦️.ts";
import { bufferView, clone, recipe } from "../🧬️recipe-registry/🟦️.ts";

//#region 🧬️Recipes
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

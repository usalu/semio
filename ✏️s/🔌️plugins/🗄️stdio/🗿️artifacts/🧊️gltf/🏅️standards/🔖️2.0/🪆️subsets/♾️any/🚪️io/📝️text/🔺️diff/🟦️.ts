/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfTouchedRegion,GltfDiffDerivation,GltfJsonPresence,GltfModified,GltfAdded,GltfCollectionDiff,GltfAssetDiff,GltfSceneDiff,GltfNodeDiff,GltfMeshDiff,GltfAccessorDiff,GltfMaterialDiff,GltfBufferDiff,GltfScenesDiff,GltfNodesDiff,GltfMeshesDiff,GltfAccessorsDiff,GltfMaterialsDiff,GltfBuffersDiff,GltfBufferViewsDiff,GltfBufferBytesDiff,GltfTexturesDiff,GltfImagesDiff,GltfSamplersDiff,GltfSkinsDiff,GltfAnimationsDiff,GltfCamerasDiff,GltfDiff,GltfPhase} from "../../../🧬️schema/🔺️diff/🟦️.ts";
export type * from "../../../🧬️schema/🔺️diff/🟦️.ts";
import type {Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 🔺️ GltfDiff twin: the sparse per-field diff with index-keyed collection triples and no full-replace `snapshot` slot,
 * member for member as `🦀️.rs` writes it, the readers that decode it, and the adjacently tagged phase wire every
 * mutation leaf shares.
 * @see ./🔣️.json
 * @see ./🦀️.rs */
import {
  gltfWireArray,
  gltfWireBoolean,
  gltfWireByte,
  gltfWireIndex,
  gltfWireLiteral,
  gltfWireNullable,
  gltfWireNumber,
  gltfWireObject,
  gltfWireOptional,
  gltfWireRequired,
  gltfWireString,
  gltfWireTagged,
  gltfWireTuple,
  parseGltfAccessorType,
  parseGltfAlphaMode,
  parseGltfAnimation,
  parseGltfBuffer,
  parseGltfBufferView,
  parseGltfCamera,
  parseGltfComponentType,
  parseGltfImage,
  parseGltfJson,
  parseGltfMaterial,
  parseGltfMesh,
  parseGltfNode,
  parseGltfNormalTextureInfo,
  parseGltfOcclusionTextureInfo,
  parseGltfPbrMetallicRoughness,
  parseGltfPrimitive,
  parseGltfSampler,
  parseGltfScene,
  parseGltfSkin,
  parseGltfSourceForm,
  parseGltfSparseAccessor,
  parseGltfTexture,
  parseGltfTextureInfo,
  parseGltfAccessor,
  type GltfAccessor,
  type GltfAccessorType,
  type GltfAlphaMode,
  type GltfAnimation,
  type GltfBuffer,
  type GltfBufferView,
  type GltfCamera,
  type GltfComponentType,
  type GltfImage,
  type GltfJson,
  type GltfMaterial,
  type GltfMesh,
  type GltfNode,
  type GltfNormalTextureInfo,
  type GltfOcclusionTextureInfo,
  type GltfPbrMetallicRoughness,
  type GltfPrimitive,
  type GltfSampler,
  type GltfScene,
  type GltfSkin,
  type GltfSourceForm,
  type GltfSparseAccessor,
  type GltfTexture,
  type GltfTextureInfo,
  type GltfWireReader,
} from "../📸️snapshot/🔣️json/🟦️.ts";
//#endregion 🧾️Records

//#region 📥️Parsers
export const parseGltfTouchedRegion = gltfWireLiteral("asset", "scene", "scenes", "nodes", "meshes", "accessors", "bufferViews", "buffers", "bufferBytes", "materials", "textures", "images", "samplers", "skins", "animations", "cameras", "extensionsUsed", "extensionsRequired", "extensions", "extras", "sourceForm");
export const parseGltfJsonPresence = gltfWireTagged<GltfJsonPresence, "state">("state", {
  absent: gltfWireObject<{ state: "absent" }>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<{ state: "present"; value: GltfJson }>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
const presence = gltfWireOptional(parseGltfJsonPresence);
const nullableName = gltfWireOptional(gltfWireNullable(gltfWireString));
const nullableIndex = gltfWireOptional(gltfWireNullable(gltfWireIndex));
const indices = gltfWireOptional(gltfWireArray(gltfWireIndex));
const numbers = gltfWireOptional(gltfWireArray(gltfWireNumber));
const vector3 = gltfWireTuple(gltfWireNumber, gltfWireNumber, gltfWireNumber);
const vector4 = gltfWireTuple(gltfWireNumber, gltfWireNumber, gltfWireNumber, gltfWireNumber);
const collection = <T, D>(item: GltfWireReader<T>, diff: GltfWireReader<D>): GltfWireReader<GltfCollectionDiff<T, D>> =>
  gltfWireObject<GltfCollectionDiff<T, D>>({
    removed: indices,
    modified: gltfWireOptional(gltfWireArray(gltfWireObject<GltfModified<D>>({ index: gltfWireRequired(gltfWireIndex), diff: gltfWireRequired(diff) }))),
    added: gltfWireOptional(gltfWireArray(gltfWireObject<GltfAdded<T>>({ index: gltfWireRequired(gltfWireIndex), item: gltfWireRequired(item) }))),
  });

export const parseGltfAssetDiff = gltfWireObject<GltfAssetDiff>({
  version: gltfWireOptional(gltfWireString),
  generator: nullableName,
  copyright: nullableName,
  minVersion: nullableName,
  extensions: presence,
  extras: presence,
});
export const parseGltfSceneDiff = gltfWireObject<GltfSceneDiff>({ nodes: indices, name: nullableName, extensions: presence, extras: presence });
export const parseGltfNodeDiff = gltfWireObject<GltfNodeDiff>({
  children: indices,
  mesh: nullableIndex,
  camera: nullableIndex,
  skin: nullableIndex,
  matrix: gltfWireOptional(gltfWireNullable(gltfWireArray(gltfWireNumber, 16))),
  translation: gltfWireOptional(gltfWireNullable(vector3)),
  rotation: gltfWireOptional(gltfWireNullable(vector4)),
  scale: gltfWireOptional(gltfWireNullable(vector3)),
  weights: numbers,
  name: nullableName,
  extensions: presence,
  extras: presence,
});
export const parseGltfMeshDiff = gltfWireObject<GltfMeshDiff>({ primitives: gltfWireOptional(gltfWireArray(parseGltfPrimitive)), weights: numbers, name: nullableName, extensions: presence, extras: presence });
export const parseGltfAccessorDiff = gltfWireObject<GltfAccessorDiff>({
  bufferView: nullableIndex,
  byteOffset: gltfWireOptional(gltfWireIndex),
  componentType: gltfWireOptional(parseGltfComponentType),
  normalized: gltfWireOptional(gltfWireBoolean),
  count: gltfWireOptional(gltfWireIndex),
  kind: gltfWireOptional(parseGltfAccessorType),
  max: gltfWireOptional(gltfWireNullable(gltfWireArray(gltfWireNumber))),
  min: gltfWireOptional(gltfWireNullable(gltfWireArray(gltfWireNumber))),
  sparse: gltfWireOptional(gltfWireNullable(parseGltfSparseAccessor)),
  name: nullableName,
  extensions: presence,
  extras: presence,
});
export const parseGltfMaterialDiff = gltfWireObject<GltfMaterialDiff>({
  name: nullableName,
  pbrMetallicRoughness: gltfWireOptional(gltfWireNullable(parseGltfPbrMetallicRoughness)),
  normalTexture: gltfWireOptional(gltfWireNullable(parseGltfNormalTextureInfo)),
  occlusionTexture: gltfWireOptional(gltfWireNullable(parseGltfOcclusionTextureInfo)),
  emissiveTexture: gltfWireOptional(gltfWireNullable(parseGltfTextureInfo)),
  emissiveFactor: gltfWireOptional(vector3),
  alphaMode: gltfWireOptional(parseGltfAlphaMode),
  alphaCutoff: gltfWireOptional(gltfWireNumber),
  doubleSided: gltfWireOptional(gltfWireBoolean),
  extensions: presence,
  extras: presence,
});
export const parseGltfBufferDiff = gltfWireObject<GltfBufferDiff>({ byteLength: gltfWireOptional(gltfWireIndex), uri: nullableName, name: nullableName, extensions: presence, extras: presence });
export const parseGltfScenesDiff = collection(parseGltfScene, parseGltfSceneDiff);
export const parseGltfNodesDiff = collection(parseGltfNode, parseGltfNodeDiff);
export const parseGltfMeshesDiff = collection(parseGltfMesh, parseGltfMeshDiff);
export const parseGltfAccessorsDiff = collection(parseGltfAccessor, parseGltfAccessorDiff);
export const parseGltfMaterialsDiff = collection(parseGltfMaterial, parseGltfMaterialDiff);
export const parseGltfBuffersDiff = collection(parseGltfBuffer, parseGltfBufferDiff);
export const parseGltfBufferViewsDiff = collection(parseGltfBufferView, parseGltfBufferView);
export const parseGltfBufferBytesDiff = collection(gltfWireArray(gltfWireByte), gltfWireArray(gltfWireByte));
export const parseGltfTexturesDiff = collection(parseGltfTexture, parseGltfTexture);
export const parseGltfImagesDiff = collection(parseGltfImage, parseGltfImage);
export const parseGltfSamplersDiff = collection(parseGltfSampler, parseGltfSampler);
export const parseGltfSkinsDiff = collection(parseGltfSkin, parseGltfSkin);
export const parseGltfAnimationsDiff = collection(parseGltfAnimation, parseGltfAnimation);
export const parseGltfCamerasDiff = collection(parseGltfCamera, parseGltfCamera);
export const parseGltfDiff = gltfWireObject<GltfDiff>({
  asset: gltfWireOptional(parseGltfAssetDiff),
  scene: nullableIndex,
  scenes: gltfWireOptional(parseGltfScenesDiff),
  nodes: gltfWireOptional(parseGltfNodesDiff),
  meshes: gltfWireOptional(parseGltfMeshesDiff),
  accessors: gltfWireOptional(parseGltfAccessorsDiff),
  bufferViews: gltfWireOptional(parseGltfBufferViewsDiff),
  buffers: gltfWireOptional(parseGltfBuffersDiff),
  bufferBytes: gltfWireOptional(parseGltfBufferBytesDiff),
  materials: gltfWireOptional(parseGltfMaterialsDiff),
  textures: gltfWireOptional(parseGltfTexturesDiff),
  images: gltfWireOptional(parseGltfImagesDiff),
  samplers: gltfWireOptional(parseGltfSamplersDiff),
  skins: gltfWireOptional(parseGltfSkinsDiff),
  animations: gltfWireOptional(parseGltfAnimationsDiff),
  cameras: gltfWireOptional(parseGltfCamerasDiff),
  extensionsUsed: gltfWireOptional(gltfWireArray(gltfWireString)),
  extensionsRequired: gltfWireOptional(gltfWireArray(gltfWireString)),
  extensions: presence,
  extras: presence,
  sourceForm: gltfWireOptional(parseGltfSourceForm),
});
export const parseGltfDiffDerivation = gltfWireObject<GltfDiffDerivation>({
  forward: gltfWireRequired(parseGltfDiff),
  inverse: gltfWireRequired(parseGltfDiff),
  touchedPaths: gltfWireRequired(gltfWireArray(gltfWireString)),
  touchedRegions: gltfWireRequired(gltfWireArray(parseGltfTouchedRegion)),
});

/** 🔀️ Reads a leaf's phase wire: `apply` through the leaf's own payload reader, `restore` through the recorded diff's. */
export const gltfWirePhase = <P, R>(apply: GltfWireReader<P>, restore: GltfWireReader<R>): GltfWireReader<GltfPhase<P, R>> =>
  gltfWireTagged<GltfPhase<P, R>, "phase">("phase", {
    apply: gltfWireObject<{ readonly phase: "apply"; readonly value: P }>({ phase: gltfWireRequired(gltfWireLiteral("apply")), value: gltfWireRequired(apply) }),
    restore: gltfWireObject<{ readonly phase: "restore"; readonly value: R }>({ phase: gltfWireRequired(gltfWireLiteral("restore")), value: gltfWireRequired(restore) }),
  });
//#endregion 📥️Parsers

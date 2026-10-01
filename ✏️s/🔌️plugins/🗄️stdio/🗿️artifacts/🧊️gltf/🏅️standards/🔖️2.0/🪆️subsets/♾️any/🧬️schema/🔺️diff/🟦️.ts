import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
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
} from "../📸️snapshot/🟦️.ts";

//#region 🧾️Records
export type GltfTouchedRegion = "asset" | "scene" | "scenes" | "nodes" | "meshes" | "accessors" | "bufferViews" | "buffers" | "bufferBytes" | "materials" | "textures" | "images" | "samplers" | "skins" | "animations" | "cameras" | "extensionsUsed" | "extensionsRequired" | "extensions" | "extras" | "sourceForm";
/** ↩️ Law: `inverse.apply(forward.apply(base)) === base`; paths are sorted and deduplicated. */
export interface GltfDiffDerivation {
  forward: GltfDiff;
  inverse: GltfDiff;
  touchedPaths: string[];
  touchedRegions: GltfTouchedRegion[];
}

/** 🈳️ `Option<Option<GltfJson>>` on the wire: `absent` clears the slot, `present` sets it (a present JSON null stays distinct). */
export type GltfJsonPresence = { state: "absent" } | { state: "present"; value: GltfJson };
export interface GltfModified<D> {
  index: bigint;
  diff: D;
}
export interface GltfAdded<T> {
  index: bigint;
  item: T;
}
export interface GltfCollectionDiff<T, D> {
  removed?: bigint[];
  modified?: GltfModified<D>[];
  added?: GltfAdded<T>[];
}

export interface GltfAssetDiff {
  version?: string;
  generator?: string | null;
  copyright?: string | null;
  minVersion?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfSceneDiff {
  nodes?: bigint[];
  name?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfNodeDiff {
  children?: bigint[];
  mesh?: bigint | null;
  camera?: bigint | null;
  skin?: bigint | null;
  matrix?: Binary64[] | null;
  translation?: [Binary64, Binary64, Binary64] | null;
  rotation?: [Binary64, Binary64, Binary64, Binary64] | null;
  scale?: [Binary64, Binary64, Binary64] | null;
  weights?: Binary64[];
  name?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfMeshDiff {
  primitives?: GltfPrimitive[];
  weights?: Binary64[];
  name?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfAccessorDiff {
  bufferView?: bigint | null;
  byteOffset?: bigint;
  componentType?: GltfComponentType;
  normalized?: boolean;
  count?: bigint;
  kind?: GltfAccessorType;
  max?: Binary64[] | null;
  min?: Binary64[] | null;
  sparse?: GltfSparseAccessor | null;
  name?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfMaterialDiff {
  name?: string | null;
  pbrMetallicRoughness?: GltfPbrMetallicRoughness | null;
  normalTexture?: GltfNormalTextureInfo | null;
  occlusionTexture?: GltfOcclusionTextureInfo | null;
  emissiveTexture?: GltfTextureInfo | null;
  emissiveFactor?: [Binary64, Binary64, Binary64];
  alphaMode?: GltfAlphaMode;
  alphaCutoff?: Binary64;
  doubleSided?: boolean;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfBufferDiff {
  byteLength?: bigint;
  uri?: string | null;
  name?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export type GltfScenesDiff = GltfCollectionDiff<GltfScene, GltfSceneDiff>;
export type GltfNodesDiff = GltfCollectionDiff<GltfNode, GltfNodeDiff>;
export type GltfMeshesDiff = GltfCollectionDiff<GltfMesh, GltfMeshDiff>;
export type GltfAccessorsDiff = GltfCollectionDiff<GltfAccessor, GltfAccessorDiff>;
export type GltfMaterialsDiff = GltfCollectionDiff<GltfMaterial, GltfMaterialDiff>;
export type GltfBuffersDiff = GltfCollectionDiff<GltfBuffer, GltfBufferDiff>;
export type GltfBufferViewsDiff = GltfCollectionDiff<GltfBufferView, GltfBufferView>;
export type GltfBufferBytesDiff = GltfCollectionDiff<number[], number[]>;
export type GltfTexturesDiff = GltfCollectionDiff<GltfTexture, GltfTexture>;
export type GltfImagesDiff = GltfCollectionDiff<GltfImage, GltfImage>;
export type GltfSamplersDiff = GltfCollectionDiff<GltfSampler, GltfSampler>;
export type GltfSkinsDiff = GltfCollectionDiff<GltfSkin, GltfSkin>;
export type GltfAnimationsDiff = GltfCollectionDiff<GltfAnimation, GltfAnimation>;
export type GltfCamerasDiff = GltfCollectionDiff<GltfCamera, GltfCamera>;

export interface GltfDiff {
  asset?: GltfAssetDiff;
  scene?: bigint | null;
  scenes?: GltfScenesDiff;
  nodes?: GltfNodesDiff;
  meshes?: GltfMeshesDiff;
  accessors?: GltfAccessorsDiff;
  bufferViews?: GltfBufferViewsDiff;
  buffers?: GltfBuffersDiff;
  bufferBytes?: GltfBufferBytesDiff;
  materials?: GltfMaterialsDiff;
  textures?: GltfTexturesDiff;
  images?: GltfImagesDiff;
  samplers?: GltfSamplersDiff;
  skins?: GltfSkinsDiff;
  animations?: GltfAnimationsDiff;
  cameras?: GltfCamerasDiff;
  extensionsUsed?: string[];
  extensionsRequired?: string[];
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
  sourceForm?: GltfSourceForm;
}

/** 🔀️ A leaf's `#[value(tag = "phase", content = "value")]` wire: the editable `Apply` payload, or the inert recorded restore. */
export type GltfPhase<P, R = GltfDiff> = { readonly phase: "apply"; readonly value: P } | { readonly phase: "restore"; readonly value: R };
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

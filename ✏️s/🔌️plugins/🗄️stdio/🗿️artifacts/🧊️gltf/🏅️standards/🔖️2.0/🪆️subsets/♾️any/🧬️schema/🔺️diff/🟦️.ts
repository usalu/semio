import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
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
  parseGltfAnimationChannel,
  parseGltfAnimationSampler,
  parseGltfMorphTarget,
  type GltfAnimationChannel,
  type GltfAnimationSampler,
  type GltfMorphTarget,
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
  amended?: GltfModified<D>[];
}

export interface GltfAttribute {
  semantic: string;
  accessor: bigint;
}
export interface GltfListRemoval<K> {
  id: K;
  index: bigint;
}
export interface GltfListInsertion<R> {
  index: bigint;
  row: R;
}
export interface GltfListRelocation<K> {
  id: K;
  from: bigint;
  to: bigint;
}
export interface GltfListDelta<K, R> {
  removed?: GltfListRemoval<K>[];
  inserted?: GltfListInsertion<R>[];
  moved?: GltfListRelocation<K>[];
}
export type GltfRefsDelta = GltfListDelta<bigint, bigint>;
export type GltfStringsDelta = GltfListDelta<string, string>;
export type GltfAttributesDelta = GltfListDelta<string, GltfAttribute>;
export type GltfTargetsDelta = GltfListDelta<string, GltfMorphTarget>;
export type GltfChannelsDelta = GltfListDelta<string, GltfAnimationChannel>;
export type GltfAnimationSamplersDelta = GltfListDelta<string, GltfAnimationSampler>;

export interface GltfAssetDiff {
  version?: string;
  generator?: string | null;
  copyright?: string | null;
  minVersion?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfSceneDiff {
  nodes?: GltfRefsDelta;
  name?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfNodeDiff {
  children?: GltfRefsDelta;
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
  primitives?: GltfPrimitivesDiff;
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

export interface GltfPrimitiveDiff {
  attributes?: GltfAttributesDelta;
  indices?: bigint | null;
  material?: bigint | null;
  mode?: bigint | null;
  targets?: GltfTargetsDelta;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfTextureDiff {
  sampler?: bigint | null;
  source?: bigint | null;
  name?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfImageDiff {
  uri?: string | null;
  mimeType?: string | null;
  bufferView?: bigint | null;
  name?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfBufferViewDiff {
  buffer?: bigint;
  byteOffset?: bigint;
  byteLength?: bigint;
  byteStride?: bigint | null;
  target?: bigint | null;
  name?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfSkinDiff {
  inverseBindMatrices?: bigint | null;
  skeleton?: bigint | null;
  joints?: GltfRefsDelta;
  name?: string | null;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
}

export interface GltfAnimationDiff {
  channels?: GltfChannelsDelta;
  samplers?: GltfAnimationSamplersDelta;
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
export type GltfBufferViewsDiff = GltfCollectionDiff<GltfBufferView, GltfBufferViewDiff>;
export type GltfPrimitivesDiff = GltfCollectionDiff<GltfPrimitive, GltfPrimitiveDiff>;
export type GltfBufferBytesDiff = GltfCollectionDiff<number[], number[]>;
export type GltfTexturesDiff = GltfCollectionDiff<GltfTexture, GltfTextureDiff>;
export type GltfImagesDiff = GltfCollectionDiff<GltfImage, GltfImageDiff>;
export type GltfSamplersDiff = GltfCollectionDiff<GltfSampler, GltfSampler>;
export type GltfSkinsDiff = GltfCollectionDiff<GltfSkin, GltfSkinDiff>;
export type GltfAnimationsDiff = GltfCollectionDiff<GltfAnimation, GltfAnimationDiff>;
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
  extensionsUsed?: GltfStringsDelta;
  extensionsRequired?: GltfStringsDelta;
  extensions?: GltfJsonPresence;
  extras?: GltfJsonPresence;
  sourceForm?: GltfSourceForm;
}

/** 🔀️ A leaf's `#[value(tag = "phase", content = "value")]` wire: the editable `Apply` payload. */
export type GltfApplyPhase<P> = { readonly phase: "apply"; readonly value: P };
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
    amended: gltfWireOptional(gltfWireArray(gltfWireObject<GltfModified<D>>({ index: gltfWireRequired(gltfWireIndex), diff: gltfWireRequired(diff) }))),
  });

export const parseGltfAttribute = gltfWireObject<GltfAttribute>({ semantic: gltfWireRequired(gltfWireString), accessor: gltfWireRequired(gltfWireIndex) });
const listDelta = <K, R>(id: GltfWireReader<K>, row: GltfWireReader<R>): GltfWireReader<GltfListDelta<K, R>> =>
  gltfWireObject<GltfListDelta<K, R>>({
    removed: gltfWireOptional(gltfWireArray(gltfWireObject<GltfListRemoval<K>>({ id: gltfWireRequired(id), index: gltfWireRequired(gltfWireIndex) }))),
    inserted: gltfWireOptional(gltfWireArray(gltfWireObject<GltfListInsertion<R>>({ index: gltfWireRequired(gltfWireIndex), row: gltfWireRequired(row) }))),
    moved: gltfWireOptional(gltfWireArray(gltfWireObject<GltfListRelocation<K>>({ id: gltfWireRequired(id), from: gltfWireRequired(gltfWireIndex), to: gltfWireRequired(gltfWireIndex) }))),
  });
export const parseGltfRefsDelta = listDelta(gltfWireIndex, gltfWireIndex);
export const parseGltfStringsDelta = listDelta(gltfWireString, gltfWireString);
export const parseGltfAttributesDelta = listDelta(gltfWireString, parseGltfAttribute);
export const parseGltfTargetsDelta = listDelta(gltfWireString, parseGltfMorphTarget);
export const parseGltfChannelsDelta = listDelta(gltfWireString, parseGltfAnimationChannel);
export const parseGltfAnimationSamplersDelta = listDelta(gltfWireString, parseGltfAnimationSampler);
export const parseGltfAssetDiff = gltfWireObject<GltfAssetDiff>({
  version: gltfWireOptional(gltfWireString),
  generator: nullableName,
  copyright: nullableName,
  minVersion: nullableName,
  extensions: presence,
  extras: presence,
});
export const parseGltfSceneDiff = gltfWireObject<GltfSceneDiff>({ nodes: gltfWireOptional(parseGltfRefsDelta), name: nullableName, extensions: presence, extras: presence });
export const parseGltfNodeDiff = gltfWireObject<GltfNodeDiff>({
  children: gltfWireOptional(parseGltfRefsDelta),
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
export const parseGltfPrimitiveDiff = gltfWireObject<GltfPrimitiveDiff>({ attributes: gltfWireOptional(parseGltfAttributesDelta), indices: nullableIndex, material: nullableIndex, mode: nullableIndex, targets: gltfWireOptional(parseGltfTargetsDelta), extensions: presence, extras: presence });
export const parseGltfTextureDiff = gltfWireObject<GltfTextureDiff>({ sampler: nullableIndex, source: nullableIndex, name: nullableName, extensions: presence, extras: presence });
export const parseGltfImageDiff = gltfWireObject<GltfImageDiff>({ uri: nullableName, mimeType: nullableName, bufferView: nullableIndex, name: nullableName, extensions: presence, extras: presence });
export const parseGltfBufferViewDiff = gltfWireObject<GltfBufferViewDiff>({ buffer: gltfWireOptional(gltfWireIndex), byteOffset: gltfWireOptional(gltfWireIndex), byteLength: gltfWireOptional(gltfWireIndex), byteStride: nullableIndex, target: nullableIndex, name: nullableName, extensions: presence, extras: presence });
export const parseGltfSkinDiff = gltfWireObject<GltfSkinDiff>({ inverseBindMatrices: nullableIndex, skeleton: nullableIndex, joints: gltfWireOptional(parseGltfRefsDelta), name: nullableName, extensions: presence, extras: presence });
export const parseGltfAnimationDiff = gltfWireObject<GltfAnimationDiff>({ channels: gltfWireOptional(parseGltfChannelsDelta), samplers: gltfWireOptional(parseGltfAnimationSamplersDelta), name: nullableName, extensions: presence, extras: presence });
export const parseGltfPrimitivesDiff = collection(parseGltfPrimitive, parseGltfPrimitiveDiff);
export const parseGltfMeshDiff = gltfWireObject<GltfMeshDiff>({ primitives: gltfWireOptional(parseGltfPrimitivesDiff), weights: numbers, name: nullableName, extensions: presence, extras: presence });
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
export const parseGltfBufferViewsDiff = collection(parseGltfBufferView, parseGltfBufferViewDiff);
export const parseGltfBufferBytesDiff = collection(gltfWireArray(gltfWireByte), gltfWireArray(gltfWireByte));
export const parseGltfTexturesDiff = collection(parseGltfTexture, parseGltfTextureDiff);
export const parseGltfImagesDiff = collection(parseGltfImage, parseGltfImageDiff);
export const parseGltfSamplersDiff = collection(parseGltfSampler, parseGltfSampler);
export const parseGltfSkinsDiff = collection(parseGltfSkin, parseGltfSkinDiff);
export const parseGltfAnimationsDiff = collection(parseGltfAnimation, parseGltfAnimationDiff);
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
  extensionsUsed: gltfWireOptional(parseGltfStringsDelta),
  extensionsRequired: gltfWireOptional(parseGltfStringsDelta),
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

/** 🔀️ Reads a leaf's phase wire through the leaf's own payload reader. */
export const gltfWireApplyPhase = <P>(apply: GltfWireReader<P>): GltfWireReader<GltfApplyPhase<P>> =>
  gltfWireObject<GltfApplyPhase<P>>({ phase: gltfWireRequired(gltfWireLiteral("apply")), value: gltfWireRequired(apply) });
//#endregion 📥️Parsers

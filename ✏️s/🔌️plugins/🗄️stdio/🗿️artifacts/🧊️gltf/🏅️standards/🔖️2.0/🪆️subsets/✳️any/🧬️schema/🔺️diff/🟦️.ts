/** 🔺️ GltfDiff — sparse per-field diff. Index-keyed collection triples for every top-level array;
 * no full-replace `snapshot` slot anywhere. Mirrors `🦀️.rs` field-for-field. */
import type {
  GltfAsset, GltfScene, GltfNode, GltfMesh, GltfAccessor, GltfBufferView, GltfBuffer, GltfMaterial,
  GltfTexture, GltfImage, GltfSampler, GltfSkin, GltfAnimation, GltfCamera, GltfJson, GltfSourceForm,
} from '../📸️snapshot/🟦️.ts';

export type GltfTouchedRegion = 'asset' | 'scene' | 'scenes' | 'nodes' | 'meshes' | 'accessors' | 'bufferViews' | 'buffers' | 'bufferBytes' | 'materials' | 'textures' | 'images' | 'samplers' | 'skins' | 'animations' | 'cameras' | 'extensionsUsed' | 'extensionsRequired' | 'extensions' | 'extras' | 'sourceForm';
/** ↩️ Law: `inverse.apply(forward.apply(base)) === base`; paths are sorted and deduplicated. */
export interface GltfDiffDerivation { forward: GltfDiff; inverse: GltfDiff; touchedPaths: string[]; touchedRegions: GltfTouchedRegion[] }

export interface GltfModified<D> { index: number; diff: D }
export interface GltfAdded<T> { index: number; item: T }
export interface GltfCollectionDiff<T, D> {
  removed?: number[];
  modified?: GltfModified<D>[];
  added?: GltfAdded<T>[];
}
/** Weak collection: diff IS the whole new item (D = T). */
export type GltfWeakCollectionDiff<T> = GltfCollectionDiff<T, T>;

export interface GltfAssetDiff {
  version?: string;
  generator?: string | null;
  copyright?: string | null;
  minVersion?: string | null;
  extensions?: GltfJson | null;
  extras?: GltfJson | null;
}

export interface GltfSceneDiff { nodes?: number[]; name?: string | null; extensions?: GltfJson | null; extras?: GltfJson | null }

export interface GltfNodeDiff {
  children?: number[];
  mesh?: number | null;
  camera?: number | null;
  skin?: number | null;
  matrix?: number[] | null;
  translation?: [number, number, number] | null;
  rotation?: [number, number, number, number] | null;
  scale?: [number, number, number] | null;
  weights?: number[];
  name?: string | null;
  extensions?: GltfJson | null;
  extras?: GltfJson | null;
}

export interface GltfMeshDiff {
  primitives?: GltfMesh['primitives'];
  weights?: number[];
  name?: string | null;
  extensions?: GltfJson | null;
  extras?: GltfJson | null;
}

export interface GltfAccessorDiff {
  bufferView?: number | null;
  byteOffset?: number;
  componentType?: number;
  normalized?: boolean;
  count?: number;
  kind?: GltfAccessor['type'];
  max?: number[] | null;
  min?: number[] | null;
  sparse?: GltfAccessor['sparse'] | null;
  name?: string | null;
  extensions?: GltfJson | null;
  extras?: GltfJson | null;
}

export interface GltfMaterialDiff {
  name?: string | null;
  pbrMetallicRoughness?: GltfMaterial['pbrMetallicRoughness'] | null;
  normalTexture?: GltfMaterial['normalTexture'] | null;
  occlusionTexture?: GltfMaterial['occlusionTexture'] | null;
  emissiveTexture?: GltfMaterial['emissiveTexture'] | null;
  emissiveFactor?: [number, number, number];
  alphaMode?: GltfMaterial['alphaMode'];
  alphaCutoff?: number;
  doubleSided?: boolean;
  extensions?: GltfJson | null;
  extras?: GltfJson | null;
}

export interface GltfBufferDiff {
  byteLength?: number;
  uri?: string | null;
  name?: string | null;
  extensions?: GltfJson | null;
  extras?: GltfJson | null;
}

export type GltfScenesDiff = GltfCollectionDiff<GltfScene, GltfSceneDiff>;
export type GltfNodesDiff = GltfCollectionDiff<GltfNode, GltfNodeDiff>;
export type GltfMeshesDiff = GltfCollectionDiff<GltfMesh, GltfMeshDiff>;
export type GltfAccessorsDiff = GltfCollectionDiff<GltfAccessor, GltfAccessorDiff>;
export type GltfMaterialsDiff = GltfCollectionDiff<GltfMaterial, GltfMaterialDiff>;
export type GltfBuffersDiff = GltfCollectionDiff<GltfBuffer, GltfBufferDiff>;
export type GltfBufferViewsDiff = GltfWeakCollectionDiff<GltfBufferView>;
export type GltfBufferBytesDiff = GltfWeakCollectionDiff<number[]>;
export type GltfTexturesDiff = GltfWeakCollectionDiff<GltfTexture>;
export type GltfImagesDiff = GltfWeakCollectionDiff<GltfImage>;
export type GltfSamplersDiff = GltfWeakCollectionDiff<GltfSampler>;
export type GltfSkinsDiff = GltfWeakCollectionDiff<GltfSkin>;
export type GltfAnimationsDiff = GltfWeakCollectionDiff<GltfAnimation>;
export type GltfCamerasDiff = GltfWeakCollectionDiff<GltfCamera>;

export interface GltfDiff {
  asset?: GltfAssetDiff;
  scene?: number | null;
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
  extensions?: GltfJson | null;
  extras?: GltfJson | null;
  sourceForm?: GltfSourceForm;
}

/** 🧬️ GltfSnapshot schema — fully typed glTF 2.0 document model (mirrors the Rust shape 1:1;
 * `GltfJson` mirrors this artifact's own extras/extensions value enum, structurally like real
 * JSON but a locally-owned type). */
export type GltfJson =
  | null
  | boolean
  | number
  | string
  | GltfJson[]
  | { [key: string]: GltfJson };

export interface GltfAsset {
  version: string;
  generator?: string;
  copyright?: string;
  minVersion?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfScene {
  nodes: number[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfNode {
  children: number[];
  mesh?: number;
  camera?: number;
  skin?: number;
  matrix?: number[]; // 16
  translation?: [number, number, number];
  rotation?: [number, number, number, number];
  scale?: [number, number, number];
  weights: number[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfPrimitive {
  attributes: Record<string, number>;
  indices?: number;
  material?: number;
  mode?: number;
  targets: Record<string, number>[];
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfMesh {
  primitives: GltfPrimitive[];
  weights: number[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfSparseIndices { bufferView: number; byteOffset: number; componentType: number }
export interface GltfSparseValues { bufferView: number; byteOffset: number }
export interface GltfSparseAccessor { count: number; indices: GltfSparseIndices; values: GltfSparseValues }

export interface GltfAccessor {
  bufferView?: number;
  byteOffset: number;
  componentType: number; // 5120|5121|5122|5123|5125|5126
  normalized: boolean;
  count: number;
  type: 'SCALAR' | 'VEC2' | 'VEC3' | 'VEC4' | 'MAT2' | 'MAT3' | 'MAT4';
  max?: number[];
  min?: number[];
  sparse?: GltfSparseAccessor;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfBufferView {
  buffer: number;
  byteOffset: number;
  byteLength: number;
  byteStride?: number;
  target?: number;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfBuffer {
  byteLength: number;
  uri?: string;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfTextureInfo { index: number; texCoord: number; extensions?: GltfJson; extras?: GltfJson }
export interface GltfNormalTextureInfo { index: number; texCoord: number; scale: number; extensions?: GltfJson; extras?: GltfJson }
export interface GltfOcclusionTextureInfo { index: number; texCoord: number; strength: number; extensions?: GltfJson; extras?: GltfJson }

export interface GltfPbrMetallicRoughness {
  baseColorFactor: [number, number, number, number];
  baseColorTexture?: GltfTextureInfo;
  metallicFactor: number;
  roughnessFactor: number;
  metallicRoughnessTexture?: GltfTextureInfo;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export type GltfAlphaMode = 'OPAQUE' | 'MASK' | 'BLEND';

export interface GltfMaterial {
  name?: string;
  pbrMetallicRoughness?: GltfPbrMetallicRoughness;
  normalTexture?: GltfNormalTextureInfo;
  occlusionTexture?: GltfOcclusionTextureInfo;
  emissiveTexture?: GltfTextureInfo;
  emissiveFactor: [number, number, number];
  alphaMode: GltfAlphaMode;
  alphaCutoff: number;
  doubleSided: boolean;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfTexture { sampler?: number; source?: number; name?: string; extensions?: GltfJson; extras?: GltfJson }
export interface GltfImage { uri?: string; mimeType?: string; bufferView?: number; name?: string; extensions?: GltfJson; extras?: GltfJson }
export interface GltfSampler { magFilter?: number; minFilter?: number; wrapS: number; wrapT: number; name?: string; extensions?: GltfJson; extras?: GltfJson }
export interface GltfSkin { inverseBindMatrices?: number; skeleton?: number; joints: number[]; name?: string; extensions?: GltfJson; extras?: GltfJson }

export type GltfAnimationPath = 'translation' | 'rotation' | 'scale' | 'weights';
export interface GltfAnimationChannelTarget { node?: number; path: GltfAnimationPath; extensions?: GltfJson; extras?: GltfJson }
export interface GltfAnimationChannel { sampler: number; target: GltfAnimationChannelTarget; extensions?: GltfJson; extras?: GltfJson }
export type GltfInterpolation = 'LINEAR' | 'STEP' | 'CUBICSPLINE';
export interface GltfAnimationSampler { input: number; interpolation: GltfInterpolation; output: number; extensions?: GltfJson; extras?: GltfJson }
export interface GltfAnimation { channels: GltfAnimationChannel[]; samplers: GltfAnimationSampler[]; name?: string; extensions?: GltfJson; extras?: GltfJson }

export interface GltfOrthographic { xmag: number; ymag: number; zfar: number; znear: number; extensions?: GltfJson; extras?: GltfJson }
export interface GltfPerspective { aspectRatio?: number; yfov: number; zfar?: number; znear: number; extensions?: GltfJson; extras?: GltfJson }
export type GltfCamera =
  | ({ type: 'perspective'; perspective: GltfPerspective })
  | ({ type: 'orthographic'; orthographic: GltfOrthographic })
  & { name?: string; extensions?: GltfJson; extras?: GltfJson };

export interface GltfDocument {
  asset: GltfAsset;
  scene?: number;
  scenes: GltfScene[];
  nodes: GltfNode[];
  meshes: GltfMesh[];
  accessors: GltfAccessor[];
  bufferViews: GltfBufferView[];
  buffers: GltfBuffer[];
  materials: GltfMaterial[];
  textures: GltfTexture[];
  images: GltfImage[];
  samplers: GltfSampler[];
  skins: GltfSkin[];
  animations: GltfAnimation[];
  cameras: GltfCamera[];
  extensionsUsed: string[];
  extensionsRequired: string[];
  extensions?: GltfJson;
  extras?: GltfJson;
}

export type GltfSourceForm = 'json' | 'glb';

export interface GltfSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ document: GltfDocument;
  /** @state artifact */ buffers: number[][]; // raw payload bytes, index-aligned with document.buffers
  /** @state artifact */ sourceForm: GltfSourceForm;
}

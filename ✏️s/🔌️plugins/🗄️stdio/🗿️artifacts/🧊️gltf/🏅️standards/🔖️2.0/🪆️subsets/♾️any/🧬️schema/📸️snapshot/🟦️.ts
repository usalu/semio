/** 🧬️ GltfSnapshot twin: the typed glTF 2.0 document model exactly as its Rust `ToValue` wire writes it, and the
 * readers that decode that wire. A member the wire omits when empty or default is optional here, as in `🔣️.json`.
 * @see ./🔣️.json
 * @see ./🦀️.rs */

//#region 🚪️Wire
/** 🚫️ One refused wire position: where the instance breaks its contract and why. */
export class GltfWireRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

/** 📖️ Reads one wire position into its typed twin, refusing whatever that position's schema refuses. */
export type GltfWireReader<T> = (value: unknown, at?: string) => T;

/** 🧷️ One object member: its reader and whether the wire may omit it. */
export interface GltfWireMember<T, Optional extends boolean = boolean> {
  readonly read: GltfWireReader<T>;
  readonly optional: Optional;
}

/** 🗺️ The member table of `T`, in wire order: a required member reads `T[K]`, an omissible one reads it without `undefined`. */
export type GltfWireMembers<T> = { readonly [K in keyof T]-?: {} extends Pick<T, K> ? GltfWireMember<Exclude<T[K], undefined>, true> : GltfWireMember<T[K], false> };

/** 🏷️ One reader per tag value of a union discriminated by `K`. */
export type GltfWireVariants<T, K extends keyof T> = { readonly [V in T[K] & string]: GltfWireReader<Extract<T, Readonly<Record<K, V>>>> };

export const gltfWireRefuse = (at: string, why: string): never => {
  throw new GltfWireRefusal(at, why);
};
const gltfWireRecord = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gltfWireRefuse(at, "value is not an object");

export const gltfWireString: GltfWireReader<string> = (value, at = "$") => (typeof value === "string" ? value : gltfWireRefuse(at, "value is not a string"));
export const gltfWireBoolean: GltfWireReader<boolean> = (value, at = "$") => (typeof value === "boolean" ? value : gltfWireRefuse(at, "value is not a boolean"));
export const gltfWireNumber: GltfWireReader<number> = (value, at = "$") => (typeof value === "number" && Number.isFinite(value) ? value : gltfWireRefuse(at, "value is not a finite number"));
export const gltfWireInteger = (maximum = Number.MAX_SAFE_INTEGER): GltfWireReader<number> => (value, at = "$") =>
  typeof value === "number" && Number.isSafeInteger(value) && value >= 0 && value <= maximum ? value : gltfWireRefuse(at, `value is not an integer in 0..=${maximum}`);
export const gltfWireIndex = gltfWireInteger();
export const gltfWireByte = gltfWireInteger(255);
export const gltfWireLiteral = <const T extends readonly (string | number)[]>(...members: T): GltfWireReader<T[number]> => (value, at = "$") =>
  members.includes(value as T[number]) ? (value as T[number]) : gltfWireRefuse(at, `value is not one of ${members.join(", ")}`);
export const gltfWireNullable = <T>(read: GltfWireReader<T>): GltfWireReader<T | null> => (value, at = "$") => (value === null ? null : read(value, at));
export const gltfWireArray = <T>(item: GltfWireReader<T>, length?: number): GltfWireReader<T[]> => (value, at = "$") => {
  if (!Array.isArray(value)) return gltfWireRefuse(at, "value is not an array");
  if (length !== undefined && value.length !== length) gltfWireRefuse(at, `array does not hold exactly ${length} items`);
  return value.map((entry, index) => item(entry, `${at}[${index}]`));
};
export const gltfWireTuple = <T extends readonly unknown[]>(...items: { readonly [K in keyof T]: GltfWireReader<T[K]> }): GltfWireReader<[...T]> => (value, at = "$") => {
  if (!Array.isArray(value) || value.length !== items.length) return gltfWireRefuse(at, `value is not an array of exactly ${items.length} items`);
  return value.map((entry, index) => (items[index] as GltfWireReader<unknown>)(entry, `${at}[${index}]`)) as [...T];
};
export const gltfWireMap = <T>(read: GltfWireReader<T>): GltfWireReader<Record<string, T>> => (value, at = "$") =>
  Object.fromEntries(Object.entries(gltfWireRecord(value, at)).map(([key, entry]) => [key, read(entry, `${at}.${key}`)]));
export const gltfWireRequired = <T>(read: GltfWireReader<T>): GltfWireMember<T, false> => ({ read, optional: false });
export const gltfWireOptional = <T>(read: GltfWireReader<T>): GltfWireMember<T, true> => ({ read, optional: true });
export const gltfWireObject = <T extends object>(members: GltfWireMembers<T>): GltfWireReader<T> => (value, at = "$") => {
  const row = gltfWireRecord(value, at);
  for (const key of Object.keys(row)) if (!Object.hasOwn(members, key)) gltfWireRefuse(`${at}.${key}`, "member is not part of the contract");
  return Object.fromEntries(
    Object.entries(members as Readonly<Record<string, GltfWireMember<unknown>>>).flatMap(([key, member]) =>
      Object.hasOwn(row, key) ? [[key, member.read(row[key], `${at}.${key}`)]] : member.optional ? [] : gltfWireRefuse(`${at}.${key}`, "required member is absent"),
    ),
  ) as T;
};
export const gltfWireTagged = <T extends object, K extends keyof T & string>(tag: K, variants: GltfWireVariants<T, K>): GltfWireReader<T> => (value, at = "$") => {
  const key = gltfWireRecord(value, at)[tag];
  const table = variants as Readonly<Record<string, GltfWireReader<T>>>;
  return (typeof key === "string" && Object.hasOwn(table, key) ? table[key]! : gltfWireRefuse(`${at}.${tag}`, `value is not one of ${Object.keys(table).join(", ")}`))(value, at);
};
//#endregion 🚪️Wire

//#region 🧾️Records
/** 🧩️ This artifact's extras/extensions value: any JSON, owned locally rather than borrowed from a JSON library. */
export type GltfJson = null | boolean | number | string | GltfJson[] | { [key: string]: GltfJson };

export interface GltfAsset {
  version: string;
  generator?: string;
  copyright?: string;
  minVersion?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfScene {
  nodes?: number[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfNode {
  children?: number[];
  mesh?: number;
  camera?: number;
  skin?: number;
  matrix?: number[];
  translation?: [number, number, number];
  rotation?: [number, number, number, number];
  scale?: [number, number, number];
  weights?: number[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

/** 🎯️ One morph target: attribute semantic to accessor index. */
export type GltfMorphTarget = Record<string, number>;

export interface GltfPrimitive {
  attributes: Record<string, number>;
  indices?: number;
  material?: number;
  mode?: number;
  targets?: GltfMorphTarget[];
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfMesh {
  primitives?: GltfPrimitive[];
  weights?: number[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

/** 🔢️ `accessor.componentType` as its numeric glTF code. */
export type GltfComponentType = 5120 | 5121 | 5122 | 5123 | 5125 | 5126;
/** 🔠️ `accessor.type` as its glTF spelling. */
export type GltfAccessorType = "SCALAR" | "VEC2" | "VEC3" | "VEC4" | "MAT2" | "MAT3" | "MAT4";

export interface GltfSparseIndices {
  bufferView: number;
  byteOffset?: number;
  componentType: GltfComponentType;
}
export interface GltfSparseValues {
  bufferView: number;
  byteOffset?: number;
}
export interface GltfSparseAccessor {
  count: number;
  indices: GltfSparseIndices;
  values: GltfSparseValues;
}

export interface GltfAccessor {
  bufferView?: number;
  byteOffset?: number;
  componentType: GltfComponentType;
  normalized?: boolean;
  count: number;
  type: GltfAccessorType;
  max?: number[];
  min?: number[];
  sparse?: GltfSparseAccessor;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfBufferView {
  buffer: number;
  byteOffset?: number;
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

export interface GltfTextureInfo {
  index: number;
  texCoord?: number;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfNormalTextureInfo {
  index: number;
  texCoord?: number;
  scale?: number;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfOcclusionTextureInfo {
  index: number;
  texCoord?: number;
  strength?: number;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfPbrMetallicRoughness {
  baseColorFactor?: [number, number, number, number];
  baseColorTexture?: GltfTextureInfo;
  metallicFactor?: number;
  roughnessFactor?: number;
  metallicRoughnessTexture?: GltfTextureInfo;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export type GltfAlphaMode = "OPAQUE" | "MASK" | "BLEND";

export interface GltfMaterial {
  name?: string;
  pbrMetallicRoughness?: GltfPbrMetallicRoughness;
  normalTexture?: GltfNormalTextureInfo;
  occlusionTexture?: GltfOcclusionTextureInfo;
  emissiveTexture?: GltfTextureInfo;
  emissiveFactor?: [number, number, number];
  alphaMode?: GltfAlphaMode;
  alphaCutoff?: number;
  doubleSided?: boolean;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfTexture {
  sampler?: number;
  source?: number;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfImage {
  uri?: string;
  mimeType?: string;
  bufferView?: number;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfSampler {
  magFilter?: number;
  minFilter?: number;
  wrapS?: number;
  wrapT?: number;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfSkin {
  inverseBindMatrices?: number;
  skeleton?: number;
  joints?: number[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export type GltfAnimationPath = "translation" | "rotation" | "scale" | "weights";
export interface GltfAnimationChannelTarget {
  node?: number;
  path: GltfAnimationPath;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfAnimationChannel {
  sampler: number;
  target: GltfAnimationChannelTarget;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export type GltfInterpolation = "LINEAR" | "STEP" | "CUBICSPLINE";
export interface GltfAnimationSampler {
  input: number;
  interpolation?: GltfInterpolation;
  output: number;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfAnimation {
  channels?: GltfAnimationChannel[];
  samplers?: GltfAnimationSampler[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfOrthographic {
  xmag: number;
  ymag: number;
  zfar: number;
  znear: number;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfPerspective {
  aspectRatio?: number;
  yfov: number;
  zfar?: number;
  znear: number;
  extensions?: GltfJson;
  extras?: GltfJson;
}
/** 📷️ A camera's projection: the `type` tag beside the member it names. */
export type GltfCameraProjection = { type: "perspective"; perspective: GltfPerspective } | { type: "orthographic"; orthographic: GltfOrthographic };
export type GltfCamera = GltfCameraProjection & { name?: string; extensions?: GltfJson; extras?: GltfJson };

export interface GltfDocument {
  asset: GltfAsset;
  scene?: number;
  scenes?: GltfScene[];
  nodes?: GltfNode[];
  meshes?: GltfMesh[];
  accessors?: GltfAccessor[];
  bufferViews?: GltfBufferView[];
  buffers?: GltfBuffer[];
  materials?: GltfMaterial[];
  textures?: GltfTexture[];
  images?: GltfImage[];
  samplers?: GltfSampler[];
  skins?: GltfSkin[];
  animations?: GltfAnimation[];
  cameras?: GltfCamera[];
  extensionsUsed?: string[];
  extensionsRequired?: string[];
  extensions?: GltfJson;
  extras?: GltfJson;
}

export type GltfSourceForm = "json" | "glb";

export interface GltfSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ document: GltfDocument;
  /** @state artifact */ buffers: number[][];
  /** @state artifact */ sourceForm: GltfSourceForm;
}
//#endregion 🧾️Records

//#region 📥️Parsers
/** 🧩️ Decodes any JSON value into a fresh `GltfJson`, keeping member order. */
export function parseGltfJson(value: unknown, at = "$"): GltfJson {
  if (value === null || typeof value === "boolean" || typeof value === "string") return value;
  if (typeof value === "number") return gltfWireNumber(value, at);
  if (Array.isArray(value)) return value.map((entry, index) => parseGltfJson(entry, `${at}[${index}]`));
  return Object.fromEntries(Object.entries(gltfWireRecord(value, at)).map(([key, entry]) => [key, parseGltfJson(entry, `${at}.${key}`)]));
}

const json = gltfWireOptional(parseGltfJson);
const name = gltfWireOptional(gltfWireString);
const index = gltfWireRequired(gltfWireIndex);
const optionalIndex = gltfWireOptional(gltfWireIndex);
const indices = gltfWireOptional(gltfWireArray(gltfWireIndex));
const numbers = gltfWireOptional(gltfWireArray(gltfWireNumber));
const vector3 = gltfWireTuple(gltfWireNumber, gltfWireNumber, gltfWireNumber);
const vector4 = gltfWireTuple(gltfWireNumber, gltfWireNumber, gltfWireNumber, gltfWireNumber);

export const parseGltfComponentType = gltfWireLiteral(5120, 5121, 5122, 5123, 5125, 5126);
export const parseGltfAccessorType = gltfWireLiteral("SCALAR", "VEC2", "VEC3", "VEC4", "MAT2", "MAT3", "MAT4");
export const parseGltfAlphaMode = gltfWireLiteral("OPAQUE", "MASK", "BLEND");
export const parseGltfAnimationPath = gltfWireLiteral("translation", "rotation", "scale", "weights");
export const parseGltfInterpolation = gltfWireLiteral("LINEAR", "STEP", "CUBICSPLINE");
export const parseGltfSourceForm = gltfWireLiteral("json", "glb");
export const parseGltfMorphTarget = gltfWireMap(gltfWireIndex);

export const parseGltfAsset = gltfWireObject<GltfAsset>({ version: gltfWireRequired(gltfWireString), generator: name, copyright: name, minVersion: name, extensions: json, extras: json });
export const parseGltfScene = gltfWireObject<GltfScene>({ nodes: indices, name, extensions: json, extras: json });
export const parseGltfNode = gltfWireObject<GltfNode>({
  children: indices,
  mesh: optionalIndex,
  camera: optionalIndex,
  skin: optionalIndex,
  matrix: gltfWireOptional(gltfWireArray(gltfWireNumber, 16)),
  translation: gltfWireOptional(vector3),
  rotation: gltfWireOptional(vector4),
  scale: gltfWireOptional(vector3),
  weights: numbers,
  name,
  extensions: json,
  extras: json,
});
export const parseGltfPrimitive = gltfWireObject<GltfPrimitive>({
  attributes: gltfWireRequired(gltfWireMap(gltfWireIndex)),
  indices: optionalIndex,
  material: optionalIndex,
  mode: optionalIndex,
  targets: gltfWireOptional(gltfWireArray(parseGltfMorphTarget)),
  extensions: json,
  extras: json,
});
export const parseGltfMesh = gltfWireObject<GltfMesh>({ primitives: gltfWireOptional(gltfWireArray(parseGltfPrimitive)), weights: numbers, name, extensions: json, extras: json });
export const parseGltfSparseIndices = gltfWireObject<GltfSparseIndices>({ bufferView: index, byteOffset: optionalIndex, componentType: gltfWireRequired(parseGltfComponentType) });
export const parseGltfSparseValues = gltfWireObject<GltfSparseValues>({ bufferView: index, byteOffset: optionalIndex });
export const parseGltfSparseAccessor = gltfWireObject<GltfSparseAccessor>({ count: index, indices: gltfWireRequired(parseGltfSparseIndices), values: gltfWireRequired(parseGltfSparseValues) });
export const parseGltfAccessor = gltfWireObject<GltfAccessor>({
  bufferView: optionalIndex,
  byteOffset: optionalIndex,
  componentType: gltfWireRequired(parseGltfComponentType),
  normalized: gltfWireOptional(gltfWireBoolean),
  count: index,
  type: gltfWireRequired(parseGltfAccessorType),
  max: numbers,
  min: numbers,
  sparse: gltfWireOptional(parseGltfSparseAccessor),
  name,
  extensions: json,
  extras: json,
});
export const parseGltfBufferView = gltfWireObject<GltfBufferView>({
  buffer: index,
  byteOffset: optionalIndex,
  byteLength: index,
  byteStride: optionalIndex,
  target: optionalIndex,
  name,
  extensions: json,
  extras: json,
});
export const parseGltfBuffer = gltfWireObject<GltfBuffer>({ byteLength: index, uri: name, name, extensions: json, extras: json });
export const parseGltfTextureInfo = gltfWireObject<GltfTextureInfo>({ index, texCoord: optionalIndex, extensions: json, extras: json });
export const parseGltfNormalTextureInfo = gltfWireObject<GltfNormalTextureInfo>({ index, texCoord: optionalIndex, scale: gltfWireOptional(gltfWireNumber), extensions: json, extras: json });
export const parseGltfOcclusionTextureInfo = gltfWireObject<GltfOcclusionTextureInfo>({ index, texCoord: optionalIndex, strength: gltfWireOptional(gltfWireNumber), extensions: json, extras: json });
export const parseGltfPbrMetallicRoughness = gltfWireObject<GltfPbrMetallicRoughness>({
  baseColorFactor: gltfWireOptional(vector4),
  baseColorTexture: gltfWireOptional(parseGltfTextureInfo),
  metallicFactor: gltfWireOptional(gltfWireNumber),
  roughnessFactor: gltfWireOptional(gltfWireNumber),
  metallicRoughnessTexture: gltfWireOptional(parseGltfTextureInfo),
  extensions: json,
  extras: json,
});
export const parseGltfMaterial = gltfWireObject<GltfMaterial>({
  name,
  pbrMetallicRoughness: gltfWireOptional(parseGltfPbrMetallicRoughness),
  normalTexture: gltfWireOptional(parseGltfNormalTextureInfo),
  occlusionTexture: gltfWireOptional(parseGltfOcclusionTextureInfo),
  emissiveTexture: gltfWireOptional(parseGltfTextureInfo),
  emissiveFactor: gltfWireOptional(vector3),
  alphaMode: gltfWireOptional(parseGltfAlphaMode),
  alphaCutoff: gltfWireOptional(gltfWireNumber),
  doubleSided: gltfWireOptional(gltfWireBoolean),
  extensions: json,
  extras: json,
});
export const parseGltfTexture = gltfWireObject<GltfTexture>({ sampler: optionalIndex, source: optionalIndex, name, extensions: json, extras: json });
export const parseGltfImage = gltfWireObject<GltfImage>({ uri: name, mimeType: name, bufferView: optionalIndex, name, extensions: json, extras: json });
export const parseGltfSampler = gltfWireObject<GltfSampler>({ magFilter: optionalIndex, minFilter: optionalIndex, wrapS: optionalIndex, wrapT: optionalIndex, name, extensions: json, extras: json });
export const parseGltfSkin = gltfWireObject<GltfSkin>({ inverseBindMatrices: optionalIndex, skeleton: optionalIndex, joints: indices, name, extensions: json, extras: json });
export const parseGltfAnimationChannelTarget = gltfWireObject<GltfAnimationChannelTarget>({ node: optionalIndex, path: gltfWireRequired(parseGltfAnimationPath), extensions: json, extras: json });
export const parseGltfAnimationChannel = gltfWireObject<GltfAnimationChannel>({ sampler: index, target: gltfWireRequired(parseGltfAnimationChannelTarget), extensions: json, extras: json });
export const parseGltfAnimationSampler = gltfWireObject<GltfAnimationSampler>({ input: index, interpolation: gltfWireOptional(parseGltfInterpolation), output: index, extensions: json, extras: json });
export const parseGltfAnimation = gltfWireObject<GltfAnimation>({
  channels: gltfWireOptional(gltfWireArray(parseGltfAnimationChannel)),
  samplers: gltfWireOptional(gltfWireArray(parseGltfAnimationSampler)),
  name,
  extensions: json,
  extras: json,
});
export const parseGltfOrthographic = gltfWireObject<GltfOrthographic>({
  xmag: gltfWireRequired(gltfWireNumber),
  ymag: gltfWireRequired(gltfWireNumber),
  zfar: gltfWireRequired(gltfWireNumber),
  znear: gltfWireRequired(gltfWireNumber),
  extensions: json,
  extras: json,
});
export const parseGltfPerspective = gltfWireObject<GltfPerspective>({
  aspectRatio: gltfWireOptional(gltfWireNumber),
  yfov: gltfWireRequired(gltfWireNumber),
  zfar: gltfWireOptional(gltfWireNumber),
  znear: gltfWireRequired(gltfWireNumber),
  extensions: json,
  extras: json,
});
export const parseGltfCameraProjection = gltfWireTagged<GltfCameraProjection, "type">("type", {
  perspective: gltfWireObject<Extract<GltfCameraProjection, { type: "perspective" }>>({ type: gltfWireRequired(gltfWireLiteral("perspective")), perspective: gltfWireRequired(parseGltfPerspective) }),
  orthographic: gltfWireObject<Extract<GltfCameraProjection, { type: "orthographic" }>>({ type: gltfWireRequired(gltfWireLiteral("orthographic")), orthographic: gltfWireRequired(parseGltfOrthographic) }),
});
export const parseGltfCamera = gltfWireTagged<GltfCamera, "type">("type", {
  perspective: gltfWireObject<Extract<GltfCamera, { type: "perspective" }>>({ type: gltfWireRequired(gltfWireLiteral("perspective")), perspective: gltfWireRequired(parseGltfPerspective), name, extensions: json, extras: json }),
  orthographic: gltfWireObject<Extract<GltfCamera, { type: "orthographic" }>>({ type: gltfWireRequired(gltfWireLiteral("orthographic")), orthographic: gltfWireRequired(parseGltfOrthographic), name, extensions: json, extras: json }),
});
export const parseGltfDocument = gltfWireObject<GltfDocument>({
  asset: gltfWireRequired(parseGltfAsset),
  scene: optionalIndex,
  scenes: gltfWireOptional(gltfWireArray(parseGltfScene)),
  nodes: gltfWireOptional(gltfWireArray(parseGltfNode)),
  meshes: gltfWireOptional(gltfWireArray(parseGltfMesh)),
  accessors: gltfWireOptional(gltfWireArray(parseGltfAccessor)),
  bufferViews: gltfWireOptional(gltfWireArray(parseGltfBufferView)),
  buffers: gltfWireOptional(gltfWireArray(parseGltfBuffer)),
  materials: gltfWireOptional(gltfWireArray(parseGltfMaterial)),
  textures: gltfWireOptional(gltfWireArray(parseGltfTexture)),
  images: gltfWireOptional(gltfWireArray(parseGltfImage)),
  samplers: gltfWireOptional(gltfWireArray(parseGltfSampler)),
  skins: gltfWireOptional(gltfWireArray(parseGltfSkin)),
  animations: gltfWireOptional(gltfWireArray(parseGltfAnimation)),
  cameras: gltfWireOptional(gltfWireArray(parseGltfCamera)),
  extensionsUsed: gltfWireOptional(gltfWireArray(gltfWireString)),
  extensionsRequired: gltfWireOptional(gltfWireArray(gltfWireString)),
  extensions: json,
  extras: json,
});
export const parseGltfSnapshot = gltfWireObject<GltfSnapshot>({
  schema: gltfWireRequired(gltfWireString),
  document: gltfWireRequired(parseGltfDocument),
  buffers: gltfWireRequired(gltfWireArray(gltfWireArray(gltfWireByte))),
  sourceForm: gltfWireRequired(parseGltfSourceForm),
});
//#endregion 📥️Parsers

import {binary64,type Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
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
  readonly missing?: () => T;
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
export const gltfWireNumber: GltfWireReader<Binary64> = (value, at = "$") => (typeof value === "number" && Number.isFinite(value) ? binary64(value) : gltfWireRefuse(at, "value is not a finite native-wire number"));
export const gltfWireInteger = (maximum = Number.MAX_SAFE_INTEGER): GltfWireReader<number> => (value, at = "$") =>
  typeof value === "number" && Number.isSafeInteger(value) && value >= 0 && value <= maximum ? value : gltfWireRefuse(at, `value is not an integer in 0..=${maximum}`);
export const gltfWireIndex: GltfWireReader<bigint> = (value, at = "$") => BigInt(gltfWireInteger()(value,at));
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
export const gltfWireMap = <T>(read: GltfWireReader<T>): GltfWireReader<[string,T][]> => (value, at = "$") =>
  Object.entries(gltfWireRecord(value, at)).map(([key, entry]) => [key, read(entry, `${at}.${key}`)]);
export const gltfWireRequired = <T>(read: GltfWireReader<T>): GltfWireMember<T, false> => ({ read, optional: false });
export const gltfWireDefault = <T>(read: GltfWireReader<T>, missing: () => T): GltfWireMember<T,false> => ({read,optional:false,missing});
export const gltfWireOptional = <T>(read: GltfWireReader<T>): GltfWireMember<T, true> => ({ read, optional: true });
export const gltfWireObject = <T extends object>(members: GltfWireMembers<T>): GltfWireReader<T> => (value, at = "$") => {
  const row = gltfWireRecord(value, at);
  for (const key of Object.keys(row)) if (!Object.hasOwn(members, key)) gltfWireRefuse(`${at}.${key}`, "member is not part of the contract");
  return Object.fromEntries(
    Object.entries(members as Readonly<Record<string, GltfWireMember<unknown>>>).flatMap(([key, member]) =>
      Object.hasOwn(row, key) ? [[key, member.read(row[key], `${at}.${key}`)]] : member.missing ? [[key,member.missing()]] : member.optional ? [] : gltfWireRefuse(`${at}.${key}`, "required member is absent"),
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
export type GltfJson = {kind:"null"}|{kind:"boolean";value:boolean}|{kind:"number";value:Binary64}|{kind:"string";value:string}|{kind:"array";values:GltfJson[]}|{kind:"object";members:[string,GltfJson][]};

export interface GltfAsset {
  version: string;
  generator?: string;
  copyright?: string;
  minVersion?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfScene {
  nodes: bigint[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfNode {
  children: bigint[];
  mesh?: bigint;
  camera?: bigint;
  skin?: bigint;
  matrix?: Binary64[];
  translation?: [Binary64, Binary64, Binary64];
  rotation?: [Binary64, Binary64, Binary64, Binary64];
  scale?: [Binary64, Binary64, Binary64];
  weights: Binary64[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

/** 🎯️ One morph target: attribute semantic to accessor index. */
export type GltfMorphTarget = [string,bigint][];

export interface GltfPrimitive {
  attributes: [string,bigint][];
  indices?: bigint;
  material?: bigint;
  mode?: bigint;
  targets: GltfMorphTarget[];
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfMesh {
  primitives: GltfPrimitive[];
  weights: Binary64[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

/** 🔢️ `accessor.componentType` as its numeric glTF code. */
export type GltfComponentType = 5120 | 5121 | 5122 | 5123 | 5125 | 5126;
/** 🔠️ `accessor.type` as its glTF spelling. */
export type GltfAccessorType = "SCALAR" | "VEC2" | "VEC3" | "VEC4" | "MAT2" | "MAT3" | "MAT4";

export interface GltfSparseIndices {
  bufferView: bigint;
  byteOffset: bigint;
  componentType: GltfComponentType;
}
export interface GltfSparseValues {
  bufferView: bigint;
  byteOffset: bigint;
}
export interface GltfSparseAccessor {
  count: bigint;
  indices: GltfSparseIndices;
  values: GltfSparseValues;
}

export interface GltfAccessor {
  bufferView?: bigint;
  byteOffset: bigint;
  componentType: GltfComponentType;
  normalized: boolean;
  count: bigint;
  type: GltfAccessorType;
  max?: Binary64[];
  min?: Binary64[];
  sparse?: GltfSparseAccessor;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfBufferView {
  buffer: bigint;
  byteOffset: bigint;
  byteLength: bigint;
  byteStride?: bigint;
  target?: bigint;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfBuffer {
  byteLength: bigint;
  uri?: string;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfTextureInfo {
  index: bigint;
  texCoord: bigint;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfNormalTextureInfo {
  index: bigint;
  texCoord: bigint;
  scale: Binary64;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfOcclusionTextureInfo {
  index: bigint;
  texCoord: bigint;
  strength: Binary64;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfPbrMetallicRoughness {
  baseColorFactor: [Binary64, Binary64, Binary64, Binary64];
  baseColorTexture?: GltfTextureInfo;
  metallicFactor: Binary64;
  roughnessFactor: Binary64;
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
  emissiveFactor: [Binary64, Binary64, Binary64];
  alphaMode: GltfAlphaMode;
  alphaCutoff: Binary64;
  doubleSided: boolean;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfTexture {
  sampler?: bigint;
  source?: bigint;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfImage {
  uri?: string;
  mimeType?: string;
  bufferView?: bigint;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfSampler {
  magFilter?: bigint;
  minFilter?: bigint;
  wrapS: bigint;
  wrapT: bigint;
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfSkin {
  inverseBindMatrices?: bigint;
  skeleton?: bigint;
  joints: bigint[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export type GltfAnimationPath = "translation" | "rotation" | "scale" | "weights";
export interface GltfAnimationChannelTarget {
  node?: bigint;
  path: GltfAnimationPath;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfAnimationChannel {
  sampler: bigint;
  target: GltfAnimationChannelTarget;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export type GltfInterpolation = "LINEAR" | "STEP" | "CUBICSPLINE";
export interface GltfAnimationSampler {
  input: bigint;
  interpolation: GltfInterpolation;
  output: bigint;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfAnimation {
  channels: GltfAnimationChannel[];
  samplers: GltfAnimationSampler[];
  name?: string;
  extensions?: GltfJson;
  extras?: GltfJson;
}

export interface GltfOrthographic {
  xmag: Binary64;
  ymag: Binary64;
  zfar: Binary64;
  znear: Binary64;
  extensions?: GltfJson;
  extras?: GltfJson;
}
export interface GltfPerspective {
  aspectRatio?: Binary64;
  yfov: Binary64;
  zfar?: Binary64;
  znear: Binary64;
  extensions?: GltfJson;
  extras?: GltfJson;
}
/** 📷️ A camera's projection: the `type` tag beside the member it names. */
export type GltfCameraProjection = { type: "perspective"; perspective: GltfPerspective } | { type: "orthographic"; orthographic: GltfOrthographic };
export type GltfCamera = GltfCameraProjection & { name?: string; extensions?: GltfJson; extras?: GltfJson };

export interface GltfDocument {
  asset: GltfAsset;
  scene?: bigint;
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
 if(value===null)return{kind:"null"};
 if(typeof value==="boolean")return{kind:"boolean",value};
 if(typeof value==="string")return{kind:"string",value};
 if(typeof value==="number")return{kind:"number",value:gltfWireNumber(value,at)};
 if(Array.isArray(value))return{kind:"array",values:value.map((value,index)=>parseGltfJson(value,at+"["+index+"]"))};
 return{kind:"object",members:Object.entries(gltfWireRecord(value,at)).map(([key,value])=>[key,parseGltfJson(value,at+"."+key)])};
}

const json = gltfWireOptional(parseGltfJson);
const name = gltfWireOptional(gltfWireString);
const index = gltfWireRequired(gltfWireIndex);
const optionalIndex = gltfWireOptional(gltfWireIndex);
const indices = gltfWireDefault(gltfWireArray(gltfWireIndex),()=>[]);
const numbers = gltfWireDefault(gltfWireArray(gltfWireNumber),()=>[]);
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
  targets: gltfWireDefault(gltfWireArray(parseGltfMorphTarget),()=>[]),
  extensions: json,
  extras: json,
});
export const parseGltfMesh = gltfWireObject<GltfMesh>({ primitives: gltfWireDefault(gltfWireArray(parseGltfPrimitive),()=>[]), weights: numbers, name, extensions: json, extras: json });
export const parseGltfSparseIndices = gltfWireObject<GltfSparseIndices>({ bufferView: index, byteOffset: gltfWireDefault(gltfWireIndex,()=>0n), componentType: gltfWireRequired(parseGltfComponentType) });
export const parseGltfSparseValues = gltfWireObject<GltfSparseValues>({ bufferView: index, byteOffset: gltfWireDefault(gltfWireIndex,()=>0n) });
export const parseGltfSparseAccessor = gltfWireObject<GltfSparseAccessor>({ count: index, indices: gltfWireRequired(parseGltfSparseIndices), values: gltfWireRequired(parseGltfSparseValues) });
export const parseGltfAccessor = gltfWireObject<GltfAccessor>({
  bufferView: optionalIndex,
  byteOffset: gltfWireDefault(gltfWireIndex,()=>0n),
  componentType: gltfWireRequired(parseGltfComponentType),
  normalized: gltfWireDefault(gltfWireBoolean,()=>false),
  count: index,
  type: gltfWireRequired(parseGltfAccessorType),
  max: gltfWireOptional(gltfWireArray(gltfWireNumber)),
  min: gltfWireOptional(gltfWireArray(gltfWireNumber)),
  sparse: gltfWireOptional(parseGltfSparseAccessor),
  name,
  extensions: json,
  extras: json,
});
export const parseGltfBufferView = gltfWireObject<GltfBufferView>({
  buffer: index,
  byteOffset: gltfWireDefault(gltfWireIndex,()=>0n),
  byteLength: index,
  byteStride: optionalIndex,
  target: optionalIndex,
  name,
  extensions: json,
  extras: json,
});
export const parseGltfBuffer = gltfWireObject<GltfBuffer>({ byteLength: index, uri: name, name, extensions: json, extras: json });
export const parseGltfTextureInfo = gltfWireObject<GltfTextureInfo>({ index, texCoord: gltfWireDefault(gltfWireIndex,()=>0n), extensions: json, extras: json });
export const parseGltfNormalTextureInfo = gltfWireObject<GltfNormalTextureInfo>({ index, texCoord: gltfWireDefault(gltfWireIndex,()=>0n), scale: gltfWireDefault(gltfWireNumber,()=>binary64(1)), extensions: json, extras: json });
export const parseGltfOcclusionTextureInfo = gltfWireObject<GltfOcclusionTextureInfo>({ index, texCoord: gltfWireDefault(gltfWireIndex,()=>0n), strength: gltfWireDefault(gltfWireNumber,()=>binary64(1)), extensions: json, extras: json });
export const parseGltfPbrMetallicRoughness = gltfWireObject<GltfPbrMetallicRoughness>({
  baseColorFactor: gltfWireDefault(vector4,()=>[binary64(1),binary64(1),binary64(1),binary64(1)]),
  baseColorTexture: gltfWireOptional(parseGltfTextureInfo),
  metallicFactor: gltfWireDefault(gltfWireNumber,()=>binary64(1)),
  roughnessFactor: gltfWireDefault(gltfWireNumber,()=>binary64(1)),
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
  emissiveFactor: gltfWireDefault(vector3,()=>[binary64(0),binary64(0),binary64(0)]),
  alphaMode: gltfWireDefault(parseGltfAlphaMode,()=>"OPAQUE"),
  alphaCutoff: gltfWireDefault(gltfWireNumber,()=>binary64(0.5)),
  doubleSided: gltfWireDefault(gltfWireBoolean,()=>false),
  extensions: json,
  extras: json,
});
export const parseGltfTexture = gltfWireObject<GltfTexture>({ sampler: optionalIndex, source: optionalIndex, name, extensions: json, extras: json });
export const parseGltfImage = gltfWireObject<GltfImage>({ uri: name, mimeType: name, bufferView: optionalIndex, name, extensions: json, extras: json });
export const parseGltfSampler = gltfWireObject<GltfSampler>({ magFilter: optionalIndex, minFilter: optionalIndex, wrapS: gltfWireDefault(gltfWireIndex,()=>10497n), wrapT: gltfWireDefault(gltfWireIndex,()=>10497n), name, extensions: json, extras: json });
export const parseGltfSkin = gltfWireObject<GltfSkin>({ inverseBindMatrices: optionalIndex, skeleton: optionalIndex, joints: indices, name, extensions: json, extras: json });
export const parseGltfAnimationChannelTarget = gltfWireObject<GltfAnimationChannelTarget>({ node: optionalIndex, path: gltfWireRequired(parseGltfAnimationPath), extensions: json, extras: json });
export const parseGltfAnimationChannel = gltfWireObject<GltfAnimationChannel>({ sampler: index, target: gltfWireRequired(parseGltfAnimationChannelTarget), extensions: json, extras: json });
export const parseGltfAnimationSampler = gltfWireObject<GltfAnimationSampler>({ input: index, interpolation: gltfWireDefault(parseGltfInterpolation,()=>"LINEAR"), output: index, extensions: json, extras: json });
export const parseGltfAnimation = gltfWireObject<GltfAnimation>({
  channels: gltfWireDefault(gltfWireArray(parseGltfAnimationChannel),()=>[]),
  samplers: gltfWireDefault(gltfWireArray(parseGltfAnimationSampler),()=>[]),
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
  scenes: gltfWireDefault(gltfWireArray(parseGltfScene),()=>[]),
  nodes: gltfWireDefault(gltfWireArray(parseGltfNode),()=>[]),
  meshes: gltfWireDefault(gltfWireArray(parseGltfMesh),()=>[]),
  accessors: gltfWireDefault(gltfWireArray(parseGltfAccessor),()=>[]),
  bufferViews: gltfWireDefault(gltfWireArray(parseGltfBufferView),()=>[]),
  buffers: gltfWireDefault(gltfWireArray(parseGltfBuffer),()=>[]),
  materials: gltfWireDefault(gltfWireArray(parseGltfMaterial),()=>[]),
  textures: gltfWireDefault(gltfWireArray(parseGltfTexture),()=>[]),
  images: gltfWireDefault(gltfWireArray(parseGltfImage),()=>[]),
  samplers: gltfWireDefault(gltfWireArray(parseGltfSampler),()=>[]),
  skins: gltfWireDefault(gltfWireArray(parseGltfSkin),()=>[]),
  animations: gltfWireDefault(gltfWireArray(parseGltfAnimation),()=>[]),
  cameras: gltfWireDefault(gltfWireArray(parseGltfCamera),()=>[]),
  extensionsUsed: gltfWireDefault(gltfWireArray(gltfWireString),()=>[]),
  extensionsRequired: gltfWireDefault(gltfWireArray(gltfWireString),()=>[]),
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

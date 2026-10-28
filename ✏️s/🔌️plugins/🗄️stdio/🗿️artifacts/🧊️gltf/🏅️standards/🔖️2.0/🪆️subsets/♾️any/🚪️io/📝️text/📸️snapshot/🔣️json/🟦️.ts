import {binary64,type Binary64} from "@semio-tech/framework/value/ieee754";
import type {JsonSyntaxNode,JsonSyntaxMember} from '@semio-tech/pack-json-decode-ts';
import type {GltfJson,GltfAsset,GltfScene,GltfNode,GltfMorphTarget,GltfPrimitive,GltfMesh,GltfComponentType,GltfAccessorType,GltfSparseIndices,GltfSparseValues,GltfSparseAccessor,GltfAccessor,GltfBufferView,GltfBuffer,GltfTextureInfo,GltfNormalTextureInfo,GltfOcclusionTextureInfo,GltfPbrMetallicRoughness,GltfAlphaMode,GltfMaterial,GltfTexture,GltfImage,GltfSampler,GltfSkin,GltfAnimationPath,GltfAnimationChannelTarget,GltfAnimationChannel,GltfInterpolation,GltfAnimationSampler,GltfAnimation,GltfOrthographic,GltfPerspective,GltfCameraProjection,GltfCamera,GltfDocument,GltfSourceForm,GltfSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
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

export type GltfSyntaxProjectionStep=void|{readonly kind:'number';readonly codeUnits:number;readonly at:string};
type SyntaxReader<T>=(node:JsonSyntaxNode,at:string)=>Generator<GltfSyntaxProjectionStep,T>;
const syntaxReaders=new WeakMap<GltfWireReader<unknown>,SyntaxReader<unknown>>();
function withSyntax<T>(read:GltfWireReader<T>,steps:SyntaxReader<T>):GltfWireReader<T>{syntaxReaders.set(read,steps);return read;}
function* readSyntax<T>(read:GltfWireReader<T>,node:JsonSyntaxNode,at:string):Generator<GltfSyntaxProjectionStep,T>{
 yield node.kind==='number'?{kind:'number',codeUnits:node.text.length,at}:undefined;const steps=syntaxReaders.get(read);if(steps)return (yield* steps(node,at)) as T;
 const value=node.kind==='null'?null:node.kind==='number'?Number(node.text):node.kind==='string'||node.kind==='boolean'?node.value:node.kind==='array'?[]:Object.create(null);
 return read(value,at);
}
function syntaxRecord(node:JsonSyntaxNode,at:string):readonly JsonSyntaxMember[]{return node.kind==='object'?node.members:gltfWireRefuse(at,'value is not an object');}
function arrayIndex(key:string):number|undefined{if(key.length>10||!/^(0|[1-9][0-9]*)$/u.test(key))return undefined;const index=Number(key);return index<0xffffffff?index:undefined;}
function* orderedSyntaxMembers(members:readonly JsonSyntaxMember[]):Generator<void,readonly JsonSyntaxMember[]>{
 const indices:{index:number;member:JsonSyntaxMember}[]=[],rest:JsonSyntaxMember[]=[];
 for(const member of members){yield;const index=arrayIndex(member.name);if(index===undefined)rest.push(member);else indices.push({index,member});}
 function* sift(root:number,end:number):Generator<void>{for(;;){yield;const left=root*2+1;if(left>=end)return;const right=left+1,child=right<end&&indices[right]!.index>indices[left]!.index?right:left;if(indices[root]!.index>=indices[child]!.index)return;[indices[root],indices[child]]=[indices[child]!,indices[root]!];root=child;}}
 for(let root=Math.floor(indices.length/2)-1;root>=0;root--)yield* sift(root,indices.length);
 for(let end=indices.length-1;end>0;end--){yield;[indices[0],indices[end]]=[indices[end]!,indices[0]!];yield* sift(0,end);}
 const ordered:JsonSyntaxMember[]=[];for(const row of indices){yield;ordered.push(row.member);}for(const member of rest){yield;ordered.push(member);}return ordered;
}

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
export const gltfWireNullable = <T>(read: GltfWireReader<T>): GltfWireReader<T | null> => withSyntax((value, at = "$") => (value === null ? null : read(value, at)),function*(node,at){return node.kind==='null'?null:yield* readSyntax(read,node,at);});
export const gltfWireArray = <T>(item: GltfWireReader<T>, length?: number): GltfWireReader<T[]> => withSyntax((value, at = "$") => {
  if (!Array.isArray(value)) return gltfWireRefuse(at, "value is not an array");
  if (length !== undefined && value.length !== length) gltfWireRefuse(at, `array does not hold exactly ${length} items`);
  return value.map((entry, index) => item(entry, `${at}[${index}]`));
},function*(node,at){if(node.kind!=='array')return gltfWireRefuse(at,'value is not an array');if(length!==undefined&&node.items.length!==length)return gltfWireRefuse(at,`array does not hold exactly ${length} items`);const result:T[]=[];for(let index=0;index<node.items.length;index++)result.push(yield* readSyntax(item,node.items[index]!,`${at}[${index}]`));return result;});
export const gltfWireTuple = <T extends readonly unknown[]>(...items: { readonly [K in keyof T]: GltfWireReader<T[K]> }): GltfWireReader<[...T]> => withSyntax((value, at = "$") => {
  if (!Array.isArray(value) || value.length !== items.length) return gltfWireRefuse(at, `value is not an array of exactly ${items.length} items`);
  return value.map((entry, index) => (items[index] as GltfWireReader<unknown>)(entry, `${at}[${index}]`)) as [...T];
},function*(node,at){if(node.kind!=='array'||node.items.length!==items.length)return gltfWireRefuse(at,`value is not an array of exactly ${items.length} items`);const result:unknown[]=[];for(let index=0;index<items.length;index++)result.push(yield* readSyntax(items[index]!,node.items[index]!,`${at}[${index}]`));return result as [...T];});
export const gltfWireMap = <T>(read: GltfWireReader<T>): GltfWireReader<[string,T][]> => withSyntax((value, at = "$") =>
  Object.entries(gltfWireRecord(value, at)).map(([key, entry]) => [key, read(entry, `${at}.${key}`)]),function*(node,at){const members=yield* orderedSyntaxMembers(syntaxRecord(node,at)),result:[string,T][]=[];for(const member of members)result.push([member.name,yield* readSyntax(read,member.value,`${at}.${member.name}`)]);return result;});
export const gltfWireRequired = <T>(read: GltfWireReader<T>): GltfWireMember<T, false> => ({ read, optional: false });
export const gltfWireDefault = <T>(read: GltfWireReader<T>, missing: () => T): GltfWireMember<T,false> => ({read,optional:false,missing});
export const gltfWireOptional = <T>(read: GltfWireReader<T>): GltfWireMember<T, true> => ({ read, optional: true });
export const gltfWireObject = <T extends object>(members: GltfWireMembers<T>): GltfWireReader<T> => withSyntax((value, at = "$") => {
  const row = gltfWireRecord(value, at);
  for (const key of Object.keys(row)) if (!Object.hasOwn(members, key)) gltfWireRefuse(`${at}.${key}`, "member is not part of the contract");
  return Object.fromEntries(
    Object.entries(members as Readonly<Record<string, GltfWireMember<unknown>>>).flatMap(([key, member]) =>
      Object.hasOwn(row, key) ? [[key, member.read(row[key], `${at}.${key}`)]] : member.missing ? [[key,member.missing()]] : member.optional ? [] : gltfWireRefuse(`${at}.${key}`, "required member is absent"),
    ),
  ) as T;
},function*(node,at){
 const row=new Map<string,JsonSyntaxNode>();for(const member of yield* orderedSyntaxMembers(syntaxRecord(node,at))){yield;if(!Object.hasOwn(members,member.name))return gltfWireRefuse(`${at}.${member.name}`,'member is not part of the contract');row.set(member.name,member.value);}
 const result={};for(const [key,member] of Object.entries(members as Readonly<Record<string,GltfWireMember<unknown>>>)){yield;const node=row.get(key);if(node===undefined&&!member.missing&&member.optional)continue;const value=node!==undefined?yield* readSyntax(member.read,node,`${at}.${key}`):member.missing?member.missing():gltfWireRefuse(`${at}.${key}`,'required member is absent');Object.defineProperty(result,key,{value,enumerable:true,writable:true,configurable:true});}return result as T;
});
export const gltfWireTagged = <T extends object, K extends keyof T & string>(tag: K, variants: GltfWireVariants<T, K>): GltfWireReader<T> => withSyntax((value, at = "$") => {
  const key = gltfWireRecord(value, at)[tag];
  const table = variants as Readonly<Record<string, GltfWireReader<T>>>;
  return (typeof key === "string" && Object.hasOwn(table, key) ? table[key]! : gltfWireRefuse(`${at}.${tag}`, `value is not one of ${Object.keys(table).join(", ")}`))(value, at);
},function*(node,at){let key:JsonSyntaxNode|undefined;for(const member of syntaxRecord(node,at)){yield;if(member.name===tag)key=member.value;}const table=variants as Readonly<Record<string,GltfWireReader<T>>>;return key?.kind==='string'&&Object.hasOwn(table,key.value)?yield* readSyntax(table[key.value]!,node,at):gltfWireRefuse(`${at}.${tag}`,`value is not one of ${Object.keys(table).join(', ')}`);});
//#endregion 🚪️Wire

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
withSyntax(parseGltfJson,function*(node,at):Generator<GltfSyntaxProjectionStep,GltfJson>{
 if(node.kind==='null')return {kind:'null'};if(node.kind==='boolean'||node.kind==='string')return {kind:node.kind,value:node.value} as GltfJson;if(node.kind==='number')return {kind:'number',value:gltfWireNumber(Number(node.text),at)};
 if(node.kind==='array'){const values:GltfJson[]=[];for(let index=0;index<node.items.length;index++)values.push(yield* readSyntax(parseGltfJson,node.items[index]!,`${at}[${index}]`));return {kind:'array',values};}
 const members:[string,GltfJson][]=[];for(const member of yield* orderedSyntaxMembers(node.members))members.push([member.name,yield* readSyntax(parseGltfJson,member.value,`${at}.${member.name}`)]);return {kind:'object',members};
});

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

/** 🧵️ Project owned lossless syntax with the same wire declarations, yielding each node and ordering transition. */
export function* parseGltfDocumentSyntaxSteps(node:JsonSyntaxNode):Generator<GltfSyntaxProjectionStep,GltfDocument>{return yield* readSyntax(parseGltfDocument,node,'$');}
//#endregion 📥️Parsers

export type * from "../../../../🧬️schema/📸️snapshot/🟦️.ts";

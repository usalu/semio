/** 🔣️ Literal shared Block transport fields with exact floating words. */
import * as m from "../../🟦️.ts";
import {binary64,parseBinary64,type Binary64} from "../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export type BlockJsonRow=Record<string,unknown>;
export const reject=(why:string):never=>{throw new Error("Block transport "+why)};
export const row=(v:unknown):BlockJsonRow=>v!==null&&typeof v==="object"&&!Array.isArray(v)?v as BlockJsonRow:reject("object required");
export const text=(v:unknown):string=>typeof v==="string"?v:reject("TEXT required");
export const defaultText=(v:unknown):string=>v===undefined?"":text(v);
export const optionalText=(v:unknown):string|null=>v===null||v===undefined?null:text(v);
export const list=<T>(v:unknown,read:(v:unknown)=>T):T[]=>v===undefined?[]:Array.isArray(v)?v.map(read):reject("array required");
export const bool=(v:unknown):boolean=>v===undefined?false:typeof v==="boolean"?v:reject("Boolean required");
/** 🔢️ Admit only the declared closed word or finite JSON number. */
export function word(v:unknown):Binary64{if(typeof v==="number")return Number.isFinite(v)?binary64(v):reject("finite numeric input required");const r=row(v);if(Object.keys(r).length!==1||typeof r.bits!=="string"||!/^[0-9a-f]{16}$/.test(r.bits))return reject("closed IEEE word required");return{bits:BigInt("0x"+r.bits)}}
export const floating=(v:unknown):Binary64=>v===undefined?binary64(0):word(v);
export const optionalWord=(v:unknown):Binary64|null=>v===null||v===undefined?null:word(v);
/** 📐️ Admit each exact coordinate and declared zero default. */
export function xyz(v:unknown,defaults=false):[Binary64,Binary64,Binary64]{if(v===undefined&&defaults)return[binary64(0),binary64(0),binary64(0)];if(!Array.isArray(v)||v.length!==3)return reject("vector3 required");return[word(v[0]),word(v[1]),word(v[2])]}
/** 🪪️ Construct all literal kind identity fields. */
export function kind(v:unknown):m.BlockKindIdentity{const r=row(v);return{id:text(r.id),name:text(r.name),label:text(r.label),variant:optionalText(r.variant),description:defaultText(r.description),icon:optionalText(r.icon),unit:optionalText(r.unit)}}
/** 🏷️ Construct every literal attribute field. */
export function attribute(v:unknown):m.BlockAttribute{const r=row(v);return{key:text(r.key),value:text(r.value),definition:optionalText(r.definition)}}
/** 👤️ Construct every literal author field. */
export function author(v:unknown):m.BlockAuthor{const r=row(v);return{id:text(r.id),name:text(r.name),email:optionalText(r.email)}}
/** 🔗️ Construct unresolved native compatibility references literally. */
export function compatible(v:unknown):m.BlockCompatibilityRule{const r=row(v);return{id:text(r.id),source:text(r.source),target:text(r.target),bidirectional:bool(r.bidirectional)}}
/** 🧱️ Construct literal representation ownership and ordered members. */
export function representation(v:unknown):m.BlockRepresentation{const r=row(v);return{id:text(r.id),name:text(r.name),meshUrl:optionalText(r.meshUrl),tags:list(r.tags,text),lod:optionalText(r.lod),description:defaultText(r.description),attributes:list(r.attributes,attribute)}}
/** 🎥️ Construct actual 2d camera defaults and exact scalar words. */
export function camera2d(v:unknown):m.BlockCamera2d{const r=v===undefined?{}:row(v);return{x:floating(r.x),y:floating(r.y),zoom:r.zoom===undefined?binary64(1):word(r.zoom)}}
/** 🎥️ Construct actual 3d camera defaults and exact coordinate words. */
export function camera3d(v:unknown):m.BlockCamera3d{const r=v===undefined?{}:row(v);return{position:xyz(r.position,true),target:xyz(r.target,true),zoom:r.zoom===undefined?binary64(1):word(r.zoom)}}
export const out=(v:Binary64):{bits:string}=>({bits:parseBinary64(v).bits.toString(16).padStart(16,"0")});
export const optionalOut=(v:Binary64|null):{bits:string}|null=>v===null?null:out(v);
export const xyzOut=(v:readonly Binary64[]):{bits:string}[]=>[out(v[0]!),out(v[1]!),out(v[2]!)];
/** 📝️ Admit the declared DSL floating spelling without numeric NaN canonicalization. */
export function wordFromDsl(literal:string):Binary64{
 if(/^nan64_[0-9a-f]{16}$/.test(literal)){const bits=BigInt("0x"+literal.slice(6));if((bits&0x7ff0000000000000n)!==0x7ff0000000000000n||(bits&0xfffffffffffffn)===0n)return reject("NaN word required");return{bits}}
 if(literal==="inf")return{bits:0x7ff0000000000000n};
 if(literal==="-inf")return{bits:0xfff0000000000000n};
 if(literal==="nan")return{bits:0x7ff8000000000000n};
 if(!/^[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?$/.test(literal))return reject("floating literal required");
 const value=Number(literal);return Number.isFinite(value)?binary64(value):reject("finite decimal required");
}


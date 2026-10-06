/** 📐️ Literal Block-owned primitive admission shared by its real document records. */
import {type Binary64,parseBinary64} from "../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {textUtf8ByteLength} from "../../../../../../🧰️framework/🔨️modules/🌱️value/📝️text/🟦️.ts";
export type BlockVector3=[Binary64,Binary64,Binary64];
export type BlockVector4=[Binary64,Binary64,Binary64,Binary64];
export const fail=(why:string):never=>{throw new Error("Block "+why)};
export const row=(v:unknown):Record<string,unknown>=>v!==null&&typeof v==="object"&&!Array.isArray(v)?v as Record<string,unknown>:fail("object required");
export function text(v:unknown):string{if(typeof v!=="string")return fail("well-formed UTF-16 TEXT required");textUtf8ByteLength(v);return v}
export const boolean=(v:unknown):boolean=>typeof v==="boolean"?v:fail("Boolean required");
export const word=(v:unknown):Binary64=>parseBinary64(v);
export const optional=<T>(v:unknown,parse:(v:unknown)=>T):T|null=>v===null?null:parse(v);
export const list=<T>(v:unknown,parse:(v:unknown)=>T):T[]=>Array.isArray(v)?v.map(parse):fail("array required");
export function vector3(v:unknown):BlockVector3{if(!Array.isArray(v)||v.length!==3)return fail("three words required");return[word(v[0]),word(v[1]),word(v[2])]}
export function vector4(v:unknown):BlockVector4{if(!Array.isArray(v)||v.length!==4)return fail("four words required");return[word(v[0]),word(v[1]),word(v[2]),word(v[3])]}

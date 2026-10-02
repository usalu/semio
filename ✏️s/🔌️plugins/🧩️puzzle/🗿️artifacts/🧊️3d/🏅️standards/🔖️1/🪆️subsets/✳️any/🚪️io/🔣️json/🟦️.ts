/** 🔣️ Declared Puzzle3d JSON transport with literal complete native fields. */
import * as model from "../../🧬️schema/📸️snapshot/🟦️.ts";
import {binary64,parseBinary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
type Row=Record<string,unknown>;
const fail=(why:string):never=>{throw new Error("Puzzle3d JSON "+why)};
const record=(v:unknown):Row=>v!==null&&typeof v==="object"&&!Array.isArray(v)?v as Row:fail("object required");
const defaultRecord=(v:unknown,out:boolean):Row=>v===undefined&&!out?{}:record(v);
const scalar=(v:unknown,out:boolean,optional=false):unknown=>{
 if(optional&&(v===null||v===undefined&&!out))return null;
 if(out)return{bits:parseBinary64(v).bits.toString(16).padStart(16,"0")};
 if(v===undefined)return binary64(0);
 if(typeof v==="number")return Number.isFinite(v)?binary64(v):fail("finite numeric transport required");
 const row=record(v);if(Object.keys(row).length!==1||typeof row.bits!=="string"||!/^[0-9a-f]{16}$/.test(row.bits))return fail("closed binary64 word required");return{bits:BigInt("0x"+row.bits)};
};
function axes(v:unknown,n:3|4,out:boolean,optional=false,defaults:readonly number[]=[0,0,0]):unknown{
 if(optional&&(v===null||v===undefined&&!out))return null;
 if(v===undefined&&!out)v=defaults;
 if(!Array.isArray(v)||v.length!==n)return fail("vector width");return v.map(x=>scalar(x,out));
}
const scale=(v:unknown,out:boolean):unknown=>v===null||v===undefined&&!out?null:Array.isArray(v)?axes(v,3,out):scalar(v,out);
const list=(v:unknown,fn:(value:unknown,out:boolean)=>unknown,out:boolean):unknown[]=>v===undefined&&!out?[]:Array.isArray(v)?v.map(x=>fn(x,out)):fail("array required");
const nullable=(v:unknown,out:boolean):unknown=>v===undefined&&!out?null:v;
const defaultText=(v:unknown,out:boolean):unknown=>v===undefined&&!out?"":v;
const defaultBool=(v:unknown,out:boolean):unknown=>v===undefined&&!out?false:v;
const literal=(v:unknown):unknown=>v;
function vortex(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,vortexKind:nullable(r.vortexKind,out),label:nullable(r.label,out),position:axes(r.position,3,out),direction:axes(r.direction,3,out,true),radius:scalar(r.radius,out,true),hidden:defaultBool(r.hidden,out),locked:defaultBool(r.locked,out)}}
function object(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,label:nullable(r.label,out),objectKind:nullable(r.objectKind,out),anchor:r.anchor===undefined&&!out?"fixed":r.anchor,origin:axes(r.origin,3,out),orientation:axes(r.orientation,4,out,true),scale:scale(r.scale,out),meshUrl:nullable(r.meshUrl,out),vortices:list(r.vortices,vortex,out),hidden:defaultBool(r.hidden,out),locked:defaultBool(r.locked,out)}}
function attraction(value:unknown,out:boolean):unknown{const r=record(value);return{id:defaultText(r.id,out),attracting:r.attracting,attracted:r.attracted,gap:scalar(r.gap,out),shift:scalar(r.shift,out),rise:scalar(r.rise,out),rotation:scalar(r.rotation,out),turn:scalar(r.turn,out),tilt:scalar(r.tilt,out),x:scalar(r.x,out),y:scalar(r.y,out)}}
function target(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,origin:axes(r.origin,3,out),orientation:axes(r.orientation,4,out,true),scale:scale(r.scale,out),hidden:defaultBool(r.hidden,out),locked:defaultBool(r.locked,out)}}
function reference(value:unknown,out:boolean):unknown{const r=record(value),source=defaultRecord(r.source,out);return{id:r.id,source:{url:defaultText(source.url,out),mediaKind:nullable(source.mediaKind,out)},origin:axes(r.origin,3,out),widthWorld:scalar(r.widthWorld,out),hidden:defaultBool(r.hidden,out),locked:defaultBool(r.locked,out)}}
function compatibility(value:unknown,out:boolean):unknown{const r=record(value);return{source:r.source,target:r.target,bidirectional:defaultBool(r.bidirectional,out),important:defaultBool(r.important,out),specificity:r.specificity===undefined&&!out?"vortex":r.specificity}}
function attribute(value:unknown,out:boolean):unknown{const r=record(value);return{id:defaultText(r.id,out),key:defaultText(r.key,out),value:defaultText(r.value,out),definition:nullable(r.definition,out)}}
function author(value:unknown,out:boolean):unknown{const r=record(value);return{id:defaultText(r.id,out),name:defaultText(r.name,out),email:defaultText(r.email,out),role:nullable(r.role,out),rank:nullable(r.rank,out)}}
function representation(value:unknown,out:boolean):unknown{const r=record(value);return{id:defaultText(r.id,out),name:defaultText(r.name,out),url:defaultText(r.url,out),mime:defaultText(r.mime,out),tags:list(r.tags,literal,out),lod:nullable(r.lod,out),description:defaultText(r.description,out)}}
function template(value:unknown,out:boolean):unknown{const r=record(value);return{id:defaultText(r.id,out),name:defaultText(r.name,out),label:defaultText(r.label,out),description:defaultText(r.description,out),icon:defaultText(r.icon,out),vortexKind:nullable(r.vortexKind,out),point:axes(r.point,3,out),direction:axes(r.direction,3,out,false,[0,0,1]),t:scalar(r.t,out,true),mandatory:nullable(r.mandatory,out),radius:scalar(r.radius,out,true)}}
function objectKind(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,name:defaultText(r.name,out),label:defaultText(r.label,out),description:defaultText(r.description,out),icon:defaultText(r.icon,out),image:defaultText(r.image,out),unit:defaultText(r.unit,out),abstract:defaultBool(r.abstract,out),baseKinds:list(r.baseKinds,literal,out),representations:list(r.representations,representation,out),vortices:list(r.vortices,template,out),attributes:list(r.attributes,attribute,out),authors:list(r.authors,author,out)}}
function vortexKind(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,code:nullable(r.code,out),label:nullable(r.label,out),order:nullable(r.order,out),compatibleWith:list(r.compatibleWith,literal,out),description:defaultText(r.description,out),icon:defaultText(r.icon,out),color:defaultText(r.color,out),defaultCableKind:defaultText(r.defaultCableKind,out)}}
function cableKind(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,label:defaultText(r.label,out),name:defaultText(r.name,out),defaultAttractionKind:defaultText(r.defaultAttractionKind,out)}}
function attractionKind(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,label:defaultText(r.label,out),name:defaultText(r.name,out)}}
function catalog(value:unknown,out:boolean):unknown{if(value===null||value===undefined&&!out)return null;const r=record(value);return{objects:list(r.objects,objectKind,out),vortices:list(r.vortices,vortexKind,out),cables:list(r.cables,cableKind,out),attractions:list(r.attractions,attractionKind,out)}}
function snapshot(value:unknown,out:boolean):unknown{const r=record(value),meta=defaultRecord(r.meta,out);return{schema:r.schema,domain:defaultText(r.domain,out),meta:{kindCatalogs:catalog(meta.kindCatalogs,out),kindCompatibility:list(meta.kindCompatibility,compatibility,out)},objects:list(r.objects,object,out),attractions:list(r.attractions,attraction,out),targetVolumes:list(r.targetVolumes,target,out),references:list(r.references,reference,out)}}
/** 📥️ Decode exact word JSON or the explicitly declared finite numeric file input. */
export function puzzle3dSnapshotFromJsonText(text:string):model.Puzzle3dSnapshot{return model.parsePuzzle3dSnapshot(snapshot(JSON.parse(text),false))}
/** 📤️ Emit every native field through the declared complete canonical word boundary. */
export function puzzle3dSnapshotToJsonText(value:model.Puzzle3dSnapshot):string{return JSON.stringify(snapshot(model.parsePuzzle3dSnapshot(value),true),null,2)}

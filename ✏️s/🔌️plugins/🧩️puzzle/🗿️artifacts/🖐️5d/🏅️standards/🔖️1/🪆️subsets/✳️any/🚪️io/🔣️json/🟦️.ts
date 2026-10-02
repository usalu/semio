/** 🔣️ Declared Puzzle5d JSON transport with literal native field bindings. */
import * as model from "../../🧬️schema/📸️snapshot/🟦️.ts";
import {binary64,parseBinary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
type Row=Record<string,unknown>;
const fail=(why:string):never=>{throw new Error("Puzzle5d JSON "+why)};
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
function board(value:unknown,out:boolean):unknown{const r=defaultRecord(value,out);return{x:scalar(r.x,out),y:scalar(r.y,out),shape:nullable(r.shape,out),radius:scalar(r.radius,out,true),width:scalar(r.width,out,true),height:scalar(r.height,out,true),text:nullable(r.text,out),iconKind:nullable(r.iconKind,out),hidden:nullable(r.hidden,out),locked:nullable(r.locked,out)}}
function world(value:unknown,out:boolean):unknown{const r=defaultRecord(value,out);return{origin:axes(r.origin,3,out),meshUrl:nullable(r.meshUrl,out),orientation:axes(r.orientation,4,out,true),scale:scale(r.scale,out),label:nullable(r.label,out)}}
function gripBoard(value:unknown,out:boolean):unknown{const r=defaultRecord(value,out);return{angle:scalar(r.angle,out),gripKind:nullable(r.gripKind,out),radius:scalar(r.radius,out,true)}}
function gripWorld(value:unknown,out:boolean):unknown{const r=defaultRecord(value,out);return{position:axes(r.position,3,out),direction:axes(r.direction,3,out,true),radius:scalar(r.radius,out,true),label:nullable(r.label,out)}}
function grip(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,gripKind:nullable(r.gripKind,out),"2d":gripBoard(r["2d"],out),"3d":gripWorld(r["3d"],out)}}
function part(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,partKind:nullable(r.partKind,out),anchor:r.anchor===undefined&&!out?"fixed":r.anchor,"2d":board(r["2d"],out),"3d":world(r["3d"],out),grips:list(r.grips,grip,out)}}
function fastener(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,source:r.source,target:r.target,fastenerKind:nullable(r.fastenerKind,out),gap:scalar(r.gap,out),shift:scalar(r.shift,out),rise:scalar(r.rise,out),rotation:scalar(r.rotation,out),turn:scalar(r.turn,out),tilt:scalar(r.tilt,out),x:scalar(r.x,out),y:scalar(r.y,out)}}
function target(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,origin:axes(r.origin,3,out),orientation:axes(r.orientation,4,out,true),scale:scale(r.scale,out),hidden:defaultBool(r.hidden,out),locked:defaultBool(r.locked,out)}}
function compatibility(value:unknown,out:boolean):unknown{const r=record(value);return{source:r.source,target:r.target,bidirectional:defaultBool(r.bidirectional,out),important:defaultBool(r.important,out),specificity:r.specificity===undefined&&!out?"general":r.specificity}}
function attribute(value:unknown,out:boolean):unknown{const r=record(value);return{id:defaultText(r.id,out),key:defaultText(r.key,out),value:defaultText(r.value,out),definition:nullable(r.definition,out)}}
function author(value:unknown,out:boolean):unknown{const r=record(value);return{id:defaultText(r.id,out),name:defaultText(r.name,out),email:defaultText(r.email,out),role:nullable(r.role,out),rank:nullable(r.rank,out)}}
function representation(value:unknown,out:boolean):unknown{const r=record(value);return{id:defaultText(r.id,out),name:defaultText(r.name,out),url:defaultText(r.url,out),mime:defaultText(r.mime,out),tags:list(r.tags,literal,out),lod:nullable(r.lod,out),description:defaultText(r.description,out)}}
function template(value:unknown,out:boolean):unknown{const r=record(value);return{id:defaultText(r.id,out),name:defaultText(r.name,out),label:defaultText(r.label,out),description:defaultText(r.description,out),icon:defaultText(r.icon,out),gripKind:nullable(r.gripKind,out),point:axes(r.point,3,out),direction:axes(r.direction,3,out,false,[0,0,1]),t:scalar(r.t,out,true),mandatory:nullable(r.mandatory,out),radius:scalar(r.radius,out,true)}}
function partKind(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,name:defaultText(r.name,out),label:defaultText(r.label,out),description:defaultText(r.description,out),icon:defaultText(r.icon,out),image:defaultText(r.image,out),unit:defaultText(r.unit,out),abstract:defaultBool(r.abstract,out),baseKinds:list(r.baseKinds,literal,out),representations:list(r.representations,representation,out),grips:list(r.grips,template,out),attributes:list(r.attributes,attribute,out),authors:list(r.authors,author,out)}}
function gripKind(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,code:nullable(r.code,out),label:nullable(r.label,out),order:nullable(r.order,out),compatibleWith:list(r.compatibleWith,literal,out),description:defaultText(r.description,out),icon:defaultText(r.icon,out),color:defaultText(r.color,out),defaultRopeKind:defaultText(r.defaultRopeKind,out)}}
function fastenerKind(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,name:defaultText(r.name,out),label:nullable(r.label,out)}}
function ropeKind(value:unknown,out:boolean):unknown{const r=record(value);return{id:r.id,name:defaultText(r.name,out),label:defaultText(r.label,out),defaultFastenerKind:defaultText(r.defaultFastenerKind,out)}}
function extra(value:unknown,out:boolean):unknown{if(value===null||value===undefined&&!out)return null;const r=record(value);return{parts:list(r.parts,partKind,out),grips:list(r.grips,gripKind,out),fasteners:list(r.fasteners,fastenerKind,out),ropes:list(r.ropes,ropeKind,out)}}
function snapshot(value:unknown,out:boolean):unknown{const r=record(value),meta=defaultRecord(r.meta,out);return{schema:r.schema,domain:defaultText(r.domain,out),label:nullable(r.label,out),meta:{description:defaultText(meta.description,out)},kindCatalogs:nullable(r.kindCatalogs,out),kindCatalogsExtra:extra(r.kindCatalogsExtra,out),kindCompatibility:list(r.kindCompatibility,compatibility,out),parts:list(r.parts,part,out),fasteners:list(r.fasteners,fastener,out),targetVolumes:list(r.targetVolumes,target,out)}}
/** 📥️ Decode exact word JSON or explicitly admitted finite numeric file input. */
export function puzzle5dSnapshotFromJsonText(text:string):model.Puzzle5dSnapshot{return model.parsePuzzle5dSnapshot(snapshot(JSON.parse(text),false))}
/** 📤️ Emit the complete canonical word representation through the declared JSON file boundary. */
export function puzzle5dSnapshotToJsonText(value:model.Puzzle5dSnapshot):string{return JSON.stringify(snapshot(model.parsePuzzle5dSnapshot(value),true),null,2)}

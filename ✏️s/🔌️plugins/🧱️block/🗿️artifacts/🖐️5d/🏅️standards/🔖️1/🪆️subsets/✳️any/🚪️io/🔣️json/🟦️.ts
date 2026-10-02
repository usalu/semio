/** 🔣️ Declared JSON transport of exact Block5d canonical fields. */
import * as m from "../../🧬️schema/📸️snapshot/🟦️.ts";
import {binary64,parseBinary64,type Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
type Row=Record<string,unknown>;
const row=(v:unknown):Row=>v!==null&&typeof v==="object"&&!Array.isArray(v)?v as Row:fail("object required");
const fail=(why:string):never=>{throw new Error("Block5d JSON "+why)};
const text=(v:unknown):string=>typeof v==="string"?v:fail("TEXT required");
const string=(v:unknown):string=>v===undefined?"":text(v);
const optionalText=(v:unknown):string|null=>v===null||v===undefined?null:text(v);
const list=<T>(v:unknown,read:(v:unknown)=>T):T[]=>v===undefined?[]:Array.isArray(v)?v.map(read):fail("array required");
const bool=(v:unknown):boolean=>v===undefined?false:typeof v==="boolean"?v:fail("Boolean required");
function word(v:unknown):Binary64{if(typeof v==="number")return Number.isFinite(v)?binary64(v):fail("finite numeric input required");const r=row(v);if(Object.keys(r).length!==1||typeof r.bits!=="string"||!/^[0-9a-f]{16}$/.test(r.bits))fail("closed IEEE word required");return{bits:BigInt("0x"+r.bits)}}
const floating=(v:unknown):Binary64=>v===undefined?binary64(0):word(v);
const optionalWord=(v:unknown):Binary64|null=>v===null||v===undefined?null:word(v);
function xyz(v:unknown,defaults=false):[Binary64,Binary64,Binary64]{if(v===undefined&&defaults)return[binary64(0),binary64(0),binary64(0)];if(!Array.isArray(v)||v.length!==3)fail("vector3 required");return[word(v[0]),word(v[1]),word(v[2])]}
function quaternion(v:unknown):[Binary64,Binary64,Binary64,Binary64]|null{if(v===null||v===undefined)return null;if(!Array.isArray(v)||v.length!==4)fail("vector4 required");return[word(v[0]),word(v[1]),word(v[2]),word(v[3])]}
function kind(v:unknown):m.BlockKindIdentity{const r=row(v);return{id:text(r.id),name:text(r.name),label:text(r.label),variant:optionalText(r.variant),description:string(r.description),icon:optionalText(r.icon),unit:optionalText(r.unit)}}
function attribute(v:unknown):m.BlockAttribute{const r=row(v);return{key:text(r.key),value:text(r.value),definition:optionalText(r.definition)}}
function author(v:unknown):m.BlockAuthor{const r=row(v);return{id:text(r.id),name:text(r.name),email:optionalText(r.email)}}
function compatible(v:unknown):m.BlockCompatibilityRule{const r=row(v);return{id:text(r.id),source:text(r.source),target:text(r.target),bidirectional:bool(r.bidirectional)}}
function representation(v:unknown):m.BlockRepresentation{const r=row(v);return{id:text(r.id),name:text(r.name),meshUrl:optionalText(r.meshUrl),tags:list(r.tags,text),lod:optionalText(r.lod),description:string(r.description),attributes:list(r.attributes,attribute)}}
function part2d(v:unknown):m.Block5dPart2d{const r=v===undefined?{}:row(v);return{shape:optionalText(r.shape),radius:optionalWord(r.radius),width:optionalWord(r.width),height:optionalWord(r.height),color:optionalText(r.color),iconKind:optionalText(r.iconKind)}}
function part3d(v:unknown):m.Block5dPart3d{const r=v===undefined?{}:row(v);return{orientation:quaternion(r.orientation),scale:r.scale===null||r.scale===undefined?null:xyz(r.scale)}}
function gripKind(v:unknown):m.Block5dGripKind{const r=row(v);return{id:text(r.id),name:text(r.name),label:text(r.label),color:text(r.color),defaultRopeKind:text(r.defaultRopeKind)}}
function grip(v:unknown):m.Block5dGripTemplate{const r=row(v);return{id:text(r.id),gripKind:text(r.gripKind),angle:floating(r.angle),radius2d:floating(r.radius2d),position:xyz(r.position,true),direction:xyz(r.direction,true),radius3d:floating(r.radius3d)}}
function camera2d(v:unknown):m.BlockCamera2d{const r=v===undefined?{}:row(v);return{x:floating(r.x),y:floating(r.y),zoom:r.zoom===undefined?binary64(1):word(r.zoom)}}
function camera3d(v:unknown):m.BlockCamera3d{const r=v===undefined?{}:row(v);return{position:xyz(r.position,true),target:xyz(r.target,true),zoom:r.zoom===undefined?binary64(1):word(r.zoom)}}
/** 📥️ Apply only actual declared file defaults and explicitly admitted finite numeric inputs. */
export function block5dFromJsonText(textValue:string):m.Block5dSnapshot{const r=row(JSON.parse(textValue)),meta=r.meta===undefined?{}:row(r.meta);return m.parseBlock5dSnapshot({schema:text(r.schema),partKind:kind(r.partKind),part2d:part2d(r.part2d),part3d:part3d(r.part3d),representations:list(r.representations,representation),gripKinds:list(r.gripKinds,gripKind),grips:list(r.grips,grip),compatibility:list(r.compatibility,compatible),attributes:list(r.attributes,attribute),authors:list(r.authors,author),camera2d:camera2d(r.camera2d),camera3d:camera3d(r.camera3d),meta:{description:string(meta.description)}})}
const out=(v:Binary64):{bits:string}=>({bits:parseBinary64(v).bits.toString(16).padStart(16,"0")});
const optionalOut=(v:Binary64|null):{bits:string}|null=>v===null?null:out(v);
const xyzOut=(v:readonly Binary64[]):{bits:string}[]=>[out(v[0]!),out(v[1]!),out(v[2]!)];
/** 📤️ Emit the literal declared word transport independently of SQLite representation. */
export function block5dToJsonText(value:m.Block5dSnapshot):string{const v=m.parseBlock5dSnapshot(value);return JSON.stringify({schema:v.schema,partKind:v.partKind,part2d:{shape:v.part2d.shape,radius:optionalOut(v.part2d.radius),width:optionalOut(v.part2d.width),height:optionalOut(v.part2d.height),color:v.part2d.color,iconKind:v.part2d.iconKind},part3d:{orientation:v.part3d.orientation===null?null:[out(v.part3d.orientation[0]),out(v.part3d.orientation[1]),out(v.part3d.orientation[2]),out(v.part3d.orientation[3])],scale:v.part3d.scale===null?null:xyzOut(v.part3d.scale)},representations:v.representations,gripKinds:v.gripKinds,grips:v.grips.map(g=>({id:g.id,gripKind:g.gripKind,angle:out(g.angle),radius2d:out(g.radius2d),position:xyzOut(g.position),direction:xyzOut(g.direction),radius3d:out(g.radius3d)})),compatibility:v.compatibility,attributes:v.attributes,authors:v.authors,camera2d:{x:out(v.camera2d.x),y:out(v.camera2d.y),zoom:out(v.camera2d.zoom)},camera3d:{position:xyzOut(v.camera3d.position),target:xyzOut(v.camera3d.target),zoom:out(v.camera3d.zoom)},meta:v.meta})}
/** 🔁️ The fixed point of the actual typed JSON boundary. */
export const block5dCanonicalJsonText=(text:string):string=>block5dToJsonText(block5dFromJsonText(text));


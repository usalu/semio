/** 💬️ Canonical BCF topic and viewpoint domain. */
import { binary64, type Binary64 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import { parseBinary64 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export interface BcfPoint3{readonly x:Binary64;readonly y:Binary64;readonly z:Binary64}
export type BcfCamera=Readonly<{kind:"perspective";viewPoint:BcfPoint3;direction:BcfPoint3;upVector:BcfPoint3;fieldOfView:Binary64}>|Readonly<{kind:"orthogonal";viewPoint:BcfPoint3;direction:BcfPoint3;upVector:BcfPoint3;viewToWorldScale:Binary64}>;
export interface BcfVisibility{readonly defaultVisibility:boolean;readonly exceptions:readonly string[]}
export interface BcfColoring{readonly color:string;readonly components:readonly string[]}
export interface BcfComponents{readonly selection:readonly string[];readonly visibility:BcfVisibility;readonly coloring:readonly BcfColoring[]}
export interface BcfComment{readonly guid:string;readonly date:string;readonly author:string;readonly text:string;readonly viewpointRef:string|null}
export interface BcfViewpoint{readonly guid:string;readonly camera:BcfCamera|null;readonly components:BcfComponents|null;readonly snapshot:readonly number[]|null}
export interface BcfTopic{readonly guid:string;readonly title:string;readonly description:string;readonly status:string;readonly priority:string;readonly labels:readonly string[];readonly creationDate:string;readonly creationAuthor:string;readonly comments:readonly BcfComment[];readonly viewpoints:readonly BcfViewpoint[]}
export interface BcfRawPart{readonly name:string;readonly data:readonly number[]}
export interface BcfSnapshot{readonly schema:string;readonly version:string;readonly topics:readonly BcfTopic[];readonly parts:readonly BcfRawPart[]}
const object=(value:unknown):Record<string,unknown>=>{if(value===null||typeof value!=="object"||Array.isArray(value))throw Error("BCF requires an object");return value as Record<string,unknown>};
const text=(value:unknown):string=>{if(typeof value!=="string")throw Error("BCF requires text");return value};
const flag=(value:unknown):boolean=>{if(typeof value!=="boolean")throw Error("BCF requires a boolean");return value};
const array=<T>(value:unknown,parse:(value:unknown)=>T):T[]=>{if(!Array.isArray(value))throw Error("BCF requires an array");return value.map(parse)};
const octets=(value:unknown):number[]=>array(value,byte=>{if(typeof byte!=="number"||!Number.isInteger(byte)||byte<0||byte>255)throw Error("BCF octet exceeds u8");return byte});
const optional=<T>(value:unknown,parse:(value:unknown)=>T):T|null=>value===null||value===undefined?null:parse(value);
type CameraParser=(value:unknown)=>BcfCamera;
/** 📐️ Validates all three exact binary64 point words. */
export function parseBcfPoint3(value:unknown):BcfPoint3{const row=object(value);return{x:parseBinary64(row.x),y:parseBinary64(row.y),z:parseBinary64(row.z)}}
/** 📷️ Validates the explicit camera choice and its complete scalar domain. */
export function parseBcfCamera(value:unknown):BcfCamera{const row=object(value);const common={viewPoint:parseBcfPoint3(row.viewPoint),direction:parseBcfPoint3(row.direction),upVector:parseBcfPoint3(row.upVector)};if(row.kind==="perspective")return{kind:"perspective",...common,fieldOfView:parseBinary64(row.fieldOfView)};if(row.kind==="orthogonal")return{kind:"orthogonal",...common,viewToWorldScale:parseBinary64(row.viewToWorldScale)};throw Error("BCF camera kind is unknown")}
/** 👁️ Validates visibility and ordered component exceptions. */
export function parseBcfVisibility(value:unknown):BcfVisibility{const row=object(value);return{defaultVisibility:flag(row.defaultVisibility),exceptions:array(row.exceptions??[],text)}}
/** 🎨️ Validates an ordered coloring group without imposing wire color syntax. */
export function parseBcfColoring(value:unknown):BcfColoring{const row=object(value);return{color:text(row.color),components:array(row.components??[],text)}}
/** 🧩️ Validates selection, visibility and coloring together. */
export function parseBcfComponents(value:unknown):BcfComponents{const row=object(value);return{selection:array(row.selection??[],text),visibility:parseBcfVisibility(row.visibility??{defaultVisibility:false,exceptions:[]}),coloring:array(row.coloring??[],parseBcfColoring)}}
/** 🗨️ Validates a comment and its optional native string reference. */
export function parseBcfComment(value:unknown):BcfComment{const row=object(value);return{guid:text(row.guid),date:text(row.date),author:text(row.author),text:text(row.text),viewpointRef:optional(row.viewpointRef,text)}}
function viewpoint(value:unknown,camera:CameraParser):BcfViewpoint{const row=object(value);return{guid:text(row.guid),camera:optional(row.camera,camera),components:optional(row.components,parseBcfComponents),snapshot:optional(row.snapshot,octets)}}
/** 🖼️ Validates the full viewpoint and optional image presence. */
export function parseBcfViewpoint(value:unknown):BcfViewpoint{return viewpoint(value,parseBcfCamera)}
function topic(value:unknown,camera:CameraParser):BcfTopic{const row=object(value);return{guid:text(row.guid),title:text(row.title),description:text(row.description??""),status:text(row.status),priority:text(row.priority??""),labels:array(row.labels??[],text),creationDate:text(row.creationDate??""),creationAuthor:text(row.creationAuthor??""),comments:array(row.comments??[],parseBcfComment),viewpoints:array(row.viewpoints??[],value=>viewpoint(value,camera))}}
/** 🗂️ Validates all topic fields and their ordered children. */
export function parseBcfTopic(value:unknown):BcfTopic{return topic(value,parseBcfCamera)}
/** 📦️ Validates a retained part name and intrinsic octets. */
export function parseBcfRawPart(value:unknown):BcfRawPart{const row=object(value);return{name:text(row.name),data:octets(row.data??[])}}
function snapshot(value:unknown,camera:CameraParser):BcfSnapshot{const row=object(value);return{schema:text(row.schema),version:text(row.version??""),topics:array(row.topics??[],value=>topic(value,camera)),parts:array(row.parts??[],parseBcfRawPart)}}
/** 🛂️ Validates canonical owned state independently of BCF wire admission. */
export function parseBcfSnapshot(value:unknown):BcfSnapshot{return snapshot(value,parseBcfCamera)}
const jsonNumber=(value:unknown):Binary64=>{if(typeof value!=="number"||!Number.isFinite(value))throw Error("BCF JSON requires a finite numeric scalar");return binary64(value)};
function jsonPoint(value:unknown):BcfPoint3{const row=object(value);return{x:jsonNumber(row.x),y:jsonNumber(row.y),z:jsonNumber(row.z)}}
function jsonCamera(value:unknown):BcfCamera{const row=object(value);const common={viewPoint:jsonPoint(row.viewPoint),direction:jsonPoint(row.direction),upVector:jsonPoint(row.upVector)};if(row.kind==="perspective")return{kind:"perspective",...common,fieldOfView:jsonNumber(row.fieldOfView)};if(row.kind==="orthogonal")return{kind:"orthogonal",...common,viewToWorldScale:jsonNumber(row.viewToWorldScale)};throw Error("BCF JSON camera kind is unknown")}
/** 🧾️ Admits native numeric JSON at an explicit binary64 conversion boundary. */
export function parseBcfSnapshotJson(value:unknown):BcfSnapshot{return snapshot(value,jsonCamera)}

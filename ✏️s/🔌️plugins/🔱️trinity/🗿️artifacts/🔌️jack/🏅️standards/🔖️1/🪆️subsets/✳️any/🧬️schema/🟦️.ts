/** 🔌️ The complete persisted Jack parent, inline manifest and literal content identity. */
import {parseArtifactChild,type ArtifactChild} from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import {type Binary64,parseBinary64Transport} from "../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {readValueType,type ValueType} from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🏷️type/🟦️.ts";
export type {ValueType,Binary64};
export interface JackArtifact {schema:string;name:string;manifestId?:string;manifest:Manifest;camera:Camera;content:ArtifactChild;rootNodeId?:string;query:string}
export interface Camera {x:Binary64;y:Binary64;zoom:Binary64}
export interface PropertyDef {name:string;kind:"data"|"derived";valueType:ValueType;expr?:string}
export interface NodeKindDef {name:string;properties:PropertyDef[];portKinds:string[]}
export interface EdgeKindDef {name:string;properties:PropertyDef[]}
export interface PortKindDef {name:string;direction:"in"|"out";properties:PropertyDef[]}
export interface Manifest {nodeKinds:NodeKindDef[];edgeKinds:EdgeKindDef[];portKinds:PortKindDef[]}
function row(value:unknown,required:readonly string[],optional:readonly string[]=[],at="$"):Record<string,unknown>{if(!value||typeof value!=="object"||Array.isArray(value))throw Error(at+": object required");const result=value as Record<string,unknown>;if(required.some(key=>!Object.hasOwn(result,key))||Object.keys(result).some(key=>!required.includes(key)&&!optional.includes(key)))throw Error(at+": Jack fields differ");return result;}
function text(value:unknown):string{if(typeof value!=="string"||/[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?<![\uD800-\uDBFF])[\uDC00-\uDFFF]/.test(value))throw Error("Jack native UTF8 text required");return value;}
function array<T>(value:unknown,parse:(value:unknown)=>T):T[]{if(!Array.isArray(value))throw Error("Jack ordered list required");return value.map(parse);}
function word(value:unknown):Binary64{return parseBinary64Transport(value);}
function property(value:unknown):PropertyDef{const input=row(value,["name","kind","valueType"],["expr"]);if(input.kind!=="data"&&input.kind!=="derived")throw Error("Jack property kind differs");const valueType=readValueType(input.valueType,{checkpoint(){}});let current=valueType;while(current.kind==="list")current=current.of;if(current.kind==="schema")text(current.of);return{name:text(input.name),kind:input.kind,valueType,...(input.expr===undefined?{}:{expr:text(input.expr)})};}
/** 🪪️ Parse exactly the persisted parent without registry lookup or scene materialization. */
export function parseJackArtifact(value:unknown,at="$"):JackArtifact{const input=row(value,["schema","name","manifest","camera","content","query"],["manifestId","rootNodeId"],at),content=parseArtifactChild(input.content);for(const item of[content.childId,content.target.artifactId,content.target.dialect.artifactKind,content.target.dialect.standard,content.target.dialect.subset])text(item);return{schema:text(input.schema),name:text(input.name),...(input.manifestId===undefined?{}:{manifestId:text(input.manifestId)}),manifest:parseManifest(input.manifest),camera:parseCamera(input.camera),content,...(input.rootNodeId===undefined?{}:{rootNodeId:text(input.rootNodeId)}),query:text(input.query)};}
/** 🎥️ Own each camera component as its complete native binary64 word. */
export function parseCamera(value:unknown,at="$"):Camera{const input=row(value,["x","y","zoom"],[],at);return{x:word(input.x),y:word(input.y),zoom:word(input.zoom)};}
/** 📜️ Preserve all literal ordered kind/property declarations independently of manifestId. */
export function parseManifest(value:unknown,at="$"):Manifest{const input=row(value,["nodeKinds","edgeKinds","portKinds"],[],at);return{nodeKinds:array(input.nodeKinds,value=>{const input=row(value,["name","properties","portKinds"]);return{name:text(input.name),properties:array(input.properties,property),portKinds:array(input.portKinds,text)};}),edgeKinds:array(input.edgeKinds,value=>{const input=row(value,["name","properties"]);return{name:text(input.name),properties:array(input.properties,property)};}),portKinds:array(input.portKinds,value=>{const input=row(value,["name","direction","properties"]);if(input.direction!=="in"&&input.direction!=="out")throw Error("Jack port direction differs");return{name:text(input.name),direction:input.direction,properties:array(input.properties,property)};})};}

/** ♻️ Complete Rewriting state owns its rule program and composed Jack parent. */
import {parsePropertyValue,propertyText,type PropertyValue} from "../../../../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🌱️value/🟦️.ts";
import {type Binary64,parseBinary64} from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export {parsePropertyValue};export type{PropertyValue,Binary64};
import {parseJackSnapshot,type JackSnapshot} from "./../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export type {JackSnapshot};
export interface RewritingArtifact{workingGraph:JackSnapshot;lhs:Lhs;rhs:Rhs;parameterBindings:Record<string,PropertyValue>;ruleLayout:Record<string,LayoutPoint>}
export interface Pattern{leftVar:string;leftKind:string;edgeVar?:string;edgeKind?:string;rightVar?:string;rightKind?:string}
export interface Lhs{pattern:Pattern;whereClause?:string}
export interface Assignment{var:string;prop:string;value:PropertyValue}
export interface ParameterSpec{name:string;kind:"string"|"number"|"boolean";default:PropertyValue}
export interface Rhs{create:Pattern[];delete:string[];set:Assignment[];merge:Pattern[];parameters:ParameterSpec[]}
export interface LayoutPoint{x:Binary64;y:Binary64}
function record(value:unknown,required:readonly string[]=[],optional:readonly string[]=[]):Record<string,unknown>{if(!value||typeof value!=="object"||Array.isArray(value))throw Error("rewriting object required");const row=value as Record<string,unknown>;if(required.some(key=>!Object.hasOwn(row,key))||(required.length&&Object.keys(row).some(key=>!required.includes(key)&&!optional.includes(key))))throw Error("rewriting fields differ");return row;}
function list<T>(value:unknown,parse:(value:unknown)=>T):T[]{if(!Array.isArray(value))throw Error("rewriting ordered list required");return value.map(parse);}
/** 🔗️ Preserve independently optional pattern companions. */
export function parsePattern(value:unknown):Pattern{const row=record(value,["leftVar","leftKind"],["edgeVar","edgeKind","rightVar","rightKind"]);return{leftVar:propertyText(row.leftVar),leftKind:propertyText(row.leftKind),...Object.fromEntries(["edgeVar","edgeKind","rightVar","rightKind"].filter(key=>Object.hasOwn(row,key)).map(key=>[key,propertyText(row[key])]))};}
/** ◀️ Own the match pattern and optional literal predicate. */
export function parseLhs(value:unknown):Lhs{const row=record(value,["pattern"],["whereClause"]);return{pattern:parsePattern(row.pattern),...(Object.hasOwn(row,"whereClause")?{whereClause:propertyText(row.whereClause)}:{})};}
/** ▶️ Own every ordered program operation and typed dynamic property. */
export function parseRhs(value:unknown,property:(value:unknown)=>PropertyValue=parsePropertyValue):Rhs{const row=record(value,["create","delete","set","merge","parameters"]);return{create:list(row.create,parsePattern),delete:list(row.delete,propertyText),set:list(row.set,value=>{const row=record(value,["var","prop","value"]);return{var:propertyText(row.var),prop:propertyText(row.prop),value:property(row.value)};}),merge:list(row.merge,parsePattern),parameters:list(row.parameters,value=>{const row=record(value,["name","kind","default"]);if(row.kind!=="string"&&row.kind!=="number"&&row.kind!=="boolean")throw Error("rewriting parameter kind differs");return{name:propertyText(row.name),kind:row.kind,default:property(row.default)};})};}
/** 📐️ Both coordinates retain the actual native binary64 word. */
export function parseLayoutPoint(value:unknown):LayoutPoint{const row=record(value,["x","y"]);return{x:parseBinary64(row.x),y:parseBinary64(row.y)};}
/** 🪪️ Parse the full domain parent through the actual Jack and property owners. */
export function parseRewritingArtifact(value:unknown):RewritingArtifact{const row=record(value,["workingGraph","lhs","rhs","parameterBindings","ruleLayout"]);return{workingGraph:parseJackSnapshot(row.workingGraph),lhs:parseLhs(row.lhs),rhs:parseRhs(row.rhs),parameterBindings:Object.fromEntries(Object.entries(record(row.parameterBindings)).map(([key,value])=>[propertyText(key),parsePropertyValue(value)])),ruleLayout:Object.fromEntries(Object.entries(record(row.ruleLayout)).map(([key,value])=>[propertyText(key),parseLayoutPoint(value)]))};}

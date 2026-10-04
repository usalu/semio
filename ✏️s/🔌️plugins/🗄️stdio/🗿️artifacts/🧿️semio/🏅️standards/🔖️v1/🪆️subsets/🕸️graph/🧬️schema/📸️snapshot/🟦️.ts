/** 🧬️ SemioGraphSnapshot schema — real facet mirror of the Rust `🦀️.rs` sibling. */
import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type {SemioValue} from "../../../🔢️value/🧬️schema/📸️snapshot/🟦️.ts";
export type SemioGraphPortKind = "in" | "out" | "inOut";

export interface SemioGraphPort {
  name: string;
  kind: SemioGraphPortKind;
  category: string;
  properties: { key: string; value: SemioValue }[];
}

export interface GraphNodeId { value: string }
export interface GraphEdgeId { value: string }

export interface SemioGraphNode {
  id: GraphNodeId;
  /** freeform node-type tag, mirrors flow's FlowNode.kind */
  kind: string;
  label: string;
  position: { x: Binary64; y: Binary64 };
  width: Binary64;
  height: Binary64;
  ports: SemioGraphPort[];
  properties: { key: string; value: SemioValue }[];
}

/** edges are id-keyed ENTITIES — source/target are ordinary data fields, not an attach handle */
export interface SemioGraphEdge {
  id: GraphEdgeId;
  source: GraphNodeId;
  target: GraphNodeId;
  sourcePort?: string;
  targetPort?: string;
  kind: string;
  label: string;
  properties: { key: string; value: SemioValue }[];
}

export interface SemioGraphSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ nodes: SemioGraphNode[];
  /** @state artifact */ edges: SemioGraphEdge[];
}

function graphRecord(value:unknown,required:readonly string[],optional:readonly string[]=[]):Record<string,unknown>{if(!value||typeof value!=="object"||Array.isArray(value))throw Error("Semio graph declared object required");const object=value as Record<string,unknown>;for(const key of required)if(!Object.hasOwn(object,key))throw Error("Semio graph declared field missing");for(const key of Object.keys(object))if(!required.includes(key)&&!optional.includes(key))throw Error("Semio graph undeclared field");return object}
function graphText(value:unknown):string{if(typeof value!=="string")throw Error("Semio graph literal text required");return value}
function graphList(value:unknown):unknown[]{if(!Array.isArray(value))throw Error("Semio graph ordered collection required");return value}
function graphId(value:unknown):{value:string}{return{value:graphText(graphRecord(value,["value"]).value)}}
function graphProperties(input:unknown):{key:string;value:SemioValue}[]{
 const entries=graphList(input),pending:unknown[]=[];
 for(const value of entries){const entry=graphRecord(value,["key","value"]);graphText(entry.key);pending.push(entry.value)}
 while(pending.length){const raw=pending.pop(),branch=graphRecord(raw,["kind"],["value","lexeme","items","entries","id"]),kind=graphText(branch.kind);switch(kind){
 case"null":graphRecord(raw,["kind"]);break;
 case"bool":{const value=graphRecord(raw,["kind","value"]).value;if(typeof value!=="boolean")throw Error("Semio boolean required");break}
 case"int":case"float":graphText(graphRecord(raw,["kind","lexeme"]).lexeme);break;
 case"str":graphText(graphRecord(raw,["kind","value"]).value);break;
 case"bytes":for(const byte of graphList(graphRecord(raw,["kind","value"]).value))if(typeof byte!=="number"||!Number.isInteger(byte)||byte<0||byte>255)throw Error("Semio octet required");break;
 case"list":for(const value of graphList(graphRecord(raw,["kind","items"]).items))pending.push(value);break;
 case"map":for(const member of graphList(graphRecord(raw,["kind","entries"]).entries)){const entry=graphRecord(member,["key","value"]);graphText(entry.key);pending.push(entry.value)}break;
 case"ref":graphId(graphRecord(raw,["kind","id"]).id);break;
 default:throw Error("Semio intrinsic value kind undeclared");
 }}
 return structuredClone(entries) as {key:string;value:SemioValue}[];
}
function graphWord(input:unknown,decode:boolean):Binary64|{bits:string}{const bits=graphRecord(input,["bits"]).bits;if(decode){if(typeof bits!=="string"||!/^[0-9a-f]{16}$/.test(bits))throw Error("Semio lowercase binary64 word required");return{bits:BigInt("0x"+bits)}}if(typeof bits!=="bigint"||bits<0n||bits>0xffffffffffffffffn)throw Error("Semio binary64 owner word required");return{bits:bits.toString(16).padStart(16,"0")}}
function graphJson(value:unknown,decode:boolean):unknown{
 const root=graphRecord(value,["schema","nodes","edges"]);
 const nodes=graphList(root.nodes).map(value=>{const node=graphRecord(value,["id","kind","label","position","width","height","ports","properties"]),position=graphRecord(node.position,["x","y"]);return{id:graphId(node.id),kind:graphText(node.kind),label:graphText(node.label),position:{x:graphWord(position.x,decode),y:graphWord(position.y,decode)},width:graphWord(node.width,decode),height:graphWord(node.height,decode),ports:graphList(node.ports).map(value=>{const port=graphRecord(value,["name","kind","category","properties"]),kind=graphText(port.kind);if(!["in","out","inOut"].includes(kind))throw Error("Semio port direction undeclared");return{name:graphText(port.name),kind,category:graphText(port.category),properties:graphProperties(port.properties)}}),properties:graphProperties(node.properties)}});
 const edges=graphList(root.edges).map(value=>{const edge=graphRecord(value,["id","source","target","kind","label","properties"],["sourcePort","targetPort"]);return{id:graphId(edge.id),source:graphId(edge.source),target:graphId(edge.target),kind:graphText(edge.kind),label:graphText(edge.label),...(Object.hasOwn(edge,"sourcePort")?{sourcePort:graphText(edge.sourcePort)}:{}),...(Object.hasOwn(edge,"targetPort")?{targetPort:graphText(edge.targetPort)}:{}),properties:graphProperties(edge.properties)}});
 return{schema:graphText(root.schema),nodes,edges};
}
/** 📥️ Binds the declared graph JSON scalar roles while retaining all nine intrinsic families. */
export function parseSemioGraphJsonValue(value:unknown):SemioGraphSnapshot{return graphJson(value,true) as SemioGraphSnapshot}
/** 📤️ Projects the actual graph owner with exact binary64 words and ordered intrinsic entries. */
export function semioGraphJsonValue(value:SemioGraphSnapshot):unknown{return graphJson(value,false)}

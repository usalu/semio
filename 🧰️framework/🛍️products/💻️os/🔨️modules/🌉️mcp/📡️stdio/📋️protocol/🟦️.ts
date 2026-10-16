import{types}from"node:util";
import{compileJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import schema from"./🧬️schema/🔣️.json" with{type:"json"};

/** 🔣️ Represents bounded immutable JSON DATA without external runtime types. */
export type McpJsonValueV1=null|boolean|number|string|readonly McpJsonValueV1[]|McpJsonRecordV1;
/** 🗂️ Contains own immutable JSON values only. */
export interface McpJsonRecordV1{readonly[key:string]:McpJsonValueV1}
/** 🪪️ Correlates selected MCP requests without a nullable or fractional identifier. */
export type McpRequestIdV1=number|string;
/** 📤️ Carries one schema-selected request. */
export interface McpWireRequestV1{readonly jsonrpc:"2.0";readonly id:McpRequestIdV1;readonly method:string;readonly params?:McpJsonRecordV1}
/** 📣️ Carries one schema-selected notification. */
export interface McpWireNotificationV1{readonly jsonrpc:"2.0";readonly method:string;readonly params?:McpJsonRecordV1}
/** ✅️ Carries one object-valued successful response. */
export interface McpWireSuccessV1{readonly jsonrpc:"2.0";readonly id:McpRequestIdV1;readonly result:McpJsonRecordV1}
/** ❌️ Carries one protocol failure independently of tool-level replies. */
export interface McpWireFailureV1{readonly jsonrpc:"2.0";readonly id:McpRequestIdV1;readonly error:Readonly<{code:number;message:string;data?:McpJsonValueV1}>}
/** 📨️ Selects exactly one supported message branch from the authored wire schema. */
export type McpWireMessageV1=McpWireRequestV1|McpWireNotificationV1|McpWireSuccessV1|McpWireFailureV1;
/** 📏️ Bounds one wire message and its complete immutable JSON tree. */
export const MCP_WIRE_POLICY_V1=Object.freeze({...schema.definitions.Policy.const});
const wireContract=compileJsonSchemaSubset(schema);
function stringValue(value:string):number{if(value.length>MCP_WIRE_POLICY_V1.maximumWireBytes)throw Error("Finite MCP JSON string required");for(let index=0;index<value.length;index++){const unit=value.charCodeAt(index);if(unit>=0xd800&&unit<=0xdbff){const next=value.charCodeAt(++index);if(!(next>=0xdc00&&next<=0xdfff))throw Error("Complete MCP JSON Unicode scalar required");}else if(unit>=0xdc00&&unit<=0xdfff)throw Error("Complete MCP JSON Unicode scalar required");}return Buffer.byteLength(JSON.stringify(value),"utf8");}
/** 📸️ Copies a finite own-value tree without invoking getters, proxies or serialization methods. */
export function snapshotMcpJsonV1(value:unknown):McpJsonValueV1{
 let nodes=0,bytes=0;const ancestors=new WeakSet<object>(),charge=(amount:number)=>{bytes+=amount;if(bytes>MCP_WIRE_POLICY_V1.maximumWireBytes)throw Error("Finite MCP JSON byte limit exceeded");};
 const copy=(value:unknown,depth:number):McpJsonValueV1=>{
  if(++nodes>MCP_WIRE_POLICY_V1.maximumNodes||depth>MCP_WIRE_POLICY_V1.maximumDepth)throw Error("Finite MCP JSON tree limit exceeded");
  if(value===null){charge(4);return null;}
  if(typeof value==="boolean"){charge(value?4:5);return value;}
  if(typeof value==="number"){if(!Number.isFinite(value))throw Error("Finite MCP JSON number required");const actual=Object.is(value,-0)?0:value;charge(JSON.stringify(actual).length);return actual;}
  if(typeof value==="string"){if(value.length>MCP_WIRE_POLICY_V1.maximumWireBytes)throw Error("Finite MCP JSON string required");charge(stringValue(value));return value;}
  if(typeof value!=="object"||types.isProxy(value)||ancestors.has(value))throw Error("Own acyclic MCP JSON values required");
  const array=Array.isArray(value),prototype=Object.getPrototypeOf(value);if(array?prototype!==Array.prototype:prototype!==Object.prototype&&prototype!==null)throw Error("Plain MCP JSON containers required");
  const keys=Reflect.ownKeys(value);if(keys.some(key=>typeof key!=="string"))throw Error("String MCP JSON keys required");
  ancestors.add(value);try{
   if(array){const length=Object.getOwnPropertyDescriptor(value,"length")?.value as unknown;if(!Number.isSafeInteger(length)||typeof length!=="number"||length<0||length+nodes>MCP_WIRE_POLICY_V1.maximumNodes||keys.length!==length+1)throw Error("Finite dense MCP JSON array required");charge(2+Math.max(0,length-1));const result:McpJsonValueV1[]=[];for(let index=0;index<length;index++){const entry=Object.getOwnPropertyDescriptor(value,String(index));if(!entry||!("value"in entry)||!entry.enumerable)throw Error("Own dense MCP JSON array values required");result.push(copy(entry.value,depth+1));}return Object.freeze(result);}
   if(keys.length>MCP_WIRE_POLICY_V1.maximumObjectEntries)throw Error("Finite MCP JSON object entry limit exceeded");charge(2+Math.max(0,keys.length-1));const result:Record<string,McpJsonValueV1>=Object.create(null);for(const key of(keys as string[]).sort()){const entry=Object.getOwnPropertyDescriptor(value,key);if(!entry||!("value"in entry)||!entry.enumerable)throw Error("Own enumerable MCP JSON object values required");charge(stringValue(key)+1);result[key]=copy(entry.value,depth+1);}return Object.freeze(result);
  }finally{ancestors.delete(value);}
 };
 return copy(value,0);
}
function admittedMessage(value:unknown):McpWireMessageV1{const captured=snapshotMcpJsonV1(value);if(wireContract.validate(captured).length)throw Error("Closed MCP wire message required");return captured as unknown as McpWireMessageV1;}
/** 📥️ Reads one bounded single-line UTF8 message into its immutable selected schema branch. */
export function readMcpWireMessageV1(wire:string):McpWireMessageV1{if(typeof wire!=="string"||Buffer.byteLength(wire,"utf8")>MCP_WIRE_POLICY_V1.maximumWireBytes||wire.includes("\n")||wire.includes("\r"))throw Error("Finite single-line MCP wire required");return admittedMessage(JSON.parse(wire));}
function encodedJson(value:McpJsonValueV1,parts:string[]):void{if(value===null||typeof value!=="object"){parts.push(JSON.stringify(value));return;}if(Array.isArray(value)){parts.push("[");for(let index=0;index<value.length;index++){if(index)parts.push(",");encodedJson(value[index]!,parts);}parts.push("]");return;}const record=value as McpJsonRecordV1;parts.push("{");for(const[index,key]of Object.keys(record).sort().entries()){if(index)parts.push(",");parts.push(JSON.stringify(key),":");encodedJson(record[key]!,parts);}parts.push("}");}
/** 📦️ Serializes the same selected wire DATA with deterministic UTF16 property ordering. */
export function writeMcpWireMessageV1(value:unknown):string{const message=admittedMessage(value),parts:string[]=[];encodedJson(message as unknown as McpJsonValueV1,parts);const wire=parts.join("");if(Buffer.byteLength(wire,"utf8")>MCP_WIRE_POLICY_V1.maximumWireBytes)throw Error("Finite MCP wire byte limit exceeded");return wire;}

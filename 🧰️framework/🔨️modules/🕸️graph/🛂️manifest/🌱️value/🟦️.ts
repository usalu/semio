/** 🌱️ The native graph manifest's six literal property variants, with exact binary64 numbers. */
import {parseBinary64,type Binary64} from "../../../🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export type PropertyValue={kind:"null"}|{kind:"bool";value:boolean}|{kind:"number";value:Binary64}|{kind:"string";value:string}|{kind:"array";values:PropertyValue[]}|{kind:"object";values:Record<string,PropertyValue>};
/** 🔤 Native graph text has the same Unicode scalar domain as Rust String. */
export function propertyText(value:unknown):string{if(typeof value!=="string"||/[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?<![\uD800-\uDBFF])[\uDC00-\uDFFF]/.test(value))throw Error("graph property requires native UTF8 text");return value;}
/** 🗂️ BTreeMap keys follow Unicode scalar order, including supplementary characters. */
export function comparePropertyKeys(a:string,b:string):number{let x=0,y=0;while(x<a.length&&y<b.length){const p=a.codePointAt(x)!,q=b.codePointAt(y)!;if(p!==q)return p-q;x+=p>65535?2:1;y+=q>65535?2:1;}return a.length-b.length;}
function record(value:unknown):Record<string,unknown>{if(!value||typeof value!=="object"||Array.isArray(value)||(Object.getPrototypeOf(value)!==Object.prototype&&Object.getPrototypeOf(value)!==null))throw Error("graph property object required");const descriptors=Object.getOwnPropertyDescriptors(value);for(const descriptor of Object.values(descriptors))if(!descriptor.enumerable||!("value"in descriptor))throw Error("graph property data fields required");return value as Record<string,unknown>;}
/** 📖️ Construct complete owned properties iteratively; repeated active owners are cycles. */
function readPropertyValue(value:unknown,readNumber:(value:unknown)=>Binary64):PropertyValue{
 let root:PropertyValue|undefined;const active=new Set<object>(),stack:{input:unknown;set:(value:PropertyValue)=>void;close?:object}[]=[{input:value,set:value=>{root=value;}}];
 while(stack.length){const frame=stack.pop()!;if(frame.close){active.delete(frame.close);continue;}const row=record(frame.input);if(active.has(row))throw Error("graph property ownership cycles");const kind=row.kind,fields=kind==="null"?["kind"]:kind==="array"||kind==="object"?["kind","values"]:["kind","value"];if(Object.keys(row).length!==fields.length||fields.some(key=>!Object.hasOwn(row,key)))throw Error("graph property fields differ");active.add(row);stack.push({input:null,set:()=>{},close:row});
 switch(kind){case"null":frame.set({kind});break;case"bool":if(typeof row.value!=="boolean")throw Error("graph boolean required");frame.set({kind,value:row.value});break;case"number":frame.set({kind,value:readNumber(row.value)});break;case"string":frame.set({kind,value:propertyText(row.value)});break;case"array":{if(!Array.isArray(row.values))throw Error("graph property array required");const values:PropertyValue[]=new Array(row.values.length);frame.set({kind,values});for(let index=row.values.length-1;index>=0;index--){if(!Object.hasOwn(row.values,index))throw Error("graph array has missing occurrences");stack.push({input:row.values[index],set:value=>{values[index]=value;}});}break;}case"object":{const raw=record(row.values),values:Record<string,PropertyValue>={},keys=Object.keys(raw).sort(comparePropertyKeys);frame.set({kind,values});for(let index=keys.length-1;index>=0;index--){const key=propertyText(keys[index]);stack.push({input:raw[key],set:value=>{Object.defineProperty(values,key,{value,writable:true,enumerable:true,configurable:true});}});}break;}default:throw Error("graph property variant differs");}
 }return root!;
}

/** 📖️ Read only the canonical graph property model; its number owns a bigint word. */
export function parsePropertyValue(value:unknown):PropertyValue{return readPropertyValue(value,parseBinary64);}
/** 🔢 The declared JSON scalar word is closed and never normalizes through Number. */
export function binary64FromJson(value:unknown):Binary64{const row=record(value);if(Object.keys(row).length!==1||typeof row.bits!=="string"||!/^[0-9a-f]{16}$/.test(row.bits))throw Error("graph binary64 JSON word differs");return{bits:BigInt("0x"+row.bits)};}
/** 📥️ Convert only the explicitly declared graph property JSON word boundary. */
export function parsePropertyValueJson(value:unknown):PropertyValue{return readPropertyValue(value,binary64FromJson);}
/** 📤️ Produce tagged primitive fields through an iterative declared JSON boundary. */
export function propertyValueToJson(value:PropertyValue):unknown{
 const owned=parsePropertyValue(value);let root:unknown;const stack:{value:PropertyValue;set:(value:unknown)=>void}[]=[{value:owned,set:value=>{root=value;}}];
 while(stack.length){const frame=stack.pop()!,v=frame.value;switch(v.kind){case"null":frame.set({kind:v.kind});break;case"bool":case"string":frame.set({kind:v.kind,value:v.value});break;case"number":frame.set({kind:v.kind,value:{bits:v.value.bits.toString(16).padStart(16,"0")}});break;case"array":{const values:unknown[]=new Array(v.values.length);frame.set({kind:v.kind,values});for(let i=v.values.length-1;i>=0;i--)stack.push({value:v.values[i]!,set:value=>{values[i]=value;}});break;}case"object":{const values:Record<string,unknown>={},keys=Object.keys(v.values).sort(comparePropertyKeys);frame.set({kind:v.kind,values});for(let i=keys.length-1;i>=0;i--){const key=keys[i]!;stack.push({value:v.values[key]!,set:value=>{Object.defineProperty(values,key,{value,writable:true,enumerable:true,configurable:true});}});}break;}}
 }return root;
}

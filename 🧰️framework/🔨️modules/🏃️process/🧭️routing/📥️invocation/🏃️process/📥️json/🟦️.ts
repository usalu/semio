import schema from "../🧬️schema/📥️json.json" with {type:"json"};
import{types}from"node:util";

/** 📄️ Contains only privately captured immutable JSON values. */
export type ScriptProcessJsonValueV1=null|boolean|number|string|readonly ScriptProcessJsonValueV1[]|Readonly<{[key:string]:ScriptProcessJsonValueV1}>;
const limits=schema.properties;

/** 📥️ Refuses oversized carried wire before parsing or allocating its object graph. */
export function readScriptProcessJsonWireV1(value:string):Readonly<{[key:string]:ScriptProcessJsonValueV1}>{
 if(typeof value!=="string"||value.length>limits.maximumWireUtf8Bytes.const||Buffer.byteLength(value)>limits.maximumWireUtf8Bytes.const)throw Error("Bounded original Script process wire required");return readScriptProcessJsonV1(JSON.parse(value));
}

/** 🧱️ Captures bounded plain capability data before schema recursion, without executing accessors or serialization hooks. */
export function readScriptProcessJsonV1(value:unknown):Readonly<{[key:string]:ScriptProcessJsonValueV1}>{
 let nodes=1,total=0,wire=0,result:ScriptProcessJsonValueV1|undefined;const active=new Set<object>(),refuse=():never=>{throw Error("Bounded plain original Script capability JSON required");},charge=(bytes:number):void=>{wire+=bytes;if(wire>limits.maximumWireUtf8Bytes.const)refuse();},text=(value:string,maximum:number):number=>{if(value.length>maximum)refuse();const bytes=Buffer.byteLength(value);if(bytes>maximum||Buffer.from(value).toString("utf8")!==value||(total+=bytes)>limits.maximumTotalUtf8Bytes.const)refuse();let quoted=bytes+2;for(let index=0;index<value.length;index++){const code=value.charCodeAt(index);if(code===34||code===92)quoted++;else if(code<32)quoted+=[8,9,10,12,13].includes(code)?1:5;}return quoted;};
 type Task={kind:"value";input:unknown;depth:number;assign:(value:ScriptProcessJsonValueV1)=>void}|{kind:"close";input:object;output:object};
 if(value===null||typeof value!=="object"||types.isProxy(value)||Array.isArray(value))return refuse();const tasks:Task[]=[{kind:"value",input:value,depth:0,assign:value=>{result=value;}}];
 while(tasks.length){const task=tasks.pop()!;if(task.kind==="close"){Object.freeze(task.output);active.delete(task.input);continue;}const{input,depth,assign}=task;if(depth>limits.maximumDepth.const)refuse();if(input===null){charge(4);assign(null);continue;}if(typeof input==="boolean"){charge(input?4:5);assign(input);continue;}if(typeof input==="number"){if(!Number.isFinite(input))refuse();charge(String(input).length);assign(input);continue;}if(typeof input==="string"){charge(text(input,limits.maximumStringUtf8Bytes.const));assign(input);continue;}if(typeof input!=="object"||types.isProxy(input))return refuse();const object=input as object,array=Array.isArray(input),prototype=Object.getPrototypeOf(object);if(active.has(object)||(array?prototype!==Array.prototype:prototype!==null&&prototype!==Object.prototype))refuse();const keys=Reflect.ownKeys(object);if(keys.some(key=>typeof key!=="string"))refuse();const count=array?(input as unknown[]).length:keys.length;if(count>(array?limits.maximumArrayItems.const:limits.maximumObjectMembers.const)||(nodes+=count)>limits.maximumNodes.const||array&&keys.length!==count+1)refuse();charge(2+Math.max(0,count-1));const output:Record<string,ScriptProcessJsonValueV1>|ScriptProcessJsonValueV1[]=array?Array(count):Object.create(null);assign(output);active.add(object);tasks.push({kind:"close",input:object,output});for(let index=count-1;index>=0;index--){const key=array?String(index):keys[index] as string,descriptor=Object.getOwnPropertyDescriptor(object,key);if(!descriptor?.enumerable||!("value" in descriptor))throw Error("Bounded plain original Script capability JSON required");if(!array)charge(text(key,limits.maximumKeyUtf8Bytes.const)+1);tasks.push({kind:"value",input:descriptor.value,depth:depth+1,assign:value=>{Object.defineProperty(output,key,{enumerable:true,value});}});}}
 return result as Readonly<{[key:string]:ScriptProcessJsonValueV1}>;
}

/** 📋️ Data: module exporting `captureSelectedTaskDataV1`. */
import{types}from"node:util";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
const policy=Object.freeze({...schema.definitions.Policy.const});

/** 🧊️ Captures finite inert Nx request DATA without invoking caller accessors or transferring authority. */
export function captureSelectedTaskDataV1(value:unknown):unknown{
 let count=0,text=0;const ancestors=new Set<object>(),charge=(value:string)=>{text+=Buffer.byteLength(value);if(text>policy.maximumTextBytes)throw Error("Selected request text capacity exceeded");};
 const capture=(current:unknown,depth:number):unknown=>{
  if(++count>policy.maximumValues||depth>policy.maximumDepth)throw Error("Selected request traversal capacity exceeded");
  if(typeof current==="string"){charge(current);return current;}if(current===null||current===undefined||typeof current==="boolean")return current;if(typeof current==="number"&&Number.isFinite(current))return current;
  if(typeof current!=="object"||types.isProxy(current)||ancestors.has(current))throw Error("Inert acyclic selected request DATA required");
  const array=Array.isArray(current),prototype=Object.getPrototypeOf(current);if(array?prototype!==Array.prototype:prototype!==Object.prototype&&prototype!==null)throw Error("Plain selected request DATA required");const keys=Reflect.ownKeys(current);if(keys.length>policy.maximumMembers+(array?1:0)||keys.some(key=>typeof key!=="string"))throw Error("Bounded own selected request members required");
  if(array&&(current.length>policy.maximumMembers||keys.length!==current.length+1))throw Error("Dense selected request array required");
  let output:unknown[]|Record<string,unknown>|undefined=Object.isFrozen(current)?undefined:array?[]:Object.create(prototype);ancestors.add(current);
  try{for(const key of keys){if(array&&key==="length")continue;if(typeof key!=="string")throw Error("Selected request string key required");if(array&&(!/^(?:0|[1-9][0-9]*)$/u.test(key)||Number(key)>=current.length))throw Error("Dense selected request index required");const row=Object.getOwnPropertyDescriptor(current,key);if(!row||!("value"in row)||!row.enumerable)throw Error("Own selected request DATA required");charge(key);const child=capture(row.value,depth+1);if(!output&&!Object.is(child,row.value)){output=array?[]:Object.create(prototype);for(const prior of keys){if(prior===key)break;if(array&&prior==="length")continue;const before=Object.getOwnPropertyDescriptor(current,prior);if(!before||!("value"in before))throw Error("Original selected request member required");Object.defineProperty(output,prior,{value:before.value,enumerable:true});}}if(output)Object.defineProperty(output,key,{value:child,enumerable:true});}return output?Object.freeze(output):current;}finally{ancestors.delete(current);}
 };return capture(value,0);
}

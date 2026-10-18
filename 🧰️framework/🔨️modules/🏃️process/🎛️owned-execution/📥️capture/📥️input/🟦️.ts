import{types}from"node:util";
import schema from"./🧬️schema/🔣️.json" with{type:"json"};
/** 📥️ Carries immutable UTF-8 text or canonical binary DATA under one finite input policy. */
export type OwnedCommandInputV1=Readonly<{version:1;text:string}>|Readonly<{version:1;base64:string}>;
const policy=Object.freeze({...schema.definitions.Policy.const});
/** 🔐️ Captures closed primitive input fields before child birth and refuses ambiguous binary encodings. */
export function readOwnedCommandInputV1(value:unknown):OwnedCommandInputV1{
 if(!value||typeof value!=="object"||types.isProxy(value)||![Object.prototype,null].includes(Object.getPrototypeOf(value)))throw Error("Owned child input data required");
 const keys=Reflect.ownKeys(value),field=keys.includes("text")?"text":"base64";if(keys.length!==2||!keys.includes("version")||!keys.includes(field))throw Error("Closed owned child input required");
 const version=Object.getOwnPropertyDescriptor(value,"version"),payload=Object.getOwnPropertyDescriptor(value,field);if(!version||!("value"in version)||!version.enumerable||version.value!==1||!payload||!("value"in payload)||!payload.enumerable||typeof payload.value!=="string")throw Error("Own primitive child input required");
 if(field==="text"){const text=payload.value;if(text.length>policy.maximumBytes||Buffer.byteLength(text)>policy.maximumBytes||Buffer.from(text,"utf8").toString("utf8")!==text)throw Error("Bounded own-data UTF-8 child input required");return Object.freeze({version:1,text});}
 const base64=payload.value;if(base64.length>policy.maximumEncodedBinaryCharacters||base64.length%4!==0)throw Error("Bounded canonical binary child input required");const bytes=Buffer.from(base64,"base64");try{if(bytes.length>policy.maximumBinaryBytes||bytes.toString("base64")!==base64)throw Error("Bounded canonical binary child input required");return Object.freeze({version:1,base64});}finally{bytes.fill(0);}
}

/** 🧾️ Parses the append-only creator claim sidecar records of one native storage child request; the storage core reads them below the claim owner, which depends on the core. */

import { types } from "node:util";
import { validateJsonSchemaSubset } from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import exportedStorageSchema from "../../🧬️schema/🔣️.json" with{type:"json"};
import exportedSchema from "../🧾️claim/🧬️schema/🔣️.json" with{type:"json"};

export const storageSchema=JSON.parse(JSON.stringify(exportedStorageSchema))as typeof exportedStorageSchema,schema=JSON.parse(JSON.stringify(exportedSchema))as typeof exportedSchema,protocol=Object.freeze({...schema.definitions.Protocol.const});

export type Identity=Readonly<{dev:string;ino:string;mode:number}>;

/** 📋️ Immutable declaration DATA never grants a Native image or parent allocation. */
export type NativeCreatorClaimRequestV1=import("../../📜️journal/🟦️.ts").NativeStorageChildRequestV1;

export type RecordV1=Readonly<{version:1;request:NativeCreatorClaimRequestV1;stage:"declared"|"directory"|"root";parent:Identity;directory:Identity|null;root:Identity|null}>;

export const same=(left:unknown,right:unknown)=>JSON.stringify(left)===JSON.stringify(right);

/** 🧬️ Copies the closed request without invoking getters or retaining mutable caller DATA. */
export function readRequest(value:unknown):NativeCreatorClaimRequestV1{const definition=storageSchema.definitions.ChildRequest;if(!value||typeof value!=="object"||types.isProxy(value)||Array.isArray(value)||Object.getPrototypeOf(value)!==Object.prototype&&Object.getPrototypeOf(value)!==null)throw Error("Plain Native creator request DATA required");const keys=Reflect.ownKeys(value);if(keys.length!==definition.required.length||keys.some(key=>typeof key!=="string"||!definition.required.includes(key)))throw Error("Closed Native creator request fields required");const captured:Record<string,string|number>=Object.create(null);for(const key of definition.required){const descriptor=Object.getOwnPropertyDescriptor(value,key);if(!descriptor||!("value"in descriptor)||!descriptor.enumerable||typeof descriptor.value!=="string"&&typeof descriptor.value!=="number")throw Error("Primitive Native creator request required");captured[key]=descriptor.value;}if(validateJsonSchemaSubset(definition,captured).length)throw Error("Finite Native creator request required");return Object.freeze(captured as NativeCreatorClaimRequestV1);}

/** 📜️ Decodes complete immutable claim DATA without adopting a path or granting a creator. */
export function readNativeCreatorClaimRecordsV1(text:string,input:NativeCreatorClaimRequestV1):readonly RecordV1[]{const request=readRequest(input);if(typeof text!=="string"||!text.endsWith("\n")||Buffer.byteLength(text)>protocol.maximumBytes)throw Error("Complete bounded recovered creator claim required");const lines=text.slice(0,-1).split("\n");if(lines.length!==protocol.maximumRecords)throw Error("Complete recovered creator stages required");const records:RecordV1[]=[];for(const[index,line]of lines.entries()){const row=JSON.parse(line)as RecordV1,prior=records.at(-1);if(validateJsonSchemaSubset(schema,row,schema,[storageSchema]).length||!same(row.request,request)||row.stage!==protocol.stages[index]||index===0&&(row.directory!==null||row.root!==null)||index===1&&(!row.directory||row.root!==null)||index===2&&(!row.directory||!row.root)||prior&&(!same(prior.parent,row.parent)||prior.directory&&!same(prior.directory,row.directory)))throw Error("Exact recovered creator record required");records.push(Object.freeze({...row,request,parent:Object.freeze({...row.parent}),directory:row.directory?Object.freeze({...row.directory}):null,root:row.root?Object.freeze({...row.root}):null}));}return Object.freeze(records);}

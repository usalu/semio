import{createHash}from"node:crypto";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import data from"./🔣️.json"with{type:"json"};

export interface CargoProfileConstraintV1{readonly package:string;readonly optLevel:"1"|"2"|"3";readonly origin:"native"|"wasm";readonly owner:string;readonly evidence:string}
const policy:typeof data=JSON.parse(JSON.stringify(data)),protocol=Object.freeze({...schema.definitions.Protocol.const,evidence:Object.freeze({...schema.definitions.Protocol.const.evidence})});
if(Buffer.byteLength(JSON.stringify(policy))>protocol.maximumPolicyBytes||validateJsonSchemaSubset(schema,policy).length)throw Error("Closed bounded canonical Cargo profile constraints required");
const rows=new Map<string,readonly CargoProfileConstraintV1[]>();
for(const[owner,group]of Object.entries(policy.owners)){
 const selected=policy.groups[group as keyof typeof policy.groups],values:CargoProfileConstraintV1[]=[],seen=new Set<string>();
 for(const origin of["native","wasm"]as const)for(const row of selected[origin]){const key=origin+":"+row.package;if(seen.has(key))throw Error("Unique canonical Cargo package constraint required");seen.add(key);values.push(Object.freeze({package:row.package,optLevel:row.optLevel as"1"|"2"|"3",origin,owner,evidence:protocol.evidence[origin]}));}
 if(values.length>protocol.maximumPackageRows)throw Error("Canonical Cargo profile constraint capacity exceeded");rows.set(owner,Object.freeze(values));
}
export const CARGO_PROFILE_CONSTRAINT_SOURCE_V1=Object.freeze({path:"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/📋️constraints/🔣️.json",digest:createHash("sha256").update(JSON.stringify(policy)).digest("hex")});
/** 📋️ Returns immutable package requirements for one exact authored workspace; DATA grants no compiler authority. */
export function cargoProfileConstraintsForOwnerV1(owner:string):readonly CargoProfileConstraintV1[]{const value=rows.get(owner);if(!value)throw Error("Registered actual Cargo profile workspace owner required");return value;}

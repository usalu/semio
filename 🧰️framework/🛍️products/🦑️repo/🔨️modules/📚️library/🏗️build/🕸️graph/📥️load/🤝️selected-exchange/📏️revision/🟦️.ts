import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";

const sorted=(value:unknown):unknown=>Array.isArray(value)?value.map(sorted):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value as Record<string,unknown>).sort(([a],[b])=>a<b?-1:a>b?1:0).map(([key,item])=>[key,sorted(item)])):value;
const digest=(value:unknown):string=>createHash("sha256").update(JSON.stringify(sorted(value))).digest("hex");

/** 📏️ What the semantics revision of the exchange is pinned to: the declared stamp of every inferrer (the protocol of the stamps), the fixture corpus of the derived inferrers (the expected results their stamps are settled from) and the schema of the graph a read produces. A result of an inference that changes through one of these changes this digest.
 * @see ../🧬️schema/📏️revision.json */
export function semanticsInputsV1(root:string):Readonly<{stamps:Readonly<Record<string,string>>;derived:readonly string[];corpus:string;graphSchema:string}>{
 const read=(path:string)=>JSON.parse(readFileSync(root+"/"+path,"utf8")) as Record<string,any>;
 const protocol=read("📥️inference/🔖️stamps/🧬️schema/🔣️.json").definitions.Protocol.const as {inferrers:Record<string,{stamp:string}>;derived:string[]};
 return{stamps:Object.fromEntries(Object.entries(protocol.inferrers).map(([id,row])=>[id,row.stamp])),derived:[...protocol.derived].sort(),corpus:digest(read("📥️inference/🔖️stamps/🧮️conformance/🔣️.json")),graphSchema:digest(read("🧬️schema/🔣️.json"))};
}

/** 📏️ The digest of the inputs of the semantics revision. */
export const semanticsDigestV1=(inputs:ReturnType<typeof semanticsInputsV1>):string=>digest({...inputs,derived:[...inputs.derived].sort()});

import {createHash} from "node:crypto";
import policy from "./📋️policy/🔣️.json" with {type:"json"};

/** 🌍️ Original graph resolution excludes task controls and source-proof transport, retaining unknown resolver variables. */
export function graphEnvironmentIdentityV1(environment:Readonly<Record<string,string|undefined>>):string{
 if(!environment||typeof environment!=="object"||Array.isArray(environment))throw Error("Original graph environment required");
 const excluded=new Set([...policy.taskOnly,...policy.proofTransport]),rows=Object.entries(environment).filter(([key,value])=>!excluded.has(key)&&value!==undefined).sort(([left],[right])=>Buffer.compare(Buffer.from(left),Buffer.from(right)));
 if(rows.some(([key,value])=>!key||key.includes("\0")||typeof value!=="string"))throw Error("Original graph environment values refused");return createHash("sha256").update(JSON.stringify(rows)).digest("hex");
}

import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=Object.freeze({...schema.$defs.Policy.const});
const canonical=(value:unknown):unknown=>Array.isArray(value)?value.map(canonical):value!==null&&typeof value==="object"?Object.fromEntries(Object.keys(value).sort().map(key=>[key,canonical((value as Record<string,unknown>)[key])])):value;

/** 📜️ The declared policy of the receipt-relevant manifest projection. */
export const manifestProfilePolicyV1=policy;

/** 🪪️ Digests what a dependency receipt depends on in one workspace manifest: every top-level field but the declared inert one (`scripts`), canonically ordered, so an edit of `scripts` or a reordered key keeps the digest and an edit of `exports`, `semio`, a dependency group, `name` or `version` changes it; `null` for a file that is not a manifest object, which a receipt then compares by its exact bytes.
 * @see ./🧬️schema/🔣️.json */
export function manifestProfileSha256V1(path:string,content:string|Uint8Array):string|null{
 if(String(path).split("/").at(-1)!==policy.fileName)return null;
 const bytes=typeof content==="string"?Buffer.from(content,"utf8"):Buffer.from(content);
 if(bytes.length>policy.maximumBytes)return null;
 try{
  const document=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes))as unknown;
  if(document===null||typeof document!=="object"||Array.isArray(document))return null;
  const kept=Object.fromEntries(Object.entries(document as Record<string,unknown>).filter(([key])=>!(policy.excludedTopLevelFields as readonly string[]).includes(key)));
  return createHash("sha256").update(JSON.stringify(canonical(kept))).digest("hex");
 }catch{return null;}
}

import {posix,win32} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🔗️ Qualifies fixed managed namespace DATA before any old target is followed. */
export function readManagedNxActivationTargetV1(root:string,target:string,style:"posix"|"windows"):string{
 if(validateJsonSchemaSubset(schema,{root,target,style}).length)throw Error("Closed managed Nx activation target required");const paths=style==="windows"?win32:posix,protocol=schema.$defs.Protocol.const;if(!paths.isAbsolute(root)||paths.normalize(root)!==root||!paths.isAbsolute(target)||paths.normalize(target)!==target||paths.dirname(target)!==paths.join(root,...protocol.store)||!new RegExp(protocol.digest,"u").test(paths.basename(target)))throw Error("Exact canonical managed Nx generation target required");return target;
}

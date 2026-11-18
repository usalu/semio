import {posix,win32} from "node:path";
import {machinePathV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🗄️ The directory of the managed generations of a checkout: below the declared machine state directory on this platform, else below the checkout. @see ../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts */
function managedStoreV1(root:string,style:"posix"|"windows"):string{
 const protocol=schema.$defs.Protocol.const;
 return style==="windows"?win32.join(root,...protocol.store):machinePathV1(root,...protocol.store);
}
/** 🔗️ Qualifies fixed managed namespace DATA before any old target is followed. */
export function readManagedNxActivationTargetV1(root:string,target:string,style:"posix"|"windows"):string{
 if(validateJsonSchemaSubset(schema,{root,target,style}).length)throw Error("Closed managed Nx activation target required");const paths=style==="windows"?win32:posix,protocol=schema.$defs.Protocol.const;if(!paths.isAbsolute(root)||paths.normalize(root)!==root||!paths.isAbsolute(target)||paths.normalize(target)!==target||paths.dirname(target)!==managedStoreV1(root,style)||!new RegExp(protocol.digest,"u").test(paths.basename(target)))throw Error("Exact canonical managed Nx generation target required");return target;
}

/** 🔗️ Classifies the target of an existing installation link: `owned` when it is a generation under this root, `relocated` when it is a generation of the same managed namespace under another root (the tree was moved, copied or replicated from a machine with another root, so the generation is not in this store and the link is dangling here); anything else is foreign and refused. */
export function classifyManagedNxActivationLinkV1(root:string,target:string,style:"posix"|"windows"):"owned"|"relocated"{
 if(validateJsonSchemaSubset(schema,{root,target,style}).length)throw Error("Closed managed Nx activation target required");const paths=style==="windows"?win32:posix,protocol=schema.$defs.Protocol.const;
 if(!paths.isAbsolute(target)||paths.normalize(target)!==target||!new RegExp(protocol.digest,"u").test(paths.basename(target)))throw Error("Exact canonical managed Nx generation target required");
 if(paths.dirname(target)===managedStoreV1(root,style))return"owned";
 const namespace=paths.sep+paths.join(...protocol.store),directory=paths.dirname(target);
 if(directory.length>namespace.length&&directory.endsWith(namespace)&&paths.isAbsolute(directory.slice(0,-namespace.length)||paths.sep))return"relocated";
 throw Error("Exact canonical managed Nx generation target required");
}

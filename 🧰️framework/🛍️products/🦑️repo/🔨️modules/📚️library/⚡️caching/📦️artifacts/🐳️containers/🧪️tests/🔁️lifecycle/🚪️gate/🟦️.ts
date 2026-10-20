import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
/** 🚪️ Only the exact declared lifecycle selector may enter the bounded suite. */
export function requireDevcontainerLifecycleGateV1(args:readonly string[]):void{if(validateJsonSchemaSubset(schema.$defs.Arguments,args,schema).length)throw Error("Expected test devcontainer-lifecycle");}

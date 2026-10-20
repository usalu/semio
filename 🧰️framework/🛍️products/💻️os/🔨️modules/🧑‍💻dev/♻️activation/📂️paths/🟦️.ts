import { repositoryCachePartitionRootDataV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
export type BrowserProductModeV1="prototype"|"release";
/** 📂️ Describes mode-qualified browser products without physical, compiler or Script authority. */
export function browserProductRootsV1(workspace:string,mode:BrowserProductModeV1):Readonly<{modules:string;sessions:string;runtime:string}>{
 if(validateJsonSchemaSubset(schema,{version:1,mode}).length)throw Error("Exact browser product mode required");const protocol=schema.definitions.Protocol.const,root=join(repositoryCachePartitionRootDataV1(workspace,"products"),protocol.directory,mode),paths={modules:join(root,protocol.modules),sessions:join(root,protocol.sessions),runtime:join(root,protocol.runtime)};if(Object.values(paths).some(path=>path.length>200))throw Error("Bounded browser product locations required");return Object.freeze(paths);
}

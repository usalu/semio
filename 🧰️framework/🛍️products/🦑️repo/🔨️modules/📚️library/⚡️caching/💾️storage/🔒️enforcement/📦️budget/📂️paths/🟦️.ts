import {isAbsolute,join,resolve} from "node:path";
import type {RepositoryCachePartitionIdV1} from "../🟦️.ts";
import policy from "../🔣️policy.json" with {type:"json"};
import native from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json" with {type:"json"};

/** 🗺️ Names one authored mounted partition as DATA without opening storage or issuing custody. */
export function repositoryCachePartitionRootDataV1(workspace:string,id:RepositoryCachePartitionIdV1):string{
 if(typeof workspace!=="string"||!isAbsolute(workspace)||resolve(workspace)!==workspace||/[\u0000-\u001f\u007f]/u.test(workspace)||!Object.hasOwn(policy.partitions,id))throw Error("Canonical workspace and declared cache partition required");
 const directory=join(workspace,policy.partitions[id].directory,native.definitions.ImageProtocol.const.mountDirectory);if(directory.length>256)throw Error("Finite mounted partition path required");return directory;
}

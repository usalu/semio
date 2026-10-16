import {isAbsolute,relative,resolve,sep} from "node:path";
import {repositoryCachePartitionRootDataV1} from "../../💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import modes from "../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🧬️schema/🔣️.json" with {type:"json"};

/** 🗂️ Validates an explicitly supplied compiler cache path as DATA inside the authored products namespace. */
export function viteCacheDirectory(workspace:string,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 if(!modes.enum.includes(environment.SEMIO_BUILD_MODE??""))throw Error("Build mode must be prototype or release");
 const directory=environment.SEMIO_VITE_CACHE_DIRECTORY;if(!directory)throw Error("Vite config requires its original bounded compiler cache");
 const root=repositoryCachePartitionRootDataV1(workspace,"products"),inside=relative(root,directory);if(!isAbsolute(directory)||resolve(directory)!==directory||directory.length>256||/[\u0000-\u001f\u007f]/u.test(directory)||!inside||isAbsolute(inside)||inside===".."||inside.startsWith(".."+sep))throw Error("Canonical bounded Vite compiler cache path required");return directory;
}

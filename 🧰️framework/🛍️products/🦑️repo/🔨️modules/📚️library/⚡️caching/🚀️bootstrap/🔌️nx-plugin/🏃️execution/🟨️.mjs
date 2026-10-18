import {createRequire} from "node:module";
import {dirname,resolve} from "node:path";
import {executeLiteralBuildArgumentsV1} from "../📤️arguments/🟨️.mjs";

/** 🔌️ Delegates portable source execution through Nx's actual task lifecycle. */
export default async function executor(options,context){
 const require=createRequire(import.meta.url),registryPath=require.resolve("nx/executors.json"),registry=JSON.parse(require("node:fs").readFileSync(registryPath,"utf8")),implementation=registry.executors?.["run-commands"]?.implementation;
 if(typeof implementation!=="string")throw Error("Nx has no registered process executor");
 return executeLiteralBuildArgumentsV1(options,context,require(resolve(dirname(registryPath),implementation)).default);
}

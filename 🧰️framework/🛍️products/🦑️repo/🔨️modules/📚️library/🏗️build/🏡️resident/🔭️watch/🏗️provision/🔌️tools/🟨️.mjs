/** 🔌 Binds the watcher provisioning to the tool registry: resolution through `resolveToolV1`, compilation through `runToolV1("cargo")`, the binary kept under the tools partition. Loads only where the tool interface exists. */
import {dirname,join} from "node:path";
import {ensureWatcherV1} from "../🟨️.mjs";
import {findToolV1,resolveToolV1,runToolV1,systemToolHostV1,toolRootV1,TOOL_REGISTRY_V1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";


/** 🔌 The path of the watcher for a worktree, building it once when no installed copy matches the pinned source. */
export async function ensureResidentWatcherV1(root=toolRootV1(),{signal,progress,environment=process.env,writable,workRoot}={}){
 const entry=findToolV1("repo-watch",TOOL_REGISTRY_V1),host=systemToolHostV1(root,environment),pinPath=join(root,entry.pin.version.file),rust=dirname(pinPath);
 return(await ensureWatcherV1({
  sourcePath:join(rust,"🦀️.rs"),pinPath:join(rust,"🔣️.json"),outputRoot:join(root,TOOL_REGISTRY_V1.partitions.tools,"repo-watch"),writable,signal,progress,workRoot,
  locate:async()=>resolveToolV1(entry,host,TOOL_REGISTRY_V1).command??undefined,
  compile:async({directory,signal:cancel})=>{
   await runToolV1("cargo",["build","--release","--manifest-path",join(directory,"Cargo.toml"),"--target-dir",join(directory,"target")],{cwd:directory,root,env:environment,signal:cancel,label:"repo-watch:compile",caller:import.meta.url});
   return join(directory,"target","release",process.platform==="win32"?"semio-repo-watch.exe":"semio-repo-watch");
  },
 })).path;
}

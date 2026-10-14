import {join,resolve} from "node:path";
import {AsyncLocalStorage} from "node:async_hooks";
import {statSync,lstatSync,mkdirSync} from "node:fs";
import {withRepositoryCacheStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {checkScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import policy from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
const writers=new AsyncLocalStorage();

/** 🗂️ Names graph facts inside their sole authored finite backing. */
export function repositoryBuildGraphDataDirectoryV1(workspace,namespace){
 if(!schema.enum.includes(namespace))throw Error("Declared bounded graph fact namespace required");
 return join(resolve(workspace),policy.partitions.graph.directory,"m",namespace);
}

/** 🎟️ Holds enforcing storage before any synchronous graph fact producer runs. */
export async function withRepositoryBuildGraphFactStoreV1(workspace,invocation,body){
 const current=writers.getStore();
 if(current?.workspace===resolve(workspace)&&current.store.invocation===invocation){requireRepositoryBuildGraphWriterV1(workspace);return body(current.store);}
 return withRepositoryCacheStoreV1(workspace,"graph",invocation,async store=>{await store.recheck();const status=statSync(store.root),scope={workspace:resolve(workspace),store,device:status.dev,inode:status.ino,live:true};try{return await writers.run(scope,()=>body(store));}finally{scope.live=false;}});
}

/** 🔒️ Refuses graph fact publication outside live original fixed-volume custody. */
export function requireRepositoryBuildGraphWriterV1(workspace){
 const current=writers.getStore();if(!current?.live||current.workspace!==resolve(workspace))throw Error("Original bounded graph writer custody required");checkScriptInvocation(current.store.invocation);const actual=statSync(current.store.root);if(actual.dev!==current.device||actual.ino!==current.inode)throw Error("Original bounded graph writer volume changed");
}

/** 📁️ Admits the closed fact directory on the original mounted device before a writer follows it. */
export function repositoryBuildGraphWriterDirectoryV1(workspace,namespace){
 requireRepositoryBuildGraphWriterV1(workspace);const path=repositoryBuildGraphDataDirectoryV1(workspace,namespace),current=writers.getStore();try{mkdirSync(path);}catch(error){if(error?.code!=="EEXIST")throw error;}const status=lstatSync(path);if(!status.isDirectory()||status.isSymbolicLink()||status.dev!==current.device)throw Error("Original bounded graph fact directory required");return path;
}

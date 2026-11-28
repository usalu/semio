import {existsSync,lstatSync,realpathSync}from"node:fs";
import {tmpdir}from"node:os";
import {dirname,join,resolve,sep}from"node:path";
import {fileURLToPath}from"node:url";
import {refuseProcessV1}from"../../../../📖️texts/🟦️.ts";
import protocol from"./🧬️schema/🔣️.json"with{type:"json"};
/** 🛂️ The authority to destroy storage below one root: issued by `issueNativeStorageRootCapabilityV1` only, so every destructive operation names the root it may touch and, under a test runner, only the sandbox ever gets one. */
export type NativeStorageRootCapabilityV1=Readonly<{root:string;operation:string;kind:"production"|"sandbox"|"unmaterialized"}>;
/** 🔎️ Whether this process is a test run: `NODE_ENV=test` (which `bun test` sets itself), an isolated build test or any process born with a test sandbox or artifact directory (each child of a test inherits them); the command line is never consulted, because `repo test` itself manages live storage. */
export function nativeStorageUnderTestRunnerV1(env:NodeJS.ProcessEnv=process.env):boolean{
 return env.NODE_ENV===protocol.runner.nodeEnvironment||protocol.runner.environment.some(name=>env[name]!==undefined&&env[name]!=="");
}
function variants(path:string):string[]{
 const absolute=resolve(path),rows=[absolute];let probe=absolute,tail="";
 for(;;){try{const real=realpathSync(probe),joined=tail===""?real:join(real,tail);if(joined!==absolute)rows.push(joined);break;}catch{const parent=dirname(probe);if(parent===probe)break;tail=tail===""?probe.slice(parent.length+1):join(probe.slice(parent.length+1),tail);probe=parent;}}
 return[...new Set(rows)];
}
function inside(root:string,path:string):boolean{return path===root||path.startsWith(root.endsWith(sep)?root:root+sep);}
function ancestorWithMarker(start:string):string|undefined{
 for(let directory=resolve(start);;directory=dirname(directory)){if(protocol.live.markers.some(marker=>existsSync(join(directory,marker))))return directory;if(dirname(directory)===directory)return undefined;}
}
/** 🏠️ Every root that is live for this process: the checkout of this very source, the declared checkout, the checkout of the working directory and the fleet store. */
export function nativeStorageLiveRootsV1(env:NodeJS.ProcessEnv=process.env,cwd:string=process.cwd()):readonly string[]{
 const given=protocol.live.checkoutEnvironment.flatMap(name=>env[name]?[env[name]!]:[]),found=[ancestorWithMarker(dirname(fileURLToPath(import.meta.url))),ancestorWithMarker(cwd)].flatMap(row=>row===undefined?[]:[row]);
 return Object.freeze([...new Set([...given,...found,...protocol.live.storeRoots].flatMap(variants))]);
}
function sandboxRoots(env:NodeJS.ProcessEnv):readonly string[]{return protocol.sandbox.environment.flatMap(name=>env[name]?variants(env[name]!):[]);}
function storageRoot(directory:string):string{
 const names=protocol.live.partitionParent,parts=resolve(directory).split(sep);
 for(let index=parts.length-names.length;index>=1;index--)if(names.every((name,offset)=>parts[index+offset]===name))return parts.slice(0,index).join(sep)||sep;
 return resolve(directory);
}
function materialized(path:string):boolean{try{lstatSync(path);return true;}catch{return false;}}
/** 🪪️ Issues the capability to destroy `operation` on the storage below `directory`. Outside a test runner every root is admitted. Under a test runner only a root inside the sandbox of that run or a temporary directory is admitted; the `.cache/b` partitions of a live checkout, the rest of a live checkout and the fleet store are refused with a bilingual error, whatever the caller passes. A directory that does not exist yet holds nothing to destroy and is only checked against the live roots. */
export function issueNativeStorageRootCapabilityV1(directory:string,operation:string,env:NodeJS.ProcessEnv=process.env,cwd:string=process.cwd()):NativeStorageRootCapabilityV1{
 const root=storageRoot(directory);
 if(!nativeStorageUnderTestRunnerV1(env))return Object.freeze({root,operation,kind:"production"});
 const paths=variants(directory),sandbox=sandboxRoots(env),live=nativeStorageLiveRootsV1(env,cwd),temporary=[...variants(tmpdir()),...protocol.sandbox.temporaryRoots.flatMap(variants),...live.flatMap(row=>protocol.sandbox.checkoutSandboxRoots.map(names=>join(row,...names)))],within=(roots:readonly string[])=>paths.every(path=>roots.some(row=>inside(row,path)));
 const refuse=(reason:"live"|"outside")=>refuseProcessV1("refused","storage.root-capability-refused",{operation,root,reason});
 const partitions=live.map(row=>join(row,...protocol.live.partitionParent));
 if(paths.some(path=>partitions.some(row=>inside(row,path))))throw refuse("live");
 if(within(sandbox)||within(temporary))return Object.freeze({root,operation,kind:materialized(directory)?"sandbox":"unmaterialized"});
 if(paths.some(path=>live.some(row=>inside(row,path))))throw refuse("live");
 if(!materialized(directory))return Object.freeze({root,operation,kind:"unmaterialized"});
 throw refuse("outside");
}
/** 🛑️ Requires the capability for one destructive operation (retire, delete, compact, reborn or detach of an image, partition, shard or generation) on `directory`. */
export function requireNativeStorageRootCapabilityV1(directory:string,operation:string):NativeStorageRootCapabilityV1{return issueNativeStorageRootCapabilityV1(directory,operation);}

import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{createNativeImageSharedReaderV1,NATIVE_IMAGE_SHARED_READ_POLICY_V1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts";
import{createHash}from"node:crypto";
import{mkdir,readFile,rename,rm,writeFile}from"node:fs/promises";
import{dirname,join}from"node:path";
import{canonicalJsonV1}from"../../🕰️generations/🧮️fact-codec/🟨️.mjs";
import{sharedClosureSettlerV1}from"../🧩️closure-kinds/🟨️.mjs";

const digestOf=value=>createHash("sha256").update(canonicalJsonV1(value)).digest("hex");

/** 🎯️ The declared limits of the task state: how many task records one workspace keeps. */
export const TASK_STATE_POLICY_V1=Object.freeze({version:1,maximumTasks:262144});

/**
 * 🔑️ The key of every planned task: the digest of its complete declaration (`task-key` node: executor options, configurations, outputs, resolved inputs, environment) the ordered forwarded arguments and the keys of the tasks it depends on, so an edit that reaches a dependency changes the key of every task above it. `tasks` maps a task id to `{root, target, declaration, dependsOn, cacheable}`; ids come in dependency order. A task whose declaration cannot be keyed has the key `null` and is never up to date.
 * @param {{graph:*;tasks:*;order:*;environment?:object;forwarded?:readonly unknown[];signal?:AbortSignal}} options
 * @see ../🧩️kinds/🟨️.mjs
 */
export async function taskKeysV1({graph,tasks,order,environment={},forwarded=[],signal}){
 const own=new Map(),keys=new Map(),argumentsDigest=forwarded.length?digestOf(forwarded):undefined;
 const nodes=await graph.getMany(order.map(id=>{const task=tasks.get(id);return graph.key("task-key",task.root+":"+task.target,{root:task.root,target:task.target,declaration:task.declaration,env:environment});}),{signal});
 order.forEach((id,index)=>own.set(id,nodes[index].value.key));
 for(const id of order){
  const task=tasks.get(id),mine=own.get(id),dependencies=[...task.dependsOn].filter(dependency=>keys.has(dependency)).sort().map(dependency=>keys.get(dependency));
  keys.set(id,mine===null||dependencies.includes(null)?null:digestOf({own:mine,dependencies,...argumentsDigest===undefined?{}:{arguments:argumentsDigest}}));
 }
 return keys;
}

/**
 * 🧾️ The record of the tasks a workspace ran successfully: one record per task in the host content store (`cas`, see `🗄️cas`), named by the digest of its content, and one small pointer file per task below `directory`. A task is up to date when its pointer names a record that still exists with the key the plan computes now; a record that the store collected makes the task stale again, so the state is bounded by the store's own retention.
 */
export function createTaskRecorderV1({directory,cas,now=Date.now,leased=body=>body()}){
 const pointerOf=(workspace,id)=>{const name=digestOf([workspace,id]);return join(directory,name.slice(0,2),name+".json");};
 return Object.freeze({
  record:(workspace,entries)=>leased(async()=>{
   for(const{id,key}of entries){
    if(key===null||key===undefined)continue;
    const record=await cas.put(new TextEncoder().encode(canonicalJsonV1({version:1,workspace,id,key,at:now()}))),path=pointerOf(workspace,id),temporary=path+"."+process.pid+"."+Math.random().toString(36).slice(2)+".tmp";
    await mkdir(dirname(path),{recursive:true});
    await writeFile(temporary,JSON.stringify({key,record:record.hash}));await rename(temporary,path).catch(async error=>{await rm(temporary,{force:true});throw error;});
   }
  }),
  upToDate:(workspace,keys)=>leased(async()=>{
   const fresh=new Set(),checked=[...keys].filter(([,key])=>key!==null);
   await Promise.all(checked.map(async([id,key])=>{
    let pointer;try{pointer=JSON.parse(await readFile(pointerOf(workspace,id),"utf8"));}catch{return;}
    if(pointer?.key!==key||typeof pointer.record!=="string")return;
    if((await cas.has([pointer.record]))[0]===true){await cas.touch(pointer.record);fresh.add(id);}
   }));
   return fresh;
  }),
 });
}

/** 🏭️ The task state of the host service: per workspace the shared workspace graph (the one the closure settler runs on) and the recorder below `<root>/.cache/b/g/m/task-keys`, both over the host content store. Everything they read below the graph image (node store, stat index, task keys) belongs inside `leased(body)`: it runs `body` under the shared lease of the image (one reader per workspace, one lease per burst) and closes the part-id descriptor of the settler before the lease is given back. */
export function createHostTaskStateV1({cas}){
 const readers=new Map();
 return async({root})=>{
  const cacheRoot=machinePathV1(root,".cache/b/g/m"),settler=sharedClosureSettlerV1({workspaceRoot:root,cacheRoot,blobs:cas});
  let reader=readers.get(root);
  if(reader===undefined){reader=createNativeImageSharedReaderV1({leaseDirectory:machinePathV1(root,".cache/b/l/m/data"),imageDirectory:machinePathV1(root,".cache/b/g"),signal:new AbortController().signal,timeoutMs:NATIVE_IMAGE_SHARED_READ_POLICY_V1.sampleWaitMilliseconds});readers.set(root,reader);}
  const shared=reader;
  return{graph:settler.graph,recorder:createTaskRecorderV1({directory:join(cacheRoot,"task-keys"),cas}),leased:body=>shared.run(async()=>{try{return await body();}finally{settler.releaseFiles();}})};
 };
}

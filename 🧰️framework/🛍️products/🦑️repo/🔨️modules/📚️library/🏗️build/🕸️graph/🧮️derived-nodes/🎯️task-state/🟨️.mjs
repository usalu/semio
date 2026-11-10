import{createHash}from"node:crypto";
import{mkdir,readFile,rename,rm,writeFile}from"node:fs/promises";
import{dirname,join}from"node:path";
import{canonicalJsonV1}from"../../🕰️generations/🧮️fact-codec/🟨️.mjs";
import{sharedClosureSettlerV1}from"../🧩️closure-kinds/🟨️.mjs";

const digestOf=value=>createHash("sha256").update(canonicalJsonV1(value)).digest("hex");

/** 🎯️ The declared limits of the task state: how many task records one workspace keeps. */
export const TASK_STATE_POLICY_V1=Object.freeze({version:1,maximumTasks:262144});

/**
 * 🔑️ The key of every planned task: the digest of its complete declaration (`task-key` node: executor options, configurations, outputs, resolved inputs, environment) and the keys of the tasks it depends on, so an edit that reaches a dependency changes the key of every task above it. `tasks` maps a task id to `{root, target, declaration, dependsOn, cacheable}`; ids come in dependency order. A task whose declaration cannot be keyed has the key `null` and is never up to date.
 * @see ../🧩️kinds/🟨️.mjs
 */
export async function taskKeysV1({graph,tasks,order,environment={},signal}){
 const own=new Map(),keys=new Map();
 const nodes=await graph.getMany(order.map(id=>{const task=tasks.get(id);return graph.key("task-key",task.root+":"+task.target,{root:task.root,target:task.target,declaration:task.declaration,env:environment});}),{signal});
 order.forEach((id,index)=>own.set(id,nodes[index].value.key));
 for(const id of order){
  const task=tasks.get(id),mine=own.get(id),dependencies=[...task.dependsOn].filter(dependency=>keys.has(dependency)).sort().map(dependency=>keys.get(dependency));
  keys.set(id,mine===null||dependencies.includes(null)?null:digestOf({own:mine,dependencies}));
 }
 return keys;
}

/**
 * 🧾️ The record of the tasks a workspace ran successfully: one record per task in the host content store (`cas`, see `🗄️cas`), named by the digest of its content, and one small pointer file per task below `directory`. A task is up to date when its pointer names a record that still exists with the key the plan computes now; a record that the store collected makes the task stale again, so the state is bounded by the store's own retention.
 */
export function createTaskRecorderV1({directory,cas,now=Date.now}){
 const pointerOf=(workspace,id)=>{const name=digestOf([workspace,id]);return join(directory,name.slice(0,2),name+".json");};
 return Object.freeze({
  async record(workspace,entries){
   for(const{id,key}of entries){
    if(key===null||key===undefined)continue;
    const record=await cas.put(new TextEncoder().encode(canonicalJsonV1({version:1,workspace,id,key,at:now()}))),path=pointerOf(workspace,id),temporary=path+"."+process.pid+"."+Math.random().toString(36).slice(2)+".tmp";
    await mkdir(dirname(path),{recursive:true});
    await writeFile(temporary,JSON.stringify({key,record:record.hash}));await rename(temporary,path).catch(async error=>{await rm(temporary,{force:true});throw error;});
   }
  },
  async upToDate(workspace,keys){
   const fresh=new Set(),checked=[...keys].filter(([,key])=>key!==null);
   await Promise.all(checked.map(async([id,key])=>{
    let pointer;try{pointer=JSON.parse(await readFile(pointerOf(workspace,id),"utf8"));}catch{return;}
    if(pointer?.key!==key||typeof pointer.record!=="string")return;
    if((await cas.has([pointer.record]))[0]===true){await cas.touch(pointer.record);fresh.add(id);}
   }));
   return fresh;
  },
 });
}

/** 🏭️ The task state of the host service: per workspace the shared workspace graph (the one the closure settler runs on) and the recorder below `<root>/.cache/b/g/m/task-keys`, both over the host content store. */
export function createHostTaskStateV1({cas}){
 return async({root})=>{
  const cacheRoot=join(root,".cache/b/g/m"),settler=sharedClosureSettlerV1({workspaceRoot:root,cacheRoot,blobs:cas});
  return{graph:settler.graph,recorder:createTaskRecorderV1({directory:join(cacheRoot,"task-keys"),cas})};
 };
}

import{copyFileSync,lstatSync,mkdirSync,readdirSync,readlinkSync,renameSync,rmdirSync,statfsSync,symlinkSync,unlinkSync}from"node:fs";
import{dirname,join,relative,sep}from"node:path";
import{nativeImageShardFoldV1,nativeImageShardHomeLayoutV1,nativeImageShardKeyV1,nativeImageShardLayoutsV1,nativeImageShardMountedV1,nativeImageShardPathV1,ensureNativeImageShardV1,readNativeImageShardLedgerV1,type NativeImageShardLayoutV1,type NativeImageShardLifecycleV1,type NativeImageShardRuleV1,type NativeImageShardStateV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";
import{repositoryCacheBudgetV1,type RepositoryCachePartitionIdV1}from"../🟦️.ts";

/** 🧩️ One image of a partition with the logical path it is mounted at below the home mount (`""` for the home image): callers address the partition through this model and never through a hardcoded mount point. */
export type RepositoryPartitionImageV1=Readonly<NativeImageShardLayoutV1&{logicalPath:string}>;
/** 🗂️ Every image of a partition, home first, then the live shards: the one place that knows where a sub-area of a split partition is mounted. @see ../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts */
export function repositoryPartitionImagesV1(workspace:string,partition:RepositoryCachePartitionIdV1):readonly RepositoryPartitionImageV1[]{
 const directory=repositoryCacheBudgetV1(workspace).partitions[partition].directory,home=nativeImageShardHomeLayoutV1(directory).mountPoint;
 return nativeImageShardLayoutsV1(directory,nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory))).map(layout=>Object.freeze({...layout,logicalPath:relative(home,layout.mountPoint).split(sep).join("/")}));
}
/** 🧭️ The image that holds a logical path of a partition: the longest mount point that leads it, the home image otherwise. */
export function repositoryPartitionImageOfV1(images:readonly RepositoryPartitionImageV1[],path:string):RepositoryPartitionImageV1{
 return images.filter(image=>image.logicalPath===""||path===image.logicalPath||path.startsWith(image.logicalPath+"/")).sort((a,b)=>b.logicalPath.length-a.logicalPath.length)[0]!;
}
/** 🏷️ The prefix of the directory that holds the files of a key while they move into its image. */
export const REPOSITORY_RELAYOUT_ASIDE_V1=".relayout-";
/** 📋️ One step of a re-layout: attach a live image, or move what the home image holds of a key into its image (born first when it is not live). */
export type RepositoryRelayoutStepV1=Readonly<{id:string;key:string;root:number;path:string;action:"attach"|"migrate"}>;
/** 🧮️ Plans the re-layout of a partition from what the home image holds below the declared roots and the live images: a live key attaches, unless an aside directory of an interrupted move remains; any other key present migrates. Pure and language-agnostic. */
export function repositoryRelayoutPlanV1(rule:NativeImageShardRuleV1,children:(prefix:string)=>readonly string[],live:NativeImageShardStateV1):readonly RepositoryRelayoutStepV1[]{
 const steps:RepositoryRelayoutStepV1[]=[];
 for(const[root,declared]of rule.roots.entries()){
  const names=children(declared.prefix),asides=new Set(names.filter(name=>name.startsWith(REPOSITORY_RELAYOUT_ASIDE_V1)).map(name=>name.slice(REPOSITORY_RELAYOUT_ASIDE_V1.length))),keys=new Set([...names.filter(name=>!name.startsWith(REPOSITORY_RELAYOUT_ASIDE_V1)),...asides]);
  for(const key of[...keys].sort()){
   const path=nativeImageShardPathV1(rule,root,key),found=nativeImageShardKeyV1(rule,path);if(found===null||found.root!==root)continue;
   steps.push(Object.freeze({id:found.id,key,root,path,action:live.has(found.id)&&!asides.has(key)?"attach":"migrate"}));
  }
 }
 return Object.freeze(steps);
}
/** ⚠️ A move that stopped because the host volume would fall below the reserve; the files already moved stay moved and a repeat resumes. */
export class RepositoryRelayoutSpaceErrorV1 extends Error{constructor(readonly freeBytes:number,readonly needBytes:number){super("Host volume free "+freeBytes+" bytes, move needs "+needBytes);}}
function moveTreeV1(source:string,target:string,ports:Readonly<{check:()=>void;free:()=>number;reserveBytes:number}>):number{
 let moved=0;mkdirSync(target,{recursive:true});
 for(const entry of readdirSync(source)){
  ports.check();const from=join(source,entry),to=join(target,entry),row=lstatSync(from);
  if(row.isDirectory())moved+=moveTreeV1(from,to,ports);
  else if(row.isSymbolicLink()){const link=readlinkSync(from);try{unlinkSync(to);}catch{}symlinkSync(link,to);unlinkSync(from);}
  else{const free=ports.free();if(free<row.size+ports.reserveBytes)throw new RepositoryRelayoutSpaceErrorV1(free,row.size+ports.reserveBytes);copyFileSync(from,to);unlinkSync(from);moved+=row.size;}
 }
 rmdirSync(source);return moved;
}
/** 🚚️ What a re-layout did per key and in total. */
export type RepositoryRelayoutOutcomeV1=Readonly<{status:"busy"|"unmounted"|"disabled"|"done"|"space"|"refused";steps:readonly Readonly<{id:string;action:RepositoryRelayoutStepV1["action"];result:"attached"|"born"|"moved"|"refused"|"space"}>[];movedBytes:number}>;
/** 🚚️ Makes the physical layout of a sharded partition match its declaration: every key below a declared root gets its own image mounted at its logical path and the files of the home image move into it, one file at a time, so an interrupted run resumes. The caller must hold the exclusive whole-partition lease; the home image must be mounted. Every file keeps its logical path. A volume that would fall below `reserveBytes` stops the run cleanly. */
export async function relayoutRepositoryPartitionV1(given:Readonly<{directory:string;rule:NativeImageShardRuleV1;platform:string;lifecycle:NativeImageShardLifecycleV1;lock:<T>(body:()=>Promise<T>)=>Promise<T>;hostFreeBytes:()=>number;reserveBytes:number;now:()=>number;check?:()=>void;progress?:(done:number,total:number)=>void;mounted?:(path:string)=>boolean}>):Promise<RepositoryRelayoutOutcomeV1>{
 const check=given.check??(()=>{}),mounted=given.mounted??nativeImageShardMountedV1,home=nativeImageShardHomeLayoutV1(given.directory);
 if(!given.rule.enabled)return{status:"disabled",steps:[],movedBytes:0};
 if(!mounted(home.mountPoint))return{status:"unmounted",steps:[],movedBytes:0};
 const children=(prefix:string)=>{try{return readdirSync(join(home.mountPoint,...prefix===""?[]:prefix.split("/")));}catch{return[];}};
 const plan=repositoryRelayoutPlanV1(given.rule,children,nativeImageShardFoldV1(readNativeImageShardLedgerV1(given.directory))),steps:Array<RepositoryRelayoutOutcomeV1["steps"][number]>=[];let movedBytes=0,status:RepositoryRelayoutOutcomeV1["status"]="done";
 for(const[at,step]of plan.entries()){
  check();given.progress?.(at,plan.length);
  const source=join(home.mountPoint,...step.path.split("/")),aside=join(dirname(source),REPOSITORY_RELAYOUT_ASIDE_V1+step.key),ensure=()=>ensureNativeImageShardV1({directory:given.directory,rule:given.rule,platform:given.platform,path:step.path,hostFreeBytes:given.hostFreeBytes,now:given.now,lifecycle:given.lifecycle,lock:given.lock,check});
  if(step.action==="attach"){const made=await ensure();steps.push({id:step.id,action:step.action,result:made.kind==="refused"?"refused":"attached"});if(made.kind==="refused")status="refused";continue;}
  const asideLeft=(()=>{try{return lstatSync(aside).isDirectory();}catch{return false;}})();
  if(!asideLeft&&!mounted(source))renameSync(source,aside);
  const made=await ensure();
  if(made.kind==="refused"){if(!asideLeft){try{rmdirSync(source);}catch{}renameSync(aside,source);}steps.push({id:step.id,action:step.action,result:"refused"});status="refused";continue;}
  try{movedBytes+=moveTreeV1(aside,source,{check,free:()=>{const row=statfsSync(given.directory);return row.bavail*row.bsize;},reserveBytes:given.reserveBytes});}
  catch(error){if(error instanceof RepositoryRelayoutSpaceErrorV1){steps.push({id:step.id,action:step.action,result:"space"});return{status:"space",steps,movedBytes};}throw error;}
  steps.push({id:step.id,action:step.action,result:made.kind==="birth"?"born":"moved"});
 }
 given.progress?.(plan.length,plan.length);return{status,steps,movedBytes};
}

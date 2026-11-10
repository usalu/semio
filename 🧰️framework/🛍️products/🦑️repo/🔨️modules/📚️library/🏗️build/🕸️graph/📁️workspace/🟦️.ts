import {ROSTER_RULES_V1} from "../🧭️roster/🟨️.mjs";
import {isOpaqueSourcePathV1} from "../../../⚡️caching/🔍️discovery/📂️source/🟦️.ts";
import {selectPackageManagerV1} from "../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";
import * as observationV1 from "../../👁️observation/🟨️.mjs";
import {readInventoryRecordLeanV1,rememberInventoryRecordV1} from "../../📥️inputs/🪶️lean-inventory/🟨️.mjs";
import {createGraphPhaseClockV1,writeGraphPhaseRowV1} from "../⏱️phases/🟨️.mjs";
import {captureInferenceInventoryReferencesV1} from "../../👁️observation/📥️reuse/🟨️.mjs";
import {captureSourceInputInventoryV1,captureCargoLockInventoryV1} from "../../📥️inputs/📋️data/📥️load/🟨️.mjs";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {withRepositoryBuildGraphConsumerV1} from "../../📥️inputs/💾️storage/🟨️.mjs";
import {readdir,lstat} from "node:fs/promises";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {checkScriptInvocation,advanceScriptInvocation,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {graphStringInternerV1} from "../../🧠️memory/🟨️.mjs";
import {GRAPH_LIMITS_V1,graphDeclarationV1,exactSourceReadPolicyV1} from "../🔐️authority/🟦️.ts";
import {inventoryDirectoriesV1,expandedClosuresV1,rememberExpandedClosureV1} from "../🕊️release/🟦️.ts";


/** 📁️ Discovers real workspace authority while yielding cancellation and progress. */
export async function workspaceBuildFilesV1(root:string,invocation?:ScriptInvocation):Promise<string[]> {
  const descends=buildWalkDescendsV1(),pending=[""],files:string[]=[],directories:string[]=[];
  let visited=0;
  while(pending.length){
    if(invocation)checkScriptInvocation(invocation);
    const batch=pending.splice(-32),listings=await Promise.all(batch.map(async directory=>{
      try{return {directory,entries:await readdir(join(root,directory),{withFileTypes:true})};}
      catch(error){if(directory&&["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))throw Object.assign(Error(`Build directory removed during discovery: ${directory}`),{code:"BUILD_MEMBERSHIP_CHANGED"});throw error;}
    }));
    if(invocation)checkScriptInvocation(invocation);
    for(const {directory,entries}of listings){
      directories.push(directory||".");
      for(const entry of entries){
        if(entry.isSymbolicLink())continue;
        const path=join(directory,entry.name).replaceAll("\\","/");
        if(entry.isDirectory()){if(descends(entry.name,path,directory))pending.push(path);}
        else if(entry.isFile())files.push(path);
      }
    }
    const previous=visited;visited+=batch.length;if(Math.floor(visited/GRAPH_LIMITS_V1.discoveryProgressStride)!==Math.floor(previous/GRAPH_LIMITS_V1.discoveryProgressStride)&&invocation)await advanceScriptInvocation(invocation,"build:discover","running");
  }
  files.sort();inventoryDirectoriesV1.set(files,directories.sort());if(invocation)await advanceScriptInvocation(invocation,"build:discover","complete");return files;
}


/** 🚶️ Whether the workspace walk enters a directory: not a generated, cache or dependency directory, not a build artifact directory of an extension host, not a collection directory below anything but a module member and not the composition directory. The one rule of the walk of every file inventory and of the advance of the membership frontier. */
export function buildWalkDescendsV1():(name:string,path:string,parent:string)=>boolean {
  return ROSTER_RULES_V1.descends;
}


/** 📥️ Expands private input compaction into portable source declarations. */
export function portableInputs(root:string,value:unknown,invocation:ScriptInvocation,control:ResourceLeaseControl):unknown|Promise<unknown> {
  const token=(item:unknown)=>{
    if(!item||typeof item!=="object"||Array.isArray(item))return undefined;
    const object=item as Record<string,unknown>;if(Object.keys(object).length!==1||typeof object.env!=="string")return undefined;
    const match=/^SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})$/u.exec(object.env);
    return match?{key:object.env,digest:match[2]!,directory:match[1]==="SOURCE_CLOSURE"?"build-input-closures":"build-lock-closures"}:undefined;
  };
  captureInferenceInventoryReferencesV1(value);
  const pending:unknown[]=[value],seen=new WeakSet<object>(),tokens=new Map<string,NonNullable<ReturnType<typeof token>>>();let visited=0;
  while(pending.length){
    if(invocation&&visited++%GRAPH_LIMITS_V1.invocationCheckStride===0)checkScriptInvocation(invocation);
    const item=pending.pop();if(!item||typeof item!=="object"||seen.has(item))continue;seen.add(item);
    const compact=token(item);if(compact)tokens.set(compact.key,compact);else for(const child of Object.values(item))if(child&&typeof child==="object")pending.push(child);
  }
  if(!tokens.size)return value;
  const expand=async()=>{
    const entries=[...tokens.values()],expanded=new Map<string,string[]>();
    await withRepositoryBuildGraphConsumerV1(root,control,async(store,actual)=>{
      const clock=createGraphPhaseClockV1({publish:resource=>actual.publish({resource,mode:"shared",elapsedMs:0}),write:writeGraphPhaseRowV1});
      let bytes=0,readRecords=0;const cadence=exactSourceReadPolicyV1.maximumInventoryRecordsPerYield;
      await store.recheck();checkResourceLeaseControl(actual);
      if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","running");
      await clock.begin("expansion",entries.length);
      for(const compact of entries){
        checkScriptInvocation(invocation);checkResourceLeaseControl(actual);
        const known=expandedClosuresV1.get(root+"\0"+compact.key);
        if(known)expanded.set(compact.key,known);
        else{
          const directory=join(store.root,compact.directory),file=readInventoryRecordLeanV1(directory,compact.digest,exactSourceReadPolicyV1.maximumClosureBytes);
          if(file.sha256!==compact.digest)throw Error(`Build input closure content differs: ${compact.key}`);
          bytes+=file.byteLength;if(bytes>exactSourceReadPolicyV1.maximumSourceBytes)throw Error("Build input closure bytes exceeded");
          const list=compact.directory==="build-input-closures"?captureSourceInputInventoryV1(file.value,root).value.files.map(file=>graphStringInternerV1.share("{workspaceRoot}/"+file)):["{workspaceRoot}/Cargo.lock","{workspaceRoot}/"+captureCargoLockInventoryV1(file.value,root).value.manifest];
          expanded.set(compact.key,list);rememberExpandedClosureV1(root+"\0"+compact.key,list);
          rememberInventoryRecordV1(root,directory,compact.digest,file.stamp);
        }
        if(++readRecords%cadence===0){await store.recheck();await actual.yieldContinuation();checkScriptInvocation(invocation);checkResourceLeaseControl(actual);await clock.advance("expansion",readRecords);if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","running");}
      }
      await store.recheck();checkResourceLeaseControl(actual);await clock.advance("expansion",readRecords);await clock.end("expansion",{records:readRecords,bytes});
    });
    const copies=new WeakMap<object,unknown>();visited=0;
    const transform=(item:unknown):unknown=>{
      if(invocation&&visited++%GRAPH_LIMITS_V1.invocationCheckStride===0)checkScriptInvocation(invocation);
      if(!item||typeof item!=="object")return item;
      const compact=token(item);if(compact)return expanded.get(compact.key)!;
      if(copies.has(item))return copies.get(item);
      if(Array.isArray(item)){
        let result:unknown[]|undefined;
        for(let index=0;index<item.length;index++){const child=item[index],next=transform(child),flatten=token(child)!==undefined;if(!result&&(flatten||next!==child))result=item.slice(0,index);if(result){if(flatten)for(const entry of next as unknown[])result.push(entry);else result.push(next);}}
        copies.set(item,result??item);return result??item;
      }
      let result:Record<string,unknown>|undefined;
      for(const[key,child]of Object.entries(item)){const next=transform(child);if(next!==child&&!result)result={...item};if(result)result[key]=next;}
      copies.set(item,result??item);return result??item;
    };
    const result=transform(value);if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","complete");return result;
  };
  return expand();
}


/** 🧳️ Keeps exactly the discovered files whose presence the portable graph consumes, as declared by the build policy. */
export function buildGraphMembersV1(files:readonly string[]):string[] {
  return files.filter(graphMemberPredicateV1());
}


/** 🤔️ Whether one discovered file is a member of the build graph, as declared by the build policy. */
export function graphMemberPredicateV1():(file:string)=>boolean {
  const {basenames,suffixes,lockEcosystems}=graphDeclarationV1("../🔣️policy.json").graphMembers as {basenames:string[];suffixes:string[];lockEcosystems:string[]},names=new Set([...basenames,...lockEcosystems.flatMap(ecosystem=>selectPackageManagerV1(ecosystem).lockFileNames())]);
  return file=>names.has(file.slice(file.lastIndexOf("/")+1))||suffixes.some(suffix=>file.endsWith(suffix));
}


/** 🧾️ Identifies complete admitted physical membership, types and modes independently of implementation bytes. */
export async function buildGraphMembershipV1(root:string,files:string[],invocation?:ScriptInvocation):Promise<string> {
  const records:string[]=[],entries=[...files.map(file=>({file,directory:false})),...(inventoryDirectoriesV1.get(files)??[]).map(file=>({file,directory:true}))].filter(({file})=>![".nxignore","nx.json"].includes(file)).sort((a,b)=>a.file.localeCompare(b.file));
  for(let index=0;index<entries.length;index+=64){
    if(invocation)checkScriptInvocation(invocation);
    records.push(...await Promise.all(entries.slice(index,index+64).map(async({file,directory})=>{let info;try{info=await lstat(join(root,file));}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))throw Object.assign(Error(`Build member removed during discovery: ${file}`),{code:"BUILD_MEMBERSHIP_CHANGED"});throw error;}if(info.isSymbolicLink()||(directory?!info.isDirectory():!info.isFile()))throw Object.assign(Error(`Build membership changed during discovery: ${file}`),{code:"BUILD_MEMBERSHIP_CHANGED"});return JSON.stringify([file,info.isDirectory()?"directory":"file",info.mode]);})));
  }
  return createHash("sha256").update(records.join("\n")).digest("hex");
}


/** 🚪️ Admits content-aware observations through the same owned inference mechanism. */
export async function buildGraphSourceFactsHoldV1(observations:readonly unknown[],invocation?:ScriptInvocation,onStale?:(row:{kind:string;path:string;fact:unknown})=>void):Promise<boolean>{return observationV1.observationsHold(observations,64,new Set(),new Set(),invocation?.control.signal,onStale);}
